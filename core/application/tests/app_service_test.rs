use adapter_database_sqlite::SqliteRepository;
use adapter_filesystem::LocalStorage;
use adapter_mock::{MockAppProvider, MockDataProvider, MockDeviceAdapter, MockScannerAdapter};
use domain::DeviceId;
use phone_backup_application::BackupService;
use tempfile::TempDir;

#[test]
fn test_export_apk_single_and_batch() {
    let tmp_repo_dir = TempDir::new().unwrap();
    let tmp_storage_dir = TempDir::new().unwrap();
    let tmp_export_dir = TempDir::new().unwrap();

    let db_path = tmp_repo_dir.path().join("test_app_export.db");
    let repository = SqliteRepository::new(db_path.to_str().unwrap()).unwrap();
    let storage = LocalStorage::new(tmp_storage_dir.path()).unwrap();

    let service = BackupService::new(
        MockDeviceAdapter::default(),
        MockScannerAdapter::default(),
        repository,
        storage,
        MockAppProvider,
        MockDataProvider,
        ports::NoProgress,
    );

    let dev_id = DeviceId::new("A1B2C3D4");

    // 1. List Apps
    let apps = service.list_apps(&dev_id).unwrap();
    assert!(!apps.is_empty());
    assert_eq!(apps[0].package_name, "com.whatsapp");

    // 2. Export Single APK
    let single_apk_path = tmp_export_dir.path().join("com.whatsapp.apk");
    let res_single = service.export_apk(&dev_id, "com.whatsapp", single_apk_path.to_str().unwrap());
    assert!(res_single.is_ok());
    assert!(single_apk_path.exists());

    // 3. Export Batch APKs
    let batch_dir = tmp_export_dir.path().join("apks");
    let batch_packages = vec!["com.whatsapp".to_string(), "com.instagram.android".to_string()];
    let exported = service.export_apk_batch(&dev_id, &batch_packages, batch_dir.to_str().unwrap()).unwrap();

    assert_eq!(exported.len(), 2);
    assert!(batch_dir.join("com.whatsapp.apk").exists());
    assert!(batch_dir.join("com.instagram.android.apk").exists());
}
