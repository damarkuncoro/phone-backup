use domain::{ConnectionType, Device, DeviceId};
use phone_backup_adapter_database_sqlite::SqliteRepositoryFactory;
use ports::DeviceRepositoryPort;
use tempfile::NamedTempFile;

#[test]
fn test_create_encrypted_repository() {
    let temp_file = NamedTempFile::new().unwrap();
    let db_path = temp_file.path().to_str().unwrap();

    let repo = SqliteRepositoryFactory::create_encrypted(db_path, "secret_passphrase");
    assert!(repo.is_ok());

    let r = repo.unwrap();
    let dev = Device {
        id: DeviceId::new("DEV_ENC"),
        manufacturer: "SecureCorp".to_string(),
        model: "Encrypted Phone".to_string(),
        serial: "ENC123".to_string(),
        os_version: "14".to_string(),
        sdk_version: Some(34),
        storage_total_bytes: 1000,
        storage_used_bytes: 500,
        storage_free_bytes: 500,
        connection_type: ConnectionType::Usb,
    };
    assert!(r.save_device(&dev).is_ok());

    let devices = r.list_devices().unwrap();
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].model, "Encrypted Phone");
}
