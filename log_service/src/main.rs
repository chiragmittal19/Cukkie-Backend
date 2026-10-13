use log_service::repository::InMemoryLogRepository;
use log_service::service::LogService;
use log_service::{create_router, AppState};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,log_service=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Initializing Cukkie Log Service...");

    // Clean Architecture Dependency Injection:
    // Today: InMemoryLogRepository.
    // Future: ClickHouseLogRepository or KafkaLogRepository with 0 handler or service changes.
    let log_repo = Arc::new(InMemoryLogRepository::default());
    let log_service = Arc::new(LogService::new(log_repo));

    let state = AppState { log_service };
    let app = create_router(state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8083);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Log Service listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
