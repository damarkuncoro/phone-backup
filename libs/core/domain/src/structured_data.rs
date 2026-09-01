use serde::{Deserialize, Serialize};
use std::fmt;

/// Type-safe enumeration of structured metadata backed up from an Android device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StructuredDataType {
    Contacts,
    Sms,
    CallLogs,
    Applications,
    WifiNetworks,
    DeviceSettings,
}

impl StructuredDataType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Contacts => "contacts",
            Self::Sms => "sms",
            Self::CallLogs => "call_logs",
            Self::Applications => "apps",
            Self::WifiNetworks => "wifi",
            Self::DeviceSettings => "settings",
        }
    }
}

impl fmt::Display for StructuredDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
