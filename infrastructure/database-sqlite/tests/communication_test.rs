mod common;

use common::setup_test_repo;
use domain::{Device, DeviceId, ConnectionType, Snapshot, SnapshotId, SnapshotStatus, Sms, CallLog};
use ports::{DeviceRepositoryPort, SnapshotRepositoryPort, SmsRepositoryPort, CallLogRepositoryPort};
use chrono::Utc;

#[test]
fn test_sms_persistence() {
    let repo = setup_test_repo();
    let device_id = DeviceId::new("dev-comm");
    let snap_id = SnapshotId("snap-comm".to_string());

    repo.save_device(&Device {
        id: device_id.clone(), manufacturer: "A".to_string(), model: "B".to_string(), serial: "C".to_string(),
        os_version: "12".to_string(), sdk_version: None, storage_total_bytes: 0, storage_used_bytes: 0,
        storage_free_bytes: 0, connection_type: ConnectionType::Usb,
    }).unwrap();

    repo.create_snapshot(&Snapshot {
        id: snap_id.clone(), device_id: device_id.clone(), started_at: Utc::now(),
        finished_at: None, status: SnapshotStatus::Pending, total_files: 0, total_bytes: 0, deduped_bytes: 0,
    }).unwrap();

    let sms = Sms {
        address: "+62812345678".to_string(),
        body: "Hello world!".to_string(),
        date: Utc::now(),
        type_code: 1,
    };

    repo.save_sms(&snap_id, &sms).unwrap();

    let results = repo.get_snapshot_sms(&snap_id).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].body, "Hello world!");

    // Test Search
    let search_results = repo.search_sms("world").unwrap();
    assert_eq!(search_results.len(), 1);
    assert_eq!(search_results[0].1.body, "Hello world!");
}

#[test]
fn test_call_log_persistence() {
    let repo = setup_test_repo();
    let device_id = DeviceId::new("dev-call");
    let snap_id = SnapshotId("snap-call".to_string());

    repo.save_device(&Device {
        id: device_id.clone(), manufacturer: "A".to_string(), model: "B".to_string(), serial: "C".to_string(),
        os_version: "12".to_string(), sdk_version: None, storage_total_bytes: 0, storage_used_bytes: 0,
        storage_free_bytes: 0, connection_type: ConnectionType::Usb,
    }).unwrap();

    repo.create_snapshot(&Snapshot {
        id: snap_id.clone(), device_id: device_id.clone(), started_at: Utc::now(),
        finished_at: None, status: SnapshotStatus::Pending, total_files: 0, total_bytes: 0, deduped_bytes: 0,
    }).unwrap();

    let log = CallLog {
        number: "+62812345678".to_string(),
        name: Some("John".to_string()),
        date: Utc::now(),
        duration_seconds: 120,
        type_code: 1,
        location: None,
    };

    repo.save_call_log(&snap_id, &log).unwrap();

    let results = repo.get_snapshot_call_logs(&snap_id).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].number, "+62812345678");

    // Test Search
    let search_results = repo.search_call_logs("John").unwrap();
    assert_eq!(search_results.len(), 1);
    assert_eq!(search_results[0].1.name.as_deref(), Some("John"));
}
