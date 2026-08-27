pub mod app;
pub mod device;
pub mod file;
pub mod mappers;
pub mod schema;
pub mod schedule;
pub mod snapshot;

use domain::{AppId, AppInfo, BackupSchedule, Device, DeviceId, FileEntry, FileId, Snapshot, SnapshotId};
use ports::RepositoryPort;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

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
        schema::init_db(&conn)
    }
}

impl RepositoryPort for SqliteRepository {
    fn save_device(&self, dev: &Device) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        device::save_device(&conn, dev)
    }

    fn save_file(&self, f: &FileEntry) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        file::save_file(&conn, f)
    }

    fn list_devices(&self) -> anyhow::Result<Vec<Device>> {
        let conn = self.conn.lock().unwrap();
        device::list_devices(&conn)
    }

    fn get_device(&self, id: &DeviceId) -> anyhow::Result<Option<Device>> {
        let conn = self.conn.lock().unwrap();
        device::get_device(&conn, id)
    }

    fn get_snapshot(&self, id: &SnapshotId) -> anyhow::Result<Option<Snapshot>> {
        let conn = self.conn.lock().unwrap();
        snapshot::get_snapshot(&conn, id)
    }

    fn list_files(&self, device_id: &DeviceId) -> anyhow::Result<Vec<FileEntry>> {
        let conn = self.conn.lock().unwrap();
        file::list_files(&conn, device_id)
    }

    fn create_snapshot(&self, snap: &Snapshot) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        snapshot::create_snapshot(&conn, snap)
    }

    fn update_snapshot(&self, snap: &Snapshot) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        snapshot::update_snapshot(&conn, snap)
    }

    fn link_file_to_snapshot(&self, snapshot_id: &SnapshotId, file_id: &FileId) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        snapshot::link_file_to_snapshot(&conn, snapshot_id, file_id)
    }

    fn list_snapshots(&self, device_id: &DeviceId) -> anyhow::Result<Vec<Snapshot>> {
        let conn = self.conn.lock().unwrap();
        snapshot::list_snapshots(&conn, device_id)
    }

    fn get_latest_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> {
        let conn = self.conn.lock().unwrap();
        snapshot::get_latest_snapshot(&conn, device_id)
    }

    fn get_incomplete_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> {
        let conn = self.conn.lock().unwrap();
        snapshot::get_incomplete_snapshot(&conn, device_id)
    }

    fn get_snapshot_files(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<FileEntry>> {
        let conn = self.conn.lock().unwrap();
        snapshot::get_snapshot_files(&conn, snapshot_id)
    }

    fn save_app(&self, app: &AppInfo) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        app::save_app(&conn, app)
    }

    fn link_app_to_snapshot(&self, snapshot_id: &SnapshotId, app_id: &AppId) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        app::link_app_to_snapshot(&conn, snapshot_id, app_id)
    }

    fn get_snapshot_apps(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<AppInfo>> {
        let conn = self.conn.lock().unwrap();
        app::get_snapshot_apps(&conn, snapshot_id)
    }

    fn save_structured_data_ref(&self, snapshot_id: &SnapshotId, data_type: &str, object_id: &str) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        snapshot::save_structured_data_ref(&conn, snapshot_id, data_type, object_id)
    }

    fn save_schedule(&self, sched: &BackupSchedule) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        schedule::save_schedule(&conn, sched)
    }

    fn get_schedule(&self, device_id: &DeviceId) -> anyhow::Result<Option<BackupSchedule>> {
        let conn = self.conn.lock().unwrap();
        schedule::get_schedule(&conn, device_id)
    }

    fn list_schedules(&self) -> anyhow::Result<Vec<BackupSchedule>> {
        let conn = self.conn.lock().unwrap();
        schedule::list_schedules(&conn)
    }

    fn delete_snapshot(&self, snapshot_id: &SnapshotId) -> anyhow::Result<()> {
        let mut conn = self.conn.lock().unwrap();
        snapshot::delete_snapshot(&mut conn, snapshot_id)
    }

    fn search_files(&self, query: &str) -> anyhow::Result<Vec<FileEntry>> {
        let conn = self.conn.lock().unwrap();
        file::search_files(&conn, query)
    }
}
