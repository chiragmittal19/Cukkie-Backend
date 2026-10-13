use crate::domain::{UserPreferences, UserProfile};
use async_trait::async_trait;
use common::AppError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_by_phone(&self, phone: &str) -> Result<Option<UserProfile>, AppError>;
    async fn save_user(&self, user: UserProfile) -> Result<UserProfile, AppError>;
    async fn update_preferences(
        &self,
        phone: &str,
        prefs: UserPreferences,
    ) -> Result<UserPreferences, AppError>;
    async fn set_logged_out(&self, phone: &str) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct InMemoryUserRepository {
    users: Arc<RwLock<HashMap<String, UserProfile>>>,
}

impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        // Seed initial demo profile
        let demo_phone = "+91 98765 43210".to_string();
        map.insert(
            demo_phone.clone(),
            UserProfile {
                phone_number: demo_phone,
                token: None,
                is_logged_in: true,
                preferences: UserPreferences {
                    cuisines: vec!["North Indian".to_string(), "Maharashtrian".to_string()],
                    dietary_choices: vec!["Vegetarian".to_string()],
                    spice_level: "Medium".to_string(),
                    sweetness_level: "Balanced".to_string(),
                    has_saved_preferences: true,
                },
            },
        );

        Self {
            users: Arc::new(RwLock::new(map)),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn get_by_phone(&self, phone: &str) -> Result<Option<UserProfile>, AppError> {
        let read_lock = self.users.read().await;
        Ok(read_lock.get(phone).cloned())
    }

    async fn save_user(&self, user: UserProfile) -> Result<UserProfile, AppError> {
        let mut write_lock = self.users.write().await;
        write_lock.insert(user.phone_number.clone(), user.clone());
        Ok(user)
    }

    async fn update_preferences(
        &self,
        phone: &str,
        mut prefs: UserPreferences,
    ) -> Result<UserPreferences, AppError> {
        prefs.has_saved_preferences = true;
        let mut write_lock = self.users.write().await;
        if let Some(user) = write_lock.get_mut(phone) {
            user.preferences = prefs.clone();
            Ok(prefs)
        } else {
            // Create user if not present
            let new_user = UserProfile {
                phone_number: phone.to_string(),
                token: None,
                is_logged_in: true,
                preferences: prefs.clone(),
            };
            write_lock.insert(phone.to_string(), new_user);
            Ok(prefs)
        }
    }

    async fn set_logged_out(&self, phone: &str) -> Result<(), AppError> {
        let mut write_lock = self.users.write().await;
        if let Some(user) = write_lock.get_mut(phone) {
            user.is_logged_in = false;
            user.token = None;
        }
        Ok(())
    }
}
