use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterOption {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterGroup {
    pub id: String,
    pub title: String,
    pub options: Vec<FilterOption>,
    pub is_multi_select: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortOption {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    pub is_default: bool,
}

#[derive(Debug, Clone, Default)]
pub struct FilterQuery {
    // Map of group_id -> list of selected option values (e.g. "cuisines" -> ["North Indian", "Italian"])
    pub groups: HashMap<String, Vec<String>>,
}

impl FilterQuery {
    /// Parses string formatted as "cuisines:North Indian,Italian;dietary:Vegetarian;taste:Spicy"
    pub fn from_param(param: Option<&str>) -> Self {
        let mut groups = HashMap::new();
        if let Some(p) = param {
            let p = p.trim();
            if !p.is_empty() {
                // Split by ';'
                for part in p.split(';') {
                    let part = part.trim();
                    if part.is_empty() {
                        continue;
                    }
                    if let Some((group_key, values_str)) = part.split_once(':') {
                        let group = group_key.trim().to_lowercase();
                        let values: Vec<String> = values_str
                            .split(',')
                            .map(|v| v.trim().to_string())
                            .filter(|v| !v.is_empty())
                            .collect();
                        if !values.is_empty() {
                            groups.insert(group, values);
                        }
                    }
                }
            }
        }
        Self { groups }
    }

    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }
}
