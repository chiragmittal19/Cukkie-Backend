use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DishSummary {
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub taste_profiles: Vec<String>,
    pub prep_time_minutes: u32,
    pub calories_kcal: u32,
    pub protein_grams: u32,
    pub is_favourite: bool,
    pub cuisine: String,
    pub dietary_type: String,
    pub rating: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortionDefinition {
    pub unit: String,
    pub default_amount: f64,
    pub step_increment: f64,
    pub min_amount: f64,
    pub max_amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngredientItem {
    pub id: String,
    pub name: String,
    pub base_quantity: f64,
    pub unit: String,
    pub is_optional: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeStepItem {
    pub index: usize,
    pub heading: String,
    pub instruction_text: String,
    pub audio_narration_text: String,
    pub todos: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicFormField {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "type")]
    pub field_type: String, // rating, single_choice, multiple_choice, text, image_upload
    pub is_required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DishDetail {
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub share_url: String,
    pub description: String,
    pub history: String,
    pub nutritional_breakdown: serde_json::Value,
    pub portion_definition: PortionDefinition,
    pub ingredients: Vec<IngredientItem>,
    pub steps: Vec<RecipeStepItem>,
    pub feedback_form_fields: Vec<DynamicFormField>,
    pub is_favourite: bool,
}
