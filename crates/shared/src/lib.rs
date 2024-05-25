use std::{any::type_name, future::Future, path::PathBuf, time::Duration};

// #[cfg(feature = "redis")]
pub mod redis;

use apalis_core::{error::Error, request::Request, worker::WorkerId};
use serde::{Deserialize, Serialize};

/// A serializable version of a worker.
#[derive(Debug, Serialize, Deserialize)]
pub struct Worker {
    /// The Worker's Id
    worker_id: WorkerId,
    /// Type of task being consumed by the worker, useful for display and filtering
    r#type: String,
    /// The type of job stream
    source: String,
    /// The layers that were loaded for worker.
    layers: Vec<Layer>,
    // / The last time the worker was seen. Some sources use keep alive.
    // last_seen: Timestamp,
}
impl Worker {
    pub fn new<S>(worker_id: WorkerId, r#type: String) -> Self {
        Self {
            worker_id,
            r#type,
            source: type_name::<S>().to_string(),
            layers: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Stat {
    pending: usize,
    running: usize,
    dead: usize,
    failed: usize,
    success: usize,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum JobState {
    #[default]
    Pending,
    Running,
    Dead,
    Failed,
    Success,
}

pub trait BackendExt<T>
where
    Self: Sized,
{
    /// List all Workers that are working on a backend
    fn list_workers(&self) -> impl Future<Output = Result<Vec<Worker>, Error>> + Send;

    /// Returns the counts of jobs in different states
    fn stats(&self) -> impl Future<Output = Result<Stat, Error>> + Send;

    /// Fetch jobs persisted in a backend
    fn list_jobs(
        &self,
        status: &JobState,
        page: i32,
    ) -> impl Future<Output = Result<Vec<Request<T>>, Error>> + Send;
}

#[derive(Debug, Deserialize)]
pub enum Config {
    Full(FullConfig),
    Board(BoardConfig),
}
#[derive(Debug, Deserialize)]
pub enum BoardConfig {
    Redis(String),
    Postgres(String),
    Mysql(String),
    Sqlite(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Layer {
    Retry { retries: u64 },
    Timeout { duration: Duration },
    LoadShed,
    RateLimit { num: u64, per: Duration },
    ConcurrencyLimit { max: usize },
    Buffer { bound: usize },
    Sentry { dsn: usize },
    Prometheus,
}

#[derive(Debug, Deserialize)]
pub struct FullConfig {
    worker: String,
    instances: usize,
    layers: Vec<Layer>,
    hurl: PathBuf,
    task_type: String,
}
