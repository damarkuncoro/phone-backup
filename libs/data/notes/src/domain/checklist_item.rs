use serde::{Deserialize, Serialize};

/// Value object representing a single todo/checklist item within a note.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub text: String,
    pub is_checked: bool,
}

impl ChecklistItem {
    pub fn new(text: impl Into<String>, is_checked: bool) -> Self {
        Self {
            text: text.into(),
            is_checked,
        }
    }

    pub fn unchecked(text: impl Into<String>) -> Self {
        Self::new(text, false)
    }

    pub fn checked(text: impl Into<String>) -> Self {
        Self::new(text, true)
    }
}
