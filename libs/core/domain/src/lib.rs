//! Core domain layer.
//!
//! This crate has ZERO dependency on any adapter, transport, or
//! infrastructure concern (no ADB, no MTP, no SQL, no filesystem I/O).
//! It only knows about the *concepts* of the backup platform.

mod app;
mod backup_plan;
mod capability;
mod crypto;
mod data;
mod device;
mod error;
mod event_bus;
mod events;
mod file;
mod media;
mod policy;
mod retention;
mod scan_result;
mod schedule;
mod settings;
mod snapshot;
mod structured_data;
mod value_objects;

mod manifest;

pub use app::{AppId, AppInfo};
pub use backup_plan::{BackupPlan, DeletedFile, FileReuse};
pub use capability::{Capability, CapabilityMatrix, CapabilityStatus};
pub use crypto::EncryptionMode;
pub use data::*;
pub use device::{ConnectionType, Device, DeviceId};
pub use error::DomainError;
pub use event_bus::{DomainEventBus, DomainEventHandler, EventHandlerRef};
pub use events::DomainEvent;
pub use file::{FileDiff, FileEntry, FileId};
pub use manifest::{Manifest, ManifestChunk, ManifestFile};
pub use media::MediaInfo;
pub use policy::BackupPolicy;
pub use retention::{KeepCountStrategy, KeepDailyStrategy, RetentionPolicy, RetentionStrategy};
pub use scan_result::{ScanResult, ScanSource, ScanWarning};
pub use schedule::{BackupSchedule, ScheduleFrequency};
pub use settings::{AppSettings, StorageBackend};
pub use snapshot::{Snapshot, SnapshotId, SnapshotStatus};
pub use structured_data::StructuredDataType;
pub use value_objects::{Checksum, DevicePath, StorageSize};
