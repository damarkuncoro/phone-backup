use crate::domain::CallLogItem;
use anyhow::Result;

/// Exporter for generating structured JSON dumps of call records.
pub struct JsonCallExporter;

impl JsonCallExporter {
    /// Serializes call items to pretty JSON.
    pub fn export_pretty(items: &[CallLogItem]) -> Result<String> {
        Ok(serde_json::to_string_pretty(items)?)
    }

    /// Serializes call items to compact JSON.
    pub fn export_compact(items: &[CallLogItem]) -> Result<String> {
        Ok(serde_json::to_string(items)?)
    }
}
