use phone_backup_adapter_mtp::MtpAdapter;
use domain::{ConnectionType, DeviceId};
use ports::{DevicePort, ScannerPort};
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_mtp_adapter_custom_root_discovery() {
    let dir = tempdir().unwrap();
    let dcim = dir.path().join("DCIM");
    std::fs::create_dir_all(&dcim).unwrap();

    let adapter = MtpAdapter::with_root(dir.path());
    let devices = adapter.discover().expect("Discovery failed");

    assert_eq!(devices.len(), 1);
    let dev = &devices[0];
    assert_eq!(dev.connection_type, ConnectionType::Mtp);
    assert_eq!(dev.manufacturer, "Android (MTP)");
}

#[test]
fn test_mtp_adapter_capabilities() {
    let adapter = MtpAdapter::new();
    let caps = adapter.capabilities(&DeviceId::new("mtp:test")).unwrap();

    assert!(caps.is_available(domain::Capability::ReadFiles));
    assert!(!caps.is_available(domain::Capability::ReadContacts));
    assert!(!caps.is_available(domain::Capability::ReadSms));
    assert!(!caps.is_available(domain::Capability::ReadAppData));
}

#[test]
fn test_mtp_adapter_file_operations_and_scan() {
    let dir = tempdir().unwrap();
    let dcim = dir.path().join("DCIM");
    std::fs::create_dir_all(&dcim).unwrap();

    let photo_path = dcim.join("photo1.jpg");
    let mut f = File::create(&photo_path).unwrap();
    f.write_all(b"fake jpeg content").unwrap();

    let adapter = MtpAdapter::with_root(dir.path());
    let dev_id = DeviceId::new("mtp:test");

    // 1. List directory
    let entries = adapter.list_directory(&dev_id, "/DCIM").unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "photo1.jpg");
    assert_eq!(entries[0].size_bytes, 17);

    // 2. Scan
    let scanned = adapter.scan(&dev_id, vec!["DCIM".to_string()]).unwrap();
    assert_eq!(scanned.len(), 1);
    assert_eq!(scanned[0].name, "photo1.jpg");

    // 3. Read file
    let mut reader = adapter.read_file(&dev_id, "/DCIM/photo1.jpg").unwrap();
    let mut buf = Vec::new();
    std::io::Read::read_to_end(&mut reader, &mut buf).unwrap();
    assert_eq!(buf, b"fake jpeg content");
}
