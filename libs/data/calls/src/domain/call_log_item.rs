use super::call_type::CallType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Aggregate root representing an individual call log record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallLogItem {
    pub id: String,
    pub phone_number: String,
    pub contact_name: Option<String>,
    pub call_type: CallType,
    pub timestamp: DateTime<Utc>,
    pub duration_secs: u64,
    pub sim_slot: Option<u8>,
    pub is_read: bool,
}

impl CallLogItem {
    /// Creates a new CallLogItem.
    pub fn new(
        id: impl Into<String>,
        phone_number: impl Into<String>,
        call_type: CallType,
        timestamp: DateTime<Utc>,
        duration_secs: u64,
    ) -> Self {
        Self {
            id: id.into(),
            phone_number: phone_number.into(),
            contact_name: None,
            call_type,
            timestamp,
            duration_secs,
            sim_slot: None,
            is_read: true,
        }
    }

    /// Sets contact name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.contact_name = Some(name.into());
        self
    }

    /// Sets SIM card slot.
    pub fn with_sim_slot(mut self, slot: u8) -> Self {
        self.sim_slot = Some(slot);
        self
    }

    /// Returns display name or phone number if name is missing.
    pub fn caller_label(&self) -> &str {
        self.contact_name
            .as_deref()
            .unwrap_or(&self.phone_number)
    }

    /// Formats call duration into HH:MM:SS or MM:SS.
    pub fn duration_display(&self) -> String {
        let hours = self.duration_secs / 3600;
        let mins = (self.duration_secs % 3600) / 60;
        let secs = self.duration_secs % 60;

        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, mins, secs)
        } else {
            format!("{:02}:{:02}", mins, secs)
        }
    }
}
