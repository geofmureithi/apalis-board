use apalis_core::{error::Error, worker::WorkerId};
use apalis_sql::{from_row::SqlRequest, mysql::MysqlStorage};
use serde::{de::DeserializeOwned, Serialize};

use crate::{BackendExt, JobState, Stat, Worker};

impl<J: 'static + Serialize + DeserializeOwned + Unpin + Send + Sync> BackendExt<J>
    for MysqlStorage<J>
{
    type Request = SqlRequest<J>;
    async fn stats(&self) -> Result<Stat, Error> {
        let fetch_query = "SELECT
            COUNT(CASE WHEN status = 'Pending' THEN 1 END) AS pending,
            COUNT(CASE WHEN status = 'Running' THEN 1 END) AS running,
            COUNT(CASE WHEN status = 'Done' THEN 1 END) AS done,
            COUNT(CASE WHEN status = 'Retry' THEN 1 END) AS retry,
            COUNT(CASE WHEN status = 'Failed' THEN 1 END) AS failed,
            COUNT(CASE WHEN status = 'Killed' THEN 1 END) AS killed
        FROM jobs WHERE job_type = ?";

        let res: (i64, i64, i64, i64, i64, i64) = sqlx::query_as(fetch_query)
            .bind("")
            .fetch_one(self.pool())
            .await
            .map_err(|e| Error::SourceError(Box::new(e)))?;

        Ok(Stat {
            pending: res
                .0
                .try_into()
                .map_err(|e| Error::SourceError(Box::new(e)))?,
            running: res
                .1
                .try_into()
                .map_err(|e| Error::SourceError(Box::new(e)))?,
            dead: res
                .4
                .try_into()
                .map_err(|e| Error::SourceError(Box::new(e)))?,
            failed: res
                .3
                .try_into()
                .map_err(|e| Error::SourceError(Box::new(e)))?,
            success: res
                .2
                .try_into()
                .map_err(|e| Error::SourceError(Box::new(e)))?,
        })
    }

    async fn list_jobs(&self, status: &JobState, page: i32) -> Result<Vec<SqlRequest<J>>, Error> {
        let status = status.to_string();
        let fetch_query = "SELECT * FROM jobs WHERE status = ? AND job_type = ? ORDER BY done_at DESC, run_at DESC LIMIT 10 OFFSET ?";
        let res: Vec<SqlRequest<serde_json::Value>> = sqlx::query_as(fetch_query)
            .bind(status)
            .bind(self.get_config().namespace())
            .bind(((page - 1) * 10).to_string())
            .fetch_all(self.pool())
            .await
            .map_err(|e| Error::SourceError(Box::new(e)))?;
        Ok(res
            .into_iter()
            .map(|j| {
                let (req, ctx) = j.into_tuple();
                let req = self.codec().decode(&req).unwrap();
                SqlRequest::new(req, ctx)
            })
            .collect())
    }

    async fn list_workers(&self) -> Result<Vec<Worker>, Error> {
        let fetch_query =
            "SELECT id, layers, last_seen FROM workers WHERE worker_type = ? ORDER BY last_seen DESC LIMIT 20 OFFSET ?";
        let res: Vec<(String, String, i64)> = sqlx::query_as(fetch_query)
            .bind(self.get_config().namespace())
            .bind(0)
            .fetch_all(self.pool())
            .await
            .map_err(|e| Error::SourceError(Box::from(e)))?;
        Ok(res
            .into_iter()
            .map(|w| Worker::new::<Self>(WorkerId::new(w.0), w.1))
            .collect())
    }
}
