use apalis_core::{error::Error, worker::WorkerId};
use apalis_sql::{from_row::SqlRequest, sqlite::SqliteStorage};
use serde::{de::DeserializeOwned, Serialize};

use crate::{BackendExt, JobState, Stat, Worker};

impl<J: 'static + Serialize + DeserializeOwned + Unpin + Send + Sync> BackendExt<J>
    for SqliteStorage<J>
{
    type Request = SqlRequest<J>;
    async fn stats(&self) -> Result<Stat, Error> {
        let fetch_query = "SELECT
                            COUNT(1) FILTER (WHERE status = 'Pending') AS pending,
                            COUNT(1) FILTER (WHERE status = 'Running') AS running,
                            COUNT(1) FILTER (WHERE status = 'Done') AS done,
                            COUNT(1) FILTER (WHERE status = 'Failed') AS failed,
                            COUNT(1) FILTER (WHERE status = 'Killed') AS killed
                        FROM Jobs WHERE job_type = ?";

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
        let fetch_query = "SELECT * FROM Jobs WHERE status = ? AND job_type = ? ORDER BY done_at DESC, run_at DESC LIMIT 10 OFFSET ?";
        let res: Vec<SqlRequest<String>> = sqlx::query_as(fetch_query)
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
            "SELECT id, layers, last_seen FROM Workers WHERE worker_type = ? ORDER BY last_seen DESC LIMIT 20 OFFSET ?";
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
