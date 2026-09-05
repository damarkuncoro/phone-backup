use super::media_type::TelegramMediaType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Aggregate root representing an individual Telegram message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TelegramMessage {
    pub id: i64,
    pub date: DateTime<Utc>,
    pub sender_name: Option<String>,
    pub sender_id: Option<String>,
    pub text: String,
    pub media_type: TelegramMediaType,
    pub media_path: Option<String>,
    pub duration_secs: Option<u32>,
    pub reply_to_id: Option<i64>,
}

impl TelegramMessage {
    pub fn new(id: i64, date: DateTime<Utc>, text: impl Into<String>) -> Self {
        Self {
            id,
            date,
            sender_name: None,
            sender_id: None,
            text: text.into(),
            media_type: TelegramMediaType::TextOnly,
            media_path: None,
            duration_secs: None,
            reply_to_id: None,
        }
    }

    pub fn with_sender(mut self, name: impl Into<String>) -> Self {
        self.sender_name = Some(name.into());
        self
    }

    pub fn with_media(mut self, media_type: TelegramMediaType, path: impl Into<String>) -> Self {
        self.media_type = media_type;
        self.media_path = Some(path.into());
        self
    }

    pub fn with_duration(mut self, secs: u32) -> Self {
        self.duration_secs = Some(secs);
        self
    }

    pub fn has_media(&self) -> bool {
        self.media_type != TelegramMediaType::TextOnly
    }
}
