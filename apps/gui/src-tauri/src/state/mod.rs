pub mod storage;
pub mod progress;

pub use storage::{SharedStorage, SwitchableStorage};
pub use progress::CombinedProgress;

use std::sync::Arc;
use application::BackupService;
use adapter_adb::AdbAdapter;
use adapter_database_sqlite::SqliteRepository;
use serde::Serialize;

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
