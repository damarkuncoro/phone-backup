pub struct ObjectStoreKey;

impl ObjectStoreKey {
    /// In V4.0, we use UUIDv7 for storage keys to obfuscate data.
    pub fn generate_storage_key() -> String {
        uuid::Uuid::now_v7().to_string()
    }

    pub fn compute_object_path_v4(storage_key: &str) -> String {
        // Obfuscated structure: objects/v4/aa/bb/UUID
        format!("objects/v4/{}/{}/{}", &storage_key[0..2], &storage_key[2..4], storage_key)
    }

    /// Legacy support or for structured data that doesn't need obfuscation yet
    pub fn compute_object_id(hash: &str, _mime_type: Option<&str>, _is_encrypted: bool) -> String {
        hash.to_string()
    }

    pub fn compute_object_path(hash: &str, object_id: &str) -> String {
        format!("objects/{}/{}/{}", &hash[0..2], &hash[2..4], object_id)
    }
}
