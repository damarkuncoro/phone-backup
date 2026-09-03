use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WhatsAppMessageType {
    Text,
    Image { caption: Option<String>, media_hash: Option<String> },
    Video { caption: Option<String>, duration_seconds: Option<u32> },
    Audio { duration_seconds: Option<u32> },
    VoiceNote { duration_seconds: Option<u32> },
    Document { filename: String, file_size: u64 },
    Location { latitude: f64, longitude: f64, name: Option<String> },
    Contact { name: String, vcard_data: Option<String> },
    Sticker,
    Revoked,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhatsAppMessage {
    pub id: String,
    pub chat_jid: String,
    pub sender_jid: String,
    pub sender_name: Option<String>,
    pub from_me: bool,
    pub timestamp: DateTime<Utc>,
    pub body: String,
    pub msg_type: WhatsAppMessageType,
    pub is_starred: bool,
}

impl WhatsAppMessage {
    pub fn new_text(id: impl Into<String>, chat_jid: impl Into<String>, sender_jid: impl Into<String>, from_me: bool, timestamp: DateTime<Utc>, body: impl Into<String>) -> Self {
        let b = body.into();
        Self {
            id: id.into(),
            chat_jid: chat_jid.into(),
            sender_jid: sender_jid.into(),
            sender_name: None,
            from_me,
            timestamp,
            body: b,
            msg_type: WhatsAppMessageType::Text,
            is_starred: false,
        }
    }
}
