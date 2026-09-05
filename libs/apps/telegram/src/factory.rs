use crate::domain::{ChatType, TelegramChat, TelegramMediaType, TelegramMessage};
use chrono::Utc;

/// Factory for creating pre-configured `TelegramMessage` and `TelegramChat` instances.
pub struct TelegramFactory;

impl TelegramFactory {
    /// Creates a standard text message.
    pub fn create_text_message(
        id: i64,
        sender: impl Into<String>,
        text: impl Into<String>,
    ) -> TelegramMessage {
        let mut msg = TelegramMessage::new(id, Utc::now(), text);
        msg.sender_name = Some(sender.into());
        msg
    }

    /// Creates a voice note message.
    pub fn create_voice_note(
        id: i64,
        sender: impl Into<String>,
        path: impl Into<String>,
        duration_secs: u32,
    ) -> TelegramMessage {
        let mut msg = TelegramMessage::new(id, Utc::now(), "");
        msg.sender_name = Some(sender.into());
        msg.media_type = TelegramMediaType::VoiceNote;
        msg.media_path = Some(path.into());
        msg.duration_secs = Some(duration_secs);
        msg
    }

    /// Creates a round video note message.
    pub fn create_video_note(
        id: i64,
        sender: impl Into<String>,
        path: impl Into<String>,
        duration_secs: u32,
    ) -> TelegramMessage {
        let mut msg = TelegramMessage::new(id, Utc::now(), "");
        msg.sender_name = Some(sender.into());
        msg.media_type = TelegramMediaType::VideoNote;
        msg.media_path = Some(path.into());
        msg.duration_secs = Some(duration_secs);
        msg
    }

    /// Creates a new Telegram chat aggregate.
    pub fn create_chat(id: i64, title: impl Into<String>, chat_type: ChatType) -> TelegramChat {
        TelegramChat::new(id, title, chat_type)
    }
}
