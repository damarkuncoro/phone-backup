use super::message::WhatsAppMessage;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatType {
    Individual,
    Group,
    Broadcast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppChat {
    pub jid: String,
    pub name: Option<String>,
    pub chat_type: ChatType,
    pub messages: Vec<WhatsAppMessage>,
    pub last_activity: Option<DateTime<Utc>>,
    pub unread_count: usize,
}

impl WhatsAppChat {
    pub fn new(jid: impl Into<String>, name: Option<String>, chat_type: ChatType) -> Self {
        Self {
            jid: jid.into(),
            name,
            chat_type,
            messages: Vec::new(),
            last_activity: None,
            unread_count: 0,
        }
    }

    pub fn add_message(&mut self, msg: WhatsAppMessage) {
        if self.last_activity.is_none_or(|last| msg.timestamp > last) {
            self.last_activity = Some(msg.timestamp);
        }
        self.messages.push(msg);
    }

    pub fn is_group(&self) -> bool {
        matches!(self.chat_type, ChatType::Group)
    }
}
