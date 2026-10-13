use crate::domain::{LogBatchRequest, LogBatchResponse};
use crate::service::LogService;
use axum::{extract::State, Json};
use common::AppError;
use serde::Serialize;
use std::sync::Arc;

pub async fn ingest_logs_handler(
    State(service): State<Arc<LogService>>,
    Json(payload): Json<LogBatchRequest>,
) -> Result<Json<LogBatchResponse>, AppError> {
    let count = service.ingest_batch(payload.events).await?;
    Ok(Json(LogBatchResponse {
        success: true,
        ingested: count,
    }))
}

#[derive(Serialize)]
pub struct HealthStatus {
    pub status: &'static str,
    pub service: &'static str,
}

pub async fn health_check_handler() -> Json<HealthStatus> {
    Json(HealthStatus {
        status: "ok",
        service: "log_service",
    })
}
