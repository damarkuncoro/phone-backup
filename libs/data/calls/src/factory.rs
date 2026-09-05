use crate::domain::{CallLogItem, CallType};
use chrono::Utc;

/// Factory for creating pre-configured `CallLogItem` instances.
pub struct CallLogFactory;

impl CallLogFactory {
    /// Creates an incoming connected call record.
    pub fn incoming(
        number: impl Into<String>,
        name: Option<String>,
        duration_secs: u64,
    ) -> CallLogItem {
        let mut item = CallLogItem::new(
            format!("call_in_{}", Utc::now().timestamp_nanos_opt().unwrap_or(0)),
            number,
            CallType::Incoming,
            Utc::now(),
            duration_secs,
        );
        if let Some(n) = name {
            item = item.with_name(n);
        }
        item
    }

    /// Creates an outgoing connected call record.
    pub fn outgoing(
        number: impl Into<String>,
        name: Option<String>,
        duration_secs: u64,
    ) -> CallLogItem {
        let mut item = CallLogItem::new(
            format!("call_out_{}", Utc::now().timestamp_nanos_opt().unwrap_or(0)),
            number,
            CallType::Outgoing,
            Utc::now(),
            duration_secs,
        );
        if let Some(n) = name {
            item = item.with_name(n);
        }
        item
    }

    /// Creates a missed call record.
    pub fn missed(number: impl Into<String>, name: Option<String>) -> CallLogItem {
        let mut item = CallLogItem::new(
            format!("call_miss_{}", Utc::now().timestamp_nanos_opt().unwrap_or(0)),
            number,
            CallType::Missed,
            Utc::now(),
            0,
        );
        if let Some(n) = name {
            item = item.with_name(n);
        }
        item
    }

    /// Creates a rejected call record.
    pub fn rejected(number: impl Into<String>, name: Option<String>) -> CallLogItem {
        let mut item = CallLogItem::new(
            format!("call_rej_{}", Utc::now().timestamp_nanos_opt().unwrap_or(0)),
            number,
            CallType::Rejected,
            Utc::now(),
            0,
        );
        if let Some(n) = name {
            item = item.with_name(n);
        }
        item
    }
}
