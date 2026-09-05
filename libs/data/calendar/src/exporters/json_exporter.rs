use crate::domain::CalendarEvent;
use anyhow::Result;

/// Serializer for generating JSON dumps of calendar events.
pub struct JsonCalendarExporter;

impl JsonCalendarExporter {
    pub fn export_pretty(events: &[CalendarEvent]) -> Result<String> {
        Ok(serde_json::to_string_pretty(events)?)
    }

    pub fn export_compact(events: &[CalendarEvent]) -> Result<String> {
        Ok(serde_json::to_string(events)?)
    }
}
