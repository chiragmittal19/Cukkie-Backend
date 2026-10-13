use crate::handlers::{
    get_app_strings_handler, get_preferences_config_handler, get_profile_handler,
    health_check_handler, logout_handler, send_otp_handler, update_preferences_handler,
    verify_otp_handler,
};
use crate::service::{AuthService, ConfigService, UserService};
use axum::{
    routing::{get, post, put},
    Router,
};
use common::create_cors_layer;
use std::sync::Arc;

pub struct AppState {
    pub auth_service: Arc<AuthService>,
    pub user_service: Arc<UserService>,
    pub config_service: Arc<ConfigService>,
}

pub fn create_router(state: AppState) -> Router {
    let auth_routes = Router::new()
        .route("/send-otp", post(send_otp_handler))
        .route("/verify-otp", post(verify_otp_handler))
        .route("/logout", post(logout_handler))
        .with_state(state.auth_service);

    let user_routes = Router::new()
        .route("/profile", get(get_profile_handler))
        .route("/preferences", put(update_preferences_handler))
        .with_state(state.user_service);

    let config_routes = Router::new()
        .route("/preferences", get(get_preferences_config_handler))
        .route("/strings", get(get_app_strings_handler))
        .with_state(state.config_service);

    Router::new()
        .route("/health", get(health_check_handler))
        .nest("/api/v1/auth", auth_routes)
        .nest("/api/v1/user", user_routes)
        .nest("/api/v1/config", config_routes)
        .layer(create_cors_layer())
}
