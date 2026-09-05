use crate::domain::WifiNetworkItem;
use anyhow::Result;

/// Exporter for Wi-Fi networks to JSON
pub struct WifiJsonExporter;

impl WifiJsonExporter {
    pub fn export(networks: &[WifiNetworkItem]) -> Result<String> {
        let json = serde_json::to_string_pretty(networks)?;
        Ok(json)
    }
}
