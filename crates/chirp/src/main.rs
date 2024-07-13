use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use apalis::layers::catch_panic::CatchPanicLayer;
use apalis::layers::tracing::TraceLayer;
use apalis::prelude::{Data, Monitor, WorkerBuilder, WorkerFactoryFn};
use apalis::utils::TokioExecutor;
use apalis_cron::CronStream;
use apalis_redis::RedisStorage;
use apalis_sql::mysql::{MySqlPool, MysqlStorage};
use apalis_sql::postgres::{PgPool, PostgresStorage};
use apalis_sql::sqlite::{SqlitePool, SqliteStorage};
use backend::api::ApiBuilder;
use backend::sse::Broadcaster;
use chrono::{DateTime, Utc};
use clap::Parser;
use figment::providers::{Format, Yaml};
use figment::Figment;
use futures::io::BufReader;
use futures::{future, AsyncBufReadExt, StreamExt};
use processors::docker::run_docker;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;
use std::{process::Stdio, str::FromStr};
use trace::{Subscriber, TaskSpan};
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

mod processors;
mod trace;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum LaunchJob {
    Cron(DateTime<Utc>),
    Custom { value: serde_json::Value },
}
#[derive(Debug, Clone)]
enum StorageType {
    Redis(RedisStorage<LaunchJob>),
    Mysql(MysqlStorage<LaunchJob>),
    Postgres(PostgresStorage<LaunchJob>),
    Sqlite(SqliteStorage<LaunchJob>),
}
#[derive(Deserialize, Clone, Debug)]
enum Source {
    Http {
        backend: Option<String>,
    },
    /// Cron
    Cron(String),
}

#[derive(Deserialize, Debug, Clone)]
struct LaunchConfig {
    jobs: HashMap<String, Launch>,
}

#[derive(Deserialize, Clone, Debug)]
struct Launch {
    // description: Option<String>,
    source: Source,
    task: Task,
    // layers: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
enum Task {
    Command {
        steps: HashMap<String, String>,
        docker: Option<String>, //image
    },
}

impl From<DateTime<Utc>> for LaunchJob {
    fn from(value: DateTime<Utc>) -> Self {
        LaunchJob::Cron(value)
    }
}

async fn spawn_command(cmd: &String) -> anyhow::Result<()> {
    let mut words = shlex::split(cmd).ok_or(anyhow::anyhow!("parsing error"))?;
    let cmd = words.remove(0);
    let mut cmd = async_process::Command::new(cmd);
    for word in words {
        cmd.arg(word);
    }
    let mut child = cmd.stdout(Stdio::piped()).spawn()?;
    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

    while let Some(line) = lines.next().await {
        println!("{:?}", line);
    }
    Ok(())
}

async fn run_task(command: &Launch) -> anyhow::Result<()> {
    match &command.task {
        Task::Command { steps, docker } => match docker {
            Some(image) => {
                run_docker(image, steps).await?;
            }
            None => {
                for (_name, cmd) in steps {
                    spawn_command(cmd).await?;
                }
            }
        },
    };
    Ok(())
}

async fn launch_job(job: LaunchJob, cur: Data<Launch>) {
    info!("Job started {job:?}, {cur:?}");
    let res = run_task(&cur).await;

    if let Err(e) = res {
        tracing::error!("job failed with error: {e}");
    } else {
        info!("Job done");
    };
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: String,
}

async fn new_client(broadcaster: actix_web::web::Data<Mutex<Broadcaster>>) -> impl Responder {
    let rx = broadcaster.lock().unwrap().new_client();

    HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .append_header(("Cache-Control", "no-cache"))
        .append_header(("Connection", "keep-alive"))
        .streaming(rx)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug,bollard::docker=error,sqlx::query=error");
    let broadcaster = Broadcaster::create();
    let line_sub = Subscriber {
        tx: broadcaster.clone(),
    };
    let tracer = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer().with_filter(
                EnvFilter::builder()
                    .parse("debug,bollard::docker=error,sqlx::query=error")
                    .unwrap(),
            ),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .fmt_fields(tracing_subscriber::fmt::format::JsonFields::new())
                .event_format(tracing_subscriber::fmt::format().with_ansi(false).json())
                .with_writer(line_sub)
                .with_filter(
                    EnvFilter::builder()
                        .parse("debug,bollard::docker=error,sqlx::query=error")
                        .unwrap(),
                ),
        );
    tracer.try_init().unwrap();
    let args = Args::parse();
    let config: LaunchConfig = Figment::new()
        .merge(Yaml::file(args.config))
        .extract()
        .unwrap();

    let mut monitor = Monitor::<TokioExecutor>::new();
    let mut exposed: Vec<(String, StorageType)> = vec![];

    for (job, command) in config.jobs.iter() {
        match &command.source {
            Source::Http { backend } => match backend.as_ref().map(|s| s.as_str()) {
                None | Some("default") => {
                    let cfg = apalis_sql::Config::default().set_namespace(job);
                    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

                    SqliteStorage::setup(&pool)
                        .await
                        .expect("unable to run migrations for sqlite");
                    let storage: SqliteStorage<LaunchJob> =
                        SqliteStorage::new_with_config(pool, cfg);
                    exposed.push((job.clone(), StorageType::Sqlite(storage.clone())));
                    monitor = monitor.register_with_count(
                        1,
                        WorkerBuilder::new(&job)
                            .layer(CatchPanicLayer::new())
                            .data(command.clone())
                            .data(config.clone())
                            .layer(TraceLayer::new().make_span_with(TaskSpan::new(&job)))
                            .backend(storage)
                            .build_fn(launch_job),
                    );
                }
                Some(url) if url.starts_with("redis://") => {
                    let conn = apalis_redis::connect(url).await.unwrap();
                    let cfg = apalis_redis::Config::default().set_namespace(job);

                    let redis: RedisStorage<LaunchJob> =
                        RedisStorage::new_with_config(conn.clone(), cfg);
                    exposed.push((job.clone(), StorageType::Redis(redis.clone())));
                    monitor = monitor.register_with_count(
                        1,
                        WorkerBuilder::new(&job)
                            .layer(CatchPanicLayer::new())
                            .data(command.clone())
                            .data(config.clone())
                            .layer(TraceLayer::new().make_span_with(TaskSpan::new(&job)))
                            .backend(redis)
                            .build_fn(launch_job),
                    );
                }
                Some(url) if url.starts_with("mysql://") => {
                    let cfg = apalis_sql::Config::default().set_namespace(job);
                    let pool = MySqlPool::connect(url).await.unwrap();

                    MysqlStorage::setup(&pool)
                        .await
                        .expect("unable to run migrations for mysql");
                    let storage: MysqlStorage<LaunchJob> = MysqlStorage::new_with_config(pool, cfg);
                    exposed.push((job.clone(), StorageType::Mysql(storage.clone())));
                    monitor = monitor.register_with_count(
                        1,
                        WorkerBuilder::new(&job)
                            .layer(CatchPanicLayer::new())
                            .data(command.clone())
                            .data(config.clone())
                            .layer(TraceLayer::new().make_span_with(TaskSpan::new(&job)))
                            .backend(storage)
                            .build_fn(launch_job),
                    );
                }
                Some(url) if url.starts_with("postgresql://") => {
                    let cfg = apalis_sql::Config::default().set_namespace(job);
                    let pool = PgPool::connect(url).await.unwrap();

                    PostgresStorage::setup(&pool)
                        .await
                        .expect("unable to run migrations for postgres");
                    let storage: PostgresStorage<LaunchJob> =
                        PostgresStorage::new_with_config(pool, cfg);
                    exposed.push((job.clone(), StorageType::Postgres(storage.clone())));
                    monitor = monitor.register_with_count(
                        1,
                        WorkerBuilder::new(&job)
                            .layer(CatchPanicLayer::new())
                            .data(command.clone())
                            .data(config.clone())
                            .layer(TraceLayer::new().make_span_with(TaskSpan::new(&job)))
                            .backend(storage)
                            .build_fn(launch_job),
                    );
                }
                Some(url) if url.starts_with("sqlite://") => {
                    let cfg = apalis_sql::Config::default().set_namespace(job);
                    let pool = SqlitePool::connect(url).await.unwrap();

                    SqliteStorage::setup(&pool)
                        .await
                        .expect("unable to run migrations for sqlite");
                    let storage: SqliteStorage<LaunchJob> =
                        SqliteStorage::new_with_config(pool, cfg);
                    exposed.push((job.clone(), StorageType::Sqlite(storage.clone())));
                    monitor = monitor.register_with_count(
                        1,
                        WorkerBuilder::new(&job)
                            .layer(CatchPanicLayer::new())
                            .data(command.clone())
                            .data(config.clone())
                            .layer(TraceLayer::new().make_span_with(TaskSpan::new(&job)))
                            .backend(storage)
                            .build_fn(launch_job),
                    );
                }
                _ => unimplemented!(),
            },
            Source::Cron(cron) => {
                let schedule = apalis_cron::Schedule::from_str(&cron).unwrap();
                let stream = CronStream::new(schedule);
                monitor = monitor.register(
                    WorkerBuilder::new("cron")
                        .layer(CatchPanicLayer::new())
                        .data(command.clone())
                        .data(config.clone())
                        .layer(TraceLayer::new().make_span_with(TaskSpan::new(&job)))
                        .backend(stream)
                        .build_fn(launch_job),
                );
            }
        }
    }
    let http = async {
        HttpServer::new(move || {
            let mut api = ApiBuilder::new();
            for (namespace, storage) in exposed.clone() {
                match storage {
                    StorageType::Redis(redis) => {
                        api = api.add_storage(&redis, &namespace);
                    }
                    StorageType::Sqlite(sqlite) => api = api.add_storage(&sqlite, &namespace),
                    StorageType::Mysql(mysql) => api = api.add_storage(&mysql, &namespace),
                    StorageType::Postgres(pg) => api = api.add_storage(&pg, &namespace),
                }
            }
            let scope = api.build().route("/events", web::get().to(new_client));
            App::new()
                .wrap(Cors::permissive())
                .app_data(broadcaster.clone())
                .service(web::scope("/api/v1").service(scope))
        })
        .bind("127.0.0.1:8000")?
        .run()
        .await?;
        Ok(())
    };

    future::try_join(
        http,
        monitor
            .shutdown_timeout(Duration::from_secs(3))
            .run_with_signal(tokio::signal::ctrl_c()),
    )
    .await?;

    Ok(())
}
