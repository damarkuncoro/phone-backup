use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Summary metrics and tag distributions for a note collection.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct NoteStats {
    pub total_notes: usize,
    pub checklist_count: usize,
    pub pinned_count: usize,
    pub archived_count: usize,
    pub tags_count: HashMap<String, usize>,
    pub total_tasks: usize,
    pub completed_tasks: usize,
}
