use serde::{Deserialize, Serialize};
use std::net::UdpSocket;
use std::process::Command;

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

#[tauri::command(rename_all = "snake_case")]
pub async fn connect_wireless_device(host: String, port: Option<u16>) -> Result<String, String> {
    let port_num = port.unwrap_or(5555);
    let target = format!("{}:{}", host.trim(), port_num);

    let output = Command::new("adb")
        .arg("connect")
        .arg(&target)
        .output()
        .map_err(|e| format!("Failed to execute adb connect: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.contains("connected to") || stdout.contains("already connected") {
        Ok(stdout.trim().to_string())
    } else {
        Err(stdout.trim().to_string())
    }
}

fn get_primary_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok().map(|addr| addr.ip().to_string())
}
