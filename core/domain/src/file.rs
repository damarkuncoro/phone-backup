use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::{DeviceId, MediaInfo};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub id: FileId,
    pub device_id: DeviceId,
    pub path: String,
    pub name: String,
    pub size_bytes: u64,
    pub modified_at: DateTime<Utc>,
    pub mime_type: String,
    pub permissions: String,
    pub hash_sha256: Option<String>,
    pub thumbnail_hash: Option<String>,
    pub media_info: Option<MediaInfo>,
}
