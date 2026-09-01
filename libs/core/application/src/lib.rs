//! Application layer: use cases that orchestrate the domain through
//! ports. No concrete adapter, no I/O detail, no SQL, no ADB.

pub mod backup;
pub mod storage;
pub mod analysis;
pub mod device;

// Re-exports for convenience
pub use backup::BackupService;
pub use backup::verify::{StorageStats, VerificationReport};
pub use backup::planner::BackupPlanner;
pub use backup::guard::SnapshotGuard;
pub use backup::progress::ProgressEstimator;

pub use storage::manager::ObjectManager;
pub use storage::store::ObjectStoreKey;
pub use storage::security::EncryptionEngine;
pub use storage::compression::CompressionEngine;
pub use storage::hashing::calculate_hash;

pub use analysis::media::MediaAnalyzer;
pub use analysis::vcard::VCardEngine;
