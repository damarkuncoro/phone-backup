use crate::model::SmsMessage;
use anyhow::Result;

pub struct JsonMessageFormatter;

impl JsonMessageFormatter {
    pub fn format_pretty(messages: &[SmsMessage]) -> Result<String> {
        Ok(serde_json::to_string_pretty(messages)?)
    }

    pub fn format_ndjson(messages: &[SmsMessage]) -> Result<String> {
        let mut out = String::new();
        for msg in messages {
            out.push_str(&serde_json::to_string(msg)?);
            out.push('\n');
        }
        Ok(out)
    }
}
