use anyhow::Result;
use domain::{
    AppId, AppInfo, AppSettings, BackupSchedule, Contact, Device, DeviceId, FileEntry, FileId,
    Snapshot, SnapshotId,
};
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
    fn get_latest_completed_snapshot(&self, device_id: &DeviceId) -> Result<Option<Snapshot>>;
    fn get_incomplete_snapshot(&self, device_id: &DeviceId) -> Result<Option<Snapshot>>;
    fn get_resumable_snapshot(&self, device_id: &DeviceId) -> Result<Option<Snapshot>>;
    fn delete_snapshot(&self, snapshot_id: &SnapshotId) -> Result<()>;
    fn save_structured_data_ref(
        &self,
        snapshot_id: &SnapshotId,
        data_type: domain::StructuredDataType,
        object_id: &str,
    ) -> Result<()>;
    fn get_structured_data_ref(
        &self,
        snapshot_id: &SnapshotId,
        data_type: domain::StructuredDataType,
    ) -> Result<Option<String>>;
}

pub trait FileRepositoryPort: Send + Sync {
    fn save_file(&self, file: &FileEntry) -> Result<()>;
    fn save_files_batch(&self, files: &[FileEntry]) -> Result<()>;
    fn list_files(&self, device_id: &DeviceId) -> Result<Vec<FileEntry>>;
    fn get_snapshot_files(&self, snapshot_id: &SnapshotId) -> Result<Vec<FileEntry>>;
    fn link_file_to_snapshot(&self, snapshot_id: &SnapshotId, file_id: &FileId) -> Result<()>;
    fn link_files_to_snapshot_batch(
        &self,
        snapshot_id: &SnapshotId,
        file_ids: &[FileId],
    ) -> Result<()>;
    fn search_files(&self, query: &str) -> Result<Vec<FileEntry>>;
    /// List all files that are identified as media (images/videos).
    fn list_media_files(&self, device_id: &DeviceId) -> Result<Vec<FileEntry>>;
    /// Get recent media files across all devices.
    fn get_recent_media(&self, limit: u32) -> Result<Vec<FileEntry>>;
    /// Get differences between two file snapshots.
    fn get_file_diff(
        &self,
        old_snapshot_id: &SnapshotId,
        new_snapshot_id: &SnapshotId,
    ) -> Result<domain::FileDiff>;

    // V4.0 Sub-file Chunking & Physical Objects
    fn save_logical_chunk(&self, content_hash: &str, size: u64) -> Result<String>;
    fn get_logical_chunk_by_hash(&self, content_hash: &str) -> Result<Option<String>>;
    fn save_physical_object(
        &self,
        chunk_id: &str,
        object_hash: &str,
        storage_key: &str,
        stored_size: u64,
        compression: &str,
        enc_version: u32,
    ) -> Result<String>;
    fn get_physical_object_by_hash(&self, object_hash: &str) -> Result<Option<String>>;
    fn get_storage_key_for_chunk(&self, chunk_id: &str) -> Result<Option<String>>;

    fn save_file_chunk(
        &self,
        file_id: &FileId,
        chunk_id: &str,
        offset: u64,
        length: u32,
        sequence: u32,
    ) -> Result<()>;
    fn get_file_chunks(&self, file_id: &FileId) -> Result<Vec<(String, u64, u32, String)>>; // (chunk_id, offset, length, storage_key)
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
    /// Get differences between two contact snapshots.
    fn get_contact_diff(
        &self,
        old_snapshot_id: &SnapshotId,
        new_snapshot_id: &SnapshotId,
    ) -> Result<domain::ContactDiff>;
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
    /// Remove objects (files, contacts) that are no longer referenced by any snapshot.
    fn prune_orphans(&self) -> Result<u64>;
    /// Create a live backup of the database to the specified path.
    fn create_database_backup(&self, destination_path: &str) -> Result<()>;
}

pub trait SmsRepositoryPort: Send + Sync {
    fn save_sms(&self, snapshot_id: &SnapshotId, sms: &domain::Sms) -> Result<()>;
    fn save_sms_batch(&self, snapshot_id: &SnapshotId, sms_list: &[domain::Sms]) -> Result<()>;
    fn get_snapshot_sms(&self, snapshot_id: &SnapshotId) -> Result<Vec<domain::Sms>>;
    fn search_sms(&self, query: &str) -> Result<Vec<(SnapshotId, domain::Sms)>>;
}

pub trait CallLogRepositoryPort: Send + Sync {
    fn save_call_log(&self, snapshot_id: &SnapshotId, log: &domain::CallLog) -> Result<()>;
    fn save_call_logs_batch(
        &self,
        snapshot_id: &SnapshotId,
        logs: &[domain::CallLog],
    ) -> Result<()>;
    fn get_snapshot_call_logs(&self, snapshot_id: &SnapshotId) -> Result<Vec<domain::CallLog>>;
    fn search_call_logs(&self, query: &str) -> Result<Vec<(SnapshotId, domain::CallLog)>>;
}

pub trait RepositoryPort:
    DeviceRepositoryPort
    + SnapshotRepositoryPort
    + FileRepositoryPort
    + AppRepositoryPort
    + ContactRepositoryPort
    + ScheduleRepositoryPort
    + SettingsRepositoryPort
    + MaintenanceRepositoryPort
    + SmsRepositoryPort
    + CallLogRepositoryPort
    + Send
    + Sync
{
}
