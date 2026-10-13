use crate::domain::{UserPreferences, UserProfile};
use crate::repository::UserRepository;
use common::AppError;
use std::sync::Arc;

pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn get_profile(&self, phone: &str) -> Result<UserProfile, AppError> {
        match self.user_repo.get_by_phone(phone).await? {
            Some(user) => Ok(user),
            None => Ok(UserProfile::new(phone)),
        }
    }

    pub async fn update_preferences(
        &self,
        phone: &str,
        prefs: UserPreferences,
    ) -> Result<UserPreferences, AppError> {
        self.user_repo.update_preferences(phone, prefs).await
    }
}
