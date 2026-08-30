use rusqlite::{params, Connection};
use domain::{DeviceId, FileEntry, FileId};
use crate::mappers::BackupMapper;

pub struct FileRepository;

impl FileRepository {
    pub fn save(conn: &Connection, file: &FileEntry) -> anyhow::Result<()> {
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

    pub fn list_by_device(conn: &Connection, device_id: &DeviceId) -> anyhow::Result<Vec<FileEntry>> {
        let mut stmt = conn.prepare("SELECT * FROM files WHERE device_id = ?1")?;
        let file_iter = stmt.query_map([&device_id.0], BackupMapper::to_file)?;
        let mut files = Vec::new();
        for f in file_iter { files.push(f?); }
        Ok(files)
    }

    pub fn search(conn: &Connection, query: &str) -> anyhow::Result<Vec<FileEntry>> {
        let mut stmt = conn.prepare("SELECT * FROM files WHERE name LIKE ?1 OR path LIKE ?1")?;
        let pattern = format!("%{}%", query);
        let file_iter = stmt.query_map([pattern], BackupMapper::to_file)?;
        let mut files = Vec::new();
        for f in file_iter { files.push(f?); }
        Ok(files)
    }

    pub fn save_chunk(conn: &Connection, file_id: &FileId, chunk_hash: &str, offset: u64, length: u32, sequence: u32) -> anyhow::Result<()> {
        conn.execute(
            "INSERT OR REPLACE INTO file_chunks (file_id, chunk_hash, chunk_offset, chunk_length, sequence)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![file_id.0, chunk_hash, offset, length, sequence],
        )?;
        Ok(())
    }

    pub fn get_chunks(conn: &Connection, file_id: &FileId) -> anyhow::Result<Vec<(String, u64, u32)>> {
        let mut stmt = conn.prepare(
            "SELECT chunk_hash, chunk_offset, chunk_length FROM file_chunks
             WHERE file_id = ?1 ORDER BY sequence ASC"
        )?;

        let chunk_iter = stmt.query_map([&file_id.0], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;

        let mut chunks = Vec::new();
        for c in chunk_iter {
            chunks.push(c?);
        }
        Ok(chunks)
    }
}
