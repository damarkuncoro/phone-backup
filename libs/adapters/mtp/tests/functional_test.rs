use domain::{ConnectionType, DeviceId};
use phone_backup_adapter_mtp::{CompositeDeviceAdapter, MtpAdapter};
use ports::{DevicePort, ScannerPort};
use std::fs::{self, File};
use std::io::Write;
use std::sync::Arc;
use tempfile::tempdir;

#[test]
fn test_mtp_full_lifecycle_simulation() {
    let root = tempdir().unwrap();
    let root_path = root.path();

    // Setup fake Android structure
    let dcim = root_path.join("DCIM");
    let camera = dcim.join("Camera");
    fs::create_dir_all(&camera).unwrap();

    let img_path = camera.join("test.jpg");
    let mut f = File::create(&img_path).unwrap();
    f.write_all(b"test image data").unwrap();

    let adapter = MtpAdapter::with_root(root_path);
    let dev_id = DeviceId::new("mtp:device_1");

    // 1. Test Discovery
    let devices = adapter.discover().unwrap();
    assert!(!devices.is_empty());
    assert_eq!(devices[0].connection_type, ConnectionType::Mtp);

    // 2. Test Directory Listing
    let entries = adapter.list_directory(&dev_id, "/DCIM/Camera").unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "test.jpg");

    // 3. Test File Read
    let mut reader = adapter.read_file(&dev_id, "/DCIM/Camera/test.jpg").unwrap();
    let mut content = Vec::new();
    std::io::Read::read_to_end(&mut reader, &mut content).unwrap();
    assert_eq!(content, b"test image data");

    // 4. Test File Push (Write)
    let push_data = b"new file content";
    adapter
        .push_file(&dev_id, &mut &push_data[..], "/Download/readme.txt")
        .unwrap();
    assert!(root_path.join("Download/readme.txt").exists());

    // 5. Test Delete
    adapter
        .delete_remote(&dev_id, "/Download/readme.txt")
        .unwrap();
    assert!(!root_path.join("Download/readme.txt").exists());
}

#[test]
fn test_mtp_scanner_recursive() {
    let root = tempdir().unwrap();
    let root_path = root.path();

    // Nested structure
    fs::create_dir_all(root_path.join("A/B/C")).unwrap();
    File::create(root_path.join("A/file1.txt")).unwrap();
    File::create(root_path.join("A/B/file2.txt")).unwrap();
    File::create(root_path.join("A/B/C/file3.txt")).unwrap();

    let adapter = MtpAdapter::with_root(root_path);
    let dev_id = DeviceId::new("mtp:test");

    let results = adapter.scan(&dev_id, vec!["A".to_string()]).unwrap();

    // Should find 3 files
    assert_eq!(results.len(), 3);

    let paths: Vec<String> = results.iter().map(|f| f.path.clone()).collect();
    assert!(paths.contains(&"/A/file1.txt".to_string()));
    assert!(paths.contains(&"/A/B/file2.txt".to_string()));
    assert!(paths.contains(&"/A/B/C/file3.txt".to_string()));
}

#[test]
fn test_composite_adapter_routing() {
    // This tests if the composite adapter correctly routes requests based on ID prefix
    let root_mtp = tempdir().unwrap();
    let root_adb = tempdir().unwrap();
    let mtp_adapter = Arc::new(MtpAdapter::with_root(root_mtp.path()));

    // We'll use another MtpAdapter with distinct root as a "mock" for ADB to test routing
    let adb_mock = Arc::new(MtpAdapter::with_root(root_adb.path()));

    let composite = CompositeDeviceAdapter::new(adb_mock, mtp_adapter);

    let mtp_id = DeviceId::new("mtp:anything");
    let adb_id = DeviceId::new("serial123");

    // Test routing (should not panic and should call appropriate methods)
    let _ = composite.capabilities(&mtp_id).unwrap();
    let _ = composite.capabilities(&adb_id).unwrap();

    let devs = composite.discover().unwrap();
    // Since we used two MtpAdapters with one root each, we should see 2 devices
    assert_eq!(devs.len(), 2);
}
