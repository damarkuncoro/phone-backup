use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageBackend {
    Local,
    Mock,
    S3 {
        bucket: String,
        region: String,
        endpoint: String,
        access_key: String,
        secret_key: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub storage_backend: StorageBackend,
    pub encryption_public_key: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            storage_backend: StorageBackend::Local,
            encryption_public_key: None,
        }
    }
}
