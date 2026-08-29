use anyhow::Result;
use domain::{Device, DeviceId, FileEntry, Snapshot, SnapshotId, BackupSchedule};

pub trait RepositoryPort: Send + Sync {
    /// Save or update device information.
    fn save_device(&self, device: &Device) -> Result<()>;

    /// Record a file entry in the database.
    fn save_file(&self, file: &FileEntry) -> Result<()>;

    /// List all known devices in the repository.
    fn list_devices(&self) -> Result<Vec<Device>>;

    /// Find a device by its ID.
    fn get_device(&self, id: &DeviceId) -> Result<Option<Device>>;

    /// Get a snapshot by its ID.
    fn get_snapshot(&self, id: &SnapshotId) -> Result<Option<Snapshot>>;

    /// Find files for a device.
    fn list_files(&self, device_id: &DeviceId) -> Result<Vec<FileEntry>>;

    /// Create a new snapshot record.
    fn create_snapshot(&self, snapshot: &Snapshot) -> Result<()>;

    /// Update an existing snapshot.
    fn update_snapshot(&self, snapshot: &Snapshot) -> Result<()>;

    /// Link a file to a snapshot.
    fn link_file_to_snapshot(&self, snapshot_id: &SnapshotId, file_id: &domain::FileId) -> Result<()>;

    /// List all snapshots for a device.
    fn list_snapshots(&self, device_id: &DeviceId) -> Result<Vec<Snapshot>>;

    /// Get the latest completed snapshot for a device.
    fn get_latest_snapshot(&self, device_id: &DeviceId) -> Result<Option<Snapshot>>;

    /// Get the latest incomplete (Running/Interrupted) snapshot for a device.
    fn get_incomplete_snapshot(&self, device_id: &DeviceId) -> Result<Option<Snapshot>>;

    /// Get all file entries belonging to a specific snapshot.
    fn get_snapshot_files(&self, snapshot_id: &SnapshotId) -> Result<Vec<FileEntry>>;

    /// Save app information to the repository.
    fn save_app(&self, app: &domain::AppInfo) -> Result<()>;

    /// Link an app to a snapshot.
    fn link_app_to_snapshot(&self, snapshot_id: &SnapshotId, app_id: &domain::AppId) -> Result<()>;

    /// List apps for a snapshot.
    fn get_snapshot_apps(&self, snapshot_id: &SnapshotId) -> Result<Vec<domain::AppInfo>>;

    /// Record a reference to structured data (JSON/etc) in a snapshot.
    fn save_structured_data_ref(&self, snapshot_id: &SnapshotId, data_type: &str, object_id: &str) -> Result<()>;

    /// Save or update a backup schedule.
    fn save_schedule(&self, schedule: &BackupSchedule) -> Result<()>;

    /// Get schedule for a device.
    fn get_schedule(&self, device_id: &DeviceId) -> Result<Option<BackupSchedule>>;

    /// List all enabled schedules.
    fn list_schedules(&self) -> Result<Vec<BackupSchedule>>;

    /// Delete a snapshot and its metadata (not physical objects).
    fn delete_snapshot(&self, snapshot_id: &SnapshotId) -> Result<()>;

    /// Search for files across all devices and snapshots by name pattern.
    fn search_files(&self, query: &str) -> Result<Vec<FileEntry>>;

    /// Save a chunk mapping for a file.
    fn save_file_chunk(&self, file_id: &domain::FileId, chunk_hash: &str, offset: u64, length: u32, sequence: u32) -> Result<()>;

    /// Get chunk mappings for a file.
    fn get_file_chunks(&self, file_id: &domain::FileId) -> Result<Vec<(String, u64, u32)>>;

    /// Get all unique hashes (file hashes and chunk hashes) currently referenced in the database.
    fn get_all_referenced_hashes(&self) -> Result<std::collections::HashSet<String>>;
}
