use domain::{ConnectionType, Device, DeviceId, FileEntry, Snapshot, SnapshotId, SnapshotStatus, BackupSchedule, ScheduleFrequency};
use ports::RepositoryPort;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};

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
        conn.execute(
            "CREATE TABLE IF NOT EXISTS devices (
                id TEXT PRIMARY KEY,
                manufacturer TEXT NOT NULL,
                model TEXT NOT NULL,
                serial TEXT NOT NULL,
                os_version TEXT NOT NULL,
                storage_total_bytes INTEGER NOT NULL,
                storage_used_bytes INTEGER NOT NULL,
                connection_type TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS files (
                id TEXT PRIMARY KEY,
                device_id TEXT NOT NULL,
                path TEXT NOT NULL,
                name TEXT NOT NULL,
                size_bytes INTEGER NOT NULL,
                modified_at TEXT NOT NULL,
                mime_type TEXT NOT NULL,
                permissions TEXT NOT NULL,
                hash_sha256 TEXT,
                media_info TEXT,
                FOREIGN KEY(device_id) REFERENCES devices(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS snapshots (
                id TEXT PRIMARY KEY,
                device_id TEXT NOT NULL,
                started_at TEXT NOT NULL,
                finished_at TEXT,
                status TEXT NOT NULL,
                total_files INTEGER NOT NULL,
                total_bytes INTEGER NOT NULL,
                deduped_bytes INTEGER DEFAULT 0,
                FOREIGN KEY(device_id) REFERENCES devices(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS snapshot_files (
                snapshot_id TEXT NOT NULL,
                file_id TEXT NOT NULL,
                PRIMARY KEY(snapshot_id, file_id),
                FOREIGN KEY(snapshot_id) REFERENCES snapshots(id),
                FOREIGN KEY(file_id) REFERENCES files(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS apps (
                id TEXT PRIMARY KEY,
                device_id TEXT NOT NULL,
                package_name TEXT NOT NULL,
                version_name TEXT NOT NULL,
                version_code INTEGER NOT NULL,
                installer TEXT,
                app_name TEXT NOT NULL,
                FOREIGN KEY(device_id) REFERENCES devices(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS snapshot_apps (
                snapshot_id TEXT NOT NULL,
                app_id TEXT NOT NULL,
                PRIMARY KEY(snapshot_id, app_id),
                FOREIGN KEY(snapshot_id) REFERENCES snapshots(id),
                FOREIGN KEY(app_id) REFERENCES apps(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS snapshot_data (
                snapshot_id TEXT NOT NULL,
                data_type TEXT NOT NULL,
                object_id TEXT NOT NULL,
                PRIMARY KEY(snapshot_id, data_type),
                FOREIGN KEY(snapshot_id) REFERENCES snapshots(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS schedules (
                device_id TEXT PRIMARY KEY,
                frequency TEXT NOT NULL,
                last_run_at TEXT,
                enabled INTEGER NOT NULL,
                FOREIGN KEY(device_id) REFERENCES devices(id)
            )",
            [],
        )?;
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
                device.id.0,
                device.manufacturer,
                device.model,
                device.serial,
                device.os_version,
                device.storage_total_bytes,
                device.storage_used_bytes,
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

    fn list_devices(&self) -> anyhow::Result<Vec<Device>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, manufacturer, model, serial, os_version, storage_total_bytes, storage_used_bytes, connection_type FROM devices")?;
        let device_iter = stmt.query_map([], |row| {
            Ok(Device {
                id: DeviceId(row.get(0)?),
                manufacturer: row.get(1)?,
                model: row.get(2)?,
                serial: row.get(3)?,
                os_version: row.get(4)?,
                sdk_version: None, // Not stored for simplicity now
                storage_total_bytes: row.get(5)?,
                storage_used_bytes: row.get(6)?,
                storage_free_bytes: row.get::<_, u64>(5)? - row.get::<_, u64>(6)?,
                connection_type: ConnectionType::Usb, // Simplified
            })
        })?;

        let mut devices = Vec::new();
        for d in device_iter {
            devices.push(d?);
        }
        Ok(devices)
    }

    fn get_device(&self, _id: &DeviceId) -> anyhow::Result<Option<Device>> {
        // Implementation omitted for brevity in this step
        Ok(None)
    }

    fn get_snapshot(&self, id: &SnapshotId) -> anyhow::Result<Option<Snapshot>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, device_id, started_at, finished_at, status, total_files, total_bytes, deduped_bytes
             FROM snapshots WHERE id = ?1"
        )?;

        let mut snapshot_iter = stmt.query_map([&id.0], |row| {
            let started_at_str: String = row.get(2)?;
            let finished_at_str: Option<String> = row.get(3)?;
            let status_str: String = row.get(4)?;

            Ok(Snapshot {
                id: SnapshotId(row.get(0)?),
                device_id: DeviceId(row.get(1)?),
                started_at: DateTime::parse_from_rfc3339(&started_at_str).unwrap().with_timezone(&Utc),
                finished_at: finished_at_str.map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                status: match status_str.as_str() {
                    "Pending" => SnapshotStatus::Pending,
                    "Running" => SnapshotStatus::Running,
                    "Completed" => SnapshotStatus::Completed,
                    _ => SnapshotStatus::Failed,
                },
                total_files: row.get(5)?,
                total_bytes: row.get(6)?,
                deduped_bytes: row.get(7)?,
            })
        })?;

        if let Some(s) = snapshot_iter.next() {
            Ok(Some(s?))
        } else {
            Ok(None)
        }
    }

    fn list_files(&self, device_id: &DeviceId) -> anyhow::Result<Vec<FileEntry>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, device_id, path, name, size_bytes, modified_at, mime_type, permissions, hash_sha256, media_info
             FROM files WHERE device_id = ?1"
        )?;

        let file_iter = stmt.query_map([&device_id.0], |row| {
            let modified_at_str: String = row.get(5)?;
            let media_info_str: Option<String> = row.get(9)?;
            let media_info = media_info_str.map(|s| serde_json::from_str(&s).unwrap());

            Ok(FileEntry {
                id: domain::FileId(row.get(0)?),
                device_id: DeviceId(row.get(1)?),
                path: row.get(2)?,
                name: row.get(3)?,
                size_bytes: row.get(4)?,
                modified_at: DateTime::parse_from_rfc3339(&modified_at_str).unwrap().with_timezone(&Utc),
                mime_type: row.get(6)?,
                permissions: row.get(7)?,
                hash_sha256: row.get(8)?,
                media_info,
            })
        })?;

        let mut files = Vec::new();
        for f in file_iter {
            files.push(f?);
        }
        Ok(files)
    }

    fn create_snapshot(&self, snapshot: &Snapshot) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
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

    fn update_snapshot(&self, snapshot: &Snapshot) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
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
        let mut stmt = conn.prepare(
            "SELECT id, device_id, started_at, finished_at, status, total_files, total_bytes, deduped_bytes
             FROM snapshots WHERE device_id = ?1 ORDER BY started_at DESC"
        )?;
        let snapshot_iter = stmt.query_map([&device_id.0], |row| {
            let started_at_str: String = row.get(2)?;
            let finished_at_str: Option<String> = row.get(3)?;
            let status_str: String = row.get(4)?;

            Ok(Snapshot {
                id: SnapshotId(row.get(0)?),
                device_id: DeviceId(row.get(1)?),
                started_at: DateTime::parse_from_rfc3339(&started_at_str).unwrap().with_timezone(&Utc),
                finished_at: finished_at_str.map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                status: match status_str.as_str() {
                    "Pending" => SnapshotStatus::Pending,
                    "Running" => SnapshotStatus::Running,
                    "Completed" => SnapshotStatus::Completed,
                    _ => SnapshotStatus::Failed,
                },
                total_files: row.get(5)?,
                total_bytes: row.get(6)?,
                deduped_bytes: row.get(7)?,
            })
        })?;

        let mut snapshots = Vec::new();
        for s in snapshot_iter {
            snapshots.push(s?);
        }
        Ok(snapshots)
    }

    fn get_latest_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> {
        let snapshots = self.list_snapshots(device_id)?;
        Ok(snapshots.into_iter().find(|s| s.status == SnapshotStatus::Completed))
    }

    fn get_snapshot_files(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<FileEntry>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT f.id, f.device_id, f.path, f.name, f.size_bytes, f.modified_at, f.mime_type, f.permissions, f.hash_sha256, media_info
             FROM files f
             JOIN snapshot_files sf ON f.id = sf.file_id
             WHERE sf.snapshot_id = ?1"
        )?;

        let file_iter = stmt.query_map([&snapshot_id.0], |row| {
            let modified_at_str: String = row.get(5)?;
            let media_info_str: Option<String> = row.get(9)?;
            let media_info = media_info_str.map(|s| serde_json::from_str(&s).unwrap());

            Ok(FileEntry {
                id: domain::FileId(row.get(0)?),
                device_id: DeviceId(row.get(1)?),
                path: row.get(2)?,
                name: row.get(3)?,
                size_bytes: row.get(4)?,
                modified_at: DateTime::parse_from_rfc3339(&modified_at_str).unwrap().with_timezone(&Utc),
                mime_type: row.get(6)?,
                permissions: row.get(7)?,
                hash_sha256: row.get(8)?,
                media_info,
            })
        })?;

        let mut files = Vec::new();
        for f in file_iter {
            files.push(f?);
        }
        Ok(files)
    }

    fn save_app(&self, app: &domain::AppInfo) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO apps
            (id, device_id, package_name, version_name, version_code, installer, app_name)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                app.id.0,
                app.device_id.0,
                app.package_name,
                app.version_name,
                app.version_code,
                app.installer,
                app.app_name
            ],
        )?;
        Ok(())
    }

    fn link_app_to_snapshot(&self, snapshot_id: &SnapshotId, app_id: &domain::AppId) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO snapshot_apps (snapshot_id, app_id) VALUES (?1, ?2)",
            params![snapshot_id.0, app_id.0],
        )?;
        Ok(())
    }

    fn get_snapshot_apps(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<domain::AppInfo>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT a.id, a.device_id, a.package_name, a.version_name, a.version_code, a.installer, a.app_name
             FROM apps a
             JOIN snapshot_apps sa ON a.id = sa.app_id
             WHERE sa.snapshot_id = ?1"
        )?;

        let app_iter = stmt.query_map([&snapshot_id.0], |row| {
            Ok(domain::AppInfo {
                id: domain::AppId(row.get(0)?),
                device_id: DeviceId(row.get(1)?),
                package_name: row.get(2)?,
                version_name: row.get(3)?,
                version_code: row.get(4)?,
                installer: row.get(5)?,
                app_name: row.get(6)?,
            })
        })?;

        let mut apps = Vec::new();
        for a in app_iter {
            apps.push(a?);
        }
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
            "INSERT OR REPLACE INTO schedules (device_id, frequency, last_run_at, enabled)
            VALUES (?1, ?2, ?3, ?4)",
            params![
                schedule.device_id.0,
                format!("{:?}", schedule.frequency),
                schedule.last_run_at.map(|t| t.to_rfc3339()),
                if schedule.enabled { 1 } else { 0 }
            ],
        )?;
        Ok(())
    }

    fn get_schedule(&self, device_id: &DeviceId) -> anyhow::Result<Option<BackupSchedule>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT device_id, frequency, last_run_at, enabled FROM schedules WHERE device_id = ?1")?;
        let mut schedule_iter = stmt.query_map([&device_id.0], |row| {
            let frequency_str: String = row.get(1)?;
            let last_run_at_str: Option<String> = row.get(2)?;
            let enabled: i32 = row.get(3)?;

            Ok(BackupSchedule {
                device_id: DeviceId(row.get(0)?),
                frequency: match frequency_str.as_str() {
                    "Hourly" => ScheduleFrequency::Hourly,
                    "Weekly" => ScheduleFrequency::Weekly,
                    _ => ScheduleFrequency::Daily,
                },
                last_run_at: last_run_at_str.map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                enabled: enabled == 1,
            })
        })?;

        if let Some(s) = schedule_iter.next() {
            Ok(Some(s?))
        } else {
            Ok(None)
        }
    }

    fn list_schedules(&self) -> anyhow::Result<Vec<BackupSchedule>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT device_id, frequency, last_run_at, enabled FROM schedules WHERE enabled = 1")?;
        let schedule_iter = stmt.query_map([], |row| {
            let frequency_str: String = row.get(1)?;
            let last_run_at_str: Option<String> = row.get(2)?;
            let enabled: i32 = row.get(3)?;

            Ok(BackupSchedule {
                device_id: DeviceId(row.get(0)?),
                frequency: match frequency_str.as_str() {
                    "Hourly" => ScheduleFrequency::Hourly,
                    "Weekly" => ScheduleFrequency::Weekly,
                    _ => ScheduleFrequency::Daily,
                },
                last_run_at: last_run_at_str.map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                enabled: enabled == 1,
            })
        })?;

        let mut schedules = Vec::new();
        for s in schedule_iter {
            schedules.push(s?);
        }
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
        let mut stmt = conn.prepare(
            "SELECT id, device_id, path, name, size_bytes, modified_at, mime_type, permissions, hash_sha256, media_info
             FROM files WHERE name LIKE ?1 OR path LIKE ?1"
        )?;

        let pattern = format!("%{}%", query);
        let file_iter = stmt.query_map([pattern], |row| {
            let modified_at_str: String = row.get(5)?;
            let media_info_str: Option<String> = row.get(9)?;
            let media_info = media_info_str.map(|s| serde_json::from_str(&s).unwrap());

            Ok(FileEntry {
                id: domain::FileId(row.get(0)?),
                device_id: DeviceId(row.get(1)?),
                path: row.get(2)?,
                name: row.get(3)?,
                size_bytes: row.get(4)?,
                modified_at: DateTime::parse_from_rfc3339(&modified_at_str).unwrap().with_timezone(&Utc),
                mime_type: row.get(6)?,
                permissions: row.get(7)?,
                hash_sha256: row.get(8)?,
                media_info,
            })
        })?;

        let mut files = Vec::new();
        for f in file_iter {
            files.push(f?);
        }
        Ok(files)
    }
}
