use crate::domain::*;
use crate::service::RecipeService;
use axum::{
    extract::{Query, State},
    Json,
};
use common::{AppError, AuthUser, LanguageHeader, OptionalAuthUser, SimpleSuccessResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct HomeQuery {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub search: Option<String>,
    pub filters: Option<String>,
    pub sort_by: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DishIdQuery {
    pub id: String,
}

pub async fn get_home_handler(
    State(service): State<Arc<RecipeService>>,
    LanguageHeader(lang): LanguageHeader,
    OptionalAuthUser(user): OptionalAuthUser,
    Query(query): Query<HomeQuery>,
) -> Result<Json<HomePayload>, AppError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let filter_query = FilterQuery::from_param(query.filters.as_deref());

    let payload = service
        .get_home_payload(
            lang,
            page,
            limit,
            query.search.as_deref(),
            &filter_query,
            query.sort_by.as_deref(),
            user.as_deref(),
        )
        .await?;

    Ok(Json(payload))
}

pub async fn get_drawer_handler(
    State(service): State<Arc<RecipeService>>,
    LanguageHeader(lang): LanguageHeader,
    OptionalAuthUser(user): OptionalAuthUser,
) -> Result<Json<DrawerPayload>, AppError> {
    let payload = service.get_drawer_payload(lang, user.as_deref()).await?;
    Ok(Json(payload))
}

pub async fn get_dish_detail_handler(
    State(service): State<Arc<RecipeService>>,
    LanguageHeader(lang): LanguageHeader,
    OptionalAuthUser(user): OptionalAuthUser,
    Query(query): Query<DishIdQuery>,
) -> Result<Json<DishDetail>, AppError> {
    let detail = service
        .get_dish_detail(&query.id, lang, user.as_deref())
        .await?;
    Ok(Json(detail))
}

pub async fn toggle_favourite_handler(
    State(service): State<Arc<RecipeService>>,
    AuthUser(user_id): AuthUser,
    Query(query): Query<DishIdQuery>,
) -> Result<Json<ToggleFavouriteResponse>, AppError> {
    let is_fav = service.toggle_favourite(&query.id, &user_id).await?;
    Ok(Json(ToggleFavouriteResponse {
        dish_id: query.id,
        is_favourite: is_fav,
    }))
}

pub async fn get_favourites_handler(
    State(service): State<Arc<RecipeService>>,
    LanguageHeader(lang): LanguageHeader,
    AuthUser(user_id): AuthUser,
) -> Result<Json<Vec<DishSummary>>, AppError> {
    let favs = service.get_favourites(lang, &user_id).await?;
    Ok(Json(favs))
}

pub async fn submit_feedback_handler(
    State(service): State<Arc<RecipeService>>,
    Query(query): Query<DishIdQuery>,
    Json(feedback): Json<FeedbackRequest>,
) -> Result<Json<SimpleSuccessResponse>, AppError> {
    service.save_feedback(&query.id, feedback.responses).await?;
    Ok(Json(SimpleSuccessResponse::with_message(
        "Feedback submitted successfully",
    )))
}

#[derive(Serialize)]
pub struct HealthStatus {
    pub status: &'static str,
    pub service: &'static str,
}

pub async fn health_check_handler() -> Json<HealthStatus> {
    Json(HealthStatus {
        status: "ok",
        service: "recipe_service",
    })
}
