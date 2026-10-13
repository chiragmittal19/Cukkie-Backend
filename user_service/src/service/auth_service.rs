use crate::domain::UserProfile;
use crate::repository::UserRepository;
use common::{create_jwt, AppError, DEFAULT_JWT_SECRET};
use std::sync::Arc;

pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
}

impl AuthService {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn send_otp(&self, phone: &str) -> Result<String, AppError> {
        let phone = phone.trim();
        if phone.is_empty() {
            return Err(AppError::BadRequest("Phone number is required".to_string()));
        }

        // Mock verification: "1234" is the fixed test code
        Ok("1234".to_string())
    }

    pub async fn verify_otp(&self, phone: &str, otp: &str) -> Result<UserProfile, AppError> {
        let phone = phone.trim();
        let otp = otp.trim();

        if phone.is_empty() {
            return Err(AppError::BadRequest("Phone number is required".to_string()));
        }
        if otp != "1234" && otp.len() != 4 {
            return Err(AppError::BadRequest("Invalid verification code".to_string()));
        }

        // Generate JWT token
        let token = create_jwt(phone, DEFAULT_JWT_SECRET, 24 * 30)?;

        let mut user = match self.user_repo.get_by_phone(phone).await? {
            Some(existing) => existing,
            None => UserProfile::new(phone),
        };

        user.is_logged_in = true;
        user.token = Some(token);

        let saved = self.user_repo.save_user(user).await?;
        Ok(saved)
    }

    pub async fn logout(&self, phone: &str) -> Result<(), AppError> {
        self.user_repo.set_logged_out(phone).await
    }
}
