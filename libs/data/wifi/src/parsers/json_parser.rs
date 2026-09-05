use crate::domain::WifiNetworkItem;
use anyhow::Result;

/// Parser for JSON backup collections of Wi-Fi networks
pub struct WifiJsonParser;

impl WifiJsonParser {
    pub fn parse(json_str: &str) -> Result<Vec<WifiNetworkItem>> {
        let items: Vec<WifiNetworkItem> = serde_json::from_str(json_str)?;
        Ok(items)
    }
}
