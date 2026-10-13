use crate::domain::{UserPreferences, UserProfile};
use crate::service::UserService;
use axum::{extract::State, Json};
use common::{AppError, AuthUser};
use std::sync::Arc;

pub async fn get_profile_handler(
    State(user_service): State<Arc<UserService>>,
    AuthUser(phone): AuthUser,
) -> Result<Json<UserProfile>, AppError> {
    let profile = user_service.get_profile(&phone).await?;
    Ok(Json(profile))
}

pub async fn update_preferences_handler(
    State(user_service): State<Arc<UserService>>,
    AuthUser(phone): AuthUser,
    Json(prefs): Json<UserPreferences>,
) -> Result<Json<UserPreferences>, AppError> {
    let updated = user_service.update_preferences(&phone, prefs).await?;
    Ok(Json(updated))
}
