use domain::DeviceId;
use phone_backup_adapter_filesystem::{FilesystemScanner, LocalStorage};
use ports::{ScannerPort, StoragePort};
use std::io::Read;

#[test]
fn test_local_storage_lifecycle() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = LocalStorage::new(temp_dir.path()).unwrap();

    let object_id = "objects/ab/cd/test.txt";
    let content = b"hello filesystem storage";

    // Write
    storage
        .write(object_id, &mut std::io::Cursor::new(content))
        .unwrap();

    // Exists
    assert!(storage.exists(object_id).unwrap());

    // Read
    let mut reader = storage.read(object_id).unwrap();
    let mut read_buf = Vec::new();
    reader.read_to_end(&mut read_buf).unwrap();
    assert_eq!(read_buf, content);

    // Delete
    storage.delete(object_id).unwrap();
    assert!(!storage.exists(object_id).unwrap());
}

#[test]
fn test_filesystem_scanner() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("document.pdf");
    std::fs::write(&file_path, b"pdf content").unwrap();

    let scanner = FilesystemScanner::new(temp_dir.path().to_str().unwrap());
    let device_id = DeviceId::new("DEV_TEST");
    let entries = scanner.scan(&device_id, vec![]).unwrap();

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "document.pdf");
    assert_eq!(entries[0].mime_type, "application/pdf");
}
