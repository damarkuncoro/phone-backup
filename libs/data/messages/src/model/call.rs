use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CallType {
    Incoming,
    Outgoing,
    Missed,
    Rejected,
    Blocked,
    Voicemail,
    Unknown,
}

impl CallType {
    pub fn is_missed(&self) -> bool {
        matches!(self, Self::Missed | Self::Rejected)
    }

    pub fn is_connected(&self) -> bool {
        matches!(self, Self::Incoming | Self::Outgoing)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallEntry {
    pub id: String,
    pub number: String,
    pub contact_name: Option<String>,
    pub date: DateTime<Utc>,
    pub duration_seconds: u64,
    pub call_type: CallType,
    pub subscription_id: Option<String>,
}

impl CallEntry {
    pub fn new(id: impl Into<String>, number: impl Into<String>, date: DateTime<Utc>, duration_seconds: u64, call_type: CallType) -> Self {
        Self {
            id: id.into(),
            number: number.into(),
            contact_name: None,
            date,
            duration_seconds,
            call_type,
            subscription_id: None,
        }
    }
}
