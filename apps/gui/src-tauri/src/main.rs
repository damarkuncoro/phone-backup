// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use adapter_adb::{AdbAppProvider, AdbDataProvider, AdbDeviceAdapter, AdbScannerAdapter};
use adapter_database_sqlite::SqliteRepository;
use adapter_filesystem::LocalStorage;
use application::BackupService;
use domain::{Device, DeviceId, Snapshot, SnapshotId};
use ports::{ProgressPort, StoragePort};
use socketioxide::{extract::SocketRef, SocketIo};
use std::sync::{Arc, RwLock};
use tauri::{Emitter, Manager, State};
use tower_http::cors::CorsLayer;

/// Modular Storage Switcher (SOLID - LSP)
pub struct SwitchableStorage {
    current: RwLock<Box<dyn StoragePort>>,
}

impl SwitchableStorage {
    pub fn new(initial: Box<dyn StoragePort>) -> Self {
        Self {
            current: RwLock::new(initial),
        }
    }

    pub fn switch(&self, new_storage: Box<dyn StoragePort>) {
        let mut writer = self.current.write().unwrap();
        *writer = new_storage;
    }
}

impl StoragePort for SwitchableStorage {
    fn write(&self, id: &str, data: &mut dyn std::io::Read) -> anyhow::Result<()> {
        self.current.read().unwrap().write(id, data)
    }
    fn read(&self, id: &str) -> anyhow::Result<Box<dyn std::io::Read>> {
        self.current.read().unwrap().read(id)
    }
    fn exists(&self, id: &str) -> anyhow::Result<bool> {
        self.current.read().unwrap().exists(id)
    }
    fn delete(&self, id: &str) -> anyhow::Result<()> {
        self.current.read().unwrap().delete(id)
    }
    fn list(&self) -> anyhow::Result<Vec<String>> {
        self.current.read().unwrap().list()
    }
}

/// Newtype wrapper to satisfy orphan rules for Arc + Trait
#[derive(Clone)]
pub struct SharedStorage(Arc<SwitchableStorage>);

impl StoragePort for SharedStorage {
    fn write(&self, id: &str, data: &mut dyn std::io::Read) -> anyhow::Result<()> {
        self.0.write(id, data)
    }
    fn read(&self, id: &str) -> anyhow::Result<Box<dyn std::io::Read>> {
        self.0.read(id)
    }
    fn exists(&self, id: &str) -> anyhow::Result<bool> {
        self.0.exists(id)
    }
    fn delete(&self, id: &str) -> anyhow::Result<()> {
        self.0.delete(id)
    }
    fn list(&self) -> anyhow::Result<Vec<String>> {
        self.0.list()
    }
}

// Update the AppEngine type to use our SharedStorage
type AppEngine = BackupService<
    AdbDeviceAdapter,
    AdbScannerAdapter,
    SqliteRepository,
    SharedStorage,
    AdbAppProvider,
    AdbDataProvider,
    CombinedProgress,
>;

struct AppState {
    engine: Arc<AppEngine>,
    storage_switcher: Arc<SwitchableStorage>,
}

pub struct CombinedProgress {
    app_handle: tauri::AppHandle,
    io: SocketIo,
}

impl ProgressPort for CombinedProgress {
    fn start(&self, total: u64, message: &str) {
        let payload = serde_json::json!({ "type": "start", "total": total, "message": message });
        let _ = self.app_handle.emit("progress", &payload);
        let _ = self.io.emit("progress", &payload);
    }
    fn inc(&self, amount: u64, message: &str) {
        let payload = serde_json::json!({ "type": "inc", "amount": amount, "message": message });
        let _ = self.app_handle.emit("progress", &payload);
        let _ = self.io.emit("progress", &payload);
    }
    fn finish(&self, message: &str) {
        let payload = serde_json::json!({ "type": "finish", "message": message });
        let _ = self.app_handle.emit("progress", &payload);
        let _ = self.io.emit("progress", &payload);
    }
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
async fn get_snapshots(state: State<'_, AppState>, device_id: String) -> Result<Vec<Snapshot>, String> {
    let id = DeviceId::new(device_id);
    state.engine.list_snapshots(&id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_snapshot_files(state: State<'_, AppState>, snapshot_id: String) -> Result<Vec<domain::FileEntry>, String> {
    let id = SnapshotId(snapshot_id);
    state.engine.get_snapshot_files(&id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_structured_data(
    state: State<'_, AppState>,
    snapshot_id: String,
    data_type: String
) -> Result<serde_json::Value, String> {
    let id = SnapshotId(snapshot_id);
    // Kita panggil engine untuk mengambil data terstruktur
    state.engine.get_structured_data(&id, &data_type).map_err(|e| e.to_string())
}

#[tauri::command]
async fn scan_device(state: State<'_, AppState>, device_id: String) -> Result<Vec<domain::FileEntry>, String> {
    let id = DeviceId::new(device_id);
    state.engine.scan_device(&id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn start_backup(state: State<'_, AppState>, device_id: String, include_files: Option<Vec<String>>) -> Result<Snapshot, String> {
    let id = DeviceId::new(device_id);
    let policy = include_files.map(|paths| domain::BackupPolicy::builder().include_many(paths).build());
    state.engine.perform_backup(&id, domain::EncryptionMode::None, policy).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_snapshot(state: State<'_, AppState>, snapshot_id: String) -> Result<(), String> {
    let id = SnapshotId(snapshot_id);
    state.engine.delete_snapshot(&id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn run_gc(state: State<'_, AppState>) -> Result<u64, String> {
    state.engine.garbage_collect().map_err(|e| e.to_string())
}

#[derive(serde::Serialize)]
struct DoctorReport {
    adb_found: bool,
    adb_version: String,
    device_count: usize,
    db_healthy: bool,
}

#[tauri::command]
async fn get_doctor_report(state: State<'_, AppState>) -> Result<DoctorReport, String> {
    let adb_check = std::process::Command::new("adb").arg("version").output();
    let adb_found = adb_check.is_ok();
    let adb_version = if let Ok(out) = adb_check {
        String::from_utf8_lossy(&out.stdout).lines().next().unwrap_or("Unknown").to_string()
    } else {
        "Not Found".to_string()
    };
    let devices = state.engine.list_devices().unwrap_or_default();
    Ok(DoctorReport { adb_found, adb_version, device_count: devices.len(), db_healthy: true })
}

#[tauri::command]
async fn generate_keys() -> Result<(String, String), String> {
    Ok(application::EncryptionEngine::generate_keypair())
}

#[tauri::command]
async fn restore_snapshot(state: State<'_, AppState>, snapshot_id: String, target_dir: String) -> Result<(), String> {
    let id = SnapshotId(snapshot_id);
    state.engine.perform_restore(&id, &target_dir, domain::EncryptionMode::None, None).map_err(|e| e.to_string())
}

#[tauri::command]
async fn switch_to_mock_storage(state: State<'_, AppState>) -> Result<(), String> {
    state.storage_switcher.switch(Box::new(adapter_mock::MockStorage::new()));
    Ok(())
}

#[tauri::command]
async fn add_schedule(state: State<'_, AppState>, device_id: String) -> Result<(), String> {
    state.engine.add_schedule(domain::DeviceId(device_id), domain::ScheduleFrequency::Daily).map_err(|e| e.to_string())
}

#[tauri::command]
async fn search_files(state: State<'_, AppState>, query: String) -> Result<Vec<domain::FileEntry>, String> {
    state.engine.search_files(&query).map_err(|e| e.to_string())
}

fn on_connect(socket: SocketRef) {
    println!("New remote monitor connected: {}", socket.id);
}

fn main() {
    let (layer, io) = SocketIo::new_layer();
    io.ns("/", on_connect);

    tauri::Builder::default()
        .setup(move |app| {
            let io_clone = io.clone();
            tauri::async_runtime::spawn(async move {
                let app = axum::Router::new()
                    .route("/", axum::routing::get(|| async { "Phone Backup Remote Server Active" }))
                    .layer(layer)
                    .layer(CorsLayer::permissive());
                if let Ok(listener) = tokio::net::TcpListener::bind("0.0.0.0:3030").await {
                    let _ = axum::serve(listener, app).await;
                }
            });

            let workspace_path = std::env::current_dir().unwrap().join("workspace");
            if !workspace_path.exists() { std::fs::create_dir_all(&workspace_path).unwrap(); }

            let db_path = workspace_path.join("backup.db");
            let storage_path = workspace_path.join("backups");

            let repository = SqliteRepository::new(db_path.to_str().unwrap()).unwrap();
            let switcher = Arc::new(SwitchableStorage::new(Box::new(LocalStorage::new(storage_path).unwrap())));

            let engine = Arc::new(BackupService::new(
                AdbDeviceAdapter::new(),
                AdbScannerAdapter::new(),
                repository,
                SharedStorage(switcher.clone()),
                AdbAppProvider::new(),
                AdbDataProvider::new(),
                CombinedProgress { app_handle: app.handle().clone(), io: io_clone },
            ));

            app.manage(AppState { engine: engine.clone(), storage_switcher: switcher });

            let engine_monitor = engine.clone();
            tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));
                loop {
                    interval.tick().await;
                    let _ = engine_monitor.run_pending_backups(domain::EncryptionMode::None);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_devices, get_storage_stats, get_snapshots, start_backup, delete_snapshot,
            run_gc, restore_snapshot, get_doctor_report, generate_keys, add_schedule,
            get_snapshot_files, get_structured_data, search_files, scan_device, switch_to_mock_storage
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
