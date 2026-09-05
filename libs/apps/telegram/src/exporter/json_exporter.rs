use crate::domain::TelegramChat;
use anyhow::Result;

/// Serializer for generating structured JSON exports of Telegram chats.
pub struct TelegramJsonExporter;

impl TelegramJsonExporter {
    pub fn export_pretty(chat: &TelegramChat) -> Result<String> {
        Ok(serde_json::to_string_pretty(chat)?)
    }
}
