use super::SecurityType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Aggregate Root representing a saved Wi-Fi network configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WifiNetworkItem {
    pub id: String,
    pub ssid: String,
    pub pre_shared_key: Option<String>,
    pub security_type: SecurityType,
    pub is_hidden: bool,
    pub is_metered: bool,
    pub auto_connect: bool,
    pub last_connected: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

impl WifiNetworkItem {
    /// Mask password for safe terminal display or logs
    pub fn masked_password(&self) -> String {
        match &self.pre_shared_key {
            Some(p) if p.is_empty() => "[None]".to_string(),
            Some(_) => "********".to_string(),
            None => "[None]".to_string(),
        }
    }

    /// Generate standard WiFi QR Code content (WIFI:T:WPA;S:ssid;P:password;H:false;;)
    pub fn to_qr_string(&self) -> String {
        let t = self.security_type.to_qr_type();
        let pass = self.pre_shared_key.as_deref().unwrap_or("");
        let hidden_str = if self.is_hidden { "true" } else { "false" };

        if t == "nopass" {
            format!("WIFI:T:nopass;S:{};H:{};;", self.ssid, hidden_str)
        } else {
            format!("WIFI:T:{};S:{};P:{};H:{};;", t, self.ssid, pass, hidden_str)
        }
    }
}
