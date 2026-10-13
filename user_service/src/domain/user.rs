use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserPreferences {
    pub cuisines: Vec<String>,
    pub dietary_choices: Vec<String>,
    #[serde(default = "default_spice_level")]
    pub spice_level: String,
    #[serde(default = "default_sweetness_level")]
    pub sweetness_level: String,
    #[serde(default)]
    pub has_saved_preferences: bool,
}

fn default_spice_level() -> String {
    "Medium".to_string()
}

fn default_sweetness_level() -> String {
    "Balanced".to_string()
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            cuisines: vec![],
            dietary_choices: vec![],
            spice_level: default_spice_level(),
            sweetness_level: default_sweetness_level(),
            has_saved_preferences: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub phone_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    pub is_logged_in: bool,
    pub preferences: UserPreferences,
}

impl UserProfile {
    pub fn new(phone_number: impl Into<String>) -> Self {
        Self {
            phone_number: phone_number.into(),
            token: None,
            is_logged_in: false,
            preferences: UserPreferences::default(),
        }
    }
}
