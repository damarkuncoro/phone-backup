use serde::{Deserialize, Serialize};
use std::fmt;

/// Type of note content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NoteType {
    TextNote,
    Checklist,
    RichText,
    VoiceMemo,
    ScannedDoc,
}

impl NoteType {
    pub fn display_name(&self) -> &str {
        match self {
            Self::TextNote => "Text Note",
            Self::Checklist => "Checklist",
            Self::RichText => "Rich Note",
            Self::VoiceMemo => "Voice Memo",
            Self::ScannedDoc => "Scanned Doc",
        }
    }
}

impl fmt::Display for NoteType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
