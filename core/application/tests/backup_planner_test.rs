use chrono::Utc;
use domain::{DeviceId, FileEntry, FileId};
use phone_backup_application::BackupPlanner;
use std::collections::{HashMap, HashSet};

#[test]
fn test_backup_planner_build_plan() {
    let dev_id = DeviceId::new("DEV1");
    let now = Utc::now();

    let file_a = FileEntry {
        id: FileId("1".to_string()),
        device_id: dev_id.clone(),
        path: "/sdcard/photo_a.jpg".to_string(),
        name: "photo_a.jpg".to_string(),
        size_bytes: 1000,
        modified_at: now,
        mime_type: "image/jpeg".to_string(),
        permissions: "rw-rw-r--".to_string(),
        hash_sha256: Some("hash_a".to_string()),
        thumbnail_hash: None,
        media_info: None,
    };

    let file_b_new = FileEntry {
        id: FileId("2".to_string()),
        device_id: dev_id.clone(),
        path: "/sdcard/photo_b.jpg".to_string(),
        name: "photo_b.jpg".to_string(),
        size_bytes: 2000,
        modified_at: now,
        mime_type: "image/jpeg".to_string(),
        permissions: "rw-rw-r--".to_string(),
        hash_sha256: Some("hash_b".to_string()),
        thumbnail_hash: None,
        media_info: None,
    };

    let manifest = vec![file_a.clone(), file_b_new.clone()];

    let mut previous_files = HashMap::new();
    previous_files.insert(file_a.path.clone(), file_a.clone());

    let old_deleted_file = FileEntry {
        id: FileId("3".to_string()),
        device_id: dev_id.clone(),
        path: "/sdcard/photo_old.jpg".to_string(),
        name: "photo_old.jpg".to_string(),
        size_bytes: 500,
        modified_at: now - chrono::Duration::days(5),
        mime_type: "image/jpeg".to_string(),
        permissions: "rw-rw-r--".to_string(),
        hash_sha256: Some("hash_old".to_string()),
        thumbnail_hash: None,
        media_info: None,
    };
    previous_files.insert(old_deleted_file.path.clone(), old_deleted_file);

    let already_backed_up = HashSet::new();

    let plan = BackupPlanner::build_plan(&manifest, &previous_files, &already_backed_up);

    assert_eq!(plan.upload_count(), 1);
    assert_eq!(plan.upload[0].path, "/sdcard/photo_b.jpg");
    assert_eq!(plan.reuse_count(), 1);
    assert_eq!(plan.reuse[0].path, "/sdcard/photo_a.jpg");
    assert_eq!(plan.deleted_count(), 1);
    assert_eq!(plan.deleted[0].path, "/sdcard/photo_old.jpg");
    assert_eq!(plan.upload_bytes, 2000);
    assert_eq!(plan.logical_bytes, 3000);
}
