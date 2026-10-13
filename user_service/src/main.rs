use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use user_service::repository::{InMemoryConfigRepository, InMemoryUserRepository};
use user_service::service::{AuthService, ConfigService, UserService};
use user_service::{create_router, AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,user_service=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Initializing Cukkie User Service...");

    // Clean Architecture Dependency Injection:
    // Today: InMemoryUserRepository and InMemoryConfigRepository.
    // Future: Swap with PostgresUserRepository with 0 handler or service changes.
    let user_repo = Arc::new(InMemoryUserRepository::new());
    let config_repo = Arc::new(InMemoryConfigRepository::new());

    let auth_service = Arc::new(AuthService::new(user_repo.clone()));
    let user_service = Arc::new(UserService::new(user_repo.clone()));
    let config_service = Arc::new(ConfigService::new(config_repo.clone()));

    let state = AppState {
        auth_service,
        user_service,
        config_service,
    };

    let app = create_router(state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8081);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("User Service listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
