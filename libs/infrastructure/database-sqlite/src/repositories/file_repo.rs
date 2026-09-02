use crate::mappers::BackupMapper;
use domain::{DeviceId, FileEntry, FileId, SnapshotId};
use ports::FileRepositoryPort;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use std::sync::Arc;

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
        let conn = self.pool.get()?;
        let media_info_json = file
            .media_info
            .as_ref()
            .map(|m| serde_json::to_string(m).unwrap());
        conn.execute(
            "INSERT OR REPLACE INTO files
            (id, device_id, path, name, size_bytes, modified_at, mime_type, permissions, hash_sha256, media_info)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                file.id.0, file.device_id.0, file.path, file.name, file.size_bytes,
                file.modified_at.to_rfc3339(), file.mime_type, file.permissions,
                file.hash_sha256, media_info_json
            ],
        )?;
        Ok(())
    }

    fn save_files_batch(&self, files: &[FileEntry]) -> anyhow::Result<()> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;

        {
            let mut stmt = tx.prepare_cached(
                "INSERT OR REPLACE INTO files
                (id, device_id, path, name, size_bytes, modified_at, mime_type, permissions, hash_sha256, thumbnail_hash, media_info)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)"
            )?;

            for file in files {
                let media_info_json = file
                    .media_info
                    .as_ref()
                    .map(|m| serde_json::to_string(m).unwrap());
                stmt.execute(params![
                    file.id.0,
                    file.device_id.0,
                    file.path,
                    file.name,
                    file.size_bytes,
                    file.modified_at.to_rfc3339(),
                    file.mime_type,
                    file.permissions,
                    file.hash_sha256,
                    file.thumbnail_hash,
                    media_info_json
                ])?;
            }
        }

        tx.commit()?;
        Ok(())
    }

    fn list_files(&self, device_id: &DeviceId) -> anyhow::Result<Vec<FileEntry>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT * FROM files WHERE device_id = ?1")?;
        let file_iter = stmt.query_map([&device_id.0], BackupMapper::to_file)?;
        let mut files = Vec::new();
        for f in file_iter {
            files.push(f?);
        }
        Ok(files)
    }

    fn get_snapshot_files(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<FileEntry>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT f.* FROM files f
             JOIN snapshot_files sf ON f.id = sf.file_id
             WHERE sf.snapshot_id = ?1",
        )?;
        let file_iter = stmt.query_map([&snapshot_id.0], BackupMapper::to_file)?;
        let mut files = Vec::new();
        for f in file_iter {
            files.push(f?);
        }
        Ok(files)
    }

    fn link_file_to_snapshot(
        &self,
        snapshot_id: &SnapshotId,
        file_id: &FileId,
    ) -> anyhow::Result<()> {
        let conn = self.pool.get()?;
        conn.execute(
            "INSERT OR IGNORE INTO snapshot_files (snapshot_id, file_id) VALUES (?1, ?2)",
            params![snapshot_id.0, file_id.0],
        )?;
        Ok(())
    }

    fn link_files_to_snapshot_batch(
        &self,
        snapshot_id: &SnapshotId,
        file_ids: &[FileId],
    ) -> anyhow::Result<()> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;

        {
            let mut stmt = tx.prepare_cached(
                "INSERT OR IGNORE INTO snapshot_files (snapshot_id, file_id) VALUES (?1, ?2)",
            )?;

            for file_id in file_ids {
                stmt.execute(params![snapshot_id.0, file_id.0])?;
            }
        }

        tx.commit()?;
        Ok(())
    }

    fn search_files(&self, query: &str) -> anyhow::Result<Vec<FileEntry>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT f.* FROM files f
             JOIN files_fts fts ON f.rowid = fts.rowid
             WHERE files_fts MATCH ?1 ORDER BY rank",
        )?;
        let fts_query = format!("\"{}\"*", query.replace("\"", "\"\""));
        let file_iter = stmt.query_map([fts_query], BackupMapper::to_file)?;
        let mut files = Vec::new();
        for f in file_iter {
            files.push(f?);
        }

        if files.is_empty() {
            let mut stmt_like =
                conn.prepare("SELECT * FROM files WHERE name LIKE ?1 OR path LIKE ?1 LIMIT 100")?;
            let pattern = format!("%{}%", query);
            let file_iter = stmt_like.query_map([pattern], BackupMapper::to_file)?;
            for f in file_iter {
                files.push(f?);
            }
        }

        Ok(files)
    }

    fn list_media_files(&self, device_id: &DeviceId) -> anyhow::Result<Vec<FileEntry>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT * FROM files
             WHERE device_id = ?1
             AND (mime_type LIKE 'image/%' OR mime_type LIKE 'video/%')
             ORDER BY modified_at DESC",
        )?;
        let file_iter = stmt.query_map([&device_id.0], BackupMapper::to_file)?;
        let mut files = Vec::new();
        for f in file_iter {
            files.push(f?);
        }
        Ok(files)
    }

    fn get_recent_media(&self, limit: u32) -> anyhow::Result<Vec<FileEntry>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT * FROM files
             WHERE mime_type LIKE 'image/%' OR mime_type LIKE 'video/%'
             ORDER BY modified_at DESC LIMIT ?1",
        )?;
        let file_iter = stmt.query_map([limit], BackupMapper::to_file)?;
        let mut files = Vec::new();
        for f in file_iter {
            files.push(f?);
        }
        Ok(files)
    }

    fn get_file_diff(
        &self,
        old_snapshot_id: &SnapshotId,
        new_snapshot_id: &SnapshotId,
    ) -> anyhow::Result<domain::FileDiff> {
        let old_files = self.get_snapshot_files(old_snapshot_id)?;
        let new_files = self.get_snapshot_files(new_snapshot_id)?;

        let mut diff = domain::FileDiff::default();

        let old_map: std::collections::HashMap<String, domain::FileEntry> =
            old_files.into_iter().map(|f| (f.path.clone(), f)).collect();

        let mut new_map: std::collections::HashMap<String, domain::FileEntry> =
            new_files.into_iter().map(|f| (f.path.clone(), f)).collect();

        for (path, new_file) in new_map.drain() {
            if let Some(old_file) = old_map.get(&path) {
                if old_file.hash_sha256 != new_file.hash_sha256
                    || old_file.size_bytes != new_file.size_bytes
                {
                    diff.modified.push(new_file);
                }
            } else {
                diff.added.push(new_file);
            }
        }

        // Re-collect new_files to get paths for removal check
        let new_files_again = self.get_snapshot_files(new_snapshot_id)?;
        let new_paths: std::collections::HashSet<String> =
            new_files_again.into_iter().map(|f| f.path).collect();

        for (path, old_file) in old_map {
            if !new_paths.contains(&path) {
                diff.removed.push(old_file);
            }
        }

        Ok(diff)
    }

    fn save_logical_chunk(&self, content_hash: &str, size: u64) -> anyhow::Result<String> {
        let conn = self.pool.get()?;
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        conn.execute(
            "INSERT OR IGNORE INTO chunks (id, content_hash, plaintext_size, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![id, content_hash, size, now],
        )?;

        // If it was ignored, we need to find the existing ID
        if let Some(existing_id) = self.get_logical_chunk_by_hash(content_hash)? {
            Ok(existing_id)
        } else {
            Ok(id)
        }
    }

    fn get_logical_chunk_by_hash(&self, content_hash: &str) -> anyhow::Result<Option<String>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT id FROM chunks WHERE content_hash = ?1")?;
        let mut rows = stmt.query([content_hash])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
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
        let conn = self.pool.get()?;
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        conn.execute(
            "INSERT OR IGNORE INTO chunk_objects (id, chunk_id, object_hash, storage_key, stored_size, compression_alg, encryption_version, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![id, chunk_id, object_hash, storage_key, stored_size, compression, enc_version, now],
        )?;

        if let Some(existing_id) = self.get_physical_object_by_hash(object_hash)? {
            Ok(existing_id)
        } else {
            Ok(id)
        }
    }

    fn get_physical_object_by_hash(&self, object_hash: &str) -> anyhow::Result<Option<String>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT id FROM chunk_objects WHERE object_hash = ?1")?;
        let mut rows = stmt.query([object_hash])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    fn get_storage_key_for_chunk(&self, chunk_id: &str) -> anyhow::Result<Option<String>> {
        let conn = self.pool.get()?;
        let mut stmt =
            conn.prepare("SELECT storage_key FROM chunk_objects WHERE chunk_id = ?1 LIMIT 1")?;
        let mut rows = stmt.query([chunk_id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    fn save_file_chunk(
        &self,
        file_id: &FileId,
        chunk_id: &str,
        offset: u64,
        length: u32,
        sequence: u32,
    ) -> anyhow::Result<()> {
        let conn = self.pool.get()?;
        conn.execute(
            "INSERT OR REPLACE INTO file_chunks (file_id, chunk_id, chunk_offset, chunk_length, sequence)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![file_id.0, chunk_id, offset, length, sequence],
        )?;
        Ok(())
    }

    fn get_file_chunks(&self, file_id: &FileId) -> anyhow::Result<Vec<(String, u64, u32, String)>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT fc.chunk_id, fc.chunk_offset, fc.chunk_length, co.storage_key
             FROM file_chunks fc
             JOIN (
                SELECT chunk_id, storage_key, MAX(created_at)
                FROM chunk_objects
                GROUP BY chunk_id
             ) co ON fc.chunk_id = co.chunk_id
             WHERE fc.file_id = ?1 ORDER BY fc.sequence ASC",
        )?;

        let chunk_iter = stmt.query_map([&file_id.0], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })?;

        let mut chunks = Vec::new();
        for c in chunk_iter {
            chunks.push(c?);
        }
        Ok(chunks)
    }
}
