use crate::domain::{ChecklistItem, NoteItem, NoteType};
use chrono::Utc;

/// Parser for Markdown formatted notes.
pub struct MarkdownNoteParser;

impl MarkdownNoteParser {
    pub fn parse(filename: &str, content: &str) -> NoteItem {
        let mut title = String::new();
        let mut body_lines = Vec::new();
        let mut checklist = Vec::new();
        let mut tags = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();

            if title.is_empty() && trimmed.starts_with("# ") {
                title = trimmed[2..].trim().to_string();
                continue;
            }

            if let Some(stripped) = trimmed.strip_prefix("- [ ] ") {
                checklist.push(ChecklistItem::unchecked(stripped));
            } else if let Some(stripped) = trimmed.strip_prefix("- [x] ").or_else(|| trimmed.strip_prefix("- [X] ")) {
                checklist.push(ChecklistItem::checked(stripped));
            } else {
                // Extract inline tags #word
                for word in trimmed.split_whitespace() {
                    if word.starts_with('#') && word.len() > 1 && !word.starts_with("##") {
                        let tag = word.trim_start_matches('#').trim_matches(|c: char| !c.is_alphanumeric());
                        if !tag.is_empty() && !tags.contains(&tag.to_string()) {
                            tags.push(tag.to_string());
                        }
                    }
                }
                body_lines.push(line);
            }
        }

        if title.is_empty() {
            title = std::path::Path::new(filename)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled Note")
                .to_string();
        }

        let id = format!("md_{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
        let note_type = if !checklist.is_empty() && body_lines.is_empty() {
            NoteType::Checklist
        } else {
            NoteType::TextNote
        };

        let mut note = NoteItem::new(id, title, body_lines.join("\n"));
        note.checklist_items = checklist;
        note.note_type = note_type;
        note.tags = tags;
        note
    }
}
