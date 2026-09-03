use adapter_database_sqlite::SqliteRepository;
use adapter_filesystem::LocalStorage;
use adapter_mock::{MockAppProvider, MockDataProvider, MockDeviceAdapter, MockScannerAdapter};
use phone_backup_application::BackupService;
use ports::StoragePort;
use std::fs;
use std::sync::Mutex;
use tempfile::TempDir;
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
    let scanner_adapter = MockScannerAdapter;
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
    let snapshot = service
        .perform_backup(&device_id, encryption.clone(), None)
        .expect("Backup failed");

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
    service
        .perform_restore(&snapshot.id, restore_path, encryption, None)
        .expect("Restore failed");

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
        MockScannerAdapter,
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
    let snapshot = service
        .perform_backup(&device_id, encryption, None)
        .expect("Asym backup failed");

    // 3. Restore with Secret Key
    service
        .perform_restore(&snapshot.id, restore_path, decryption, None)
        .expect("Asym restore failed");

    let restored_file = tmp_restore_dir.path().join("Documents/notes.txt");
    assert!(restored_file.exists());
    assert_eq!(
        fs::read_to_string(restored_file).unwrap(),
        "this is mock file content"
    );

    println!("✅ Asymmetric Integration test passed.");
}

#[test]
fn test_batch_checkpoint_and_resilience() {
    let _guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let tmp_repo_dir = TempDir::new().unwrap();
    let tmp_storage_dir = TempDir::new().unwrap();

    let db_path = tmp_repo_dir.path().join("test_checkpoint.db");
    let storage_path = tmp_storage_dir.path().to_str().unwrap();

    let repository = SqliteRepository::new(db_path.to_str().unwrap()).unwrap();
    let storage = LocalStorage::new(storage_path).unwrap();
    let service = BackupService::new(
        MockDeviceAdapter::with_device_id("DEV_CHECKPOINT"),
        MockScannerAdapter,
        repository,
        storage,
        MockAppProvider,
        MockDataProvider,
        ports::NoProgress,
    );

    let devices = service.list_devices().unwrap();
    let device_id = devices[0].id.clone();

    let snapshot = service
        .perform_backup(&device_id, domain::EncryptionMode::None, None)
        .expect("Backup with checkpointing should succeed");

    assert!(snapshot.total_files > 0);
    assert_eq!(snapshot.status, domain::SnapshotStatus::Completed);
    println!("✅ Batch checkpointing test passed.");
}

struct TestEventCollector {
    events: Mutex<Vec<domain::DomainEvent>>,
}

impl domain::DomainEventHandler for TestEventCollector {
    fn handle(&self, event: &domain::DomainEvent) {
        self.events.lock().unwrap().push(event.clone());
    }
}

#[test]
fn test_event_bus_and_cancellation_integration() {
    let _guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let tmp_repo_dir = TempDir::new().unwrap();
    let tmp_storage_dir = TempDir::new().unwrap();

    let db_path = tmp_repo_dir.path().join("test_events.db");
    let storage_path = tmp_storage_dir.path().to_str().unwrap();

    let repository = SqliteRepository::new(db_path.to_str().unwrap()).unwrap();
    let storage = LocalStorage::new(storage_path).unwrap();

    // Setup Event Bus and Collector
    let event_bus = domain::DomainEventBus::new();
    let collector = std::sync::Arc::new(TestEventCollector {
        events: Mutex::new(Vec::new()),
    });
    event_bus.subscribe(collector.clone());

    let token = domain::CancellationToken::new();

    let service = BackupService::builder()
        .with_device_adapter(MockDeviceAdapter::with_device_id("DEV_EVENTS"))
        .with_scanner_adapter(MockScannerAdapter)
        .with_repository(repository)
        .with_storage(storage)
        .with_app_provider(MockAppProvider)
        .with_data_provider(MockDataProvider)
        .with_progress(ports::NoProgress)
        .with_event_bus(event_bus)
        .with_cancellation_token(token)
        .build()
        .unwrap();

    let devices = service.list_devices().unwrap();
    let device_id = devices[0].id.clone();

    let snapshot = service
        .perform_backup(&device_id, domain::EncryptionMode::None, None)
        .unwrap();

    assert_eq!(snapshot.status, domain::SnapshotStatus::Completed);

    let recorded = collector.events.lock().unwrap();
    assert!(recorded.len() >= 2);
    assert!(matches!(&recorded[0], domain::DomainEvent::BackupStarted { .. }));
    assert!(matches!(&recorded.last().unwrap(), domain::DomainEvent::BackupCompleted { .. }));
    println!("✅ Event Bus and CancellationToken integration verified.");
}

#[test]
fn test_metrics_storage_decorator_with_service() {
    use ports::MetricsStorage;

    let _guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let tmp_repo_dir = TempDir::new().unwrap();
    let tmp_storage_dir = TempDir::new().unwrap();

    let db_path = tmp_repo_dir.path().join("test_metrics.db");
    let storage_path = tmp_storage_dir.path().to_str().unwrap();

    let repository = SqliteRepository::new(db_path.to_str().unwrap()).unwrap();
    let raw_storage = LocalStorage::new(storage_path).unwrap();
    let storage = MetricsStorage::new(raw_storage);

    let service = BackupService::builder()
        .with_device_adapter(MockDeviceAdapter::with_device_id("DEV_METRICS"))
        .with_scanner_adapter(MockScannerAdapter)
        .with_repository(repository)
        .with_storage(storage)
        .with_app_provider(MockAppProvider)
        .with_data_provider(MockDataProvider)
        .with_progress(ports::NoProgress)
        .build()
        .unwrap();

    let devices = service.list_devices().unwrap();
    let device_id = devices[0].id.clone();

    service
        .perform_backup(&device_id, domain::EncryptionMode::None, None)
        .unwrap();

    let metrics = service.storage.metrics();
    assert!(metrics.bytes_written > 0);
    assert!(metrics.write_ops > 0);
    println!("✅ MetricsStorage decorator integration verified: {} bytes written across {} ops", metrics.bytes_written, metrics.write_ops);
}


