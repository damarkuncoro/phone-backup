use crate::domain::{ChecklistItem, NoteItem, NoteType};
use chrono::{DateTime, Utc};
use serde_json::Value;

/// Parser for Google Keep Takeout JSON note files.
pub struct KeepJsonParser;

impl KeepJsonParser {
    pub fn parse(json_content: &str) -> Option<NoteItem> {
        let v: Value = serde_json::from_str(json_content).ok()?;

        let title = v.get("title").and_then(|t| t.as_str()).unwrap_or("").to_string();
        let text_content = v.get("textContent").and_then(|t| t.as_str()).unwrap_or("").to_string();
        let is_pinned = v.get("isPinned").and_then(|p| p.as_bool()).unwrap_or(false);
        let is_archived = v.get("isArchived").and_then(|a| a.as_bool()).unwrap_or(false);
        let color = v.get("color").and_then(|c| c.as_str()).map(|c| c.to_string());

        let created_at = v.get("createdTimestampUsec")
            .and_then(|u| u.as_i64())
            .and_then(DateTime::from_timestamp_micros)
            .unwrap_or_else(Utc::now);

        let updated_at = v.get("userEditedTimestampUsec")
            .and_then(|u| u.as_i64())
            .and_then(DateTime::from_timestamp_micros)
            .unwrap_or(created_at);

        let mut tags = Vec::new();
        if let Some(Value::Array(labels)) = v.get("labels") {
            for l in labels {
                if let Some(name) = l.get("name").and_then(|n| n.as_str()) {
                    tags.push(name.to_string());
                }
            }
        }

        let mut checklist = Vec::new();
        if let Some(Value::Array(items)) = v.get("listContent") {
            for item in items {
                let text = item.get("text").and_then(|t| t.as_str()).unwrap_or("");
                let is_checked = item.get("isChecked").and_then(|c| c.as_bool()).unwrap_or(false);
                if !text.is_empty() {
                    checklist.push(ChecklistItem::new(text, is_checked));
                }
            }
        }

        let note_type = if !checklist.is_empty() {
            NoteType::Checklist
        } else {
            NoteType::TextNote
        };

        let note_title = if title.is_empty() {
            if !text_content.is_empty() {
                text_content.lines().next().unwrap_or("Untitled Note").to_string()
            } else if !checklist.is_empty() {
                checklist[0].text.clone()
            } else {
                "Untitled Note".to_string()
            }
        } else {
            title
        };

        let id = format!("keep_{}", created_at.timestamp_micros());
        let mut note = NoteItem::new(id, note_title, text_content);
        note.checklist_items = checklist;
        note.note_type = note_type;
        note.tags = tags;
        note.is_pinned = is_pinned;
        note.is_archived = is_archived;
        note.created_at = created_at;
        note.updated_at = updated_at;
        note.color = color;

        Some(note)
    }
}
