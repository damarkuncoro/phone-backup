use crate::model::Contact;
use anyhow::Result;

pub struct JsonFormatter;

impl JsonFormatter {
    /// Serializes contacts into pretty JSON array.
    pub fn to_json(contacts: &[Contact]) -> Result<String> {
        Ok(serde_json::to_string_pretty(contacts)?)
    }

    /// Serializes contacts into Line-delimited JSON (NDJSON) for fast streaming.
    pub fn to_ndjson(contacts: &[Contact]) -> Result<String> {
        let mut out = String::new();
        for c in contacts {
            let line = serde_json::to_string(c)?;
            out.push_str(&line);
            out.push('\n');
        }
        Ok(out)
    }

    /// Deserializes contacts from JSON string.
    pub fn from_json(json_str: &str) -> Result<Vec<Contact>> {
        Ok(serde_json::from_str(json_str)?)
    }
}
