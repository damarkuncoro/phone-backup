use phone_backup_application::BackupService;
use ports::StoragePort;
use adapter_mock::{MockDeviceAdapter, MockScannerAdapter, MockAppProvider, MockDataProvider};
use adapter_filesystem::LocalStorage;
use adapter_database_sqlite::SqliteRepository;
use tempfile::TempDir;
use std::fs;
use std::sync::Mutex;
static TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_full_backup_restore_lifecycle() {
    let _guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    // 1. Setup temporary directories for test
    let tmp_repo_dir = TempDir::new().unwrap();
    let tmp_storage_dir = TempDir::new().unwrap();
    let tmp_restore_dir = TempDir::new().unwrap();

    let db_path = tmp_repo_dir.path().join("test_backup.db");
    let storage_path = tmp_storage_dir.path().to_str().unwrap();
    let restore_path = tmp_restore_dir.path().to_str().unwrap();

    // 2. Initialize Components
    let repository = SqliteRepository::new(db_path.to_str().unwrap()).unwrap();
    let storage = LocalStorage::new(storage_path).unwrap();
    let device_adapter = MockDeviceAdapter::default();
    let scanner_adapter = MockScannerAdapter::default();
    let app_provider = MockAppProvider;
    let data_provider = MockDataProvider;

    let service = BackupService::new(
        device_adapter,
        scanner_adapter,
        repository,
        storage,
        app_provider,
        data_provider,
        ports::NoProgress,
    );

    // 3. Perform Discovery
    let devices = service.list_devices().unwrap();
    assert_eq!(devices.len(), 1);
    let device_id = devices[0].id.clone();

    // 4. Perform Backup (Encrypted)
    let password = "test-password";
    let encryption = domain::EncryptionMode::Password(password.to_string());
    let snapshot = service.perform_backup(&device_id, encryption.clone(), None).expect("Backup failed");

    assert!(snapshot.total_files > 0);
    assert!(snapshot.total_bytes > 0);

    // 5. Verify Integrity
    let report = service.verify_repository(encryption.clone()).unwrap();
    if !report.is_healthy() {
        println!("Missing objects: {:?}", report.missing_objects);
        println!("Corrupted files: {:?}", report.corrupted_files);
    }
    assert!(report.is_healthy());

    // 5.1 Verify Manifest Existence (Snapshot Commit Protocol)
    let manifest_path = format!("manifests/{}.json", snapshot.id.0);
    assert!(service.storage.exists(&manifest_path).unwrap());
    println!("✅ Manifest found in storage: {}", manifest_path);
    assert_eq!(report.verified_files, snapshot.total_files);

    // 6. Perform Restore
    service.perform_restore(&snapshot.id, restore_path, encryption, None).expect("Restore failed");

    // 7. Validate Restored Content
    // MockScannerAdapter seeds "Documents/notes.txt" with "this is mock file content"
    let restored_file = tmp_restore_dir.path().join("Documents/notes.txt");
    assert!(restored_file.exists());

    let restored_content = fs::read_to_string(restored_file).unwrap();
    assert_eq!(restored_content, "this is mock file content");

    println!("✅ Integration test passed: Backup and Restore verified.");
}

#[test]
fn test_asymmetric_backup_restore_lifecycle() {
    let _guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let tmp_repo_dir = TempDir::new().unwrap();
    let tmp_storage_dir = TempDir::new().unwrap();
    let tmp_restore_dir = TempDir::new().unwrap();

    let db_path = tmp_repo_dir.path().join("test_asym.db");
    let storage_path = tmp_storage_dir.path().to_str().unwrap();
    let restore_path = tmp_restore_dir.path().to_str().unwrap();

    let repository = SqliteRepository::new(db_path.to_str().unwrap()).unwrap();
    let storage = LocalStorage::new(storage_path).unwrap();
    let service = BackupService::new(
        MockDeviceAdapter::with_device_id("DEV_ASYM"),
        MockScannerAdapter::default(),
        repository,
        storage,
        MockAppProvider,
        MockDataProvider,
        ports::NoProgress,
    );

    let devices = service.list_devices().unwrap();
    let device_id = devices[0].id.clone();

    // 1. Generate Keypair
    let (secret, public) = phone_backup_application::storage::EncryptionEngine::generate_keypair();
    let encryption = domain::EncryptionMode::PublicKey(public);
    let decryption = domain::EncryptionMode::PublicKey(secret);

    // 2. Backup with Public Key
    let snapshot = service.perform_backup(&device_id, encryption, None).expect("Asym backup failed");

    // 3. Restore with Secret Key
    service.perform_restore(&snapshot.id, restore_path, decryption, None).expect("Asym restore failed");

    let restored_file = tmp_restore_dir.path().join("Documents/notes.txt");
    assert!(restored_file.exists());
    assert_eq!(fs::read_to_string(restored_file).unwrap(), "this is mock file content");

    println!("✅ Asymmetric Integration test passed.");
}
