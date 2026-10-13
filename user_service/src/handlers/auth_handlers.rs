use crate::domain::UserProfile;
use crate::service::AuthService;
use axum::{extract::State, Json};
use common::{AppError, AuthUser, SimpleSuccessResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendOtpRequest {
    pub phone_number: String,
}

#[derive(Debug, Serialize)]
pub struct SendOtpResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyOtpRequest {
    pub phone_number: String,
    pub otp: String,
}

pub async fn send_otp_handler(
    State(auth_service): State<Arc<AuthService>>,
    Json(payload): Json<SendOtpRequest>,
) -> Result<Json<SendOtpResponse>, AppError> {
    auth_service.send_otp(&payload.phone_number).await?;
    Ok(Json(SendOtpResponse {
        success: true,
        message: "OTP sent successfully".to_string(),
    }))
}

pub async fn verify_otp_handler(
    State(auth_service): State<Arc<AuthService>>,
    Json(payload): Json<VerifyOtpRequest>,
) -> Result<Json<UserProfile>, AppError> {
    let profile = auth_service
        .verify_otp(&payload.phone_number, &payload.otp)
        .await?;
    Ok(Json(profile))
}

pub async fn logout_handler(
    State(auth_service): State<Arc<AuthService>>,
    AuthUser(phone): AuthUser,
) -> Result<Json<SimpleSuccessResponse>, AppError> {
    auth_service.logout(&phone).await?;
    Ok(Json(SimpleSuccessResponse::with_message("Logged out successfully")))
}
