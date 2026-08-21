use serde::{Deserialize, Serialize};
use std::io::{self, BufRead};

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

    pub fn process_stdin<F>(mut handler: F)
    where
        F: FnMut(OutputEvent),
    {
        let stdin = io::stdin();
        let handle = stdin.lock();

        for line in handle.lines() {
            if let Ok(line_content) = line {
                if line_content.trim().is_empty() {
                    continue;
                }
                if let Ok(event) = serde_json::from_str::<OutputEvent>(&line_content) {
                    handler(event);
                }
            }
        }
    }
}
