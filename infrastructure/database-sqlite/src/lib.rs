mod mappers;
mod schema;
mod repositories;

use std::sync::{Arc, Mutex};
use rusqlite::Connection;
use tracing::instrument;

use domain::{AppId, AppInfo, BackupSchedule, Device, DeviceId, FileEntry, Snapshot, SnapshotId, Contact, FileId};
use ports::RepositoryPort;

use crate::repositories::device_repo::DeviceRepository;
use crate::repositories::file_repo::FileRepository;
use crate::repositories::snapshot_repo::SnapshotRepository;
use crate::repositories::app_repo::AppRepository;
use crate::repositories::contact_repo::ContactRepository;
use crate::repositories::schedule_repo::ScheduleRepository;
use crate::repositories::maintenance_repo::MaintenanceRepository;
use crate::repositories::settings_repo::SettingsRepository;

/// FACTORY: Pusat pembuatan repositori
pub struct SqliteRepositoryFactory;

impl SqliteRepositoryFactory {
    pub fn create_default(path: &str) -> anyhow::Result<SqliteRepository> {
        SqliteRepository::builder()
            .with_path(path)
            .run_migrations()
            .build()
    }
}

/// BUILDER: Konfigurasi fleksibel untuk SqliteRepository
pub struct SqliteRepositoryBuilder {
    path: Option<String>,
    run_migrations: bool,
}

impl SqliteRepositoryBuilder {
    pub fn new() -> Self {
        Self {
            path: None,
            run_migrations: false,
        }
    }

    pub fn with_path(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self
    }

    pub fn run_migrations(mut self) -> Self {
        self.run_migrations = true;
        self
    }

    pub fn build(self) -> anyhow::Result<SqliteRepository> {
        let path = self.path.ok_or_else(|| anyhow::anyhow!("Database path is required"))?;
        let conn = Connection::open(path)?;

        if self.run_migrations {
            schema::init_schema(&conn)?;
        }

        Ok(SqliteRepository {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
}

/// FACADE: Implementasi utama RepositoryPort
pub struct SqliteRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteRepository {
    pub fn builder() -> SqliteRepositoryBuilder {
        SqliteRepositoryBuilder::new()
    }

    pub fn new(path: &str) -> anyhow::Result<Self> {
        SqliteRepositoryFactory::create_default(path)
    }
}

impl RepositoryPort for SqliteRepository {
    #[instrument(skip(self))]
    fn save_device(&self, device: &Device) -> anyhow::Result<()> {
        DeviceRepository::save(&self.conn.lock().unwrap(), device)
    }

    #[instrument(skip(self))]
    fn save_file(&self, file: &FileEntry) -> anyhow::Result<()> {
        FileRepository::save(&self.conn.lock().unwrap(), file)
    }

    #[instrument(skip(self))]
    fn list_devices(&self) -> anyhow::Result<Vec<Device>> {
        DeviceRepository::list(&self.conn.lock().unwrap())
    }

    #[instrument(skip(self))]
    fn get_device(&self, id: &DeviceId) -> anyhow::Result<Option<Device>> {
        DeviceRepository::get_by_id(&self.conn.lock().unwrap(), id)
    }

    #[instrument(skip(self))]
    fn get_snapshot(&self, id: &SnapshotId) -> anyhow::Result<Option<Snapshot>> {
        SnapshotRepository::get_by_id(&self.conn.lock().unwrap(), id)
    }

    #[instrument(skip(self))]
    fn list_files(&self, device_id: &DeviceId) -> anyhow::Result<Vec<FileEntry>> {
        FileRepository::list_by_device(&self.conn.lock().unwrap(), device_id)
    }

    #[instrument(skip(self))]
    fn create_snapshot(&self, snapshot: &Snapshot) -> anyhow::Result<()> {
        SnapshotRepository::create(&self.conn.lock().unwrap(), snapshot)
    }

    #[instrument(skip(self))]
    fn update_snapshot(&self, snapshot: &Snapshot) -> anyhow::Result<()> {
        SnapshotRepository::update(&self.conn.lock().unwrap(), snapshot)
    }

    #[instrument(skip(self))]
    fn link_file_to_snapshot(&self, snapshot_id: &SnapshotId, file_id: &FileId) -> anyhow::Result<()> {
        SnapshotRepository::link_file(&self.conn.lock().unwrap(), snapshot_id, file_id)
    }

    #[instrument(skip(self))]
    fn list_snapshots(&self, device_id: &DeviceId) -> anyhow::Result<Vec<Snapshot>> {
        SnapshotRepository::list_by_device(&self.conn.lock().unwrap(), device_id)
    }

    #[instrument(skip(self))]
    fn list_all_snapshots(&self) -> anyhow::Result<Vec<Snapshot>> {
        SnapshotRepository::list_all(&self.conn.lock().unwrap())
    }

    #[instrument(skip(self))]
    fn get_latest_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> {
        let snapshots = self.list_snapshots(device_id)?;
        Ok(snapshots.into_iter().find(|s| s.status == domain::SnapshotStatus::Completed))
    }

    #[instrument(skip(self))]
    fn get_incomplete_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> {
        let snapshots = self.list_snapshots(device_id)?;
        Ok(snapshots.into_iter().find(|s| s.status == domain::SnapshotStatus::Running || s.status == domain::SnapshotStatus::Interrupted))
    }

    #[instrument(skip(self))]
    fn get_snapshot_files(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<FileEntry>> {
        SnapshotRepository::get_files(&self.conn.lock().unwrap(), snapshot_id)
    }

    #[instrument(skip(self))]
    fn save_app(&self, app: &AppInfo) -> anyhow::Result<()> {
        AppRepository::save(&self.conn.lock().unwrap(), app)
    }

    #[instrument(skip(self))]
    fn link_app_to_snapshot(&self, snapshot_id: &SnapshotId, app_id: &AppId) -> anyhow::Result<()> {
        AppRepository::link_to_snapshot(&self.conn.lock().unwrap(), snapshot_id, app_id)
    }

    #[instrument(skip(self))]
    fn get_snapshot_apps(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<AppInfo>> {
        AppRepository::get_by_snapshot(&self.conn.lock().unwrap(), snapshot_id)
    }

    #[instrument(skip(self))]
    fn save_structured_data_ref(&self, snapshot_id: &SnapshotId, data_type: &str, object_id: &str) -> anyhow::Result<()> {
        SnapshotRepository::save_structured_data_ref(&self.conn.lock().unwrap(), snapshot_id, data_type, object_id)
    }

    #[instrument(skip(self))]
    fn get_structured_data_ref(&self, snapshot_id: &SnapshotId, data_type: &str) -> anyhow::Result<Option<String>> {
        SnapshotRepository::get_structured_data_ref(&self.conn.lock().unwrap(), snapshot_id, data_type)
    }

    #[instrument(skip(self))]
    fn save_contact(&self, snapshot_id: &SnapshotId, contact: &Contact) -> anyhow::Result<()> {
        ContactRepository::save(&self.conn.lock().unwrap(), snapshot_id, contact)
    }

    #[instrument(skip(self))]
    fn get_snapshot_contacts(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<Contact>> {
        ContactRepository::list_by_snapshot(&self.conn.lock().unwrap(), snapshot_id)
    }

    #[instrument(skip(self))]
    fn search_contacts(&self, query: &str) -> anyhow::Result<Vec<(SnapshotId, Contact)>> {
        ContactRepository::search(&self.conn.lock().unwrap(), query)
    }

    #[instrument(skip(self))]
    fn save_schedule(&self, schedule: &BackupSchedule) -> anyhow::Result<()> {
        ScheduleRepository::save(&self.conn.lock().unwrap(), schedule)
    }

    #[instrument(skip(self))]
    fn get_schedule(&self, device_id: &DeviceId) -> anyhow::Result<Option<BackupSchedule>> {
        ScheduleRepository::get_by_device(&self.conn.lock().unwrap(), device_id)
    }

    #[instrument(skip(self))]
    fn list_schedules(&self) -> anyhow::Result<Vec<BackupSchedule>> {
        ScheduleRepository::list_enabled(&self.conn.lock().unwrap())
    }

    #[instrument(skip(self))]
    fn delete_snapshot(&self, snapshot_id: &SnapshotId) -> anyhow::Result<()> {
        SnapshotRepository::delete(&self.conn.lock().unwrap(), snapshot_id)
    }

    #[instrument(skip(self))]
    fn search_files(&self, query: &str) -> anyhow::Result<Vec<FileEntry>> {
        FileRepository::search(&self.conn.lock().unwrap(), query)
    }

    #[instrument(skip(self))]
    fn save_file_chunk(&self, file_id: &FileId, chunk_hash: &str, offset: u64, length: u32, sequence: u32) -> anyhow::Result<()> {
        FileRepository::save_chunk(&self.conn.lock().unwrap(), file_id, chunk_hash, offset, length, sequence)
    }

    #[instrument(skip(self))]
    fn get_file_chunks(&self, file_id: &FileId) -> anyhow::Result<Vec<(String, u64, u32)>> {
        FileRepository::get_chunks(&self.conn.lock().unwrap(), file_id)
    }

    #[instrument(skip(self))]
    fn get_all_referenced_hashes(&self) -> anyhow::Result<std::collections::HashSet<String>> {
        MaintenanceRepository::get_all_referenced_hashes(&self.conn.lock().unwrap())
    }

    #[instrument(skip(self))]
    fn save_settings(&self, settings: &domain::AppSettings) -> anyhow::Result<()> {
        SettingsRepository::save(&self.conn.lock().unwrap(), settings)
    }

    #[instrument(skip(self))]
    fn get_settings(&self) -> anyhow::Result<Option<domain::AppSettings>> {
        SettingsRepository::get(&self.conn.lock().unwrap())
    }
}
