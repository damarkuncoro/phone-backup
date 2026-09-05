use crate::domain::{SecurityType, WifiNetworkItem};
use chrono::{DateTime, Utc};

/// Fluent Builder for constructing WifiNetworkItem instances
#[derive(Debug, Default, Clone)]
pub struct WifiNetworkBuilder {
    id: Option<String>,
    ssid: String,
    pre_shared_key: Option<String>,
    security_type: SecurityType,
    is_hidden: bool,
    is_metered: bool,
    auto_connect: bool,
    last_connected: Option<DateTime<Utc>>,
    created_at: Option<DateTime<Utc>>,
}

impl WifiNetworkBuilder {
    pub fn new(ssid: impl Into<String>) -> Self {
        Self {
            id: None,
            ssid: ssid.into(),
            pre_shared_key: None,
            security_type: SecurityType::Wpa2Psk,
            is_hidden: false,
            is_metered: false,
            auto_connect: true,
            last_connected: None,
            created_at: None,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn psk(mut self, psk: impl Into<String>) -> Self {
        self.pre_shared_key = Some(psk.into());
        self
    }

    pub fn security(mut self, security: SecurityType) -> Self {
        self.security_type = security;
        self
    }

    pub fn hidden(mut self, hidden: bool) -> Self {
        self.is_hidden = hidden;
        self
    }

    pub fn metered(mut self, metered: bool) -> Self {
        self.is_metered = metered;
        self
    }

    pub fn auto_connect(mut self, auto_connect: bool) -> Self {
        self.auto_connect = auto_connect;
        self
    }

    pub fn last_connected(mut self, dt: DateTime<Utc>) -> Self {
        self.last_connected = Some(dt);
        self
    }

    pub fn created_at(mut self, dt: DateTime<Utc>) -> Self {
        self.created_at = Some(dt);
        self
    }

    pub fn build(self) -> WifiNetworkItem {
        let ssid = self.ssid.trim_matches('"').to_string();
        let id = self.id.unwrap_or_else(|| ssid.clone());
        WifiNetworkItem {
            id,
            ssid,
            pre_shared_key: self.pre_shared_key.map(|p| p.trim_matches('"').to_string()),
            security_type: self.security_type,
            is_hidden: self.is_hidden,
            is_metered: self.is_metered,
            auto_connect: self.auto_connect,
            last_connected: self.last_connected,
            created_at: self.created_at,
        }
    }
}
