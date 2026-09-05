use crate::state::AppState;
use domain::SnapshotId;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::State;
use whatsapp::{ChatType, WhatsAppBackupStore, WhatsAppChatBuilder, WhatsAppExportFactory, WhatsAppExportFormat};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WhatsAppSyncStatus {
    pub has_synced_data: bool,
    pub total_chats: usize,
    pub total_messages: usize,
    pub synced_at: Option<String>,
    pub has_qr_file: bool,
}

#[derive(Deserialize)]
struct SyncMeta {
    total_chats: Option<usize>,
    total_messages: Option<usize>,
    synced_at: Option<String>,
}

fn find_file_in_workspace(name: &str) -> Option<PathBuf> {
    let candidates = [
        PathBuf::from("workspace").join(name),
        PathBuf::from("../../workspace").join(name),
        PathBuf::from("../workspace").join(name),
    ];
    for p in &candidates {
        if p.exists() {
            return Some(p.clone());
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        let p = cwd.join("workspace").join(name);
        if p.exists() {
            return Some(p);
        }
    }
    None
}

#[tauri::command(rename_all = "snake_case")]
pub async fn export_contacts_csv(
    state: State<'_, AppState>,
    snapshot_id: String,
) -> Result<String, String> {
    state
        .engine
        .export_contacts_csv(&SnapshotId(snapshot_id))
        .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn export_sms_xml(
    state: State<'_, AppState>,
    snapshot_id: String,
) -> Result<String, String> {
    state
        .engine
        .export_sms_xml(&SnapshotId(snapshot_id))
        .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn export_sms_html(
    state: State<'_, AppState>,
    snapshot_id: String,
) -> Result<String, String> {
    state
        .engine
        .export_sms_html(&SnapshotId(snapshot_id))
        .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_call_stats(
    state: State<'_, AppState>,
    snapshot_id: String,
) -> Result<messages::CallStatsSummary, String> {
    state
        .engine
        .get_call_stats(&SnapshotId(snapshot_id))
        .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn generate_whatsapp_archive_preview() -> Result<String, String> {
    let mut store = WhatsAppBackupStore::new();
    let sample = WhatsAppChatBuilder::new("sample_chat@s.whatsapp.net", ChatType::Individual)
        .with_name("WhatsApp Archive Preview")
        .add_text_message("1", "contact", false, chrono::Utc::now(), "Chat archive successfully generated!")
        .build();
    store.add_chat(sample);
    WhatsAppExportFactory::export(&store.chats, WhatsAppExportFormat::Html).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_whatsapp_sync_status() -> Result<WhatsAppSyncStatus, String> {
    let json_path = find_file_in_workspace("synced_whatsapp.json");
    let qr_path = find_file_in_workspace("scan_whatsapp_qr.html");

    if let Some(path) = json_path {
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(meta) = serde_json::from_str::<SyncMeta>(&content) {
                return Ok(WhatsAppSyncStatus {
                    has_synced_data: true,
                    total_chats: meta.total_chats.unwrap_or(0),
                    total_messages: meta.total_messages.unwrap_or(0),
                    synced_at: meta.synced_at,
                    has_qr_file: qr_path.is_some(),
                });
            }
        }
    }

    Ok(WhatsAppSyncStatus {
        has_synced_data: false,
        total_chats: 0,
        total_messages: 0,
        synced_at: None,
        has_qr_file: qr_path.is_some(),
    })
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_synced_whatsapp_html() -> Result<String, String> {
    let html_path = find_file_in_workspace("synced_whatsapp_viewer.html")
        .ok_or_else(|| "Belum ada file synced_whatsapp_viewer.html".to_string())?;
    std::fs::read_to_string(html_path).map_err(|e| format!("Gagal membaca arsip chat: {}", e))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_whatsapp_qr_html() -> Result<String, String> {
    let qr_path = find_file_in_workspace("scan_whatsapp_qr.html")
        .ok_or_else(|| "File QR Code WhatsApp tidak ditemukan".to_string())?;
    std::fs::read_to_string(qr_path).map_err(|e| format!("Gagal membaca QR code: {}", e))
}

