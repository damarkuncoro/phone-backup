use crate::domain::{ChatType, TelegramChat, TelegramMediaType, TelegramMessage};
use chrono::{DateTime, Utc};

/// Fluent builder for constructing `TelegramMessage` instances.
#[derive(Default)]
pub struct TelegramMessageBuilder {
    id: Option<i64>,
    date: Option<DateTime<Utc>>,
    sender_name: Option<String>,
    sender_id: Option<String>,
    text: Option<String>,
    media_type: Option<TelegramMediaType>,
    media_path: Option<String>,
    duration_secs: Option<u32>,
    reply_to_id: Option<i64>,
}

impl TelegramMessageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn date(mut self, date: DateTime<Utc>) -> Self {
        self.date = Some(date);
        self
    }

    pub fn sender(mut self, name: impl Into<String>) -> Self {
        self.sender_name = Some(name.into());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn media(mut self, media_type: TelegramMediaType, path: impl Into<String>) -> Self {
        self.media_type = Some(media_type);
        self.media_path = Some(path.into());
        self
    }

    pub fn duration_secs(mut self, secs: u32) -> Self {
        self.duration_secs = Some(secs);
        self
    }

    pub fn build(self) -> Result<TelegramMessage, &'static str> {
        let id = self.id.ok_or("Message ID is required")?;
        let date = self.date.unwrap_or_else(Utc::now);
        let text = self.text.unwrap_or_default();

        let mut msg = TelegramMessage::new(id, date, text);
        msg.sender_name = self.sender_name;
        msg.sender_id = self.sender_id;
        msg.media_type = self.media_type.unwrap_or(TelegramMediaType::TextOnly);
        msg.media_path = self.media_path;
        msg.duration_secs = self.duration_secs;
        msg.reply_to_id = self.reply_to_id;

        Ok(msg)
    }
}

/// Fluent builder for constructing `TelegramChat` instances.
#[derive(Default)]
pub struct TelegramChatBuilder {
    id: Option<i64>,
    title: Option<String>,
    chat_type: Option<ChatType>,
    messages: Vec<TelegramMessage>,
}

impl TelegramChatBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn chat_type(mut self, chat_type: ChatType) -> Self {
        self.chat_type = Some(chat_type);
        self
    }

    pub fn add_message(mut self, msg: TelegramMessage) -> Self {
        self.messages.push(msg);
        self
    }

    pub fn build(self) -> Result<TelegramChat, &'static str> {
        let id = self.id.ok_or("Chat ID is required")?;
        let title = self.title.unwrap_or_else(|| "Telegram Chat".to_string());
        let chat_type = self.chat_type.unwrap_or(ChatType::PersonalChat);

        let mut chat = TelegramChat::new(id, title, chat_type);
        chat.messages = self.messages;
        Ok(chat)
    }
}
