use crate::domain::dish::DishSummary;
use crate::domain::filter::{FilterGroup, SortOption};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HomePayload {
    pub header_title: String,
    pub header_subtitle: String,
    pub search_bar_hint: String,
    pub filter_groups: Vec<FilterGroup>,
    pub sorters: Vec<SortOption>,
    pub dishes: Vec<DishSummary>,
    pub total_dishes: usize,
    pub current_page: usize,
    pub has_more_pages: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawerItem {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    pub requires_auth: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawerPayload {
    pub is_logged_in: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub items: Vec<DrawerItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FeedbackRequest {
    pub responses: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleFavouriteResponse {
    pub dish_id: String,
    pub is_favourite: bool,
}
