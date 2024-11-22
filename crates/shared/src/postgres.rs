use crate::{BackendExt, JobState, SqlError, Stat, Worker};
use apalis_core::request::Parts;
use apalis_core::Codec;
use apalis_core::{codec::json::JsonCodec, request::Request, worker::WorkerId};
use apalis_sql::{context::SqlContext, from_row::SqlRequest, postgres::PostgresStorage};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

impl<J: 'static + Serialize + DeserializeOwned + Unpin + Send + Sync> BackendExt<J>
    for PostgresStorage<J>
{
    type Request = Request<J, Parts<SqlContext>>;
    type Error = SqlError;
    async fn stats(&self) -> Result<Stat, Self::Error> {
        let fetch_query = "SELECT
                            COUNT(1) FILTER (WHERE status = 'Pending') AS pending,
                            COUNT(1) FILTER (WHERE status = 'Running') AS running,
                            COUNT(1) FILTER (WHERE status = 'Done') AS done,
                            COUNT(1) FILTER (WHERE status = 'Retry') AS retry,
                            COUNT(1) FILTER (WHERE status = 'Failed') AS failed,
                            COUNT(1) FILTER (WHERE status = 'Killed') AS killed
                        FROM apalis.jobs WHERE job_type = $1";

        let res: (i64, i64, i64, i64, i64, i64) = sqlx::query_as(fetch_query)
            .bind(self.config().namespace())
            .fetch_one(self.pool())
            .await?;

        Ok(Stat {
            pending: res.0.try_into()?,
            running: res.1.try_into()?,
            dead: res.4.try_into()?,
            failed: res.3.try_into()?,
            success: res.2.try_into()?,
        })
    }

    async fn list_jobs(
        &self,
        status: &JobState,
        page: i32,
    ) -> Result<Vec<Self::Request>, Self::Error> {
        let status = status.to_string();
        let fetch_query = "SELECT * FROM apalis.jobs WHERE status = $1 AND job_type = $2 ORDER BY done_at DESC, run_at DESC LIMIT 10 OFFSET $3";
        let res: Vec<SqlRequest<serde_json::Value>> = sqlx::query_as(fetch_query)
            .bind(status)
            .bind(self.config().namespace())
            .bind(((page - 1) * 10).to_string())
            .fetch_all(self.pool())
            .await?;
        Ok(res
            .into_iter()
            .map(|j| {
                let (req, ctx) = j.req.take_parts();
                let req = JsonCodec::<Value>::decode(req).unwrap();
                Request::new_with_ctx(req, ctx)
            })
            .collect())
    }

    async fn list_workers(&self) -> Result<Vec<Worker>, Self::Error> {
        let fetch_query =
            "SELECT id, layers, last_seen FROM apalis.workers WHERE worker_type = $1 ORDER BY last_seen DESC LIMIT 20 OFFSET $2";
        let res: Vec<(String, String, i64)> = sqlx::query_as(fetch_query)
            .bind(self.config().namespace())
            .bind(0)
            .fetch_all(self.pool())
            .await?;
        Ok(res
            .into_iter()
            .map(|w| Worker::new::<Self>(WorkerId::new(w.0), w.1))
            .collect())
    }
}
