pub mod progress;
pub mod storage;

pub use progress::CombinedProgress;
pub use storage::{SharedStorage, SwitchableStorage};

use adapter_adb::AdbAdapter;
use adapter_database_sqlite::SqliteRepository;
use application::BackupService;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct DoctorReport {
    pub adb_found: bool,
    pub adb_version: String,
    pub device_count: usize,
    pub db_healthy: bool,
}

use adapter_mtp::{CompositeDeviceAdapter, CompositeScannerAdapter};

pub type AppEngine = BackupService<
    CompositeDeviceAdapter,
    CompositeScannerAdapter,
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
