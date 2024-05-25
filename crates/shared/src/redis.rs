use crate::{BackendExt, JobState, Stat, Worker};
use apalis_core::{error::Error, request::Request, storage::Job, worker::WorkerId, Codec};
use apalis_redis::RedisCodec;
use apalis_redis::RedisStorage;
use redis::{ErrorKind, Value};
use serde::{de::DeserializeOwned, Serialize};

impl<T> BackendExt<T> for RedisStorage<T>
where
    T: 'static + Job + Serialize + DeserializeOwned + Send + Unpin + Sync,
{
    async fn stats(&self) -> Result<Stat, Error> {
        let mut conn = self.get_connection();
        let queue = self.get_queue();
        let script = r#"
            local job_data_hash = KEYS[1]
            local pending_jobs_set = KEYS[2]
            local running_jobs_set = KEYS[3]
            local dead_jobs_set = KEYS[4]
            local failed_jobs_set = KEYS[5]
            local success_jobs_set = KEYS[6]

            local pending_count = redis.call('SCARD', pending_jobs_set)
            local running_count = redis.call('SCARD', running_jobs_set)
            local dead_count = redis.call('SCARD', dead_jobs_set)
            local failed_count = redis.call('SCARD', failed_jobs_set)
            local success_count = redis.call('SCARD', success_jobs_set)

            return {pending_count, running_count, dead_count, failed_count, success_count}
    "#;

        let keys = vec![
            queue.job_data_hash.to_string(), // Assuming this is the main data hash, adjust if necessary
            queue.inflight_jobs_set.to_string(),
            queue.active_jobs_list.to_string(),
            queue.dead_jobs_set.to_string(),
            queue.failed_jobs_set.to_string(),
            queue.done_jobs_set.to_string(),
        ];

        let results: Vec<usize> = redis::cmd("EVAL")
            .arg(script)
            .arg(keys.len().to_string())
            .arg(keys)
            .query_async(&mut conn)
            .await
            .map_err(|e| Error::Failed(Box::new(e)))?;

        Ok(Stat {
            pending: results[0],
            running: results[1],
            dead: results[2],
            failed: results[3],
            success: results[4],
        })
    }
    async fn list_jobs(&self, status: &JobState, page: i32) -> Result<Vec<Request<T>>, Error> {
        let mut conn = self.get_connection();
        let queue = self.get_queue();
        match status {
            JobState::Pending => {
                let active_jobs_list = &queue.active_jobs_list;
                let job_data_hash = &queue.job_data_hash;
                let ids: Vec<String> = redis::cmd("LRANGE")
                    .arg(active_jobs_list)
                    .arg(((page - 1) * 10).to_string())
                    .arg((page * 10).to_string())
                    .query_async(&mut conn)
                    .await
                    .map_err(|e| Error::Failed(Box::new(e)))?;

                if ids.is_empty() {
                    return Ok(Vec::new());
                }
                let data: Option<Value> = redis::cmd("HMGET")
                    .arg(job_data_hash)
                    .arg(&ids)
                    .query_async(&mut conn)
                    .await
                    .map_err(|e| Error::Failed(Box::new(e)))?;

                let jobs: Vec<Request<T>> =
                    deserialize_multiple_jobs(data.as_ref(), self.get_codec()).unwrap();
                Ok(jobs)
            }
            JobState::Running => {
                let consumers_set = &queue.consumers_set;
                let job_data_hash = &queue.job_data_hash;
                let workers: Vec<String> = redis::cmd("ZRANGE")
                    .arg(consumers_set)
                    .arg("0")
                    .arg("-1")
                    .query_async(&mut conn)
                    .await
                    .map_err(|e| Error::Failed(Box::new(e)))?;

                if workers.is_empty() {
                    return Ok(Vec::new());
                }
                let mut all_jobs = Vec::new();
                for worker in workers {
                    let ids: Vec<String> = redis::cmd("SMEMBERS")
                        .arg(&worker)
                        .query_async(&mut conn)
                        .await
                        .map_err(|e| Error::Failed(Box::new(e)))?;

                    if ids.is_empty() {
                        continue;
                    };
                    let data: Option<Value> = redis::cmd("HMGET")
                        .arg(job_data_hash.clone())
                        .arg(&ids)
                        .query_async(&mut conn)
                        .await
                        .map_err(|e| Error::Failed(Box::new(e)))?;

                    let jobs: Vec<Request<T>> =
                        deserialize_multiple_jobs(data.as_ref(), self.get_codec()).unwrap();
                    all_jobs.extend(jobs);
                }

                Ok(all_jobs)
            }
            JobState::Success => {
                let done_jobs_set = &queue.done_jobs_set;
                let job_data_hash = &queue.job_data_hash;
                let ids: Vec<String> = redis::cmd("ZRANGE")
                    .arg(done_jobs_set)
                    .arg(((page - 1) * 10).to_string())
                    .arg((page * 10).to_string())
                    .query_async(&mut conn)
                    .await
                    .map_err(|e| Error::Failed(Box::new(e)))?;

                if ids.is_empty() {
                    return Ok(Vec::new());
                }
                let data: Option<Value> = redis::cmd("HMGET")
                    .arg(job_data_hash)
                    .arg(&ids)
                    .query_async(&mut conn)
                    .await
                    .map_err(|e| Error::Failed(Box::new(e)))?;

                let jobs: Vec<Request<T>> =
                    deserialize_multiple_jobs(data.as_ref(), self.get_codec()).unwrap();
                Ok(jobs)
            }
            // JobState::Retry => Ok(Vec::new()),
            JobState::Failed => {
                let failed_jobs_set = &queue.failed_jobs_set;
                let job_data_hash = &queue.job_data_hash;
                let ids: Vec<String> = redis::cmd("ZRANGE")
                    .arg(failed_jobs_set)
                    .arg(((page - 1) * 10).to_string())
                    .arg((page * 10).to_string())
                    .query_async(&mut conn)
                    .await
                    .map_err(|e| Error::Failed(Box::new(e)))?;
                if ids.is_empty() {
                    return Ok(Vec::new());
                }
                let data: Option<Value> = redis::cmd("HMGET")
                    .arg(job_data_hash)
                    .arg(&ids)
                    .query_async(&mut conn)
                    .await
                    .map_err(|e| Error::Failed(Box::new(e)))?;
                let jobs: Vec<Request<T>> =
                    deserialize_multiple_jobs(data.as_ref(), self.get_codec()).unwrap();
                Ok(jobs)
            }
            JobState::Dead => {
                let dead_jobs_set = &queue.dead_jobs_set;
                let job_data_hash = &queue.job_data_hash;
                let ids: Vec<String> = redis::cmd("ZRANGE")
                    .arg(dead_jobs_set)
                    .arg(((page - 1) * 10).to_string())
                    .arg((page * 10).to_string())
                    .query_async(&mut conn)
                    .await
                    .map_err(|e| Error::Failed(Box::new(e)))?;

                if ids.is_empty() {
                    return Ok(Vec::new());
                }
                let data: Option<Value> = redis::cmd("HMGET")
                    .arg(job_data_hash)
                    .arg(&ids)
                    .query_async(&mut conn)
                    .await
                    .map_err(|e| Error::Failed(Box::new(e)))?;

                let jobs: Vec<Request<T>> =
                    deserialize_multiple_jobs(data.as_ref(), self.get_codec()).unwrap();
                Ok(jobs)
            }
        }
    }
    async fn list_workers(&self) -> Result<Vec<Worker>, Error> {
        let queue = self.get_queue();
        let consumers_set = &queue.consumers_set;
        let mut conn = self.get_connection();
        let workers: Vec<String> = redis::cmd("ZRANGE")
            .arg(consumers_set)
            .arg("0")
            .arg("-1")
            .query_async(&mut conn)
            .await
            .map_err(|e| Error::Failed(Box::new(e)))?;
        Ok(workers
            .into_iter()
            .map(|w| {
                Worker::new::<Self>(
                    WorkerId::new(w.replace(&format!("{}:", &queue.inflight_jobs_set), "")),
                    "".to_string(),
                )
            })
            .collect())
    }
}

fn deserialize_multiple_jobs<T>(
    job: Option<&Value>,
    codec: &RedisCodec<T>,
) -> Option<Vec<Request<T>>>
where
    T: DeserializeOwned,
{
    let jobs = match job {
        None => None,
        Some(Value::Bulk(val)) => Some(val),
        _ => {
            // error!(
            //     "Decoding Message Failed: {:?}",
            //     "unknown result type for next message"
            // );
            None
        }
    };

    jobs.map(|values| {
        values
            .iter()
            .filter_map(|v| match v {
                Value::Data(data) => {
                    let inner = codec
                        .decode(data)
                        .map_err(|e| (ErrorKind::IoError, "Decode error", e.to_string()))
                        .unwrap();
                    Some(inner.into())
                }
                _ => None,
            })
            .collect()
    })
}
