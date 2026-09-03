use chrono::{NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CallDirection {
    Incoming,
    Outgoing,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallRecordingInfo {
    pub phone_number: Option<String>,
    pub timestamp: Option<chrono::DateTime<Utc>>,
    pub direction: CallDirection,
}

pub struct CallRecordingParser;

impl CallRecordingParser {
    /// Extracts phone number, timestamp, and direction from Android call recording filename.
    /// Example: "Call@+628123456789_(2023-05-12_14.30.22)_in.m4a"
    pub fn parse_filename(filename: &str) -> CallRecordingInfo {
        let clean = filename.trim_end_matches(".m4a").trim_end_matches(".mp3").trim_end_matches(".amr");

        let direction = if clean.ends_with("_in") || clean.contains("incoming") {
            CallDirection::Incoming
        } else if clean.ends_with("_out") || clean.contains("outgoing") {
            CallDirection::Outgoing
        } else {
            CallDirection::Unknown
        };

        let phone_number = Self::extract_phone(clean);
        let timestamp = Self::extract_timestamp(clean);

        CallRecordingInfo {
            phone_number,
            timestamp,
            direction,
        }
    }

    fn extract_phone(s: &str) -> Option<String> {
        if let Some(start) = s.find('@') {
            let after = &s[start + 1..];
            let end = after.find('_').unwrap_or(after.len());
            let phone = &after[..end];
            if !phone.is_empty() {
                return Some(phone.to_string());
            }
        }
        None
    }

    fn extract_timestamp(s: &str) -> Option<chrono::DateTime<Utc>> {
        if let Some(open) = s.find('(') {
            if let Some(close) = s.find(')') {
                let inner = &s[open + 1..close];
                if let Ok(naive) = NaiveDateTime::parse_from_str(inner, "%Y-%m-%d_%H.%M.%S") {
                    return Some(Utc.from_utc_datetime(&naive));
                }
            }
        }
        None
    }
}
