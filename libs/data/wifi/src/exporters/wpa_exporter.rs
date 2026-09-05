use crate::domain::{SecurityType, WifiNetworkItem};

/// Exporter to standard Linux / Android wpa_supplicant.conf format
pub struct WpaSupplicantExporter;

impl WpaSupplicantExporter {
    pub fn export(networks: &[WifiNetworkItem]) -> String {
        let mut out = String::new();
        out.push_str("# phone-backup wpa_supplicant.conf export\n");
        out.push_str("ctrl_interface=/var/run/wpa_supplicant\n");
        out.push_str("update_config=1\n\n");

        for net in networks {
            out.push_str("network={\n");
            out.push_str(&format!("\tssid=\"{}\"\n", net.ssid));

            match net.security_type {
                SecurityType::Open => {
                    out.push_str("\tkey_mgmt=NONE\n");
                }
                SecurityType::Wep => {
                    if let Some(ref psk) = net.pre_shared_key {
                        out.push_str(&format!("\twep_key0=\"{}\"\n", psk));
                    }
                    out.push_str("\tkey_mgmt=NONE\n");
                }
                SecurityType::WpaPsk | SecurityType::Wpa2Psk => {
                    if let Some(ref psk) = net.pre_shared_key {
                        out.push_str(&format!("\tpsk=\"{}\"\n", psk));
                    }
                    out.push_str("\tkey_mgmt=WPA-PSK\n");
                }
                SecurityType::Wpa3Sae => {
                    if let Some(ref psk) = net.pre_shared_key {
                        out.push_str(&format!("\tsae_password=\"{}\"\n", psk));
                    }
                    out.push_str("\tkey_mgmt=SAE\n");
                    out.push_str("\tieee80211w=2\n");
                }
                SecurityType::Eap => {
                    out.push_str("\tkey_mgmt=WPA-EAP\n");
                }
                SecurityType::Unknown => {
                    if let Some(ref psk) = net.pre_shared_key {
                        out.push_str(&format!("\tpsk=\"{}\"\n", psk));
                    }
                }
            }

            if net.is_hidden {
                out.push_str("\tscan_ssid=1\n");
            }

            out.push_str("}\n\n");
        }

        out
    }
}
