use crate::domain::{CallLogItem, CallType};
use chrono::{DateTime, Utc};

/// Fluent builder for constructing `CallLogItem` aggregate instances.
#[derive(Default)]
pub struct CallLogItemBuilder {
    id: Option<String>,
    phone_number: Option<String>,
    contact_name: Option<String>,
    call_type: Option<CallType>,
    timestamp: Option<DateTime<Utc>>,
    duration_secs: Option<u64>,
    sim_slot: Option<u8>,
    is_read: Option<bool>,
}

impl CallLogItemBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn phone_number(mut self, number: impl Into<String>) -> Self {
        self.phone_number = Some(number.into());
        self
    }

    pub fn contact_name(mut self, name: impl Into<String>) -> Self {
        self.contact_name = Some(name.into());
        self
    }

    pub fn call_type(mut self, call_type: CallType) -> Self {
        self.call_type = Some(call_type);
        self
    }

    pub fn timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn duration_secs(mut self, secs: u64) -> Self {
        self.duration_secs = Some(secs);
        self
    }

    pub fn sim_slot(mut self, slot: u8) -> Self {
        self.sim_slot = Some(slot);
        self
    }

    pub fn is_read(mut self, read: bool) -> Self {
        self.is_read = Some(read);
        self
    }

    pub fn build(self) -> Result<CallLogItem, &'static str> {
        let phone_number = self.phone_number.ok_or("Phone number is required")?;
        let id = self.id.unwrap_or_else(|| {
            use std::time::{SystemTime, UNIX_EPOCH};
            let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
            format!("call_{}", nanos)
        });
        let call_type = self.call_type.unwrap_or(CallType::Unknown);
        let timestamp = self.timestamp.unwrap_or_else(Utc::now);
        let duration_secs = self.duration_secs.unwrap_or(0);

        let mut item = CallLogItem::new(id, phone_number, call_type, timestamp, duration_secs);
        if let Some(name) = self.contact_name {
            item = item.with_name(name);
        }
        if let Some(slot) = self.sim_slot {
            item = item.with_sim_slot(slot);
        }
        if let Some(read) = self.is_read {
            item.is_read = read;
        }

        Ok(item)
    }
}
