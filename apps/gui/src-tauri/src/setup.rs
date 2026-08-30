use std::sync::Arc;
use tauri::Manager;
use adapter_adb::{AdbAdapter, AdbClient};
use adapter_database_sqlite::SqliteRepository;
use adapter_filesystem::LocalStorage;
use application::BackupService;
use ports::SettingsRepositoryPort;
use crate::state::{AppState, CombinedProgress, SharedStorage, SwitchableStorage};
use socketioxide::SocketIo;

pub fn init_infrastructure(app: &mut tauri::App, io: SocketIo) -> anyhow::Result<()> {
    // 1. Initialize Paths
    let workspace_path = std::env::current_dir()?.join("workspace");
    if !workspace_path.exists() {
        std::fs::create_dir_all(&workspace_path)?;
    }

    let db_path = workspace_path.join("backup.db");
    let storage_path = workspace_path.join("backups");

    // 2. Repositories
    let repository = SqliteRepository::new(db_path.to_str().unwrap())?;
    let settings = repository.get_settings()?.unwrap_or_default();

    // 3. Storage Backend
    let initial_storage: Box<dyn ports::StoragePort> = match &settings.storage_backend {
        domain::StorageBackend::Local => Box::new(LocalStorage::new(storage_path)?),
        domain::StorageBackend::Mock => Box::new(adapter_mock::MockStorage::new()),
        domain::StorageBackend::S3 { bucket, region, endpoint, access_key, secret_key } => {
            match adapter_opendal::CloudStorage::new_s3(&bucket, &region, &endpoint, &access_key, &secret_key) {
                Ok(s3) => Box::new(s3),
                Err(e) => {
                    eprintln!("Failed to connect to saved S3 storage: {}. Falling back to Local.", e);
                    Box::new(LocalStorage::new(storage_path)?)
                }
            }
        }
    };

    let switcher = Arc::new(SwitchableStorage::new(initial_storage));

    // 4. Core Engine
    let adb_client = AdbClient::new();
    let adb_adapter = AdbAdapter::new(adb_client);
    let engine = Arc::new(BackupService::new(
        adb_adapter.clone(),
        adb_adapter.clone(),
        repository,
        SharedStorage(switcher.clone()),
        adb_adapter.clone(),
        adb_adapter.clone(),
        CombinedProgress {
            app_handle: app.handle().clone(),
            io: io
        },
    ));

    // 5. Background Monitors
    spawn_auto_backup_monitor(engine.clone());
    spawn_hardware_monitor(app.handle().clone(), adb_adapter);

    // 6. Manage State
    app.manage(AppState {
        engine,
        storage_switcher: switcher
    });

    Ok(())
}

fn spawn_auto_backup_monitor(engine: Arc<crate::state::AppEngine>) {
    tauri::async_runtime::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));
        loop {
            interval.tick().await;
            let _ = engine.run_pending_backups(domain::EncryptionMode::None);
        }
    });
}

fn spawn_hardware_monitor(app_handle: tauri::AppHandle, adb_adapter: AdbAdapter) {
    let adb_monitor = adb_adapter.monitor();
    std::thread::spawn(move || {
        use adapter_adb::DeviceEvent;
        use tauri::Emitter;

        let _ = adb_monitor.track_devices(|event| {
            match event {
                DeviceEvent::Connected(device) => {
                    let _ = app_handle.emit("device-changed", "connected");
                    let _ = app_handle.emit("device-connected", device);
                }
                DeviceEvent::Disconnected(id) => {
                    let _ = app_handle.emit("device-changed", "disconnected");
                    let _ = app_handle.emit("device-disconnected", id.0);
                }
            }
        });
    });
}
