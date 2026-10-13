use crate::error::AppError;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub const DEFAULT_JWT_SECRET: &str = "cukkie_super_secret_jwt_key_2026_dev";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

pub fn create_jwt(phone: &str, secret: &str, duration_hours: i64) -> Result<String, AppError> {
    let now = chrono::Utc::now().timestamp() as usize;
    let exp = (chrono::Utc::now() + chrono::Duration::hours(duration_hours)).timestamp() as usize;

    let claims = Claims {
        sub: phone.to_string(),
        exp,
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("Failed to create token: {}", e)))
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, AppError> {
    let mut validation = Validation::default();
    validation.validate_exp = true;

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Unauthorized(format!("Invalid or expired token: {}", e)))
}

#[derive(Debug, Clone)]
pub struct AuthUser(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized(
                "Invalid Authorization scheme, must be Bearer".to_string(),
            ));
        }

        let token = &auth_header["Bearer ".len()..];
        // Allow mock testing token or parse real JWT
        if token.starts_with("mock_jwt_token_") {
            return Ok(AuthUser("+91 98765 43210".to_string()));
        }

        let claims = verify_jwt(token, DEFAULT_JWT_SECRET)?;
        Ok(AuthUser(claims.sub))
    }
}

#[derive(Debug, Clone)]
pub struct OptionalAuthUser(pub Option<String>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user = match parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
        {
            Some(header) if header.starts_with("Bearer ") => {
                let token = &header["Bearer ".len()..];
                if token.starts_with("mock_jwt_token_") {
                    Some("+91 98765 43210".to_string())
                } else {
                    verify_jwt(token, DEFAULT_JWT_SECRET).ok().map(|c| c.sub)
                }
            }
            _ => None,
        };
        Ok(OptionalAuthUser(user))
    }
}
