//! Application layer: use cases that orchestrate the domain through
//! ports. No concrete adapter, no I/O detail, no SQL, no ADB.

mod backup_service;

pub use backup_service::BackupService;
