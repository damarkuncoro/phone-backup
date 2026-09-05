use serde::{Deserialize, Serialize};
use std::fmt;

/// Wi-Fi Security Protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SecurityType {
    Open,
    Wep,
    WpaPsk,
    #[default]
    Wpa2Psk,
    Wpa3Sae,
    Eap,
    Unknown,
}

impl SecurityType {
    /// Infer security type from key_mgmt string (e.g. from wpa_supplicant or WifiConfigStore)
    pub fn from_key_mgmt(val: &str) -> Self {
        let v = val.to_uppercase();
        if v.contains("SAE") || v.contains("WPA3") {
            Self::Wpa3Sae
        } else if v.contains("WPA2") || v.contains("WPA-PSK") || v.contains("WPA_PSK") {
            Self::Wpa2Psk
        } else if v.contains("WPA") {
            Self::WpaPsk
        } else if v.contains("WEP") {
            Self::Wep
        } else if v.contains("EAP") || v.contains("802_1X") {
            Self::Eap
        } else if v.contains("NONE") || v.is_empty() {
            Self::Open
        } else {
            Self::Unknown
        }
    }

    /// Convert to QR code security token (WPA, WEP, or nopass)
    pub fn to_qr_type(&self) -> &'static str {
        match self {
            Self::Open => "nopass",
            Self::Wep => "WEP",
            Self::WpaPsk | Self::Wpa2Psk | Self::Wpa3Sae | Self::Eap | Self::Unknown => "WPA",
        }
    }

    /// Is this network encrypted?
    pub fn is_secure(&self) -> bool {
        !matches!(self, Self::Open | Self::Unknown)
    }
}

impl fmt::Display for SecurityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "Open"),
            Self::Wep => write!(f, "WEP"),
            Self::WpaPsk => write!(f, "WPA-PSK"),
            Self::Wpa2Psk => write!(f, "WPA2-PSK"),
            Self::Wpa3Sae => write!(f, "WPA3-SAE"),
            Self::Eap => write!(f, "WPA-EAP"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
