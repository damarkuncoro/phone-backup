use crate::domain::WifiNetworkItem;

/// Wi-Fi QR Code string generator
pub struct WifiQrGenerator;

impl WifiQrGenerator {
    /// Generate standard Wi-Fi QR code connection payload: `WIFI:T:WPA;S:MySSID;P:MyPassword;H:false;;`
    pub fn generate_payload(network: &WifiNetworkItem) -> String {
        network.to_qr_string()
    }

    /// Render a compact informational card and payload for terminal display
    pub fn render_terminal_card(network: &WifiNetworkItem) -> String {
        let mut out = String::new();
        out.push_str("┌────────────────────────────────────────────────────────┐\n");
        out.push_str(&format!("│  📶 SSID: {:<44} │\n", network.ssid));
        out.push_str(&format!(
            "│  🔒 Security: {:<40} │\n",
            network.security_type.to_string()
        ));
        let pass_display = network
            .pre_shared_key
            .as_deref()
            .unwrap_or("[No Password]");
        out.push_str(&format!("│  🔑 Password: {:<40} │\n", pass_display));
        out.push_str(&format!(
            "│  👁️ Hidden:   {:<40} │\n",
            if network.is_hidden { "Yes" } else { "No" }
        ));
        out.push_str("├────────────────────────────────────────────────────────┤\n");
        out.push_str("│  📲 QR Code Connection String:                         │\n");
        let payload = Self::generate_payload(network);
        out.push_str(&format!("│  {:<52}  │\n", payload));
        out.push_str("└────────────────────────────────────────────────────────┘\n");
        out
    }
}
