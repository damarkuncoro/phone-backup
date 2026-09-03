use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaCategory {
    Images,
    Video,
    Audio,
    VoiceNotes,
    Documents,
    Stickers,
    Gifs,
    ProfilePhotos,
    Unknown,
}

impl MediaCategory {
    pub fn folder_name(&self) -> &'static str {
        match self {
            Self::Images => "WhatsApp Images",
            Self::Video => "WhatsApp Video",
            Self::Audio => "WhatsApp Audio",
            Self::VoiceNotes => "WhatsApp Voice Notes",
            Self::Documents => "WhatsApp Documents",
            Self::Stickers => "WhatsApp Stickers",
            Self::Gifs => "WhatsApp Animated Gifs",
            Self::ProfilePhotos => "WhatsApp Profile Photos",
            Self::Unknown => "Other",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppMediaItem {
    pub relative_path: String,
    pub filename: String,
    pub category: MediaCategory,
    pub is_sent: bool,
    pub size_bytes: u64,
    pub date_created: Option<DateTime<Utc>>,
}

impl WhatsAppMediaItem {
    pub fn new(relative_path: impl Into<String>, filename: impl Into<String>, category: MediaCategory, is_sent: bool, size_bytes: u64) -> Self {
        Self {
            relative_path: relative_path.into(),
            filename: filename.into(),
            category,
            is_sent,
            size_bytes,
            date_created: None,
        }
    }
}
