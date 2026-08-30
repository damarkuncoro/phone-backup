// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
mod commands;

use std::sync::Arc;
use tauri::Manager;
use socketioxide::SocketIo;
use tower_http::cors::CorsLayer;

use adapter_adb::{AdbAdapter, AdbClient};
use adapter_database_sqlite::SqliteRepository;
use adapter_filesystem::LocalStorage;
use application::BackupService;
use ports::SettingsRepositoryPort;

use crate::state::{AppState, CombinedProgress, SharedStorage, SwitchableStorage};

fn on_connect(socket: socketioxide::extract::SocketRef) {
    println!("New remote monitor connected: {}", socket.id);
}

fn main() {
    let (layer, io) = SocketIo::new_layer();
    io.ns("/", on_connect);

    tauri::Builder::default()
        .setup(move |app| {
            let io_clone = io.clone();

            // 1. Background Web Server (Socket.io)
            tauri::async_runtime::spawn(async move {
                let router = axum::Router::new()
                    .route("/", axum::routing::get(|| async { "Phone Backup Remote Server Active" }))
                    .layer(layer)
                    .layer(CorsLayer::permissive());
                if let Ok(listener) = tokio::net::TcpListener::bind("0.0.0.0:3030").await {
                    let _ = axum::serve(listener, router).await;
                }
            });

            // 2. Initialize Infrastructure
            let workspace_path = std::env::current_dir().unwrap().join("workspace");
            if !workspace_path.exists() {
                std::fs::create_dir_all(&workspace_path).unwrap();
            }

            let db_path = workspace_path.join("backup.db");
            let storage_path = workspace_path.join("backups");

            let repository = SqliteRepository::new(db_path.to_str().unwrap()).unwrap();

            // Load settings and apply storage backend
            let settings = repository.get_settings().unwrap().unwrap_or_default();
            let initial_storage: Box<dyn ports::StoragePort> = match &settings.storage_backend {
                domain::StorageBackend::Local => Box::new(LocalStorage::new(storage_path).unwrap()),
                domain::StorageBackend::Mock => Box::new(adapter_mock::MockStorage::new()),
                domain::StorageBackend::S3 { bucket, region, endpoint, access_key, secret_key } => {
                    match adapter_opendal::CloudStorage::new_s3(&bucket, &region, &endpoint, &access_key, &secret_key) {
                        Ok(s3) => Box::new(s3),
                        Err(e) => {
                            eprintln!("Failed to connect to saved S3 storage: {}. Falling back to Local.", e);
                            Box::new(LocalStorage::new(storage_path).unwrap())
                        }
                    }
                }
            };

            let switcher = Arc::new(SwitchableStorage::new(initial_storage));

            // 3. Initialize Core Engine
            let adb_client = AdbClient::new();
            let adb_adapter = AdbAdapter::new(adb_client);
            let engine = Arc::new(BackupService::new(
                adb_adapter.clone(),
                adb_adapter.clone(),
                repository,
                SharedStorage(switcher.clone()),
                adb_adapter.clone(),
                adb_adapter,
                CombinedProgress {
                    app_handle: app.handle().clone(),
                    io: io_clone
                },
            ));

            // 4. Manage State
            app.manage(AppState {
                engine: engine.clone(),
                storage_switcher: switcher
            });

            // 5. Background Auto-Backup Monitor
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
            commands::device::get_devices,
            commands::device::scan_device,
            commands::device::get_live_data,
            commands::backup::get_storage_stats,
            commands::backup::start_backup,
            commands::backup::get_snapshots,
            commands::backup::get_snapshot_files,
            commands::backup::get_structured_data,
            commands::backup::restore_snapshot,
            commands::backup::delete_snapshot,
            commands::system::get_doctor_report,
            commands::system::run_gc,
            commands::system::prune_failed_snapshots,
            commands::system::generate_keys,
            commands::system::switch_to_mock_storage,
            commands::system::switch_to_s3_storage,
            commands::system::search_files,
            commands::system::add_schedule,
            commands::system::get_settings,
            commands::system::save_settings,
            commands::contact::search_contacts
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
