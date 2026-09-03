use domain::FileId;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use std::sync::Arc;

pub struct ChunkOps;

impl ChunkOps {
    pub fn save_logical_chunk(pool: &Arc<Pool<SqliteConnectionManager>>, content_hash: &str, size: u64) -> anyhow::Result<String> {
        let conn = pool.get()?;
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        conn.execute(
            "INSERT OR IGNORE INTO chunks (id, content_hash, plaintext_size, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![id, content_hash, size, now],
        )?;

        if let Some(existing_id) = Self::get_logical_chunk_by_hash(pool, content_hash)? {
            Ok(existing_id)
        } else {
            Ok(id)
        }
    }

    pub fn get_logical_chunk_by_hash(pool: &Arc<Pool<SqliteConnectionManager>>, content_hash: &str) -> anyhow::Result<Option<String>> {
        let conn = pool.get()?;
        let mut stmt = conn.prepare("SELECT id FROM chunks WHERE content_hash = ?1")?;
        let mut rows = stmt.query([content_hash])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn save_physical_object(
        pool: &Arc<Pool<SqliteConnectionManager>>,
        chunk_id: &str,
        object_hash: &str,
        storage_key: &str,
        stored_size: u64,
        compression: &str,
        enc_version: u32,
    ) -> anyhow::Result<String> {
        let conn = pool.get()?;
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        conn.execute(
            "INSERT OR IGNORE INTO chunk_objects (id, chunk_id, object_hash, storage_key, stored_size, compression_alg, encryption_version, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![id, chunk_id, object_hash, storage_key, stored_size, compression, enc_version, now],
        )?;

        if let Some(existing_id) = Self::get_physical_object_by_hash(pool, object_hash)? {
            Ok(existing_id)
        } else {
            Ok(id)
        }
    }

    pub fn get_physical_object_by_hash(pool: &Arc<Pool<SqliteConnectionManager>>, object_hash: &str) -> anyhow::Result<Option<String>> {
        let conn = pool.get()?;
        let mut stmt = conn.prepare("SELECT id FROM chunk_objects WHERE object_hash = ?1")?;
        let mut rows = stmt.query([object_hash])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn get_storage_key_for_chunk(pool: &Arc<Pool<SqliteConnectionManager>>, chunk_id: &str) -> anyhow::Result<Option<String>> {
        let conn = pool.get()?;
        let mut stmt = conn.prepare("SELECT storage_key FROM chunk_objects WHERE chunk_id = ?1 LIMIT 1")?;
        let mut rows = stmt.query([chunk_id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn save_file_chunk(
        pool: &Arc<Pool<SqliteConnectionManager>>,
        file_id: &FileId,
        chunk_id: &str,
        offset: u64,
        length: u32,
        sequence: u32,
    ) -> anyhow::Result<()> {
        let conn = pool.get()?;
        conn.execute(
            "INSERT OR REPLACE INTO file_chunks (file_id, chunk_id, chunk_offset, chunk_length, sequence)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![file_id.0, chunk_id, offset, length, sequence],
        )?;
        Ok(())
    }

    pub fn get_file_chunks(pool: &Arc<Pool<SqliteConnectionManager>>, file_id: &FileId) -> anyhow::Result<Vec<(String, u64, u32, String)>> {
        let conn = pool.get()?;
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
