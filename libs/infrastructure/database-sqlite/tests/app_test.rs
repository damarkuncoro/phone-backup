mod common;

use chrono::Utc;
use common::setup_test_repo;
use domain::{
    AppId, AppInfo, ConnectionType, Device, DeviceId, Snapshot, SnapshotId, SnapshotStatus,
};
use ports::{AppRepositoryPort, DeviceRepositoryPort, SnapshotRepositoryPort};

#[test]
fn test_app_crud_and_linking() {
    let repo = setup_test_repo();
    let device_id = DeviceId::new("dev-app");
    let snap_id = SnapshotId("snap-app".to_string());
    let app_id = AppId("com.whatsapp".to_string());

    // Setup Device & Snapshot
    repo.save_device(&Device {
        id: device_id.clone(),
        manufacturer: "A".to_string(),
        model: "B".to_string(),
        serial: "C".to_string(),
        os_version: "12".to_string(),
        sdk_version: None,
        storage_total_bytes: 0,
        storage_used_bytes: 0,
        storage_free_bytes: 0,
        connection_type: ConnectionType::Usb,
    })
    .unwrap();

    repo.create_snapshot(&Snapshot {
        id: snap_id.clone(),
        device_id: device_id.clone(),
        started_at: Utc::now(),
        finished_at: None,
        status: SnapshotStatus::Pending,
        total_files: 0,
        total_bytes: 0,
        deduped_bytes: 0,
    })
    .unwrap();

    // CREATE (Save App)
    let app = AppInfo {
        id: app_id.clone(),
        device_id: device_id.clone(),
        package_name: "com.whatsapp".to_string(),
        version_name: "2.23.1".to_string(),
        version_code: 12345,
        installer: Some("com.android.vending".to_string()),
        app_name: "WhatsApp".to_string(),
    };
    repo.save_app(&app).unwrap();

    // LINK to snapshot
    repo.link_app_to_snapshot(&snap_id, &app_id).unwrap();

    // READ
    let apps = repo.get_snapshot_apps(&snap_id).unwrap();
    assert_eq!(apps.len(), 1);
    assert_eq!(apps[0].app_name, "WhatsApp");

    // UPDATE (Same app, new version)
    let mut updated_app = app;
    updated_app.version_name = "2.23.2".to_string();
    repo.save_app(&updated_app).unwrap();

    let apps_after = repo.get_snapshot_apps(&snap_id).unwrap();
    assert_eq!(apps_after[0].version_name, "2.23.2");
}
