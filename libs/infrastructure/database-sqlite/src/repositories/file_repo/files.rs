use crate::mappers::BackupMapper;
use domain::{DeviceId, FileEntry, FileId, SnapshotId};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use std::sync::Arc;

pub struct FileMetadataOps;

impl FileMetadataOps {
    pub fn save_file(pool: &Arc<Pool<SqliteConnectionManager>>, file: &FileEntry) -> anyhow::Result<()> {
        let conn = pool.get()?;
        let media_info_json = file.media_info.as_ref().map(|m| serde_json::to_string(m).unwrap());
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

    pub fn save_files_batch(pool: &Arc<Pool<SqliteConnectionManager>>, files: &[FileEntry]) -> anyhow::Result<()> {
        let mut conn = pool.get()?;
        let tx = conn.transaction()?;
        {
            let mut stmt = tx.prepare_cached(
                "INSERT OR REPLACE INTO files
                (id, device_id, path, name, size_bytes, modified_at, mime_type, permissions, hash_sha256, thumbnail_hash, media_info)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)"
            )?;

            for file in files {
                let media_info_json = file.media_info.as_ref().map(|m| serde_json::to_string(m).unwrap());
                stmt.execute(params![
                    file.id.0, file.device_id.0, file.path, file.name, file.size_bytes,
                    file.modified_at.to_rfc3339(), file.mime_type, file.permissions,
                    file.hash_sha256, file.thumbnail_hash, media_info_json
                ])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn list_files(pool: &Arc<Pool<SqliteConnectionManager>>, device_id: &DeviceId) -> anyhow::Result<Vec<FileEntry>> {
        let conn = pool.get()?;
        let mut stmt = conn.prepare("SELECT * FROM files WHERE device_id = ?1")?;
        let file_iter = stmt.query_map([&device_id.0], BackupMapper::to_file)?;
        let mut files = Vec::new();
        for f in file_iter {
            files.push(f?);
        }
        Ok(files)
    }

    pub fn get_snapshot_files(pool: &Arc<Pool<SqliteConnectionManager>>, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<FileEntry>> {
        let conn = pool.get()?;
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

    pub fn link_file_to_snapshot(pool: &Arc<Pool<SqliteConnectionManager>>, snapshot_id: &SnapshotId, file_id: &FileId) -> anyhow::Result<()> {
        let conn = pool.get()?;
        conn.execute(
            "INSERT OR IGNORE INTO snapshot_files (snapshot_id, file_id) VALUES (?1, ?2)",
            params![snapshot_id.0, file_id.0],
        )?;
        Ok(())
    }

    pub fn link_files_to_snapshot_batch(pool: &Arc<Pool<SqliteConnectionManager>>, snapshot_id: &SnapshotId, file_ids: &[FileId]) -> anyhow::Result<()> {
        let mut conn = pool.get()?;
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

    pub fn search_files(pool: &Arc<Pool<SqliteConnectionManager>>, query: &str) -> anyhow::Result<Vec<FileEntry>> {
        let conn = pool.get()?;
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
            let mut stmt_like = conn.prepare("SELECT * FROM files WHERE name LIKE ?1 OR path LIKE ?1 LIMIT 100")?;
            let pattern = format!("%{}%", query);
            let file_iter = stmt_like.query_map([pattern], BackupMapper::to_file)?;
            for f in file_iter {
                files.push(f?);
            }
        }

        Ok(files)
    }

    pub fn list_media_files(pool: &Arc<Pool<SqliteConnectionManager>>, device_id: &DeviceId) -> anyhow::Result<Vec<FileEntry>> {
        let conn = pool.get()?;
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

    pub fn get_recent_media(pool: &Arc<Pool<SqliteConnectionManager>>, limit: u32) -> anyhow::Result<Vec<FileEntry>> {
        let conn = pool.get()?;
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

    pub fn get_file_diff(pool: &Arc<Pool<SqliteConnectionManager>>, old_snapshot_id: &SnapshotId, new_snapshot_id: &SnapshotId) -> anyhow::Result<domain::FileDiff> {
        let old_files = Self::get_snapshot_files(pool, old_snapshot_id)?;
        let new_files = Self::get_snapshot_files(pool, new_snapshot_id)?;
        let mut diff = domain::FileDiff::default();

        let old_map: std::collections::HashMap<String, domain::FileEntry> =
            old_files.into_iter().map(|f| (f.path.clone(), f)).collect();

        let mut new_map: std::collections::HashMap<String, domain::FileEntry> =
            new_files.into_iter().map(|f| (f.path.clone(), f)).collect();

        for (path, new_file) in new_map.drain() {
            if let Some(old_file) = old_map.get(&path) {
                if old_file.hash_sha256 != new_file.hash_sha256 || old_file.size_bytes != new_file.size_bytes {
                    diff.modified.push(new_file);
                }
            } else {
                diff.added.push(new_file);
            }
        }

        let new_files_again = Self::get_snapshot_files(pool, new_snapshot_id)?;
        let new_paths: std::collections::HashSet<String> =
            new_files_again.into_iter().map(|f| f.path).collect();

        for (path, old_file) in old_map {
            if !new_paths.contains(&path) {
                diff.removed.push(old_file);
            }
        }

        Ok(diff)
    }
}
