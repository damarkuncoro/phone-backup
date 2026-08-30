//! Application layer: use cases that orchestrate the domain through
//! ports. No concrete adapter, no I/O detail, no SQL, no ADB.

mod backup_service;
pub mod security;
mod media_analysis;
mod compression;
mod hashing;
mod chunking;
pub mod object_manager;
pub mod object_store;
mod progress_estimator;
mod backup_planner;
mod snapshot_guard;

pub use backup_service::{BackupService, StorageStats, VerificationReport};
pub use compression::CompressionEngine;
pub use object_store::ObjectStoreKey;
pub use security::EncryptionEngine;
pub use progress_estimator::ProgressEstimator;
pub use backup_planner::BackupPlanner;
pub use snapshot_guard::SnapshotGuard;
