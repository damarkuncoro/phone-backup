use crate::DeviceId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

use crate::DomainError;

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

    pub fn start(&mut self) -> Result<(), DomainError> {
        match self.status {
            SnapshotStatus::Pending | SnapshotStatus::Interrupted => {
                self.status = SnapshotStatus::Running;
                Ok(())
            }
            _ => Err(DomainError::InvalidState(format!(
                "Invalid snapshot transition to Running from {:?}",
                self.status
            ))),
        }
    }

    pub fn complete(&mut self) -> Result<(), DomainError> {
        match self.status {
            SnapshotStatus::Running => {
                self.status = SnapshotStatus::Completed;
                self.finished_at = Some(Utc::now());
                Ok(())
            }
            _ => Err(DomainError::InvalidState(format!(
                "Invalid snapshot transition to Completed from {:?}",
                self.status
            ))),
        }
    }

    pub fn interrupt(&mut self) -> Result<(), DomainError> {
        match self.status {
            SnapshotStatus::Pending | SnapshotStatus::Running => {
                self.status = SnapshotStatus::Interrupted;
                Ok(())
            }
            SnapshotStatus::Completed => Err(DomainError::InvalidState(
                "Cannot interrupt a completed snapshot".to_string(),
            )),
            SnapshotStatus::Interrupted | SnapshotStatus::Failed => Ok(()),
        }
    }

    pub fn fail(&mut self) -> Result<(), DomainError> {
        if self.status != SnapshotStatus::Completed {
            self.status = SnapshotStatus::Failed;
            Ok(())
        } else {
            Err(DomainError::InvalidState(
                "Cannot fail a completed snapshot".to_string(),
            ))
        }
    }
}
