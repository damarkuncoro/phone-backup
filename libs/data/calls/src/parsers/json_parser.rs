use crate::domain::CallLogItem;
use serde_json::Value;

/// Parser for structured Call Log JSON datasets.
pub struct JsonCallParser;

impl JsonCallParser {
    /// Parses JSON string into `CallLogItem` list.
    pub fn parse(json_content: &str) -> Vec<CallLogItem> {
        if let Ok(items) = serde_json::from_str::<Vec<CallLogItem>>(json_content) {
            return items;
        }

        // Fallback: parse array with loose keys
        let parsed: Value = match serde_json::from_str(json_content) {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };

        let list = match parsed {
            Value::Array(arr) => arr,
            Value::Object(obj) => {
                if let Some(Value::Array(calls)) = obj.get("calls") {
                    calls.clone()
                } else {
                    return Vec::new();
                }
            }
            _ => return Vec::new(),
        };

        list.into_iter()
            .filter_map(|v| serde_json::from_value::<CallLogItem>(v).ok())
            .collect()
    }
}
