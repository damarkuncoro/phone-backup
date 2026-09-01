use std::sync::Arc;
use tauri::Manager;
use adapter_adb::{AdbAdapter, AdbClient};
use adapter_mtp::{CompositeDeviceAdapter, CompositeScannerAdapter, MtpAdapter};
use adapter_database_sqlite::SqliteRepository;
use adapter_filesystem::LocalStorage;
use application::BackupService;
use ports::SettingsRepositoryPort;
use crate::state::{AppState, CombinedProgress, SharedStorage, SwitchableStorage};
use socketioxide::SocketIo;
use tracing::{info, warn, error};

pub fn init_infrastructure(app: &mut tauri::App, io: SocketIo) -> anyhow::Result<()> {
    // 1. Initialize Paths - Use App Data Dir for stability on macOS/Windows
    info!("  -> Step 1: Resolving application paths...");
    let app_handle = app.handle();
    let workspace_path = app_handle.path().app_data_dir()?.join("workspace");

    if !workspace_path.exists() {
        info!("Creating workspace directory at {:?}", workspace_path);
        std::fs::create_dir_all(&workspace_path)?;
    }

    let db_path = workspace_path.join("backup.db");
    let storage_path = workspace_path.join("backups");

    // 2. Repositories
    info!("  -> Step 2: Initializing SQLite database at {:?}", db_path);
    let db_path_str = db_path.to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid database path encoding"))?;
    let repository = SqliteRepository::new(db_path_str)?;
    let settings = repository.get_settings()?.unwrap_or_default();

    // 3. Storage Backend
    info!("  -> Step 3: Configuring storage backend...");
    let initial_storage: Box<dyn ports::StoragePort> = match &settings.storage_backend {
        domain::StorageBackend::Local => {
            info!("Using Local Storage at {:?}", storage_path);
            Box::new(LocalStorage::new(storage_path)?)
        },
        domain::StorageBackend::Mock => {
            warn!("Using Mock Storage Backend (In-Memory)");
            Box::new(adapter_mock::MockStorage::new())
        },
        domain::StorageBackend::S3 { bucket, region, endpoint, access_key, secret_key } => {
            info!("Connecting to S3 bucket: {}", bucket);
            match adapter_opendal::CloudStorage::new_s3(&bucket, &region, &endpoint, &access_key, &secret_key) {
                Ok(s3) => Box::new(s3),
                Err(e) => {
                    error!("Failed to connect to saved S3 storage: {}. Falling back to Local.", e);
                    Box::new(LocalStorage::new(storage_path)?)
                }
            }
        }
    };

    let switcher = Arc::new(SwitchableStorage::new(initial_storage));

    // 4. Core Engine
    info!("  -> Step 4: Building core engine service...");
    let adb_client = AdbClient::new();
    let adb_adapter = AdbAdapter::new(adb_client);
    let mtp_adapter = MtpAdapter::new();

    let composite_device = CompositeDeviceAdapter::new(
        Arc::new(adb_adapter.clone()),
        Arc::new(mtp_adapter.clone()),
    );
    let composite_scanner = CompositeScannerAdapter::new(
        Arc::new(adb_adapter.clone()),
        Arc::new(mtp_adapter),
    );

    let engine = Arc::new(BackupService::new(
        composite_device.clone(),
        composite_scanner,
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
    info!("  -> Step 5: Spawning background monitors...");
    spawn_auto_backup_monitor(engine.clone());
    spawn_hardware_monitor(app.handle().clone(), engine.clone(), adb_adapter.clone());
    spawn_status_poller(app.handle().clone(), composite_device);

    // 6. Manage State
    info!("  -> Step 6: Registering application state...");
    app.manage(AppState {
        engine,
        storage_switcher: switcher
    });

    info!("  -> Infrastructure initialized successfully.");
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

fn spawn_hardware_monitor(app_handle: tauri::AppHandle, engine: Arc<crate::state::AppEngine>, adb_adapter: AdbAdapter) {
    let adb_monitor = adb_adapter.monitor();
    std::thread::spawn(move || {
        use adapter_adb::DeviceEvent;
        use tauri::Emitter;

        let _ = adb_monitor.track_devices(|event| {
            match event {
                DeviceEvent::Connected(device) => {
                    info!("Hardware Monitor: Device connected: {}", device.model);
                    let _ = app_handle.emit("device-changed", "connected");
                    let _ = app_handle.emit("device-connected", device.clone());

                    // Auto-Backup Daemon (Plug & Forget)
                    let engine_clone = engine.clone();
                    let app_handle_clone = app_handle.clone();
                    let dev_id = device.id.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = app_handle_clone.emit("auto-backup-started", dev_id.0.clone());
                        let result = engine_clone.trigger_on_connect_backup(&dev_id, domain::EncryptionMode::None);
                        if let Ok(triggered) = result {
                            if triggered {
                                info!("Auto-backup triggered for device {}", dev_id.0);
                                let _ = app_handle_clone.emit("auto-backup-finished", dev_id.0.clone());
                            }
                        }
                    });
                }
                DeviceEvent::Disconnected(id) => {
                    info!("Hardware Monitor: Device disconnected: {}", id.0);
                    let _ = app_handle.emit("device-changed", "disconnected");
                    let _ = app_handle.emit("device-disconnected", id.0);
                }
            }
        });
    });
}

fn spawn_status_poller(app_handle: tauri::AppHandle, device_adapter: CompositeDeviceAdapter) {
    use ports::DevicePort;
    use tauri::Emitter;

    std::thread::spawn(move || {
        loop {
            if let Ok(devices) = device_adapter.discover() {
                for device in devices {
                    if let Ok((level, temp)) = device_adapter.battery_status(&device.id) {
                        let payload = serde_json::json!({
                            "device_id": device.id.0,
                            "battery_level": level,
                            "temperature": temp
                        });
                        let _ = app_handle.emit("device-status-update", payload);
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(30));
        }
    });
}
