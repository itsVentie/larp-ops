use plugin_sdk::{register_plugin, LarpPlugin, OutputEvent};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SigmaRule {
    pub title: String,
    pub match_field: String,
    pub match_value: String,
}

pub struct SigmaEngine {
    rules: Vec<SigmaRule>,
}

impl Default for SigmaEngine {
    fn default() -> Self {
        Self {
            rules: vec![
                SigmaRule {
                    title: "Suspicious PowerShell Execution".into(),
                    match_field: "command".into(),
                    match_value: "powershell -enc".into(),
                },
                SigmaRule {
                    title: "Critical System Error Alert".into(),
                    match_field: "level".into(),
                    match_value: "CRITICAL".into(),
                },
            ],
        }
    }
}

impl LarpPlugin for SigmaEngine {
    fn filter(&mut self, event: &OutputEvent) -> bool {
        for rule in &self.rules {
            if rule.match_field == "level" && event.level == rule.match_value {
                return true;
            }
            if rule.match_field == "module" && event.module == rule.match_value {
                return true;
            }

            if let Some(val) = event.payload.get(&rule.match_field) {
                if let Some(s) = val.as_str() {
                    if s.contains(&rule.match_value) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

register_plugin!(SigmaEngine);
