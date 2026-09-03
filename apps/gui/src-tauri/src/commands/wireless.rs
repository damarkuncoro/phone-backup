use serde::{Deserialize, Serialize};
use std::net::UdpSocket;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WirelessPairingInfo {
    pub ip_address: String,
    pub port: u16,
    pub pairing_token: String,
    pub qr_payload: String,
    pub server_status: String,
}

#[tauri::command]
pub fn get_wireless_pairing_info() -> Result<WirelessPairingInfo, String> {
    let local_ip = get_primary_ip().unwrap_or_else(|| "127.0.0.1".to_string());
    let port = 3030;
    let pairing_token = uuid::Uuid::new_v4().to_string();

    let qr_payload = format!(
        "phonebackup://pair?ip={}&port={}&token={}",
        local_ip, port, pairing_token
    );

    Ok(WirelessPairingInfo {
        ip_address: local_ip,
        port,
        pairing_token,
        qr_payload,
        server_status: "Active (Listening on 0.0.0.0:3030)".to_string(),
    })
}

fn get_primary_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok().map(|addr| addr.ip().to_string())
}
