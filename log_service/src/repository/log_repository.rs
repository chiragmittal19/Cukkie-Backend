use crate::domain::LogRecord;
use async_trait::async_trait;
use common::AppError;
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
pub trait LogRepository: Send + Sync {
    async fn ingest_batch(&self, records: Vec<LogRecord>) -> Result<usize, AppError>;
    async fn count(&self) -> usize;
}

#[derive(Clone)]
pub struct InMemoryLogRepository {
    buffer: Arc<RwLock<Vec<LogRecord>>>,
    max_capacity: usize,
}

impl Default for InMemoryLogRepository {
    fn default() -> Self {
        Self::new(10_000)
    }
}

impl InMemoryLogRepository {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            buffer: Arc::new(RwLock::new(Vec::new())),
            max_capacity,
        }
    }
}

#[async_trait]
impl LogRepository for InMemoryLogRepository {
    async fn ingest_batch(&self, records: Vec<LogRecord>) -> Result<usize, AppError> {
        let count = records.len();
        let mut write_lock = self.buffer.write().await;

        for record in records {
            tracing::info!(
                target: "log_service",
                type = %record.event_type,
                name = %record.name,
                value = ?record.value,
                params = %record.parameters,
                "📊 Ingested telemetry record"
            );

            if write_lock.len() >= self.max_capacity {
                write_lock.remove(0); // Evict oldest
            }
            write_lock.push(record);
        }

        Ok(count)
    }

    async fn count(&self) -> usize {
        let read_lock = self.buffer.read().await;
        read_lock.len()
    }
}
