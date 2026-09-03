use super::sms::SmsMessage;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadParticipant {
    pub address: String,
    pub contact_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationThread {
    pub thread_id: String,
    pub participant: ThreadParticipant,
    pub messages: Vec<SmsMessage>,
    pub last_message_date: Option<DateTime<Utc>>,
    pub unread_count: usize,
}

impl ConversationThread {
    pub fn new(thread_id: impl Into<String>, address: impl Into<String>, contact_name: Option<String>) -> Self {
        Self {
            thread_id: thread_id.into(),
            participant: ThreadParticipant {
                address: address.into(),
                contact_name,
            },
            messages: Vec::new(),
            last_message_date: None,
            unread_count: 0,
        }
    }

    pub fn add_message(&mut self, msg: SmsMessage) {
        if !msg.read {
            self.unread_count += 1;
        }
        if self.last_message_date.map_or(true, |last| msg.date > last) {
            self.last_message_date = Some(msg.date);
        }
        self.messages.push(msg);
    }

    pub fn total_messages(&self) -> usize {
        self.messages.len()
    }
}
