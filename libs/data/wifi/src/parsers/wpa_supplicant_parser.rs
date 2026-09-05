use crate::builder::WifiNetworkBuilder;
use crate::domain::{SecurityType, WifiNetworkItem};

/// Parser for Linux / Android wpa_supplicant.conf files
pub struct WpaSupplicantParser;

impl WpaSupplicantParser {
    pub fn parse(content: &str) -> Vec<WifiNetworkItem> {
        let mut networks = Vec::new();
        let mut in_network = false;

        let mut ssid = String::new();
        let mut psk = Option::<String>::None;
        let mut key_mgmt = String::new();
        let mut is_hidden = false;

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }

            if trimmed.starts_with("network={") || trimmed == "network={" {
                in_network = true;
                ssid.clear();
                psk = None;
                key_mgmt.clear();
                is_hidden = false;
                continue;
            }

            if in_network {
                if trimmed == "}" {
                    if !ssid.is_empty() {
                        let security = SecurityType::from_key_mgmt(&key_mgmt);
                        let mut builder = WifiNetworkBuilder::new(&ssid)
                            .security(security)
                            .hidden(is_hidden);

                        if let Some(p) = psk.take() {
                            builder = builder.psk(p);
                        }

                        networks.push(builder.build());
                    }
                    in_network = false;
                    continue;
                }

                if let Some((k, v)) = trimmed.split_once('=') {
                    let key = k.trim();
                    let val = v.trim().trim_matches('"');

                    match key {
                        "ssid" => ssid = val.to_string(),
                        "psk" => psk = Some(val.to_string()),
                        "key_mgmt" => key_mgmt = val.to_string(),
                        "scan_ssid" => is_hidden = val == "1",
                        _ => {}
                    }
                }
            }
        }

        networks
    }
}
