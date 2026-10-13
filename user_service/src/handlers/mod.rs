pub mod auth_handlers;
pub mod config_handlers;
pub mod user_handlers;

pub use auth_handlers::*;
pub use config_handlers::*;
pub use user_handlers::*;

use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthStatus {
    pub status: &'static str,
    pub service: &'static str,
}

pub async fn health_check_handler() -> Json<HealthStatus> {
    Json(HealthStatus {
        status: "ok",
        service: "user_service",
    })
}
