use std::{collections::HashSet, fmt::Display};

use actix_web::{web, HttpResponse, Scope};
use apalis_core::storage::{Job, Storage};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use shared::{BackendExt, JobState, Stat};

#[derive(Deserialize, Debug)]
struct Filter {
    #[serde(default)]
    status: JobState,
    #[serde(default = "default_page")]
    page: i32,
}

fn default_page() -> i32 {
    1
}

pub struct ApiBuilder {
    scope: Scope,
    list: HashSet<String>,
}

impl ApiBuilder {
    pub fn add_storage<J, S>(mut self, storage: S) -> Self
    where
        J: Job + Serialize + DeserializeOwned + 'static,
        S: BackendExt<J> + Clone,
        S: Storage<Job = J>,
        S: 'static + Send,
        S::Identifier: Display + DeserializeOwned,
        S::Error: Display,
    {
        let name = J::NAME.to_string();
        self.list.insert(name);

        Self {
            scope: self.scope.service(
                Scope::new(J::NAME)
                    .app_data(web::Data::new(storage))
                    .route("", web::get().to(get_jobs::<J, S>)) // Fetch jobs in queue
                    .route("/workers", web::get().to(get_workers::<J, S>)) // Fetch jobs in queue
                    .route("/job", web::put().to(push_job::<J, S>)) // Allow add jobs via api
                    .route("/job/{job_id}", web::get().to(get_job::<J, S>)), // Allow fetch specific job
            ),
            list: self.list,
        }
    }

    pub fn build(self) -> Scope {
        async fn fetch_queues(queues: web::Data<HashSet<String>>) -> HttpResponse {
            HttpResponse::Ok().json(queues)
        }

        self.scope
            .app_data(web::Data::new(self.list))
            .route("", web::get().to(fetch_queues))
    }

    pub fn new() -> Self {
        Self {
            scope: Scope::new("backend"),
            list: HashSet::new(),
        }
    }
}

async fn push_job<J, S>(job: web::Json<J>, storage: web::Data<S>) -> HttpResponse
where
    J: Job + Serialize + DeserializeOwned + 'static,
    S: Storage<Job = J> + Clone,
    S::Identifier: Display,
    S::Error: Display,
{
    let storage = &*storage.into_inner();
    let mut storage = storage.clone();
    let res = storage.push(job.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Ok().body(format!("Job with ID [{id}] added to queue")),
        Err(e) => HttpResponse::InternalServerError().body(format!("{e}")),
    }
}

async fn get_jobs<J, S>(storage: web::Data<S>, filter: web::Query<Filter>) -> HttpResponse
where
    J: Job + Serialize + DeserializeOwned + 'static,
    S: Storage<Job = J> + BackendExt<J> + Send,
{
    dbg!(&filter);
    // TODO: fix unwrap
    let stats = storage.stats().await.unwrap_or_default();
    let jobs = storage
        .list_jobs(&filter.status, filter.page)
        .await
        .unwrap();
    #[derive(Debug, Serialize)]
    struct GetJobsResult<T> {
        stats: Stat,
        jobs: Vec<T>,
    }

    HttpResponse::Ok().json(GetJobsResult { stats, jobs })
}

async fn get_workers<J, S>(storage: web::Data<S>) -> HttpResponse
where
    J: Job + Serialize + DeserializeOwned + 'static,
    S: Storage<Job = J> + BackendExt<J>,
{
    let workers = storage.list_workers().await;
    match workers {
        Ok(workers) => HttpResponse::Ok().json(workers),
        Err(e) => HttpResponse::InternalServerError().body(format!("{e}")),
    }
}

async fn get_job<J, S>(job_id: web::Path<S::Identifier>, storage: web::Data<S>) -> HttpResponse
where
    J: Job + Serialize + DeserializeOwned + 'static,
    S: Storage<Job = J> + 'static,
    S::Error: Display,
{
    let res = storage.fetch_by_id(&*job_id).await;
    match res {
        Ok(Some(job)) => HttpResponse::Ok().json(job),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("{e}")),
    }
}
