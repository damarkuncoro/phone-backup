mod common;

use common::setup_test_repo;
use domain::{Device, DeviceId, ConnectionType, FileEntry, FileId};
use ports::{DeviceRepositoryPort, FileRepositoryPort, MaintenanceRepositoryPort};
use chrono::Utc;

#[test]
fn test_maintenance_referenced_hashes() {
    let repo = setup_test_repo();
    let device_id = DeviceId::new("dev-maint");
    repo.save_device(&Device {
        id: device_id.clone(),
        manufacturer: "A".to_string(), model: "B".to_string(), serial: "C".to_string(),
        os_version: "D".to_string(), sdk_version: None,
        storage_total_bytes: 0, storage_used_bytes: 0, storage_free_bytes: 0,
        connection_type: ConnectionType::Usb,
    }).unwrap();

    // 1. Hash from a file
    repo.save_file(&FileEntry {
        id: FileId("f1".to_string()), device_id: device_id.clone(),
        path: "p1".to_string(), name: "n1".to_string(), size_bytes: 10,
        modified_at: Utc::now(), mime_type: "t".to_string(), permissions: "p".to_string(),
        hash_sha256: Some("hash-file-1".to_string()), media_info: None,
    }).unwrap();

    // 2. Hash from a chunk
    repo.save_file(&FileEntry {
        id: FileId("f2".to_string()), device_id: device_id.clone(),
        path: "p2".to_string(), name: "n2".to_string(), size_bytes: 10,
        modified_at: Utc::now(), mime_type: "t".to_string(), permissions: "p".to_string(),
        hash_sha256: None, media_info: None,
    }).unwrap();
    repo.save_file_chunk(&FileId("f2".to_string()), "hash-chunk-1", 0, 10, 0).unwrap();

    let all_hashes = repo.get_all_referenced_hashes().unwrap();
    assert!(all_hashes.contains("hash-file-1"));
    assert!(all_hashes.contains("hash-chunk-1"));
}

#[test]
fn test_database_optimization() {
    let repo = setup_test_repo();
    // Simply ensure it doesn't crash
    repo.optimize().unwrap();
}
