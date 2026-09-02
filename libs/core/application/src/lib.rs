//! Application layer: use cases that orchestrate the domain through
//! ports. No concrete adapter, no I/O detail, no SQL, no ADB.

pub mod analysis;
pub mod backup;
pub mod device;
pub mod storage;

// Re-exports for convenience
pub use backup::guard::SnapshotGuard;
pub use backup::planner::BackupPlanner;
pub use backup::progress::ProgressEstimator;
pub use backup::verify::{StorageStats, VerificationReport};
pub use backup::BackupService;

pub use storage::hashing::calculate_hash;
pub use storage::manager::ObjectManager;
pub use storage::store::ObjectStoreKey;
pub use storage::CompressionEngine;
pub use storage::EncryptionEngine;

pub use analysis::media::MediaAnalyzer;
pub use analysis::vcard::VCardEngine;
