use tauri::{State, Manager};
use crate::state::{AppState, DoctorReport};

#[tauri::command(rename_all = "snake_case")]
pub async fn get_doctor_report(state: State<'_, AppState>) -> Result<DoctorReport, String> {
    let adb_path = adapter_adb::AdbClient::find_adb();
    let adb_check = std::process::Command::new(&adb_path).arg("version").output();
    let adb_found = adb_check.is_ok();
    let adb_version = if let Ok(out) = adb_check {
        String::from_utf8_lossy(&out.stdout).lines().next().unwrap_or("Unknown").to_string()
    } else {
        "Not Found".to_string()
    };
    let devices = state.engine.list_devices().unwrap_or_default();
    Ok(DoctorReport { adb_found, adb_version, device_count: devices.len(), db_healthy: true })
}

#[tauri::command(rename_all = "snake_case")]
pub async fn run_gc(state: State<'_, AppState>) -> Result<u64, String> {
    state.engine.garbage_collect().map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn prune_failed_snapshots(state: State<'_, AppState>) -> Result<usize, String> {
    state.engine.prune_failed_snapshots().map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn generate_keys() -> Result<(String, String), String> {
    Ok(application::EncryptionEngine::generate_keypair())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn switch_to_mock_storage(state: State<'_, AppState>) -> Result<(), String> {
    state.storage_switcher.switch(Box::new(adapter_mock::MockStorage::new()));
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn switch_to_s3_storage(
    state: State<'_, AppState>,
    bucket: String,
    region: String,
    endpoint: String,
    access_key: String,
    secret_key: String,
) -> Result<(), String> {
    let s3 = adapter_opendal::CloudStorage::new_s3(
        &bucket, &region, &endpoint, &access_key, &secret_key
    ).map_err(|e| e.to_string())?;

    state.storage_switcher.switch(Box::new(s3));
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn search_files(state: State<'_, AppState>, query: String) -> Result<Vec<domain::FileEntry>, String> {
    state.engine.search_files(&query).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn add_schedule(state: State<'_, AppState>, device_id: String) -> Result<(), String> {
    state.engine.add_schedule(domain::DeviceId(device_id), domain::ScheduleFrequency::Daily).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_settings(state: State<'_, AppState>) -> Result<domain::AppSettings, String> {
    state.engine.get_settings().map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn save_settings(state: State<'_, AppState>, settings: domain::AppSettings) -> Result<(), String> {
    state.engine.save_settings(&settings).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn resolve_mtp_conflicts() -> Result<usize, String> {
    adapter_mtp::MtpConflictResolver::kill_conflicts().map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_mtp_conflicts() -> Result<Vec<String>, String> {
    Ok(adapter_mtp::MtpConflictResolver::find_conflicts())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn open_restore_folder(app_handle: tauri::AppHandle) -> Result<(), String> {
    let path = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .join("workspace")
        .join("restored_data");

    if !path.exists() {
        std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn open_downloads_folder(app_handle: tauri::AppHandle) -> Result<(), String> {
    let path = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .join("workspace")
        .join("downloads");

    if !path.exists() {
        std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
