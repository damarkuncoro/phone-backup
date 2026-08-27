use crate::mappers::{map_row_to_file, map_row_to_snapshot};
use domain::{DeviceId, FileEntry, FileId, Snapshot, SnapshotId};
use rusqlite::{params, Connection};

pub fn create_snapshot(conn: &Connection, snapshot: &Snapshot) -> anyhow::Result<()> {
    conn.execute(
        "INSERT INTO snapshots
        (id, device_id, started_at, finished_at, status, total_files, total_bytes, deduped_bytes)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            snapshot.id.0,
            snapshot.device_id.0,
            snapshot.started_at.to_rfc3339(),
            snapshot.finished_at.map(|t| t.to_rfc3339()),
            format!("{:?}", snapshot.status),
            snapshot.total_files,
            snapshot.total_bytes,
            snapshot.deduped_bytes,
        ],
    )?;
    Ok(())
}

pub fn update_snapshot(conn: &Connection, snapshot: &Snapshot) -> anyhow::Result<()> {
    conn.execute(
        "UPDATE snapshots SET
            finished_at = ?2,
            status = ?3,
            total_files = ?4,
            total_bytes = ?5,
            deduped_bytes = ?6
        WHERE id = ?1",
        params![
            snapshot.id.0,
            snapshot.finished_at.map(|t| t.to_rfc3339()),
            format!("{:?}", snapshot.status),
            snapshot.total_files,
            snapshot.total_bytes,
            snapshot.deduped_bytes,
        ],
    )?;
    Ok(())
}

pub fn get_snapshot(conn: &Connection, id: &SnapshotId) -> anyhow::Result<Option<Snapshot>> {
    let mut stmt = conn.prepare(
        "SELECT id, device_id, started_at, finished_at, status, total_files, total_bytes, deduped_bytes
         FROM snapshots WHERE id = ?1"
    )?;

    let mut snapshot_iter = stmt.query_map([&id.0], map_row_to_snapshot)?;

    if let Some(s) = snapshot_iter.next() {
        Ok(Some(s?))
    } else {
        Ok(None)
    }
}

pub fn list_snapshots(conn: &Connection, device_id: &DeviceId) -> anyhow::Result<Vec<Snapshot>> {
    let mut stmt = conn.prepare(
        "SELECT id, device_id, started_at, finished_at, status, total_files, total_bytes, deduped_bytes
         FROM snapshots WHERE device_id = ?1 ORDER BY started_at DESC"
    )?;
    let snapshot_iter = stmt.query_map([&device_id.0], map_row_to_snapshot)?;

    let mut snapshots = Vec::new();
    for s in snapshot_iter {
        snapshots.push(s?);
    }
    Ok(snapshots)
}

pub fn get_latest_snapshot(conn: &Connection, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> {
    let snapshots = list_snapshots(conn, device_id)?;
    Ok(snapshots.into_iter().find(|s| s.status == domain::SnapshotStatus::Completed))
}

pub fn get_incomplete_snapshot(conn: &Connection, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> {
    let snapshots = list_snapshots(conn, device_id)?;
    Ok(snapshots
        .into_iter()
        .find(|s| s.status == domain::SnapshotStatus::Running || s.status == domain::SnapshotStatus::Interrupted))
}

pub fn link_file_to_snapshot(conn: &Connection, snapshot_id: &SnapshotId, file_id: &FileId) -> anyhow::Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO snapshot_files (snapshot_id, file_id) VALUES (?1, ?2)",
        params![snapshot_id.0, file_id.0],
    )?;
    Ok(())
}

pub fn get_snapshot_files(conn: &Connection, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<FileEntry>> {
    let mut stmt = conn.prepare(
        "SELECT f.id, f.device_id, f.path, f.name, f.size_bytes, f.modified_at, f.mime_type, f.permissions, f.hash_sha256, media_info
         FROM files f
         JOIN snapshot_files sf ON f.id = sf.file_id
         WHERE sf.snapshot_id = ?1"
    )?;

    let file_iter = stmt.query_map([&snapshot_id.0], map_row_to_file)?;

    let mut files = Vec::new();
    for f in file_iter {
        files.push(f?);
    }
    Ok(files)
}

pub fn delete_snapshot(conn: &mut Connection, snapshot_id: &SnapshotId) -> anyhow::Result<()> {
    let tx = conn.transaction()?;

    tx.execute("DELETE FROM snapshot_files WHERE snapshot_id = ?1", [&snapshot_id.0])?;
    tx.execute("DELETE FROM snapshot_apps WHERE snapshot_id = ?1", [&snapshot_id.0])?;
    tx.execute("DELETE FROM snapshot_data WHERE snapshot_id = ?1", [&snapshot_id.0])?;
    tx.execute("DELETE FROM snapshots WHERE id = ?1", [&snapshot_id.0])?;

    tx.commit()?;
    Ok(())
}

pub fn save_structured_data_ref(conn: &Connection, snapshot_id: &SnapshotId, data_type: &str, object_id: &str) -> anyhow::Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO snapshot_data (snapshot_id, data_type, object_id) VALUES (?1, ?2, ?3)",
        params![snapshot_id.0, data_type, object_id],
    )?;
    Ok(())
}
