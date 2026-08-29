// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use adapter_adb::{AdbAppProvider, AdbDataProvider, AdbDeviceAdapter, AdbScannerAdapter};
use adapter_database_sqlite::SqliteRepository;
use adapter_filesystem::LocalStorage;
use application::BackupService;
use domain::{Device, Snapshot};
use std::sync::Arc;
use tauri::State;

// Define a type alias for our complex BackupService to keep code clean
type AppEngine = BackupService<
    AdbDeviceAdapter,
    AdbScannerAdapter,
    SqliteRepository,
    LocalStorage,
    AdbAppProvider,
    AdbDataProvider,
>;

struct AppState {
    engine: Arc<AppEngine>,
}

#[tauri::command]
async fn get_devices(state: State<'_, AppState>) -> Result<Vec<Device>, String> {
    state.engine.list_devices().map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_storage_stats(state: State<'_, AppState>) -> Result<application::StorageStats, String> {
    state.engine.get_storage_stats().map_err(|e| e.to_string())
}

#[tauri::command]
async fn start_backup(state: State<'_, AppState>, device_id: String) -> Result<Snapshot, String> {
    let id = domain::DeviceId::new(device_id);
    // GUI default uses public key if available, or None for simple local backup
    state
        .engine
        .perform_backup(&id, domain::EncryptionMode::None, None)
        .map_err(|e| e.to_string())
}

fn main() {
    // Initialize infrastructure (same as CLI)
    let repository = SqliteRepository::new("workspace/backup.db").expect("Failed to init database");
    let storage = LocalStorage::new("workspace/backups").expect("Failed to init storage");

    let engine = Arc::new(BackupService::new(
        AdbDeviceAdapter::new(),
        AdbScannerAdapter::new(),
        repository,
        storage,
        AdbAppProvider::new(),
        AdbDataProvider::new(),
    ));

    tauri::Builder::default()
        .manage(AppState { engine })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_devices,
            get_storage_stats,
            start_backup
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
