use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    Inbox,
    Sent,
    Draft,
    Outbox,
}

impl MessageType {
    pub fn is_incoming(&self) -> bool {
        matches!(self, Self::Inbox)
    }

    pub fn is_outgoing(&self) -> bool {
        matches!(self, Self::Sent | Self::Outbox)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsMessage {
    pub id: String,
    pub thread_id: Option<u64>,
    pub address: String,
    pub contact_name: Option<String>,
    pub body: String,
    pub date: DateTime<Utc>,
    pub date_sent: Option<DateTime<Utc>>,
    pub msg_type: MessageType,
    pub read: bool,
    pub service_center: Option<String>,
}

impl SmsMessage {
    pub fn new(id: impl Into<String>, address: impl Into<String>, body: impl Into<String>, date: DateTime<Utc>, msg_type: MessageType) -> Self {
        Self {
            id: id.into(),
            thread_id: None,
            address: address.into(),
            contact_name: None,
            body: body.into(),
            date,
            date_sent: None,
            msg_type,
            read: true,
            service_center: None,
        }
    }
}
