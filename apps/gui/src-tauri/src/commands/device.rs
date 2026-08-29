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
