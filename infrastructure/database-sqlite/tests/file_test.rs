mod common;

use common::setup_test_repo;
use domain::{Device, DeviceId, ConnectionType, Snapshot, SnapshotId, SnapshotStatus, FileEntry, FileId};
use ports::{DeviceRepositoryPort, SnapshotRepositoryPort, FileRepositoryPort};
use chrono::Utc;

#[test]
fn test_file_persistence_and_linking() {
    let repo = setup_test_repo();
    let device_id = DeviceId::new("dev1");
    let snap_id = SnapshotId("snap1".to_string());
    let file_id = FileId("file1".to_string());

    repo.save_device(&Device {
        id: device_id.clone(),
        manufacturer: "A".to_string(), model: "B".to_string(), serial: "C".to_string(),
        os_version: "D".to_string(), sdk_version: None,
        storage_total_bytes: 0, storage_used_bytes: 0, storage_free_bytes: 0,
        connection_type: ConnectionType::Usb,
    }).unwrap();

    repo.create_snapshot(&Snapshot {
        id: snap_id.clone(), device_id: device_id.clone(), started_at: Utc::now(),
        finished_at: None, status: SnapshotStatus::Running,
        total_files: 0, total_bytes: 0, deduped_bytes: 0,
    }).unwrap();

    repo.save_file(&FileEntry {
        id: file_id.clone(), device_id: device_id.clone(),
        path: "/sdcard/photo.jpg".to_string(), name: "photo.jpg".to_string(),
        size_bytes: 1024, modified_at: Utc::now(), mime_type: "image/jpeg".to_string(),
        permissions: "-rw-".to_string(), hash_sha256: Some("hash123".to_string()),
        thumbnail_hash: None,
        media_info: None,
    }).unwrap();

    repo.link_file_to_snapshot(&snap_id, &file_id).unwrap();

    let files = repo.get_snapshot_files(&snap_id).unwrap();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].id, file_id);
}

#[test]
fn test_file_search() {
    let repo = setup_test_repo();
    let device_id = DeviceId::new("dev-search");

    repo.save_device(&Device {
        id: device_id.clone(),
        manufacturer: "A".to_string(), model: "B".to_string(), serial: "C".to_string(),
        os_version: "D".to_string(), sdk_version: None,
        storage_total_bytes: 0, storage_used_bytes: 0, storage_free_bytes: 0,
        connection_type: ConnectionType::Usb,
    }).unwrap();

    let files = vec![
        ("f1", "/data/photo.jpg", "photo.jpg"),
        ("f2", "/data/document.pdf", "document.pdf"),
        ("f3", "/data/vacation_photo.png", "vacation_photo.png"),
    ];

    for (id, path, name) in files {
        repo.save_file(&FileEntry {
            id: FileId(id.to_string()), device_id: device_id.clone(),
            path: path.to_string(), name: name.to_string(),
            size_bytes: 100, modified_at: Utc::now(), mime_type: "any".to_string(),
            permissions: "---".to_string(), hash_sha256: None,
            thumbnail_hash: None,
            media_info: None,
        }).unwrap();
    }

    let results = repo.search_files("photo").unwrap();
    assert_eq!(results.len(), 2);
}

#[test]
fn test_media_queries() {
    let repo = setup_test_repo();
    let device_id = DeviceId::new("dev-media");

    repo.save_device(&Device {
        id: device_id.clone(),
        manufacturer: "A".to_string(), model: "B".to_string(), serial: "C".to_string(),
        os_version: "D".to_string(), sdk_version: None,
        storage_total_bytes: 0, storage_used_bytes: 0, storage_free_bytes: 0,
        connection_type: ConnectionType::Usb,
    }).unwrap();

    // 1. Image
    repo.save_file(&FileEntry {
        id: FileId("m1".to_string()), device_id: device_id.clone(),
        path: "p1.jpg".to_string(), name: "n1.jpg".to_string(), size_bytes: 10,
        modified_at: Utc::now(), mime_type: "image/jpeg".to_string(), permissions: "p".to_string(),
        hash_sha256: None,
        thumbnail_hash: None,
        media_info: None,
    }).unwrap();

    // 2. Video
    repo.save_file(&FileEntry {
        id: FileId("m2".to_string()), device_id: device_id.clone(),
        path: "p2.mp4".to_string(), name: "n2.mp4".to_string(), size_bytes: 20,
        modified_at: Utc::now(), mime_type: "video/mp4".to_string(), permissions: "p".to_string(),
        hash_sha256: None,
        thumbnail_hash: None,
        media_info: None,
    }).unwrap();

    // 3. Non-media
    repo.save_file(&FileEntry {
        id: FileId("doc1".to_string()), device_id: device_id.clone(),
        path: "p3.txt".to_string(), name: "n3.txt".to_string(), size_bytes: 5,
        modified_at: Utc::now(), mime_type: "text/plain".to_string(), permissions: "p".to_string(),
        hash_sha256: None,
        thumbnail_hash: None,
        media_info: None,
    }).unwrap();

    let media = repo.list_media_files(&device_id).unwrap();
    assert_eq!(media.len(), 2);

    let recent = repo.get_recent_media(1).unwrap();
    assert_eq!(recent.len(), 1);
}

#[test]
fn test_file_chunk_management() {
    let repo = setup_test_repo();
    let file_id = FileId("chunked-file".to_string());
    let device_id = DeviceId::new("dev-chunks");

    repo.save_device(&Device {
        id: device_id.clone(),
        manufacturer: "A".to_string(), model: "B".to_string(), serial: "C".to_string(),
        os_version: "D".to_string(), sdk_version: None,
        storage_total_bytes: 0, storage_used_bytes: 0, storage_free_bytes: 0,
        connection_type: ConnectionType::Usb,
    }).unwrap();

    repo.save_file(&FileEntry {
        id: file_id.clone(), device_id, path: "p".to_string(), name: "n".to_string(),
        size_bytes: 1000, modified_at: Utc::now(), mime_type: "t".to_string(),
        permissions: "p".to_string(), hash_sha256: Some("full-hash".to_string()),
        thumbnail_hash: None,
        media_info: None,
    }).unwrap();

    repo.save_file_chunk(&file_id, "hash-1", 0, 500, 0).unwrap();
    repo.save_file_chunk(&file_id, "hash-2", 500, 500, 1).unwrap();

    let chunks = repo.get_file_chunks(&file_id).unwrap();
    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0].0, "hash-1");
    assert_eq!(chunks[1].0, "hash-2");
}

#[test]
fn test_file_diffing() {
    let repo = setup_test_repo();
    let device_id = DeviceId::new("d1");
    let s1_id = SnapshotId("s1".to_string());
    let s2_id = SnapshotId("s2".to_string());

    repo.save_device(&Device {
        id: device_id.clone(), manufacturer: "A".to_string(), model: "B".to_string(), serial: "C".to_string(),
        os_version: "D".to_string(), sdk_version: None,
        storage_total_bytes: 0, storage_used_bytes: 0, storage_free_bytes: 0,
        connection_type: ConnectionType::Usb,
    }).unwrap();

    repo.create_snapshot(&Snapshot {
        id: s1_id.clone(), device_id: device_id.clone(), started_at: Utc::now(),
        finished_at: None, status: SnapshotStatus::Completed, total_files: 0,
        total_bytes: 0, deduped_bytes: 0,
    }).unwrap();

    repo.create_snapshot(&Snapshot {
        id: s2_id.clone(), device_id: device_id.clone(), started_at: Utc::now(),
        finished_at: None, status: SnapshotStatus::Completed, total_files: 0,
        total_bytes: 0, deduped_bytes: 0,
    }).unwrap();

    let f1_v1 = FileEntry {
        id: FileId("f1-v1".to_string()), device_id: device_id.clone(),
        path: "p1".to_string(), name: "n1".to_string(), size_bytes: 10,
        modified_at: Utc::now(), mime_type: "t".to_string(), permissions: "p".to_string(),
        hash_sha256: Some("h1".to_string()), thumbnail_hash: None, media_info: None,
    };
    let f1_v2 = FileEntry {
        id: FileId("f1-v2".to_string()), device_id: device_id.clone(),
        path: "p1".to_string(), name: "n1".to_string(), size_bytes: 10,
        modified_at: Utc::now(), mime_type: "t".to_string(), permissions: "p".to_string(),
        hash_sha256: Some("h1-new".to_string()), thumbnail_hash: None, media_info: None,
    };
    let f2 = FileEntry {
        id: FileId("f2".to_string()), device_id: device_id.clone(),
        path: "p2".to_string(), name: "n2".to_string(), size_bytes: 20,
        modified_at: Utc::now(), mime_type: "t".to_string(), permissions: "p".to_string(),
        hash_sha256: Some("h2".to_string()), thumbnail_hash: None, media_info: None,
    };

    repo.save_file(&f1_v1).unwrap();
    repo.save_file(&f2).unwrap();
    repo.save_file(&f1_v2).unwrap();

    repo.link_file_to_snapshot(&s1_id, &f1_v1.id).unwrap();
    repo.link_file_to_snapshot(&s1_id, &f2.id).unwrap();

    repo.link_file_to_snapshot(&s2_id, &f1_v2.id).unwrap();
    // f2 is removed in s2

    let diff = repo.get_file_diff(&s1_id, &s2_id).unwrap();

    assert_eq!(diff.added.len(), 0);
    assert_eq!(diff.removed.len(), 1);
    assert_eq!(diff.removed[0].id.0, "f2");
    assert_eq!(diff.modified.len(), 1);
    assert_eq!(diff.modified[0].hash_sha256.as_ref().unwrap(), "h1-new");
}
