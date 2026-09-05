use serde::{Deserialize, Serialize};
use std::fmt;

/// Type and direction of a phone call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    /// Maps standard Android CallLog.Calls.TYPE integer to `CallType`.
    pub fn from_android_type(code: u32) -> Self {
        match code {
            1 => Self::Incoming,
            2 => Self::Outgoing,
            3 => Self::Missed,
            4 => Self::Voicemail,
            5 => Self::Rejected,
            6 => Self::Blocked,
            _ => Self::Unknown,
        }
    }

    /// Converts string representation to `CallType`.
    pub fn from_str_loose(s: &str) -> Self {
        match s.to_lowercase().trim() {
            "incoming" | "in" | "1" => Self::Incoming,
            "outgoing" | "out" | "2" => Self::Outgoing,
            "missed" | "3" => Self::Missed,
            "voicemail" | "4" => Self::Voicemail,
            "rejected" | "5" => Self::Rejected,
            "blocked" | "6" => Self::Blocked,
            _ => Self::Unknown,
        }
    }

    /// Returns human-readable label.
    pub fn display_name(&self) -> &str {
        match self {
            Self::Incoming => "Incoming Call",
            Self::Outgoing => "Outgoing Call",
            Self::Missed => "Missed Call",
            Self::Rejected => "Rejected Call",
            Self::Blocked => "Blocked Call",
            Self::Voicemail => "Voicemail",
            Self::Unknown => "Unknown",
        }
    }

    /// Returns whether this call connected and had actual talk time.
    pub fn is_connected(&self) -> bool {
        matches!(self, Self::Incoming | Self::Outgoing)
    }
}

impl fmt::Display for CallType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
