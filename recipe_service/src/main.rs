use recipe_service::repository::InMemoryRecipeRepository;
use recipe_service::service::RecipeService;
use recipe_service::{create_router, AppState};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,recipe_service=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Initializing Cukkie Recipe Service...");

    // Clean Architecture Dependency Injection:
    // Today: InMemoryRecipeRepository with mock data.
    // Future: PostgresRecipeRepository with 0 handler or service changes.
    let recipe_repo = Arc::new(InMemoryRecipeRepository::new());
    let recipe_service = Arc::new(RecipeService::new(recipe_repo));

    let state = AppState { recipe_service };
    let app = create_router(state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8082);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Recipe Service listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
