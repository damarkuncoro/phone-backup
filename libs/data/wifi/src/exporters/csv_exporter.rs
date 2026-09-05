use crate::domain::WifiNetworkItem;

/// Exporter for Wi-Fi networks to CSV
pub struct WifiCsvExporter;

impl WifiCsvExporter {
    pub fn export(networks: &[WifiNetworkItem], include_passwords: bool) -> String {
        let mut out = String::new();
        out.push_str("SSID,Security,Password,Hidden,Metered,AutoConnect\n");

        for net in networks {
            let pass = if include_passwords {
                net.pre_shared_key.as_deref().unwrap_or("")
            } else {
                "********"
            };

            out.push_str(&format!(
                "\"{}\",\"{}\",\"{}\",{},{},{}\n",
                net.ssid.replace('"', "\"\""),
                net.security_type,
                pass.replace('"', "\"\""),
                net.is_hidden,
                net.is_metered,
                net.auto_connect
            ));
        }

        out
    }
}
