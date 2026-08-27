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
mod data;
mod media;
mod schedule;
mod policy;
mod retention;
mod error;

pub use capability::{Capability, CapabilityStatus, CapabilityMatrix};
pub use device::{ConnectionType, Device, DeviceId};
pub use file::{FileEntry, FileId};
pub use snapshot::{Snapshot, SnapshotId, SnapshotStatus};
pub use app::{AppInfo, AppId};
pub use data::{Contact, Sms, CallLog, StructuredData};
pub use media::MediaInfo;
pub use schedule::{BackupSchedule, ScheduleFrequency};
pub use policy::BackupPolicy;
pub use retention::RetentionPolicy;
pub use error::DomainError;
