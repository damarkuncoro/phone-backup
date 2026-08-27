use crate::mappers::map_row_to_file;
use domain::{DeviceId, FileEntry};
use rusqlite::{params, Connection};

pub fn save_file(conn: &Connection, file: &FileEntry) -> anyhow::Result<()> {
    let media_info_json = file.media_info.as_ref().map(|m| serde_json::to_string(m).unwrap());

    conn.execute(
        "INSERT OR REPLACE INTO files
        (id, device_id, path, name, size_bytes, modified_at, mime_type, permissions, hash_sha256, media_info)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            file.id.0,
            file.device_id.0,
            file.path,
            file.name,
            file.size_bytes,
            file.modified_at.to_rfc3339(),
            file.mime_type,
            file.permissions,
            file.hash_sha256,
            media_info_json
        ],
    )?;
    Ok(())
}

pub fn list_files(conn: &Connection, device_id: &DeviceId) -> anyhow::Result<Vec<FileEntry>> {
    let mut stmt = conn.prepare(
        "SELECT id, device_id, path, name, size_bytes, modified_at, mime_type, permissions, hash_sha256, media_info
         FROM files WHERE device_id = ?1"
    )?;

    let file_iter = stmt.query_map([&device_id.0], map_row_to_file)?;

    let mut files = Vec::new();
    for f in file_iter {
        files.push(f?);
    }
    Ok(files)
}

pub fn search_files(conn: &Connection, query: &str) -> anyhow::Result<Vec<FileEntry>> {
    let mut stmt = conn.prepare(
        "SELECT id, device_id, path, name, size_bytes, modified_at, mime_type, permissions, hash_sha256, media_info
         FROM files WHERE name LIKE ?1 OR path LIKE ?1"
    )?;

    let pattern = format!("%{}%", query);
    let file_iter = stmt.query_map([pattern], map_row_to_file)?;

    let mut files = Vec::new();
    for f in file_iter {
        files.push(f?);
    }
    Ok(files)
}
