use crate::domain::NoteItem;

/// Exporter for formatting notes into Markdown syntax.
pub struct MarkdownNoteExporter;

impl MarkdownNoteExporter {
    pub fn export_single(note: &NoteItem) -> String {
        let mut md = format!("# {}\n\n", note.title);

        if !note.content.is_empty() {
            md.push_str(&note.content);
            md.push_str("\n\n");
        }

        if !note.checklist_items.is_empty() {
            for item in &note.checklist_items {
                let box_str = if item.is_checked { "- [x]" } else { "- [ ]" };
                md.push_str(&format!("{} {}\n", box_str, item.text));
            }
            md.push('\n');
        }

        if !note.tags.is_empty() {
            let tags_str = note.tags.iter().map(|t| format!("#{}", t)).collect::<Vec<_>>().join(" ");
            md.push_str(&tags_str);
            md.push('\n');
        }

        md
    }
}
