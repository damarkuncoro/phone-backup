//! Core domain layer.
//!
//! This crate has ZERO dependency on any adapter, transport, or
//! infrastructure concern (no ADB, no MTP, no SQL, no filesystem I/O).
//! It only knows about the *concepts* of the backup platform.

mod device;
mod capability;
mod file;
mod snapshot;
mod app;
mod crypto;
mod data;
mod media;
mod schedule;
mod policy;
mod retention;
mod error;
mod settings;

pub use capability::{Capability, CapabilityStatus, CapabilityMatrix};
pub use device::{ConnectionType, Device, DeviceId};
pub use file::{FileEntry, FileId, FileDiff};
pub use snapshot::{Snapshot, SnapshotId, SnapshotStatus};
pub use app::{AppInfo, AppId};
pub use crypto::EncryptionMode;
pub use data::*;
pub use media::MediaInfo;
pub use schedule::{BackupSchedule, ScheduleFrequency};
pub use policy::BackupPolicy;
pub use retention::{KeepCountStrategy, KeepDailyStrategy, RetentionPolicy, RetentionStrategy};
pub use error::DomainError;
pub use settings::{AppSettings, StorageBackend};
