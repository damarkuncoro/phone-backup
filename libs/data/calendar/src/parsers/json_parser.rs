use crate::domain::CalendarEvent;
use serde_json::Value;

/// Parser for structured Calendar JSON documents.
pub struct JsonCalendarParser;

impl JsonCalendarParser {
    /// Parses JSON string into `CalendarEvent` list.
    pub fn parse(json_content: &str) -> Vec<CalendarEvent> {
        if let Ok(events) = serde_json::from_str::<Vec<CalendarEvent>>(json_content) {
            return events;
        }

        let parsed: Value = match serde_json::from_str(json_content) {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };

        let list = match parsed {
            Value::Array(arr) => arr,
            Value::Object(obj) => {
                if let Some(Value::Array(events)) = obj.get("events") {
                    events.clone()
                } else {
                    return Vec::new();
                }
            }
            _ => return Vec::new(),
        };

        list.into_iter()
            .filter_map(|v| serde_json::from_value::<CalendarEvent>(v).ok())
            .collect()
    }
}
