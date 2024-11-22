use std::{any::type_name, future::Future, num::TryFromIntError, time::Duration};

#[cfg(feature = "redis")]
pub mod redis;

#[cfg(feature = "sqlite")]
pub mod sqlite;

#[cfg(feature = "postgres")]
pub mod postgres;

#[cfg(feature = "mysql")]
pub mod mysql;

use apalis_core::worker::WorkerId;
use serde::{Deserialize, Serialize};

/// A serializable version of a worker.
#[derive(Debug, Serialize, Deserialize)]
pub struct Worker {
    /// The Worker's Id
    pub worker_id: WorkerId,
    /// Type of task being consumed by the worker, useful for display and filtering
    pub r#type: String,
    /// The type of job stream
    pub source: String,
    /// The layers that were loaded for worker.
    pub layers: Vec<Layer>,
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
    pub pending: usize,
    pub running: usize,
    pub dead: usize,
    pub failed: usize,
    pub success: usize,
}

#[derive(
    Debug, Deserialize, Serialize, Default, strum::Display, strum::EnumString, strum::EnumIter,
)]
pub enum JobState {
    #[default]
    Pending,
    Scheduled,
    Running,
    Dead,
    Failed,
    Success,
}

#[derive(Deserialize, Debug)]
pub struct Filter {
    #[serde(default)]
    pub status: JobState,
    #[serde(default = "default_page")]
    pub page: i32,
}

fn default_page() -> i32 {
    1
}

pub trait BackendExt<T>
where
    Self: Sized,
{
    type Request;
    type Error;
    /// List all Workers that are working on a backend
    fn list_workers(&self) -> impl Future<Output = Result<Vec<Worker>, Self::Error>> + Send;

    /// Returns the counts of jobs in different states
    fn stats(&self) -> impl Future<Output = Result<Stat, Self::Error>> + Send;

    /// Fetch jobs persisted in a backend
    fn list_jobs(
        &self,
        status: &JobState,
        page: i32,
    ) -> impl Future<Output = Result<Vec<Self::Request>, Self::Error>> + Send;
}

#[derive(Debug, Deserialize)]
pub enum Config {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct GetJobsResult<T> {
    pub stats: Stat,
    pub jobs: Vec<T>,
}

#[derive(Debug, thiserror::Error)]
pub enum SqlError {
    #[error("sqlx::Error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("TryFromIntError: {0}")]
    TryFromInt(#[from] TryFromIntError),
}
