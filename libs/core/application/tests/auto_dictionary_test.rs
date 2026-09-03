use adapter_database_sqlite::SqliteRepository;
use adapter_filesystem::LocalStorage;
use domain::EncryptionMode;
use phone_backup_application::storage::{AutoDictionaryService, DataCategory, ObjectManager};
use tempfile::tempdir;

#[test]
fn test_auto_dictionary_training_and_loading() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("metadata.db");
    let storage_dir = dir.path().join("storage");
    std::fs::create_dir_all(&storage_dir).unwrap();

    let repo = SqliteRepository::new(db_path.to_str().unwrap()).unwrap();
    let storage = LocalStorage::new(storage_dir.to_str().unwrap()).unwrap();
    let enc = EncryptionMode::None;

    let object_manager = ObjectManager::new(&storage, &repo, &enc);

    // Seed 10 realistic structured JSON sample chunks
    let mut sample_ids = Vec::new();
    for i in 0..10 {
        let sample = format!(
            r#"{{"user_id": {}, "timestamp": {}, "conversation_id": "conv_{}", "message": "Pesan chat simulasi nomor {} dengan teks panjang untuk melatih kamus kompresi zstd.", "status": "delivered", "read": {}}}"#,
            100 + i, 1725320000 + i, i % 3, i, i % 2
        ).repeat(8);

        let (chunk_id, _) = object_manager.put_chunk(sample.as_bytes()).unwrap();
        sample_ids.push(chunk_id);
    }

    let auto_dict_svc = AutoDictionaryService::new(&storage, &repo, &enc);

    let trained_dict = auto_dict_svc
        .train_custom_dictionary(
            "custom-chat-v1",
            DataCategory::Document,
            &sample_ids,
            4096,
        )
        .unwrap();

    assert_eq!(trained_dict.id.as_str(), "custom-chat-v1");
    assert!(!trained_dict.is_empty());

    // Verify loading persisted dictionary
    let loaded_dict = auto_dict_svc
        .load_dictionary("custom-chat-v1", DataCategory::Document)
        .unwrap();
    assert_eq!(loaded_dict.id.as_str(), "custom-chat-v1");
    assert_eq!(loaded_dict.data, trained_dict.data);
}
