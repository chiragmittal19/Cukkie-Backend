use crate::handlers::{health_check_handler, ingest_logs_handler};
use crate::service::LogService;
use axum::{
    routing::{get, post},
    Router,
};
use common::create_cors_layer;
use std::sync::Arc;

pub struct AppState {
    pub log_service: Arc<LogService>,
}

pub fn create_router(state: AppState) -> Router {
    let log_routes = Router::new()
        .route("/", post(ingest_logs_handler))
        .with_state(state.log_service);

    Router::new()
        .route("/health", get(health_check_handler))
        .nest("/api/v1/logs", log_routes)
        .layer(create_cors_layer())
}
