mod common;

use chrono::Utc;
use common::setup_test_repo;
use domain::{ConnectionType, Device, DeviceId, Snapshot, SnapshotId, SnapshotStatus};
use ports::{DeviceRepositoryPort, SnapshotRepositoryPort};

#[test]
fn test_snapshot_lifecycle() {
    let repo = setup_test_repo();
    let device_id = DeviceId::new("dev-snap");

    repo.save_device(&Device {
        id: device_id.clone(),
        manufacturer: "A".to_string(),
        model: "B".to_string(),
        serial: "C".to_string(),
        os_version: "D".to_string(),
        sdk_version: None,
        storage_total_bytes: 0,
        storage_used_bytes: 0,
        storage_free_bytes: 0,
        connection_type: ConnectionType::Usb,
    })
    .unwrap();

    let snap = Snapshot {
        id: SnapshotId("s1".to_string()),
        device_id: device_id.clone(),
        started_at: Utc::now(),
        finished_at: None,
        status: SnapshotStatus::Pending,
        total_files: 0,
        total_bytes: 0,
        deduped_bytes: 0,
    };

    repo.create_snapshot(&snap).unwrap();

    let saved = repo.get_snapshot(&snap.id).unwrap().unwrap();
    assert_eq!(saved.status, SnapshotStatus::Pending);

    let mut updated = saved;
    updated.status = SnapshotStatus::Completed;
    updated.finished_at = Some(Utc::now());
    repo.update_snapshot(&updated).unwrap();

    let after_update = repo.get_snapshot(&snap.id).unwrap().unwrap();
    assert_eq!(after_update.status, SnapshotStatus::Completed);
}

#[test]
fn test_storage_usage_calculation() {
    let repo = setup_test_repo();
    let device_id = DeviceId::new("test-device");

    repo.save_device(&Device {
        id: device_id.clone(),
        manufacturer: "Test".to_string(),
        model: "Model".to_string(),
        serial: "123".to_string(),
        os_version: "14".to_string(),
        sdk_version: Some(34),
        storage_total_bytes: 1000,
        storage_used_bytes: 500,
        storage_free_bytes: 500,
        connection_type: ConnectionType::Usb,
    })
    .unwrap();

    let s1 = Snapshot {
        id: SnapshotId("s1".to_string()),
        device_id: device_id.clone(),
        started_at: Utc::now(),
        finished_at: Some(Utc::now()),
        status: SnapshotStatus::Completed,
        total_files: 10,
        total_bytes: 100,
        deduped_bytes: 80,
    };
    let s2 = Snapshot {
        id: SnapshotId("s2".to_string()),
        device_id: device_id.clone(),
        started_at: Utc::now(),
        finished_at: Some(Utc::now()),
        status: SnapshotStatus::Completed,
        total_files: 5,
        total_bytes: 200,
        deduped_bytes: 150,
    };

    repo.create_snapshot(&s1).unwrap();
    repo.create_snapshot(&s2).unwrap();

    let usage = repo.get_storage_usage_by_device(&device_id).unwrap();
    assert_eq!(usage, 300);
}

#[test]
fn test_snapshot_deletion_cascade() {
    let repo = setup_test_repo();
    let device_id = DeviceId::new("dev-del");
    let snap_id = SnapshotId("snap-del".to_string());

    repo.save_device(&Device {
        id: device_id.clone(),
        manufacturer: "A".to_string(),
        model: "B".to_string(),
        serial: "C".to_string(),
        os_version: "D".to_string(),
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

    repo.delete_snapshot(&snap_id).unwrap();
    assert!(repo.get_snapshot(&snap_id).unwrap().is_none());
}
