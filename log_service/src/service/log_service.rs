use crate::domain::LogRecord;
use crate::repository::LogRepository;
use common::AppError;
use std::sync::Arc;

pub struct LogService {
    repo: Arc<dyn LogRepository>,
}

impl LogService {
    pub fn new(repo: Arc<dyn LogRepository>) -> Self {
        Self { repo }
    }

    pub async fn ingest_batch(&self, records: Vec<LogRecord>) -> Result<usize, AppError> {
        self.repo.ingest_batch(records).await
    }
}
