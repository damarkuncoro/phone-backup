use serde::{Deserialize, Serialize};
use std::fmt;

/// Type of media attachment in a Telegram message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TelegramMediaType {
    TextOnly,
    Photo,
    Video,
    VoiceNote,
    VideoNote,
    Sticker,
    Document,
    Audio,
    Poll,
    Location,
    Unknown,
}

impl TelegramMediaType {
    pub fn from_mime_or_ext(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "ogg" | "oga" | "opus" => Self::VoiceNote,
            "mp4" | "mov" | "mkv" => Self::Video,
            "jpg" | "jpeg" | "png" | "webp" => Self::Photo,
            "mp3" | "m4a" | "flac" | "wav" => Self::Audio,
            "tgs" => Self::Sticker,
            "pdf" | "docx" | "xlsx" | "zip" | "rar" | "apk" => Self::Document,
            _ => Self::Unknown,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::TextOnly => "Text",
            Self::Photo => "Photo",
            Self::Video => "Video",
            Self::VoiceNote => "Voice Note (PTT)",
            Self::VideoNote => "Video Note (Round)",
            Self::Sticker => "Sticker",
            Self::Document => "Document",
            Self::Audio => "Audio File",
            Self::Poll => "Poll",
            Self::Location => "Location",
            Self::Unknown => "Other",
        }
    }
}

impl fmt::Display for TelegramMediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
