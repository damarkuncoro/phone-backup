pub mod android;
pub mod backup;
pub mod device;

pub use android::AndroidMapper;
pub use backup::BackupMapper;
pub use device::DeviceMapper;

use chrono::{DateTime, Utc};

/// Helper to parse RFC3339 strings from database into Utc DateTime
pub(crate) fn parse_date(s: &str) -> rusqlite::Result<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
        })
}
