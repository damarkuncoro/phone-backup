use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::DeviceId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Interrupted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: SnapshotId,
    pub device_id: DeviceId,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: SnapshotStatus,
    pub total_files: u64,
    pub total_bytes: u64,
    pub deduped_bytes: u64,
}

impl Snapshot {
    pub fn new(device_id: DeviceId) -> Self {
        let id = SnapshotId(uuid::Uuid::new_v4().to_string());
        Self {
            id,
            device_id,
            started_at: Utc::now(),
            finished_at: None,
            status: SnapshotStatus::Pending,
            total_files: 0,
            total_bytes: 0,
            deduped_bytes: 0,
        }
    }
}
