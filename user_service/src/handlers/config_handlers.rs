use crate::domain::PreferencesConfig;
use crate::service::ConfigService;
use axum::{extract::State, Json};
use common::{AppError, LanguageHeader};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn get_preferences_config_handler(
    State(config_service): State<Arc<ConfigService>>,
    LanguageHeader(lang): LanguageHeader,
) -> Result<Json<PreferencesConfig>, AppError> {
    let config = config_service.get_preferences_config(lang).await?;
    Ok(Json(config))
}

pub async fn get_app_strings_handler(
    State(config_service): State<Arc<ConfigService>>,
    LanguageHeader(lang): LanguageHeader,
) -> Result<Json<HashMap<String, String>>, AppError> {
    let strings = config_service.get_app_strings(lang).await?;
    Ok(Json(strings))
}
