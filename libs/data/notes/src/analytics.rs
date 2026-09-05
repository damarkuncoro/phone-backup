use crate::domain::{NoteItem, NoteStats, NoteType};
use std::collections::HashMap;

/// Domain service for analyzing note collections and computing tag statistics.
pub struct NotesAnalytics;

impl NotesAnalytics {
    /// Computes summary metrics and tag distribution for a collection of notes.
    pub fn compute_stats(notes: &[NoteItem]) -> NoteStats {
        let mut stats = NoteStats {
            total_notes: notes.len(),
            ..Default::default()
        };

        let mut tag_map: HashMap<String, usize> = HashMap::new();

        for note in notes {
            if note.note_type == NoteType::Checklist || !note.checklist_items.is_empty() {
                stats.checklist_count += 1;
            }
            if note.is_pinned {
                stats.pinned_count += 1;
            }
            if note.is_archived {
                stats.archived_count += 1;
            }

            for item in &note.checklist_items {
                stats.total_tasks += 1;
                if item.is_checked {
                    stats.completed_tasks += 1;
                }
            }

            for tag in &note.tags {
                *tag_map.entry(tag.clone()).or_insert(0) += 1;
            }
        }

        stats.tags_count = tag_map;
        stats
    }

    /// Filters notes by tag, pinned status, or search query.
    pub fn filter_notes(
        notes: Vec<NoteItem>,
        tag: Option<&str>,
        pinned_only: bool,
        query: Option<&str>,
    ) -> Vec<NoteItem> {
        let t_lower = tag.map(|t| t.to_lowercase());
        let q_lower = query.map(|q| q.to_lowercase());

        notes
            .into_iter()
            .filter(|n| {
                if pinned_only && !n.is_pinned {
                    return false;
                }
                if let Some(ref target_tag) = t_lower {
                    let has_tag = n.tags.iter().any(|t| t.to_lowercase() == *target_tag);
                    if !has_tag {
                        return false;
                    }
                }
                if let Some(ref q) = q_lower {
                    let title_match = n.title.to_lowercase().contains(q);
                    let content_match = n.content.to_lowercase().contains(q);
                    let task_match = n.checklist_items.iter().any(|i| i.text.to_lowercase().contains(q));
                    if !title_match && !content_match && !task_match {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
}
