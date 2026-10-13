use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogRecord {
    #[serde(rename = "type", default = "default_event_type")]
    pub event_type: String, // "event" or "metric"
    pub name: String,       // e.g. "dish_clicked", "app_startup_time_ms"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>, // numeric value if metric
    #[serde(default)]
    pub parameters: serde_json::Value, // parameters or tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

fn default_event_type() -> String {
    "event".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogBatchRequest {
    pub events: Vec<LogRecord>,
}

#[derive(Debug, Serialize)]
pub struct LogBatchResponse {
    pub success: bool,
    pub ingested: usize,
}
