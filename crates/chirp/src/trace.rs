use std::{io::LineWriter, sync::Mutex};

use apalis::{layers::tracing::MakeSpan, prelude::*};
use backend::sse::Broadcaster;
use tracing::{Level, Span};
use tracing_subscriber::fmt::MakeWriter;

#[derive(Debug, Clone)]
pub struct TaskSpan {
    level: Level,
    name: String,
}

impl TaskSpan {
    /// Create a new `TaskSpan`.
    pub fn new(name: &str) -> Self {
        Self {
            level: Level::DEBUG,
            name: name.to_string(),
        }
    }
}

impl<B> MakeSpan<B> for TaskSpan {
    fn make_span(&mut self, req: &Request<B>) -> Span {
        let task_id: &TaskId = req.get().unwrap();
        let attempts: Attempt = req.get().cloned().unwrap_or_default();
        let span = Span::current();
        let task = &self.name;
        macro_rules! make_span {
            ($level:expr) => {
                tracing::span!(
                    parent: span,
                    $level,
                    "task",
                    task_type = task.to_string(),
                    task_id = task_id.to_string(),
                    attempt = attempts.current().to_string(),
                )
            };
        }

        match self.level {
            Level::ERROR => {
                make_span!(Level::ERROR)
            }
            Level::WARN => {
                make_span!(Level::WARN)
            }
            Level::INFO => {
                make_span!(Level::INFO)
            }
            Level::DEBUG => {
                make_span!(Level::DEBUG)
            }
            Level::TRACE => {
                make_span!(Level::TRACE)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Subscriber {
    pub tx: actix_web::web::Data<Mutex<Broadcaster>>,
}

impl<'a> MakeWriter<'a> for Subscriber {
    type Writer = LineWriter<Self>;

    fn make_writer(&self) -> Self::Writer {
        LineWriter::new(Self {
            tx: self.tx.clone(),
        })
    }
}

impl std::io::Write for Subscriber {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = buf.len();
        let _ = self
            .tx
            .try_lock()
            .map(|b| b.send(std::str::from_utf8(buf).unwrap_or_default()));
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
