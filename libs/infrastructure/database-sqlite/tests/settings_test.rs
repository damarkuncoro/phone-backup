mod common;

use common::setup_test_repo;
use domain::{AppSettings, StorageBackend};
use ports::SettingsRepositoryPort;

#[test]
fn test_settings_persistence() {
    let repo = setup_test_repo();

    let settings = AppSettings {
        storage_backend: StorageBackend::Local,
        encryption_public_key: Some("test-key".to_string()),
    };

    repo.save_settings(&settings).unwrap();

    let saved = repo
        .get_settings()
        .unwrap()
        .expect("Settings should be saved");
    assert_eq!(saved.encryption_public_key, Some("test-key".to_string()));
}
