use super::checklist_item::ChecklistItem;
use super::note_type::NoteType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Aggregate root representing an individual note or checklist document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteItem {
    pub id: String,
    pub title: String,
    pub content: String,
    pub checklist_items: Vec<ChecklistItem>,
    pub note_type: NoteType,
    pub tags: Vec<String>,
    pub is_pinned: bool,
    pub is_archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub color: Option<String>,
}

impl NoteItem {
    pub fn new(id: impl Into<String>, title: impl Into<String>, content: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: id.into(),
            title: title.into(),
            content: content.into(),
            checklist_items: Vec::new(),
            note_type: NoteType::TextNote,
            tags: Vec::new(),
            is_pinned: false,
            is_archived: false,
            created_at: now,
            updated_at: now,
            color: None,
        }
    }

    pub fn with_checklist(mut self, items: Vec<ChecklistItem>) -> Self {
        self.checklist_items = items;
        self.note_type = NoteType::Checklist;
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_pinned(mut self, pinned: bool) -> Self {
        self.is_pinned = pinned;
        self
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn snippet(&self, max_chars: usize) -> String {
        if !self.content.is_empty() {
            let s: String = self.content.chars().take(max_chars).collect();
            if self.content.chars().count() > max_chars {
                format!("{}...", s)
            } else {
                s
            }
        } else if !self.checklist_items.is_empty() {
            let completed = self.checklist_items.iter().filter(|i| i.is_checked).count();
            format!("[{}/{} tasks completed]", completed, self.checklist_items.len())
        } else {
            "-".to_string()
        }
    }
}
