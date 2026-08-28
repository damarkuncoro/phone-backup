mod mappers;
mod schema;

use domain::{Device, DeviceId, FileEntry, Snapshot, SnapshotId, AppInfo, AppId, BackupSchedule};
use ports::RepositoryPort;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};
use crate::mappers::RowMapper;

pub struct SqliteRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteRepository {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let conn = Connection::open(path)?;
        let repo = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        repo.init_db()?;
        Ok(repo)
    }

    fn init_db(&self) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        schema::init_schema(&conn)?;
        Ok(())
    }
}

impl RepositoryPort for SqliteRepository {
    fn save_device(&self, device: &Device) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO devices
            (id, manufacturer, model, serial, os_version, storage_total_bytes, storage_used_bytes, connection_type)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                device.id.0, device.manufacturer, device.model, device.serial,
                device.os_version, device.storage_total_bytes, device.storage_used_bytes,
                format!("{:?}", device.connection_type)
            ],
        )?;
        Ok(())
    }

    fn save_file(&self, file: &FileEntry) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
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

    fn list_devices(&self) -> anyhow::Result<Vec<Device>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, manufacturer, model, serial, os_version, storage_total_bytes, storage_used_bytes, connection_type FROM devices")?;
        let device_iter = stmt.query_map([], RowMapper::to_device)?;
        let mut devices = Vec::new();
        for d in device_iter { devices.push(d?); }
        Ok(devices)
    }

    fn get_device(&self, _id: &DeviceId) -> anyhow::Result<Option<Device>> {
        Ok(None)
    }

    fn list_files(&self, device_id: &DeviceId) -> anyhow::Result<Vec<FileEntry>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM files WHERE device_id = ?1")?;
        let file_iter = stmt.query_map([&device_id.0], RowMapper::to_file)?;
        let mut files = Vec::new();
        for f in file_iter { files.push(f?); }
        Ok(files)
    }

    fn create_snapshot(&self, snapshot: &Snapshot) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
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
        let conn = self.conn.lock().unwrap();
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
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM snapshots WHERE id = ?1")?;
        let mut snapshot_iter = stmt.query_map([&id.0], RowMapper::to_snapshot)?;
        if let Some(s) = snapshot_iter.next() { Ok(Some(s?)) } else { Ok(None) }
    }

    fn link_file_to_snapshot(&self, snapshot_id: &SnapshotId, file_id: &domain::FileId) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO snapshot_files (snapshot_id, file_id) VALUES (?1, ?2)",
            params![snapshot_id.0, file_id.0],
        )?;
        Ok(())
    }

    fn list_snapshots(&self, device_id: &DeviceId) -> anyhow::Result<Vec<Snapshot>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM snapshots WHERE device_id = ?1 ORDER BY started_at DESC")?;
        let snapshot_iter = stmt.query_map([&device_id.0], RowMapper::to_snapshot)?;
        let mut snapshots = Vec::new();
        for s in snapshot_iter { snapshots.push(s?); }
        Ok(snapshots)
    }

    fn get_latest_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> {
        let snapshots = self.list_snapshots(device_id)?;
        Ok(snapshots.into_iter().find(|s| s.status == domain::SnapshotStatus::Completed))
    }

    fn get_incomplete_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> {
        let snapshots = self.list_snapshots(device_id)?;
        Ok(snapshots.into_iter().find(|s| s.status == domain::SnapshotStatus::Running || s.status == domain::SnapshotStatus::Interrupted))
    }

    fn get_snapshot_files(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<FileEntry>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT f.* FROM files f JOIN snapshot_files sf ON f.id = sf.file_id WHERE sf.snapshot_id = ?1"
        )?;
        let file_iter = stmt.query_map([&snapshot_id.0], RowMapper::to_file)?;
        let mut files = Vec::new();
        for f in file_iter { files.push(f?); }
        Ok(files)
    }

    fn save_app(&self, app: &AppInfo) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO apps (id, device_id, package_name, version_name, version_code, installer, app_name)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![app.id.0, app.device_id.0, app.package_name, app.version_name, app.version_code, app.installer, app.app_name],
        )?;
        Ok(())
    }

    fn link_app_to_snapshot(&self, snapshot_id: &SnapshotId, app_id: &AppId) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO snapshot_apps (snapshot_id, app_id) VALUES (?1, ?2)",
            params![snapshot_id.0, app_id.0],
        )?;
        Ok(())
    }

    fn get_snapshot_apps(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<AppInfo>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT a.* FROM apps a JOIN snapshot_apps sa ON a.id = sa.app_id WHERE sa.snapshot_id = ?1"
        )?;
        let app_iter = stmt.query_map([&snapshot_id.0], RowMapper::to_app)?;
        let mut apps = Vec::new();
        for a in app_iter { apps.push(a?); }
        Ok(apps)
    }

    fn save_structured_data_ref(&self, snapshot_id: &SnapshotId, data_type: &str, object_id: &str) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO snapshot_data (snapshot_id, data_type, object_id) VALUES (?1, ?2, ?3)",
            params![snapshot_id.0, data_type, object_id],
        )?;
        Ok(())
    }

    fn save_schedule(&self, schedule: &BackupSchedule) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO schedules (device_id, frequency, last_run_at, enabled) VALUES (?1, ?2, ?3, ?4)",
            params![schedule.device_id.0, format!("{:?}", schedule.frequency), schedule.last_run_at.map(|t| t.to_rfc3339()), if schedule.enabled { 1 } else { 0 }],
        )?;
        Ok(())
    }

    fn get_schedule(&self, device_id: &DeviceId) -> anyhow::Result<Option<BackupSchedule>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM schedules WHERE device_id = ?1")?;
        let mut schedule_iter = stmt.query_map([&device_id.0], RowMapper::to_schedule)?;
        if let Some(s) = schedule_iter.next() { Ok(Some(s?)) } else { Ok(None) }
    }

    fn list_schedules(&self) -> anyhow::Result<Vec<BackupSchedule>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM schedules WHERE enabled = 1")?;
        let schedule_iter = stmt.query_map([], RowMapper::to_schedule)?;
        let mut schedules = Vec::new();
        for s in schedule_iter { schedules.push(s?); }
        Ok(schedules)
    }

    fn delete_snapshot(&self, snapshot_id: &SnapshotId) -> anyhow::Result<()> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;
        tx.execute("DELETE FROM snapshot_files WHERE snapshot_id = ?1", [&snapshot_id.0])?;
        tx.execute("DELETE FROM snapshot_apps WHERE snapshot_id = ?1", [&snapshot_id.0])?;
        tx.execute("DELETE FROM snapshot_data WHERE snapshot_id = ?1", [&snapshot_id.0])?;
        tx.execute("DELETE FROM snapshots WHERE id = ?1", [&snapshot_id.0])?;
        tx.commit()?;
        Ok(())
    }

    fn search_files(&self, query: &str) -> anyhow::Result<Vec<FileEntry>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM files WHERE name LIKE ?1 OR path LIKE ?1")?;
        let pattern = format!("%{}%", query);
        let file_iter = stmt.query_map([pattern], RowMapper::to_file)?;
        let mut files = Vec::new();
        for f in file_iter { files.push(f?); }
        Ok(files)
    }
}
