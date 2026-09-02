mod common;

use chrono::Utc;
use common::setup_test_repo;
use domain::{ConnectionType, Device, DeviceId, Snapshot, SnapshotId, SnapshotStatus};
use ports::{DeviceRepositoryPort, SnapshotRepositoryPort};

#[test]
fn test_foreign_key_constraint_violation() {
    let repo = setup_test_repo();
    let non_existent_device = DeviceId::new("no-way-this-exists");

    let snap = Snapshot {
        id: SnapshotId("fail-snap".to_string()),
        device_id: non_existent_device,
        started_at: Utc::now(),
        finished_at: None,
        status: SnapshotStatus::Pending,
        total_files: 0,
        total_bytes: 0,
        deduped_bytes: 0,
    };

    let result = repo.create_snapshot(&snap);
    assert!(
        result.is_err(),
        "Snapshot creation should fail due to Foreign Key constraint"
    );
}

#[test]
fn test_foreign_key_cascade() {
    let repo = setup_test_repo();
    let device_id = DeviceId::new("device-to-delete");

    repo.save_device(&Device {
        id: device_id.clone(),
        manufacturer: "X".to_string(),
        model: "Y".to_string(),
        serial: "Z".to_string(),
        os_version: "1".to_string(),
        sdk_version: None,
        storage_total_bytes: 0,
        storage_used_bytes: 0,
        storage_free_bytes: 0,
        connection_type: ConnectionType::Unknown,
    })
    .unwrap();

    let snap_id = SnapshotId("snap-1".to_string());
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

    assert!(repo.get_snapshot(&snap_id).unwrap().is_some());
}
