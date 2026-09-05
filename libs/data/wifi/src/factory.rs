use crate::builder::WifiNetworkBuilder;
use crate::domain::{SecurityType, WifiNetworkItem};

/// Factory for creating standard Wi-Fi network instances
pub struct WifiNetworkFactory;

impl WifiNetworkFactory {
    /// Create open / public Wi-Fi network without password
    pub fn create_open(ssid: impl Into<String>) -> WifiNetworkItem {
        WifiNetworkBuilder::new(ssid)
            .security(SecurityType::Open)
            .psk("")
            .build()
    }

    /// Create standard WPA2-PSK Wi-Fi network
    pub fn create_wpa2(ssid: impl Into<String>, psk: impl Into<String>) -> WifiNetworkItem {
        WifiNetworkBuilder::new(ssid)
            .security(SecurityType::Wpa2Psk)
            .psk(psk)
            .build()
    }

    /// Create modern WPA3-SAE Wi-Fi network
    pub fn create_wpa3(ssid: impl Into<String>, psk: impl Into<String>) -> WifiNetworkItem {
        WifiNetworkBuilder::new(ssid)
            .security(SecurityType::Wpa3Sae)
            .psk(psk)
            .build()
    }

    /// Create hidden Wi-Fi network
    pub fn create_hidden(
        ssid: impl Into<String>,
        psk: impl Into<String>,
        security: SecurityType,
    ) -> WifiNetworkItem {
        WifiNetworkBuilder::new(ssid)
            .security(security)
            .psk(psk)
            .hidden(true)
            .build()
    }
}
