use tauri::State;
use crate::state::AppState;
use domain::{DeviceId, Snapshot, SnapshotId};
use serde_json::Value;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_storage_stats(state: State<'_, AppState>) -> Result<application::StorageStats, String> {
    state.engine.get_storage_stats().map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn start_backup(state: State<'_, AppState>, device_id: String, include_files: Option<Vec<String>>) -> Result<Snapshot, String> {
    let id = DeviceId::new(device_id);
    let policy = include_files.map(|paths| domain::BackupPolicy::builder().include_many(paths).build());
    state.engine.perform_backup(&id, domain::EncryptionMode::None, policy).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_snapshots(state: State<'_, AppState>, device_id: String) -> Result<Vec<Snapshot>, String> {
    let id = DeviceId::new(device_id);
    state.engine.list_snapshots(&id).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_snapshot_files(state: State<'_, AppState>, snapshot_id: String) -> Result<Vec<domain::FileEntry>, String> {
    let id = SnapshotId(snapshot_id);
    state.engine.get_snapshot_files(&id).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_structured_data(
    state: State<'_, AppState>,
    snapshot_id: String,
    data_type: String,
) -> Result<Value, String> {
    let id = SnapshotId(snapshot_id);
    let dtype = match data_type.as_str() {
        "contacts" => domain::StructuredDataType::Contacts,
        "sms" => domain::StructuredDataType::Sms,
        "call_logs" => domain::StructuredDataType::CallLogs,
        "apps" => domain::StructuredDataType::Applications,
        "wifi" => domain::StructuredDataType::WifiNetworks,
        "settings" => domain::StructuredDataType::DeviceSettings,
        _ => return Err(format!("Unknown structured data type: {}", data_type)),
    };
    state.engine.get_structured_data(&id, dtype).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn restore_snapshot(
    state: State<'_, AppState>,
    snapshot_id: String,
    target_dir: String,
    filter: Option<Vec<String>>,
) -> Result<(), String> {
    let id = SnapshotId(snapshot_id);
    state
        .engine
        .perform_restore(&id, &target_dir, domain::EncryptionMode::None, filter)
        .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_snapshot_apps(state: State<'_, AppState>, snapshot_id: String) -> Result<Vec<domain::AppInfo>, String> {
    let id = SnapshotId(snapshot_id);
    state.engine.get_snapshot_apps(&id).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_file_diff(
    state: State<'_, AppState>,
    old_snapshot_id: String,
    new_snapshot_id: String,
) -> Result<domain::FileDiff, String> {
    let old_id = SnapshotId(old_snapshot_id);
    let new_id = SnapshotId(new_snapshot_id);
    state.engine.get_file_diff(&old_id, &new_id).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_snapshot(state: State<'_, AppState>, snapshot_id: String) -> Result<(), String> {
    let id = SnapshotId(snapshot_id);
    state.engine.delete_snapshot(&id).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_contact_diff(
    state: State<'_, AppState>,
    old_snapshot_id: String,
    new_snapshot_id: String,
) -> Result<domain::ContactDiff, String> {
    let old_id = SnapshotId(old_snapshot_id);
    let new_id = SnapshotId(new_snapshot_id);
    state.engine.get_contact_diff(&old_id, &new_id).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn export_contacts_vcard(
    state: State<'_, AppState>,
    snapshot_id: String,
) -> Result<String, String> {
    let id = SnapshotId(snapshot_id);
    state.engine.export_contacts_vcard(&id).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_snapshot_sms(state: State<'_, AppState>, snapshot_id: String) -> Result<Vec<domain::Sms>, String> {
    let id = SnapshotId(snapshot_id);
    state.engine.get_snapshot_sms(&id).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn search_sms(state: State<'_, AppState>, query: String) -> Result<Vec<(SnapshotId, domain::Sms)>, String> {
    state.engine.search_sms(&query).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn export_sms_json(state: State<'_, AppState>, snapshot_id: String) -> Result<String, String> {
    let id = SnapshotId(snapshot_id);
    state.engine.export_sms_json(&id).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_snapshot_call_logs(state: State<'_, AppState>, snapshot_id: String) -> Result<Vec<domain::CallLog>, String> {
    let id = SnapshotId(snapshot_id);
    state.engine.get_snapshot_call_logs(&id).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn search_call_logs(state: State<'_, AppState>, query: String) -> Result<Vec<(SnapshotId, domain::CallLog)>, String> {
    state.engine.search_call_logs(&query).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn export_call_logs_json(state: State<'_, AppState>, snapshot_id: String) -> Result<String, String> {
    let id = SnapshotId(snapshot_id);
    state.engine.export_call_logs_json(&id).map_err(|e| e.to_string())
}
