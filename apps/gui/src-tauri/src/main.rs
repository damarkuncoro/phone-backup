// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod setup;
mod state;
mod tray;

use socketioxide::SocketIo;
use tower_http::cors::CorsLayer;
use tracing::{error, info};

fn on_connect(socket: socketioxide::extract::SocketRef) {
    info!("New remote monitor connected: {}", socket.id);
}

fn main() {
    // 1. Initialize formal logging
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        .add_directive("nusb=off".parse().unwrap());
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // 2. Setup diagnostic panic hook
    std::panic::set_hook(Box::new(|info| {
        error!("====================================================");
        error!("🔥 CRITICAL: PHONE BACKUP GUI PANICKED!");
        error!("Details: {}", info);
        error!("====================================================");
    }));

    let (layer, io) = SocketIo::new_layer();
    io.ns("/", on_connect);

    let app_result = tauri::Builder::default()
        .setup(move |app| {
            info!("🚀 Tauri setup phase started");

            // 1. Background Web Server (Socket.io)
            let layer_clone = layer.clone();
            tauri::async_runtime::spawn(async move {
                let router = axum::Router::new()
                    .route(
                        "/",
                        axum::routing::get(|| async { "Phone Backup Remote Server Active" }),
                    )
                    .layer(layer_clone)
                    .layer(CorsLayer::permissive());
                if let Ok(listener) = tokio::net::TcpListener::bind("0.0.0.0:3030").await {
                    let _ = axum::serve(listener, router).await;
                }
            });

            // 2. Initialize Infrastructure & Services
            info!("📦 Initializing Infrastructure...");
            setup::init_infrastructure(app, io.clone()).map_err(|e| {
                error!("❌ Infrastructure Initialization Failed: {}", e);
                e
            })?;

            // 3. Initialize System Tray Icon
            info!("🔔 Initializing System Tray Icon...");
            let _ = tray::TrayManager::setup_tray(app.handle());

            info!("✅ Setup completed successfully");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::device::get_devices,
            commands::device::get_all_known_devices,
            commands::device::scan_device,
            commands::device::scan_device_detailed,
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
            commands::system::get_mtp_conflicts,
            commands::system::resolve_mtp_conflicts,
            commands::system::open_restore_folder,
            commands::system::open_downloads_folder,
            commands::contact::search_contacts,
            commands::communication::export_contacts_csv,
            commands::communication::export_sms_xml,
            commands::communication::export_sms_html,
            commands::communication::get_call_stats,
            commands::communication::generate_whatsapp_archive_preview,
            commands::communication::get_whatsapp_sync_status,
            commands::communication::get_synced_whatsapp_html,
            commands::communication::get_whatsapp_qr_html,
            commands::media::analyze_audio_file,
            commands::media::check_image_sharpness,
            commands::app_audit::audit_apk_file,
            commands::cloud::test_cloud_connection,
            commands::wireless::get_wireless_pairing_info,
            commands::wireless::connect_wireless_device,
            commands::datavault::get_wifi_vault,
            commands::datavault::get_wifi_qr,
            commands::datavault::get_bookmarks_vault,
            commands::datavault::get_notes_vault,
            commands::datavault::get_calendar_vault,
        ])
        .run(tauri::generate_context!());

    if let Err(e) = app_result {
        error!("❌ Tauri Application runtime error: {}", e);
    }
}
