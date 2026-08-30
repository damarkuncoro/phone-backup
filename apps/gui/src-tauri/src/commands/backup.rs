use tauri::State;
use crate::state::AppState;
use domain::{DeviceId, Snapshot, SnapshotId};
use serde_json::Value;

#[tauri::command]
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
    state.engine.get_structured_data(&id, &data_type).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn restore_snapshot(
    state: State<'_, AppState>,
    snapshot_id: String,
    target_dir: String,
    filter: Option<String>,
) -> Result<(), String> {
    let id = SnapshotId(snapshot_id);
    state
        .engine
        .perform_restore(&id, &target_dir, domain::EncryptionMode::None, filter.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_snapshot(state: State<'_, AppState>, snapshot_id: String) -> Result<(), String> {
    let id = SnapshotId(snapshot_id);
    state.engine.delete_snapshot(&id).map_err(|e| e.to_string())
}
