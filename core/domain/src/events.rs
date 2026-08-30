use crate::{DeviceId, SnapshotId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Domain Events emitted by domain operations in the phone-backup platform.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DomainEvent {
    DeviceConnected {
        device_id: DeviceId,
        timestamp: DateTime<Utc>,
    },
    DeviceDisconnected {
        device_id: DeviceId,
        timestamp: DateTime<Utc>,
    },
    BackupStarted {
        snapshot_id: SnapshotId,
        device_id: DeviceId,
        timestamp: DateTime<Utc>,
    },
    BackupCompleted {
        snapshot_id: SnapshotId,
        device_id: DeviceId,
        total_files: u64,
        total_bytes: u64,
        timestamp: DateTime<Utc>,
    },
    BackupFailed {
        snapshot_id: SnapshotId,
        device_id: DeviceId,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    SnapshotDeleted {
        snapshot_id: SnapshotId,
        device_id: DeviceId,
        timestamp: DateTime<Utc>,
    },
}
