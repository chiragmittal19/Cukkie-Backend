use crate::domain::*;
use crate::repository::RecipeRepository;
use common::{AppError, Language};
use std::sync::Arc;

pub struct RecipeService {
    repo: Arc<dyn RecipeRepository>,
}

impl RecipeService {
    pub fn new(repo: Arc<dyn RecipeRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_home_payload(
        &self,
        lang: Language,
        page: usize,
        limit: usize,
        search: Option<&str>,
        filters: &FilterQuery,
        sort_by: Option<&str>,
        user_id: Option<&str>,
    ) -> Result<HomePayload, AppError> {
        self.repo
            .get_home_payload(lang, page, limit, search, filters, sort_by, user_id)
            .await
    }

    pub async fn get_drawer_payload(
        &self,
        lang: Language,
        user_id: Option<&str>,
    ) -> Result<DrawerPayload, AppError> {
        self.repo.get_drawer_payload(lang, user_id).await
    }

    pub async fn get_dish_detail(
        &self,
        dish_id: &str,
        lang: Language,
        user_id: Option<&str>,
    ) -> Result<DishDetail, AppError> {
        self.repo.get_dish_detail(dish_id, lang, user_id).await
    }

    pub async fn toggle_favourite(&self, dish_id: &str, user_id: &str) -> Result<bool, AppError> {
        self.repo.toggle_favourite(dish_id, user_id).await
    }

    pub async fn get_favourites(
        &self,
        lang: Language,
        user_id: &str,
    ) -> Result<Vec<DishSummary>, AppError> {
        self.repo.get_favourites(lang, user_id).await
    }

    pub async fn save_feedback(
        &self,
        dish_id: &str,
        responses: serde_json::Value,
    ) -> Result<(), AppError> {
        self.repo.save_feedback(dish_id, responses).await
    }
}
