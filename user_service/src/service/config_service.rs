use crate::domain::PreferencesConfig;
use crate::repository::ConfigRepository;
use common::{AppError, Language};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ConfigService {
    config_repo: Arc<dyn ConfigRepository>,
}

impl ConfigService {
    pub fn new(config_repo: Arc<dyn ConfigRepository>) -> Self {
        Self { config_repo }
    }

    pub async fn get_preferences_config(&self, lang: Language) -> Result<PreferencesConfig, AppError> {
        self.config_repo.get_preferences_config(lang).await
    }

    pub async fn get_app_strings(&self, lang: Language) -> Result<HashMap<String, String>, AppError> {
        self.config_repo.get_app_strings(lang).await
    }
}
