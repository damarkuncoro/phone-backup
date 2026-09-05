use super::chat_type::ChatType;
use super::telegram_message::TelegramMessage;
use serde::{Deserialize, Serialize};

/// Aggregate root representing a Telegram chat channel/group/direct conversation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TelegramChat {
    pub id: i64,
    pub title: String,
    pub chat_type: ChatType,
    pub messages: Vec<TelegramMessage>,
}

impl TelegramChat {
    pub fn new(id: i64, title: impl Into<String>, chat_type: ChatType) -> Self {
        Self {
            id,
            title: title.into(),
            chat_type,
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: TelegramMessage) {
        self.messages.push(message);
    }

    pub fn total_messages(&self) -> usize {
        self.messages.len()
    }

    pub fn total_media_messages(&self) -> usize {
        self.messages.iter().filter(|m| m.has_media()).count()
    }
}
