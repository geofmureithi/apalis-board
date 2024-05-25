use actix::{clock::sleep, Addr};
use actix_web::{get, rt::signal::ctrl_c, web, App, HttpServer};
use apalis::layers::tracing::TraceLayer;
use apalis::prelude::*;
use apalis_redis::RedisStorage;
use backend::{api::ApiBuilder, sse::Broadcaster};
use core::fmt;
use futures::{
    future::{self, BoxFuture},
    FutureExt, TryFutureExt,
};
use serde::{Deserialize, Serialize};
use std::{sync::Mutex, task::Context, task::Poll, time::Duration};
use tower::{Layer, Service};

mod sse {
    use actix_web::{web::*, HttpResponse};
    use backend::sse::Broadcaster;
    use std::sync::Mutex;

    pub async fn new_client(broadcaster: Data<Mutex<Broadcaster>>) -> HttpResponse {
        let rx = broadcaster.lock().unwrap().new_client();

        HttpResponse::Ok()
            .header("content-type", "text/event-stream")
            .no_chunking(0)
            .streaming(rx)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Email {
    pub to: String,
    pub subject: String,
    pub text: String,
}

impl Job for Email {
    const NAME: &'static str = "apalis::Email";
}

pub async fn send_email(job: Email) {
    sleep(Duration::from_secs(10)).await;
    // log::info!("Attempting to send email to {}", job.to);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug,sqlx::query=error");
    env_logger::init();
    let broadcaster = Broadcaster::create();
    let redis = RedisStorage::new(apalis_redis::connect("redis://127.0.0.1/").await.unwrap());

    produce_redis_jobs(redis.clone()).await;
    let worker = Monitor::<TokioExecutor>::new()
        .register_with_count(
            1,
            WorkerBuilder::new("tasty-apple")
                .layer(TraceLayer::new())
                .layer(SseLogLayer::new(broadcaster.clone()))
                .with_storage(redis.clone())
                .build_fn(send_email),
        )
        .run_with_signal(async { ctrl_c().await });
    let http = async {
        HttpServer::new(move || {
            App::new()
                .route("/events", web::get().to(sse::new_client))
                .service(
                    web::scope("/api")
                        .service(ApiBuilder::new().add_storage(redis.clone()).build()),
                )
                .app_data(broadcaster.clone())
        })
        .bind("127.0.0.1:8000")?
        .run()
        .await?;
        Ok(())
    };

    future::try_join(http, worker).await?;

    Ok(())
}

async fn produce_redis_jobs(mut storage: RedisStorage<Email>) {
    for i in 0..10 {
        storage
            .push(Email {
                to: format!("test{i}@example.com"),
                text: "Test background job from apalis".to_string(),
                subject: "Background email job".to_string(),
            })
            .await
            .unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct SseLogLayer {
    target: actix_web::web::Data<Mutex<Broadcaster>>,
}

impl SseLogLayer {
    pub fn new(target: actix_web::web::Data<Mutex<Broadcaster>>) -> Self {
        Self { target }
    }
}

impl<S> Layer<S> for SseLogLayer {
    type Service = SseLogService<S>;

    fn layer(&self, service: S) -> Self::Service {
        SseLogService {
            target: self.target.clone(),
            service,
        }
    }
}

// This service implements the Log behavior
#[derive(Clone)]
pub struct SseLogService<S> {
    target: actix_web::web::Data<Mutex<Broadcaster>>,
    service: S,
}

impl<S, Request> Service<Request> for SseLogService<S>
where
    S: Service<Request>,
    Request: fmt::Debug,
    S::Future: Send + 'static,
    S::Response: Send + 'static,
    S::Error: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<S::Response, S::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let broadcaster = &self.target;
        broadcaster.lock().unwrap().send("Job started");
        let broadcaster = broadcaster.clone();

        self.service
            .call(request)
            .then(|res| async move {
                match res {
                    Ok(r) => {
                        broadcaster
                            .lock()
                            .unwrap()
                            .send("Job completed successfully");
                        Ok(r)
                    }
                    Err(e) => {
                        broadcaster.lock().unwrap().send("Job failed");
                        Err(e)
                    }
                }
            })
            .boxed()
    }
}
