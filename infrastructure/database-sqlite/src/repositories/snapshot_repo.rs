use rusqlite::{params, Connection};
use domain::{DeviceId, Snapshot, SnapshotId, FileId};
use crate::mappers::BackupMapper;

pub struct SnapshotRepository;

impl SnapshotRepository {
    pub fn create(conn: &Connection, snapshot: &Snapshot) -> anyhow::Result<()> {
        conn.execute(
            "INSERT INTO snapshots (id, device_id, started_at, finished_at, status, total_files, total_bytes, deduped_bytes)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                snapshot.id.0, snapshot.device_id.0, snapshot.started_at.to_rfc3339(),
                snapshot.finished_at.map(|t| t.to_rfc3339()), format!("{:?}", snapshot.status),
                snapshot.total_files, snapshot.total_bytes, snapshot.deduped_bytes
            ],
        )?;
        Ok(())
    }

    pub fn update(conn: &Connection, snapshot: &Snapshot) -> anyhow::Result<()> {
        conn.execute(
            "UPDATE snapshots SET finished_at = ?2, status = ?3, total_files = ?4, total_bytes = ?5, deduped_bytes = ?6
            WHERE id = ?1",
            params![
                snapshot.id.0, snapshot.finished_at.map(|t| t.to_rfc3339()),
                format!("{:?}", snapshot.status), snapshot.total_files,
                snapshot.total_bytes, snapshot.deduped_bytes
            ],
        )?;
        Ok(())
    }

    pub fn get_by_id(conn: &Connection, id: &SnapshotId) -> anyhow::Result<Option<Snapshot>> {
        let mut stmt = conn.prepare("SELECT * FROM snapshots WHERE id = ?1")?;
        let mut snapshot_iter = stmt.query_map([&id.0], BackupMapper::to_snapshot)?;
        if let Some(s) = snapshot_iter.next() { Ok(Some(s?)) } else { Ok(None) }
    }

    pub fn list_by_device(conn: &Connection, device_id: &DeviceId) -> anyhow::Result<Vec<Snapshot>> {
        let mut stmt = conn.prepare("SELECT * FROM snapshots WHERE device_id = ?1 ORDER BY started_at DESC")?;
        let snapshot_iter = stmt.query_map([&device_id.0], BackupMapper::to_snapshot)?;
        let mut snapshots = Vec::new();
        for s in snapshot_iter { snapshots.push(s?); }
        Ok(snapshots)
    }

    pub fn list_all(conn: &Connection) -> anyhow::Result<Vec<Snapshot>> {
        let mut stmt = conn.prepare("SELECT * FROM snapshots ORDER BY started_at DESC")?;
        let snapshot_iter = stmt.query_map([], BackupMapper::to_snapshot)?;
        let mut snapshots = Vec::new();
        for s in snapshot_iter { snapshots.push(s?); }
        Ok(snapshots)
    }

    pub fn link_file(conn: &Connection, snapshot_id: &SnapshotId, file_id: &FileId) -> anyhow::Result<()> {
        conn.execute(
            "INSERT OR IGNORE INTO snapshot_files (snapshot_id, file_id) VALUES (?1, ?2)",
            params![snapshot_id.0, file_id.0],
        )?;
        Ok(())
    }

    pub fn get_files(conn: &Connection, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<domain::FileEntry>> {
        let mut stmt = conn.prepare(
            "SELECT f.* FROM files f JOIN snapshot_files sf ON f.id = sf.file_id WHERE sf.snapshot_id = ?1"
        )?;
        let file_iter = stmt.query_map([&snapshot_id.0], BackupMapper::to_file)?;
        let mut files = Vec::new();
        for f in file_iter { files.push(f?); }
        Ok(files)
    }

    pub fn save_structured_data_ref(conn: &Connection, snapshot_id: &SnapshotId, data_type: &str, object_id: &str) -> anyhow::Result<()> {
        conn.execute(
            "INSERT OR REPLACE INTO snapshot_data (snapshot_id, data_type, object_id) VALUES (?1, ?2, ?3)",
            params![snapshot_id.0, data_type, object_id],
        )?;
        Ok(())
    }

    pub fn get_structured_data_ref(conn: &Connection, snapshot_id: &SnapshotId, data_type: &str) -> anyhow::Result<Option<String>> {
        let mut stmt = conn.prepare(
            "SELECT object_id FROM snapshot_data WHERE snapshot_id = ?1 AND data_type = ?2"
        )?;
        let mut rows = stmt.query(params![snapshot_id.0, data_type])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn delete(conn: &Connection, snapshot_id: &SnapshotId) -> anyhow::Result<()> {
        conn.execute("DELETE FROM snapshot_files WHERE snapshot_id = ?1", [&snapshot_id.0])?;
        conn.execute("DELETE FROM snapshot_apps WHERE snapshot_id = ?1", [&snapshot_id.0])?;
        conn.execute("DELETE FROM snapshot_data WHERE snapshot_id = ?1", [&snapshot_id.0])?;
        conn.execute("DELETE FROM snapshots WHERE id = ?1", [&snapshot_id.0])?;
        Ok(())
    }
}
