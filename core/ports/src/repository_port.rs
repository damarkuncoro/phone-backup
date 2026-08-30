use anyhow::Result;
use domain::{Device, DeviceId, FileEntry, Snapshot, SnapshotId, BackupSchedule, AppInfo, AppId, Contact, FileId, AppSettings};
use std::collections::HashSet;

pub trait DeviceRepositoryPort: Send + Sync {
    fn save_device(&self, device: &Device) -> Result<()>;
    fn list_devices(&self) -> Result<Vec<Device>>;
    fn get_device(&self, id: &DeviceId) -> Result<Option<Device>>;
    /// Get total storage used by all snapshots for a device (in bytes).
    fn get_storage_usage_by_device(&self, device_id: &DeviceId) -> Result<u64>;
}

pub trait SnapshotRepositoryPort: Send + Sync {
    fn get_snapshot(&self, id: &SnapshotId) -> Result<Option<Snapshot>>;
    fn create_snapshot(&self, snapshot: &Snapshot) -> Result<()>;
    fn update_snapshot(&self, snapshot: &Snapshot) -> Result<()>;
    fn list_snapshots(&self, device_id: &DeviceId) -> Result<Vec<Snapshot>>;
    fn list_all_snapshots(&self) -> Result<Vec<Snapshot>>;
    fn get_latest_snapshot(&self, device_id: &DeviceId) -> Result<Option<Snapshot>>;
    fn get_incomplete_snapshot(&self, device_id: &DeviceId) -> Result<Option<Snapshot>>;
    fn delete_snapshot(&self, snapshot_id: &SnapshotId) -> Result<()>;
    fn save_structured_data_ref(&self, snapshot_id: &SnapshotId, data_type: &str, object_id: &str) -> Result<()>;
    fn get_structured_data_ref(&self, snapshot_id: &SnapshotId, data_type: &str) -> Result<Option<String>>;
}

pub trait FileRepositoryPort: Send + Sync {
    fn save_file(&self, file: &FileEntry) -> Result<()>;
    fn list_files(&self, device_id: &DeviceId) -> Result<Vec<FileEntry>>;
    fn get_snapshot_files(&self, snapshot_id: &SnapshotId) -> Result<Vec<FileEntry>>;
    fn link_file_to_snapshot(&self, snapshot_id: &SnapshotId, file_id: &FileId) -> Result<()>;
    fn search_files(&self, query: &str) -> Result<Vec<FileEntry>>;
    fn save_file_chunk(&self, file_id: &FileId, chunk_hash: &str, offset: u64, length: u32, sequence: u32) -> Result<()>;
    fn get_file_chunks(&self, file_id: &FileId) -> Result<Vec<(String, u64, u32)>>;
}

pub trait AppRepositoryPort: Send + Sync {
    fn save_app(&self, app: &AppInfo) -> Result<()>;
    fn link_app_to_snapshot(&self, snapshot_id: &SnapshotId, app_id: &AppId) -> Result<()>;
    fn get_snapshot_apps(&self, snapshot_id: &SnapshotId) -> Result<Vec<AppInfo>>;
}

pub trait ContactRepositoryPort: Send + Sync {
    fn save_contact(&self, snapshot_id: &SnapshotId, contact: &Contact) -> Result<()>;
    fn get_snapshot_contacts(&self, snapshot_id: &SnapshotId) -> Result<Vec<Contact>>;
    fn search_contacts(&self, query: &str) -> Result<Vec<(SnapshotId, Contact)>>;
}

pub trait ScheduleRepositoryPort: Send + Sync {
    fn save_schedule(&self, schedule: &BackupSchedule) -> Result<()>;
    fn get_schedule(&self, device_id: &DeviceId) -> Result<Option<BackupSchedule>>;
    fn list_schedules(&self) -> Result<Vec<BackupSchedule>>;
}

pub trait SettingsRepositoryPort: Send + Sync {
    fn save_settings(&self, settings: &AppSettings) -> Result<()>;
    fn get_settings(&self) -> Result<Option<AppSettings>>;
}

pub trait MaintenanceRepositoryPort: Send + Sync {
    fn get_all_referenced_hashes(&self) -> Result<HashSet<String>>;
    /// Optimize database storage and update query statistics.
    fn optimize(&self) -> Result<()>;
}

pub trait RepositoryPort:
    DeviceRepositoryPort +
    SnapshotRepositoryPort +
    FileRepositoryPort +
    AppRepositoryPort +
    ContactRepositoryPort +
    ScheduleRepositoryPort +
    SettingsRepositoryPort +
    MaintenanceRepositoryPort +
    Send + Sync {}
