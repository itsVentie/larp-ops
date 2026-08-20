use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputEvent {
    pub module: String,
    pub timestamp: String,
    pub level: String,
    pub payload: serde_json::Value,
}

impl OutputEvent {
    pub fn new(module: &str, level: &str, payload: serde_json::Value) -> Self {
        Self {
            module: module.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            level: level.to_string(),
            payload,
        }
    }

    pub fn print_ndjson(&self) {
        if let Ok(json) = serde_json::to_string(self) {
            println!("{}", json);
        }
    }
}
