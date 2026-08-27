use plugin_sdk::{register_plugin, LarpPlugin};
use shared_types::OutputEvent;

#[derive(Default)]
pub struct EventFilter;

impl LarpPlugin for EventFilter {
    fn filter(&mut self, event: &OutputEvent) -> bool {
        if event.level == "ERROR" || event.level == "CRITICAL" || event.level == "WARNING" {
            return true;
        }

        if let Some(status) = event.payload.get("status") {
            if status == "completed" {
                return true;
            }
        }

        false
    }
}

register_plugin!(EventFilter);
