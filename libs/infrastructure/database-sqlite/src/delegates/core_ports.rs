use domain::{DeviceId, Device, FileEntry, FileId, Snapshot, SnapshotId};
use ports::{DeviceRepositoryPort, FileRepositoryPort, SnapshotRepositoryPort};

use crate::facade::SqliteRepository;

impl DeviceRepositoryPort for SqliteRepository {
    fn save_device(&self, device: &Device) -> anyhow::Result<()> {
        self.devices().save_device(device)
    }
    fn list_devices(&self) -> anyhow::Result<Vec<Device>> {
        self.devices().list_devices()
    }
    fn get_device(&self, id: &DeviceId) -> anyhow::Result<Option<Device>> {
        self.devices().get_device(id)
    }
    fn get_storage_usage_by_device(&self, device_id: &DeviceId) -> anyhow::Result<u64> {
        self.devices().get_storage_usage_by_device(device_id)
    }
}

impl SnapshotRepositoryPort for SqliteRepository {
    fn get_snapshot(&self, id: &SnapshotId) -> anyhow::Result<Option<Snapshot>> {
        self.snapshots().get_snapshot(id)
    }
    fn create_snapshot(&self, snapshot: &Snapshot) -> anyhow::Result<()> {
        self.snapshots().create_snapshot(snapshot)
    }
    fn update_snapshot(&self, snapshot: &Snapshot) -> anyhow::Result<()> {
        self.snapshots().update_snapshot(snapshot)
    }
    fn list_snapshots(&self, device_id: &DeviceId) -> anyhow::Result<Vec<Snapshot>> {
        self.snapshots().list_snapshots(device_id)
    }
    fn list_all_snapshots(&self) -> anyhow::Result<Vec<Snapshot>> {
        self.snapshots().list_all_snapshots()
    }
    fn get_latest_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> {
        self.snapshots().get_latest_snapshot(device_id)
    }
    fn get_latest_completed_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> {
        self.snapshots().get_latest_completed_snapshot(device_id)
    }
    fn get_incomplete_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> {
        self.snapshots().get_incomplete_snapshot(device_id)
    }
    fn get_resumable_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> {
        self.snapshots().get_resumable_snapshot(device_id)
    }
    fn delete_snapshot(&self, snapshot_id: &SnapshotId) -> anyhow::Result<()> {
        self.snapshots().delete_snapshot(snapshot_id)
    }
    fn save_structured_data_ref(
        &self,
        snapshot_id: &SnapshotId,
        data_type: domain::StructuredDataType,
        object_id: &str,
    ) -> anyhow::Result<()> {
        self.snapshots().save_structured_data_ref(snapshot_id, data_type, object_id)
    }
    fn get_structured_data_ref(
        &self,
        snapshot_id: &SnapshotId,
        data_type: domain::StructuredDataType,
    ) -> anyhow::Result<Option<String>> {
        self.snapshots().get_structured_data_ref(snapshot_id, data_type)
    }
}

impl FileRepositoryPort for SqliteRepository {
    fn save_file(&self, file: &FileEntry) -> anyhow::Result<()> {
        self.files().save_file(file)
    }
    fn save_files_batch(&self, files: &[FileEntry]) -> anyhow::Result<()> {
        self.files().save_files_batch(files)
    }
    fn list_files(&self, device_id: &DeviceId) -> anyhow::Result<Vec<FileEntry>> {
        self.files().list_files(device_id)
    }
    fn get_snapshot_files(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<FileEntry>> {
        self.files().get_snapshot_files(snapshot_id)
    }
    fn link_file_to_snapshot(&self, snapshot_id: &SnapshotId, file_id: &FileId) -> anyhow::Result<()> {
        self.files().link_file_to_snapshot(snapshot_id, file_id)
    }
    fn link_files_to_snapshot_batch(&self, snapshot_id: &SnapshotId, file_ids: &[FileId]) -> anyhow::Result<()> {
        self.files().link_files_to_snapshot_batch(snapshot_id, file_ids)
    }
    fn search_files(&self, query: &str) -> anyhow::Result<Vec<FileEntry>> {
        self.files().search_files(query)
    }
    fn list_media_files(&self, device_id: &DeviceId) -> anyhow::Result<Vec<FileEntry>> {
        self.files().list_media_files(device_id)
    }
    fn get_recent_media(&self, limit: u32) -> anyhow::Result<Vec<FileEntry>> {
        self.files().get_recent_media(limit)
    }
    fn get_file_diff(&self, old_snapshot_id: &SnapshotId, new_snapshot_id: &SnapshotId) -> anyhow::Result<domain::FileDiff> {
        self.files().get_file_diff(old_snapshot_id, new_snapshot_id)
    }
    fn save_logical_chunk(&self, content_hash: &str, size: u64) -> anyhow::Result<String> {
        self.files().save_logical_chunk(content_hash, size)
    }
    fn get_logical_chunk_by_hash(&self, content_hash: &str) -> anyhow::Result<Option<String>> {
        self.files().get_logical_chunk_by_hash(content_hash)
    }
    fn save_physical_object(
        &self,
        chunk_id: &str,
        object_hash: &str,
        storage_key: &str,
        stored_size: u64,
        compression: &str,
        enc_version: u32,
    ) -> anyhow::Result<String> {
        self.files().save_physical_object(chunk_id, object_hash, storage_key, stored_size, compression, enc_version)
    }
    fn get_physical_object_by_hash(&self, object_hash: &str) -> anyhow::Result<Option<String>> {
        self.files().get_physical_object_by_hash(object_hash)
    }
    fn get_storage_key_for_chunk(&self, chunk_id: &str) -> anyhow::Result<Option<String>> {
        self.files().get_storage_key_for_chunk(chunk_id)
    }
    fn save_file_chunk(&self, file_id: &FileId, chunk_id: &str, offset: u64, length: u32, sequence: u32) -> anyhow::Result<()> {
        self.files().save_file_chunk(file_id, chunk_id, offset, length, sequence)
    }
    fn get_file_chunks(&self, file_id: &FileId) -> anyhow::Result<Vec<(String, u64, u32, String)>> {
        self.files().get_file_chunks(file_id)
    }
}
