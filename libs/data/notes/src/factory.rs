use crate::domain::{ChecklistItem, NoteItem};
use chrono::Utc;

/// Factory for rapidly creating typed `NoteItem` instances.
pub struct NoteFactory;

impl NoteFactory {
    /// Creates a standard plain text note.
    pub fn create_text_note(title: impl Into<String>, content: impl Into<String>) -> NoteItem {
        let id = format!("note_{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
        NoteItem::new(id, title, content)
    }

    /// Creates a todo checklist note.
    pub fn create_checklist(title: impl Into<String>, tasks: Vec<(&str, bool)>) -> NoteItem {
        let items: Vec<ChecklistItem> = tasks
            .into_iter()
            .map(|(text, checked)| ChecklistItem::new(text, checked))
            .collect();
        let mut note = Self::create_text_note(title, "");
        note = note.with_checklist(items);
        note
    }

    /// Creates a pinned priority note with tags.
    pub fn create_pinned_note(
        title: impl Into<String>,
        content: impl Into<String>,
        tags: Vec<&str>,
    ) -> NoteItem {
        let mut note = Self::create_text_note(title, content);
        note.is_pinned = true;
        note.tags = tags.into_iter().map(|t| t.to_string()).collect();
        note
    }
}
