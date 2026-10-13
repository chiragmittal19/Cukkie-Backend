use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferenceOptionItem {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferenceGroupConfig {
    pub id: String,
    pub title: String,
    pub options: Vec<PreferenceOptionItem>,
    pub is_multi_select: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferencesConfig {
    pub title: String,
    pub subtitle: String,
    pub save_button_text: String,
    pub groups: Vec<PreferenceGroupConfig>,
}
