use crate::domain::NoteItem;
use serde_json::Value;

/// Parser for generic JSON note collections.
pub struct JsonNoteParser;

impl JsonNoteParser {
    pub fn parse_collection(json_content: &str) -> Vec<NoteItem> {
        if let Ok(notes) = serde_json::from_str::<Vec<NoteItem>>(json_content) {
            return notes;
        }

        let parsed: Value = match serde_json::from_str(json_content) {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };

        let list = match parsed {
            Value::Array(arr) => arr,
            Value::Object(obj) => {
                if let Some(Value::Array(notes)) = obj.get("notes") {
                    notes.clone()
                } else {
                    return Vec::new();
                }
            }
            _ => return Vec::new(),
        };

        list.into_iter()
            .filter_map(|v| serde_json::from_value::<NoteItem>(v).ok())
            .collect()
    }
}
