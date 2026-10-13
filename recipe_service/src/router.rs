use crate::handlers::{
    get_dish_detail_handler, get_drawer_handler, get_favourites_handler, get_home_handler,
    health_check_handler, submit_feedback_handler, toggle_favourite_handler,
};
use crate::service::RecipeService;
use axum::{
    routing::{get, post},
    Router,
};
use common::create_cors_layer;
use std::sync::Arc;

pub struct AppState {
    pub recipe_service: Arc<RecipeService>,
}

pub fn create_router(state: AppState) -> Router {
    let recipe_routes = Router::new()
        .route("/home", get(get_home_handler))
        .route("/drawer", get(get_drawer_handler))
        .route("/detail", get(get_dish_detail_handler))
        .route("/favourite", post(toggle_favourite_handler))
        .route("/favourites", get(get_favourites_handler))
        .route("/feedback", post(submit_feedback_handler))
        .with_state(state.recipe_service);

    Router::new()
        .route("/health", get(health_check_handler))
        .nest("/api/v1/recipes", recipe_routes)
        .layer(create_cors_layer())
}
