use super::mock_data::MockData;
use crate::domain::*;
use async_trait::async_trait;
use common::{AppError, Language};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
pub trait RecipeRepository: Send + Sync {
    async fn get_home_payload(
        &self,
        lang: Language,
        page: usize,
        limit: usize,
        search: Option<&str>,
        filters: &FilterQuery,
        sort_by: Option<&str>,
        user_id: Option<&str>,
    ) -> Result<HomePayload, AppError>;

    async fn get_drawer_payload(
        &self,
        lang: Language,
        user_id: Option<&str>,
    ) -> Result<DrawerPayload, AppError>;

    async fn get_dish_detail(
        &self,
        dish_id: &str,
        lang: Language,
        user_id: Option<&str>,
    ) -> Result<DishDetail, AppError>;

    async fn toggle_favourite(&self, dish_id: &str, user_id: &str) -> Result<bool, AppError>;

    async fn get_favourites(
        &self,
        lang: Language,
        user_id: &str,
    ) -> Result<Vec<DishSummary>, AppError>;

    async fn save_feedback(
        &self,
        dish_id: &str,
        responses: serde_json::Value,
    ) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct InMemoryRecipeRepository {
    // (user_id, dish_id)
    favourites: Arc<RwLock<HashSet<(String, String)>>>,
    // (dish_id, responses)
    feedback_records: Arc<RwLock<Vec<(String, serde_json::Value)>>>,
}

impl Default for InMemoryRecipeRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryRecipeRepository {
    pub fn new() -> Self {
        let mut set = HashSet::new();
        // Seed initial favorites for demo user
        set.insert(("+91 98765 43210".to_string(), "dish_pbm".to_string()));
        set.insert(("+91 98765 43210".to_string(), "dish_halwa".to_string()));

        Self {
            favourites: Arc::new(RwLock::new(set)),
            feedback_records: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[async_trait]
impl RecipeRepository for InMemoryRecipeRepository {
    async fn get_home_payload(
        &self,
        lang: Language,
        page: usize,
        limit: usize,
        search: Option<&str>,
        filters: &FilterQuery,
        sort_by: Option<&str>,
        user_id: Option<&str>,
    ) -> Result<HomePayload, AppError> {
        let all_dishes = MockData::get_all_dishes(lang);
        let fav_lock = self.favourites.read().await;

        let mut dishes: Vec<DishSummary> = all_dishes
            .into_iter()
            .map(|mut d| {
                if let Some(uid) = user_id {
                    d.is_favourite = fav_lock.contains(&(uid.to_string(), d.id.clone()));
                }
                d
            })
            .collect();

        // 1. Search Query filter
        if let Some(q) = search {
            let q = q.trim().to_lowercase();
            if !q.is_empty() {
                dishes.retain(|d| {
                    d.name.to_lowercase().contains(&q)
                        || d.cuisine.to_lowercase().contains(&q)
                        || d.dietary_type.to_lowercase().contains(&q)
                        || d.taste_profiles
                            .iter()
                            .any(|t| t.to_lowercase().contains(&q))
                });
            }
        }

        // 2. Structured Filter Query
        if !filters.is_empty() {
            // Cuisines filter
            if let Some(selected_cuisines) = filters.groups.get("cuisines") {
                if !selected_cuisines.is_empty() {
                    dishes.retain(|d| {
                        selected_cuisines
                            .iter()
                            .any(|c| c.eq_ignore_ascii_case(&d.cuisine))
                    });
                }
            }

            // Dietary filter
            if let Some(selected_dietary) = filters.groups.get("dietary") {
                if !selected_dietary.is_empty() {
                    dishes.retain(|d| {
                        selected_dietary
                            .iter()
                            .any(|diet| diet.eq_ignore_ascii_case(&d.dietary_type))
                    });
                }
            }

            // Taste filter
            if let Some(selected_tastes) = filters.groups.get("taste") {
                if !selected_tastes.is_empty() {
                    dishes.retain(|d| {
                        d.taste_profiles
                            .iter()
                            .any(|t| selected_tastes.iter().any(|st| st.eq_ignore_ascii_case(t)))
                    });
                }
            }
        }

        // 3. Sorters
        match sort_by {
            Some("prep_time_asc") => dishes.sort_by_key(|d| d.prep_time_minutes),
            Some("prep_time_desc") => dishes.sort_by(|a, b| b.prep_time_minutes.cmp(&a.prep_time_minutes)),
            Some("calories_asc") => dishes.sort_by_key(|d| d.calories_kcal),
            _ => dishes.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap_or(std::cmp::Ordering::Equal)),
        }

        // 4. Pagination
        let total_dishes = dishes.len();
        let page = if page == 0 { 1 } else { page };
        let limit = if limit == 0 { 10 } else { limit };
        let start = (page - 1) * limit;

        let paged_dishes = if start < total_dishes {
            dishes.into_iter().skip(start).take(limit).collect()
        } else {
            vec![]
        };

        let has_more_pages = (start + paged_dishes.len()) < total_dishes;

        Ok(HomePayload {
            header_title: MockData::get_header_title(lang).to_string(),
            header_subtitle: MockData::get_header_subtitle(lang).to_string(),
            search_bar_hint: MockData::get_search_hint(lang).to_string(),
            filter_groups: MockData::get_filter_groups(lang),
            sorters: MockData::get_sorters(lang),
            dishes: paged_dishes,
            total_dishes,
            current_page: page,
            has_more_pages,
        })
    }

    async fn get_drawer_payload(
        &self,
        lang: Language,
        user_id: Option<&str>,
    ) -> Result<DrawerPayload, AppError> {
        let is_logged_in = user_id.is_some();
        let phone = user_id.map(|s| s.to_string());
        let items = MockData::get_drawer_items(lang, is_logged_in);

        let (display_name, avatar_url) = if is_logged_in {
            (
                Some(match lang {
                    Language::Hi => "शेफ फूडी".to_string(),
                    Language::Ar => "شيف فودي".to_string(),
                    Language::En => "Chef Foodie".to_string(),
                }),
                Some("https://images.unsplash.com/photo-1577219491135-ce391730fb2c?w=150".to_string()),
            )
        } else {
            (None, None)
        };

        Ok(DrawerPayload {
            is_logged_in,
            phone_number: phone,
            display_name,
            avatar_url,
            items,
        })
    }

    async fn get_dish_detail(
        &self,
        dish_id: &str,
        lang: Language,
        user_id: Option<&str>,
    ) -> Result<DishDetail, AppError> {
        let mut detail = MockData::get_dish_detail(dish_id, lang)
            .ok_or_else(|| AppError::NotFound(format!("Dish with ID '{}' not found", dish_id)))?;

        if let Some(uid) = user_id {
            let fav_lock = self.favourites.read().await;
            detail.is_favourite = fav_lock.contains(&(uid.to_string(), dish_id.to_string()));
        }

        Ok(detail)
    }

    async fn toggle_favourite(&self, dish_id: &str, user_id: &str) -> Result<bool, AppError> {
        let mut write_lock = self.favourites.write().await;
        let key = (user_id.to_string(), dish_id.to_string());

        if write_lock.contains(&key) {
            write_lock.remove(&key);
            Ok(false)
        } else {
            write_lock.insert(key);
            Ok(true)
        }
    }

    async fn get_favourites(
        &self,
        lang: Language,
        user_id: &str,
    ) -> Result<Vec<DishSummary>, AppError> {
        let all_dishes = MockData::get_all_dishes(lang);
        let fav_lock = self.favourites.read().await;

        let user_favs: Vec<DishSummary> = all_dishes
            .into_iter()
            .filter(|d| fav_lock.contains(&(user_id.to_string(), d.id.clone())))
            .map(|mut d| {
                d.is_favourite = true;
                d
            })
            .collect();

        Ok(user_favs)
    }

    async fn save_feedback(
        &self,
        dish_id: &str,
        responses: serde_json::Value,
    ) -> Result<(), AppError> {
        let mut write_lock = self.feedback_records.write().await;
        write_lock.push((dish_id.to_string(), responses));
        Ok(())
    }
}
