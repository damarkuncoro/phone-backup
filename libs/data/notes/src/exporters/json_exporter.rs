use crate::domain::NoteItem;
use anyhow::Result;

/// Serializer for generating JSON exports of note collections.
pub struct JsonNoteExporter;

impl JsonNoteExporter {
    pub fn export_pretty(notes: &[NoteItem]) -> Result<String> {
        Ok(serde_json::to_string_pretty(notes)?)
    }
}
