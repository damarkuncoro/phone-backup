pub mod chunks;
pub mod files;

use domain::{DeviceId, FileEntry, FileId, SnapshotId};
use ports::FileRepositoryPort;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::sync::Arc;

use chunks::ChunkOps;
use files::FileMetadataOps;

pub struct FileRepository {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl FileRepository {
    pub fn new(pool: Arc<Pool<SqliteConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl FileRepositoryPort for FileRepository {
    fn save_file(&self, file: &FileEntry) -> anyhow::Result<()> {
        FileMetadataOps::save_file(&self.pool, file)
    }

    fn save_files_batch(&self, files: &[FileEntry]) -> anyhow::Result<()> {
        FileMetadataOps::save_files_batch(&self.pool, files)
    }

    fn list_files(&self, device_id: &DeviceId) -> anyhow::Result<Vec<FileEntry>> {
        FileMetadataOps::list_files(&self.pool, device_id)
    }

    fn get_snapshot_files(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<FileEntry>> {
        FileMetadataOps::get_snapshot_files(&self.pool, snapshot_id)
    }

    fn link_file_to_snapshot(&self, snapshot_id: &SnapshotId, file_id: &FileId) -> anyhow::Result<()> {
        FileMetadataOps::link_file_to_snapshot(&self.pool, snapshot_id, file_id)
    }

    fn link_files_to_snapshot_batch(&self, snapshot_id: &SnapshotId, file_ids: &[FileId]) -> anyhow::Result<()> {
        FileMetadataOps::link_files_to_snapshot_batch(&self.pool, snapshot_id, file_ids)
    }

    fn search_files(&self, query: &str) -> anyhow::Result<Vec<FileEntry>> {
        FileMetadataOps::search_files(&self.pool, query)
    }

    fn list_media_files(&self, device_id: &DeviceId) -> anyhow::Result<Vec<FileEntry>> {
        FileMetadataOps::list_media_files(&self.pool, device_id)
    }

    fn get_recent_media(&self, limit: u32) -> anyhow::Result<Vec<FileEntry>> {
        FileMetadataOps::get_recent_media(&self.pool, limit)
    }

    fn get_file_diff(&self, old_snapshot_id: &SnapshotId, new_snapshot_id: &SnapshotId) -> anyhow::Result<domain::FileDiff> {
        FileMetadataOps::get_file_diff(&self.pool, old_snapshot_id, new_snapshot_id)
    }

    fn save_logical_chunk(&self, content_hash: &str, size: u64) -> anyhow::Result<String> {
        ChunkOps::save_logical_chunk(&self.pool, content_hash, size)
    }

    fn get_logical_chunk_by_hash(&self, content_hash: &str) -> anyhow::Result<Option<String>> {
        ChunkOps::get_logical_chunk_by_hash(&self.pool, content_hash)
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
        ChunkOps::save_physical_object(&self.pool, chunk_id, object_hash, storage_key, stored_size, compression, enc_version)
    }

    fn get_physical_object_by_hash(&self, object_hash: &str) -> anyhow::Result<Option<String>> {
        ChunkOps::get_physical_object_by_hash(&self.pool, object_hash)
    }

    fn get_storage_key_for_chunk(&self, chunk_id: &str) -> anyhow::Result<Option<String>> {
        ChunkOps::get_storage_key_for_chunk(&self.pool, chunk_id)
    }

    fn save_file_chunk(&self, file_id: &FileId, chunk_id: &str, offset: u64, length: u32, sequence: u32) -> anyhow::Result<()> {
        ChunkOps::save_file_chunk(&self.pool, file_id, chunk_id, offset, length, sequence)
    }

    fn get_file_chunks(&self, file_id: &FileId) -> anyhow::Result<Vec<(String, u64, u32, String)>> {
        ChunkOps::get_file_chunks(&self.pool, file_id)
    }
}
