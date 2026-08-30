use std::sync::{Arc, RwLock};
use application::BackupService;
use adapter_adb::AdbAdapter;
use adapter_database_sqlite::SqliteRepository;
use ports::{ProgressPort, StoragePort};
use socketioxide::SocketIo;
use tauri::Emitter;
use std::io::Read;
use serde::Serialize;

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
    fn write(&self, id: &str, data: &mut dyn Read) -> anyhow::Result<()> {
        self.current.read().unwrap().write(id, data)
    }
    fn read(&self, id: &str) -> anyhow::Result<Box<dyn Read>> {
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
pub struct SharedStorage(pub Arc<SwitchableStorage>);

impl StoragePort for SharedStorage {
    fn write(&self, id: &str, data: &mut dyn Read) -> anyhow::Result<()> {
        self.0.write(id, data)
    }
    fn read(&self, id: &str) -> anyhow::Result<Box<dyn Read>> {
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

pub struct CombinedProgress {
    pub app_handle: tauri::AppHandle,
    pub io: SocketIo,
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
    fn error(&self, message: &str) {
        let payload = serde_json::json!({ "type": "error", "message": message });
        let _ = self.app_handle.emit("progress", &payload);
        let _ = self.io.emit("progress", &payload);
    }
    fn log(&self, message: &str) {
        let payload = serde_json::json!({ "type": "log", "message": message });
        let _ = self.app_handle.emit("progress", &payload);
        let _ = self.io.emit("progress", &payload);
    }
}

#[derive(Serialize)]
pub struct DoctorReport {
    pub adb_found: bool,
    pub adb_version: String,
    pub device_count: usize,
    pub db_healthy: bool,
}

pub type AppEngine = BackupService<
    AdbAdapter,
    AdbAdapter,
    SqliteRepository,
    SharedStorage,
    AdbAdapter,
    AdbAdapter,
    CombinedProgress,
>;

pub struct AppState {
    pub engine: Arc<AppEngine>,
    pub storage_switcher: Arc<SwitchableStorage>,
}
