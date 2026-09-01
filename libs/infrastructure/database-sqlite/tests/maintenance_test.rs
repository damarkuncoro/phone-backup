mod common;

use common::setup_test_repo;
use phone_backup_adapter_database_sqlite::SqliteRepository;
use domain::{Device, DeviceId, ConnectionType, FileEntry, FileId, SnapshotId};
use ports::{DeviceRepositoryPort, FileRepositoryPort, MaintenanceRepositoryPort, SnapshotRepositoryPort, ContactRepositoryPort};
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
        hash_sha256: Some("hash-file-1".to_string()),
        thumbnail_hash: None,
        media_info: None,
    }).unwrap();

    // 2. Hash from a chunk
    repo.save_file(&FileEntry {
        id: FileId("f2".to_string()), device_id: device_id.clone(),
        path: "p2".to_string(), name: "n2".to_string(), size_bytes: 10,
        modified_at: Utc::now(), mime_type: "t".to_string(), permissions: "p".to_string(),
        hash_sha256: None,
        thumbnail_hash: None,
        media_info: None,
    }).unwrap();

    let chunk_id = repo.save_logical_chunk("hash-chunk-1", 10).unwrap();
    repo.save_physical_object(&chunk_id, "obj-hash-1", "uuid-1", 10, "none", 0).unwrap();
    repo.save_file_chunk(&FileId("f2".to_string()), &chunk_id, 0, 10, 0).unwrap();

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

#[test]
fn test_prune_orphans() {
    let repo = setup_test_repo();
    let device_id = DeviceId::new("dev-prune");
    let snap_id = SnapshotId("snap-prune".to_string());

    repo.save_device(&domain::Device {
        id: device_id.clone(), manufacturer: "A".to_string(), model: "B".to_string(), serial: "C".to_string(),
        os_version: "D".to_string(), sdk_version: None, storage_total_bytes: 0, storage_used_bytes: 0,
        storage_free_bytes: 0, connection_type: domain::ConnectionType::Usb,
    }).unwrap();

    // 1. Create a contact that will be linked
    let c1 = domain::Contact {
        id: "c1".to_string(), snapshot_id: None, source_id: None, display_name: "Linked".to_string(),
        notes: None, source: "s".to_string(), source_account: None, content_hash: Some("h1".to_string()),
        metadata_json: None, names: vec![], phones: vec![], emails: vec![], addresses: vec![],
        organizations: vec![], urls: vec![], events: vec![], photos: vec![], labels: vec![],
    };

    // 2. Create a contact that will NOT be linked (Orphan)
    let c2 = domain::Contact {
        id: "c2".to_string(), snapshot_id: None, source_id: None, display_name: "Orphan".to_string(),
        notes: None, source: "s".to_string(), source_account: None, content_hash: Some("h2".to_string()),
        metadata_json: None, names: vec![], phones: vec![], emails: vec![], addresses: vec![],
        organizations: vec![], urls: vec![], events: vec![], photos: vec![], labels: vec![],
    };

    repo.create_snapshot(&domain::Snapshot {
        id: snap_id.clone(), device_id: device_id.clone(), started_at: Utc::now(),
        finished_at: None, status: domain::SnapshotStatus::Pending, total_files: 0,
        total_bytes: 0, deduped_bytes: 0,
    }).unwrap();

    repo.save_contact(&snap_id, &c1).unwrap();

    // Save c2 without a snapshot link (using a dummy snapshot then deleting it to orphan it)
    let dummy_snap = SnapshotId("dummy".to_string());
    repo.create_snapshot(&domain::Snapshot {
        id: dummy_snap.clone(), device_id: device_id.clone(), started_at: Utc::now(),
        finished_at: None, status: domain::SnapshotStatus::Pending, total_files: 0,
        total_bytes: 0, deduped_bytes: 0,
    }).unwrap();
    repo.save_contact(&dummy_snap, &c2).unwrap();
    repo.delete_snapshot(&dummy_snap).unwrap();

    // Prune orphans
    let deleted = repo.prune_orphans().unwrap();
    assert!(deleted >= 1); // c2 should be deleted

    // Verify c1 still exists, c2 is gone
    let contacts = repo.get_snapshot_contacts(&snap_id).unwrap();
    assert_eq!(contacts.len(), 1);
    assert_eq!(contacts[0].display_name, "Linked");
}

#[test]
fn test_database_backup() {
    let repo = setup_test_repo();
    let tmp_dir = tempfile::TempDir::new().unwrap();
    let backup_path = tmp_dir.path().join("backup.db");
    let backup_path_str = backup_path.to_str().unwrap();

    // Ensure we have some data
    repo.save_device(&domain::Device {
        id: domain::DeviceId::new("dev-backup"), manufacturer: "A".to_string(), model: "B".to_string(), serial: "C".to_string(),
        os_version: "12".to_string(), sdk_version: None, storage_total_bytes: 0, storage_used_bytes: 0,
        storage_free_bytes: 0, connection_type: domain::ConnectionType::Usb,
    }).unwrap();

    repo.create_database_backup(backup_path_str).unwrap();

    assert!(backup_path.exists());

    // Verify backup content by opening it
    let backup_repo = SqliteRepository::new(backup_path_str).unwrap();
    let devices = backup_repo.list_devices().unwrap();
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].id.0, "dev-backup");
}
