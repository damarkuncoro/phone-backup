use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub name: String,
    pub phones: Vec<String>,
    pub emails: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sms {
    pub address: String,
    pub body: String,
    pub date: DateTime<Utc>,
    pub type_code: u8, // 1: inbox, 2: sent, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallLog {
    pub number: String,
    pub date: DateTime<Utc>,
    pub duration_seconds: u32,
    pub type_code: u8, // 1: incoming, 2: outgoing, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StructuredData {
    Contacts(Vec<Contact>),
    SmsMessages(Vec<Sms>),
    CallLogs(Vec<CallLog>),
}
