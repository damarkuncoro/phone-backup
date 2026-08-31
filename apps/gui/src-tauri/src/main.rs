// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
mod commands;
mod setup;

use socketioxide::SocketIo;
use tower_http::cors::CorsLayer;

fn on_connect(socket: socketioxide::extract::SocketRef) {
    println!("New remote monitor connected: {}", socket.id);
}

fn main() {
    let (layer, io) = SocketIo::new_layer();
    io.ns("/", on_connect);

    tauri::Builder::default()
        .setup(move |app| {
            // 1. Background Web Server (Socket.io)
            let layer_clone = layer.clone();
            tauri::async_runtime::spawn(async move {
                let router = axum::Router::new()
                    .route("/", axum::routing::get(|| async { "Phone Backup Remote Server Active" }))
                    .layer(layer_clone)
                    .layer(CorsLayer::permissive());
                if let Ok(listener) = tokio::net::TcpListener::bind("0.0.0.0:3030").await {
                    let _ = axum::serve(listener, router).await;
                }
            });

            // 2. Initialize Infrastructure & Services
            setup::init_infrastructure(app, io.clone())?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::device::get_devices,
            commands::device::get_all_known_devices,
            commands::device::scan_device,
            commands::device::get_live_apps,
            commands::device::browse_directory,
            commands::device::delete_device_file,
            commands::device::rename_device_file,
            commands::device::copy_device_file,
            commands::device::upload_to_device,
            commands::device::download_from_device,
            commands::device::calculate_device_file_hash,
            commands::device::get_device_battery,
            commands::device::get_live_data,
            commands::backup::get_storage_stats,
            commands::backup::start_backup,
            commands::backup::get_snapshots,
            commands::backup::get_snapshot_files,
            commands::backup::get_snapshot_apps,
            commands::backup::get_file_diff,
            commands::backup::get_contact_diff,
            commands::backup::export_contacts_vcard,
            commands::backup::get_snapshot_sms,
            commands::backup::search_sms,
            commands::backup::export_sms_json,
            commands::backup::get_snapshot_call_logs,
            commands::backup::search_call_logs,
            commands::backup::export_call_logs_json,
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
            commands::system::open_restore_folder,
            commands::system::open_downloads_folder,
            commands::contact::search_contacts
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
