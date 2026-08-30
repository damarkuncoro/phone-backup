use tauri::State;
use crate::state::AppState;
use domain::{Device, DeviceId};
use serde_json::Value;

#[tauri::command]
pub async fn get_devices(state: State<'_, AppState>) -> Result<Vec<Device>, String> {
    state.engine.list_devices().map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn scan_device(state: State<'_, AppState>, device_id: String) -> Result<Vec<domain::FileEntry>, String> {
    let id = DeviceId::new(device_id);
    state.engine.scan_device(&id).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_device_battery(state: State<'_, AppState>, device_id: String) -> Result<(u32, f32), String> {
    let id = DeviceId::new(device_id);
    state.engine.get_device_battery(&id).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn browse_directory(state: State<'_, AppState>, device_id: String, path: String) -> Result<Vec<domain::FileEntry>, String> {
    let id = DeviceId::new(device_id);
    state.engine.list_directory(&id, &path).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_device_file(state: State<'_, AppState>, device_id: String, path: String) -> Result<(), String> {
    let id = DeviceId::new(device_id);
    state.engine.delete_remote(&id, &path).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn rename_device_file(state: State<'_, AppState>, device_id: String, old_path: String, new_path: String) -> Result<(), String> {
    let id = DeviceId::new(device_id);
    state.engine.rename_remote(&id, &old_path, &new_path).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn copy_device_file(state: State<'_, AppState>, device_id: String, source: String, target: String) -> Result<(), String> {
    let id = DeviceId::new(device_id);
    state.engine.copy_remote(&id, &source, &target).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn upload_to_device(state: State<'_, AppState>, device_id: String, local_path: String, remote_path: String) -> Result<(), String> {
    let id = DeviceId::new(device_id);
    state.engine.upload_file(&id, &local_path, &remote_path).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn calculate_device_file_hash(state: State<'_, AppState>, device_id: String, path: String) -> Result<String, String> {
    let id = DeviceId::new(device_id);
    state.engine.calculate_hash(&id, &path).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_live_data(
    state: State<'_, AppState>,
    device_id: String,
    data_type: String,
) -> Result<Value, String> {
    let id = DeviceId::new(device_id);
    match data_type.as_str() {
        "contacts" => {
            let data = state.engine.list_contacts(&id).map_err(|e| e.to_string())?;
            Ok(serde_json::to_value(data).unwrap())
        }
        "sms" => {
            let data = state.engine.list_sms(&id).map_err(|e| e.to_string())?;
            Ok(serde_json::to_value(data).unwrap())
        }
        "call_logs" => {
            let data = state.engine.list_call_logs(&id).map_err(|e| e.to_string())?;
            Ok(serde_json::to_value(data).unwrap())
        }
        _ => Err("Unsupported data type".to_string()),
    }
}
