use rusqlite::params;
use std::sync::Arc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use domain::{DeviceId, Snapshot, SnapshotId};
use ports::SnapshotRepositoryPort;
use crate::mappers::BackupMapper;

pub struct SnapshotRepository {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl SnapshotRepository {
    pub fn new(pool: Arc<Pool<SqliteConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl SnapshotRepositoryPort for SnapshotRepository {
    fn create_snapshot(&self, snapshot: &Snapshot) -> anyhow::Result<()> {
        let conn = self.pool.get()?;
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

    fn update_snapshot(&self, snapshot: &Snapshot) -> anyhow::Result<()> {
        let conn = self.pool.get()?;
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

    fn get_snapshot(&self, id: &SnapshotId) -> anyhow::Result<Option<Snapshot>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT * FROM snapshots WHERE id = ?1")?;
        let mut snapshot_iter = stmt.query_map([&id.0], BackupMapper::to_snapshot)?;
        if let Some(s) = snapshot_iter.next() { Ok(Some(s?)) } else { Ok(None) }
    }

    fn list_snapshots(&self, device_id: &DeviceId) -> anyhow::Result<Vec<Snapshot>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT * FROM snapshots WHERE device_id = ?1 ORDER BY started_at DESC")?;
        let snapshot_iter = stmt.query_map([&device_id.0], BackupMapper::to_snapshot)?;
        let mut snapshots = Vec::new();
        for s in snapshot_iter { snapshots.push(s?); }
        Ok(snapshots)
    }

    fn list_all_snapshots(&self) -> anyhow::Result<Vec<Snapshot>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT * FROM snapshots ORDER BY started_at DESC")?;
        let snapshot_iter = stmt.query_map([], BackupMapper::to_snapshot)?;
        let mut snapshots = Vec::new();
        for s in snapshot_iter { snapshots.push(s?); }
        Ok(snapshots)
    }

    fn get_latest_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT * FROM snapshots WHERE device_id = ?1 AND status = 'Completed' ORDER BY started_at DESC LIMIT 1"
        )?;
        let mut snapshot_iter = stmt.query_map([&device_id.0], BackupMapper::to_snapshot)?;
        if let Some(s) = snapshot_iter.next() { Ok(Some(s?)) } else { Ok(None) }
    }

    fn get_latest_completed_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> {
        self.get_latest_snapshot(device_id)
    }

    fn get_incomplete_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT * FROM snapshots WHERE device_id = ?1 AND status IN ('Running', 'Interrupted') ORDER BY started_at DESC LIMIT 1"
        )?;
        let mut snapshot_iter = stmt.query_map([&device_id.0], BackupMapper::to_snapshot)?;
        if let Some(s) = snapshot_iter.next() { Ok(Some(s?)) } else { Ok(None) }
    }

    fn get_resumable_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> {
        self.get_incomplete_snapshot(device_id)
    }

    fn save_structured_data_ref(&self, snapshot_id: &SnapshotId, data_type: domain::StructuredDataType, object_id: &str) -> anyhow::Result<()> {
        let conn = self.pool.get()?;
        conn.execute(
            "INSERT OR REPLACE INTO snapshot_data (snapshot_id, data_type, object_id) VALUES (?1, ?2, ?3)",
            params![snapshot_id.0, data_type.as_str(), object_id],
        )?;
        Ok(())
    }

    fn get_structured_data_ref(&self, snapshot_id: &SnapshotId, data_type: domain::StructuredDataType) -> anyhow::Result<Option<String>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT object_id FROM snapshot_data WHERE snapshot_id = ?1 AND data_type = ?2"
        )?;
        let mut rows = stmt.query(params![snapshot_id.0, data_type.as_str()])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    fn delete_snapshot(&self, snapshot_id: &SnapshotId) -> anyhow::Result<()> {
        let conn = self.pool.get()?;
        conn.execute("DELETE FROM snapshot_files WHERE snapshot_id = ?1", [&snapshot_id.0])?;
        conn.execute("DELETE FROM snapshot_apps WHERE snapshot_id = ?1", [&snapshot_id.0])?;
        conn.execute("DELETE FROM snapshot_data WHERE snapshot_id = ?1", [&snapshot_id.0])?;
        conn.execute("DELETE FROM snapshots WHERE id = ?1", [&snapshot_id.0])?;
        Ok(())
    }
}
