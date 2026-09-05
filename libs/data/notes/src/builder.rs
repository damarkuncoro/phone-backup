use crate::domain::{ChecklistItem, NoteItem, NoteType};
use chrono::{DateTime, Utc};

/// Fluent builder for constructing `NoteItem` instances.
#[derive(Default)]
pub struct NoteItemBuilder {
    id: Option<String>,
    title: Option<String>,
    content: Option<String>,
    checklist_items: Vec<ChecklistItem>,
    note_type: Option<NoteType>,
    tags: Vec<String>,
    is_pinned: Option<bool>,
    is_archived: Option<bool>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    color: Option<String>,
}

impl NoteItemBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn add_checklist_item(mut self, item: ChecklistItem) -> Self {
        self.checklist_items.push(item);
        self
    }

    pub fn note_type(mut self, note_type: NoteType) -> Self {
        self.note_type = Some(note_type);
        self
    }

    pub fn add_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn is_pinned(mut self, pinned: bool) -> Self {
        self.is_pinned = Some(pinned);
        self
    }

    pub fn is_archived(mut self, archived: bool) -> Self {
        self.is_archived = Some(archived);
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn build(self) -> Result<NoteItem, &'static str> {
        let title = self.title.ok_or("Note title is required")?;
        let content = self.content.unwrap_or_default();
        let now = Utc::now();
        let id = self.id.unwrap_or_else(|| {
            use std::time::{SystemTime, UNIX_EPOCH};
            let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
            format!("note_{}", nanos)
        });

        let note_type = self.note_type.unwrap_or_else(|| {
            if !self.checklist_items.is_empty() {
                NoteType::Checklist
            } else {
                NoteType::TextNote
            }
        });

        let mut note = NoteItem::new(id, title, content);
        note.checklist_items = self.checklist_items;
        note.note_type = note_type;
        note.tags = self.tags;
        note.is_pinned = self.is_pinned.unwrap_or(false);
        note.is_archived = self.is_archived.unwrap_or(false);
        note.created_at = self.created_at.unwrap_or(now);
        note.updated_at = self.updated_at.unwrap_or(now);
        note.color = self.color;

        Ok(note)
    }
}
