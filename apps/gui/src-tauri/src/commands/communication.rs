use crate::state::AppState;
use domain::SnapshotId;
use tauri::State;
use whatsapp::{ChatType, WhatsAppBackupStore, WhatsAppChatBuilder, WhatsAppExportFactory, WhatsAppExportFormat};

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
