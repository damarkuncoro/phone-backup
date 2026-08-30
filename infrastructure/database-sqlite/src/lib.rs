mod mappers;
mod schema;
mod repositories;

use std::sync::Arc;
use rusqlite::Connection;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use domain::{AppId, AppInfo, BackupSchedule, Device, DeviceId, FileEntry, Snapshot, SnapshotId, Contact, FileId, AppSettings};
use ports::{
    RepositoryPort, DeviceRepositoryPort, SnapshotRepositoryPort, FileRepositoryPort,
    AppRepositoryPort, ContactRepositoryPort, ScheduleRepositoryPort,
    SettingsRepositoryPort, MaintenanceRepositoryPort,
    SmsRepositoryPort, CallLogRepositoryPort
};

use crate::repositories::device_repo::DeviceRepository;
use crate::repositories::file_repo::FileRepository;
use crate::repositories::snapshot_repo::SnapshotRepository;
use crate::repositories::app_repo::AppRepository;
use crate::repositories::contact_repo::ContactRepository;
use crate::repositories::schedule_repo::ScheduleRepository;
use crate::repositories::maintenance_repo::MaintenanceRepository;
use crate::repositories::settings_repo::SettingsRepository;
use crate::repositories::communication_repo::CommunicationRepository;

/// Custom connection initializer to ensure PRAGMAs are set for every connection in the pool
#[derive(Debug)]
struct SqliteCustomizer {
    passphrase: Option<String>,
}

impl r2d2::CustomizeConnection<Connection, rusqlite::Error> for SqliteCustomizer {
    fn on_acquire(&self, conn: &mut Connection) -> Result<(), rusqlite::Error> {
        if let Some(ref pwd) = self.passphrase {
            let escaped = pwd.replace('\'', "''");
            let _ = conn.execute(&format!("PRAGMA key = '{}';", escaped), []);
        }
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA busy_timeout = 5000;"
        )
    }
}

/// FACTORY: Pusat pembuatan repositori
pub struct SqliteRepositoryFactory;

impl SqliteRepositoryFactory {
    pub fn create_default(path: &str) -> anyhow::Result<SqliteRepository> {
        SqliteRepository::builder()
            .with_path(path)
            .run_migrations()
            .build()
    }

    pub fn create_encrypted(path: &str, passphrase: &str) -> anyhow::Result<SqliteRepository> {
        SqliteRepository::builder()
            .with_path(path)
            .with_passphrase(passphrase)
            .run_migrations()
            .build()
    }
}

/// BUILDER: Konfigurasi fleksibel untuk SqliteRepository
pub struct SqliteRepositoryBuilder {
    path: Option<String>,
    passphrase: Option<String>,
    run_migrations: bool,
}

impl SqliteRepositoryBuilder {
    pub fn new() -> Self {
        Self {
            path: None,
            passphrase: None,
            run_migrations: false,
        }
    }

    pub fn with_path(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self
    }

    pub fn with_passphrase(mut self, passphrase: &str) -> Self {
        self.passphrase = Some(passphrase.to_string());
        self
    }

    pub fn run_migrations(mut self) -> Self {
        self.run_migrations = true;
        self
    }

    pub fn build(self) -> anyhow::Result<SqliteRepository> {
        let path = self.path.ok_or_else(|| anyhow::anyhow!("Database path is required"))?;

        let manager = SqliteConnectionManager::file(&path);
        let customizer = SqliteCustomizer {
            passphrase: self.passphrase,
        };
        let pool = Pool::builder()
            .connection_customizer(Box::new(customizer))
            .build(manager)?;

        if self.run_migrations {
            let conn = pool.get()?;
            schema::init_schema(&conn)?;
        }

        Ok(SqliteRepository {
            pool: Arc::new(pool),
        })
    }
}

/// FACADE: Implementasi utama RepositoryPort yang mengagregasi sub-repositori
/// Menggunakan Connection Pool untuk konkurensi yang lebih baik
pub struct SqliteRepository {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl SqliteRepository {
    pub fn builder() -> SqliteRepositoryBuilder {
        SqliteRepositoryBuilder::new()
    }

    pub fn new(path: &str) -> anyhow::Result<Self> {
        SqliteRepositoryFactory::create_default(path)
    }

    // Helper to get a repository instance (stateless or using the pool)
    fn devices(&self) -> DeviceRepository { DeviceRepository::new(self.pool.clone()) }
    fn snapshots(&self) -> SnapshotRepository { SnapshotRepository::new(self.pool.clone()) }
    fn files(&self) -> FileRepository { FileRepository::new(self.pool.clone()) }
    fn apps(&self) -> AppRepository { AppRepository::new(self.pool.clone()) }
    fn contacts(&self) -> ContactRepository { ContactRepository::new(self.pool.clone()) }
    fn schedules(&self) -> ScheduleRepository { ScheduleRepository::new(self.pool.clone()) }
    fn maintenance(&self) -> MaintenanceRepository { MaintenanceRepository::new(self.pool.clone()) }
    fn settings(&self) -> SettingsRepository { SettingsRepository::new(self.pool.clone()) }
    fn communication(&self) -> CommunicationRepository { CommunicationRepository::new(self.pool.clone()) }
}

impl DeviceRepositoryPort for SqliteRepository {
    fn save_device(&self, device: &Device) -> anyhow::Result<()> { self.devices().save_device(device) }
    fn list_devices(&self) -> anyhow::Result<Vec<Device>> { self.devices().list_devices() }
    fn get_device(&self, id: &DeviceId) -> anyhow::Result<Option<Device>> { self.devices().get_device(id) }
    fn get_storage_usage_by_device(&self, device_id: &DeviceId) -> anyhow::Result<u64> { self.devices().get_storage_usage_by_device(device_id) }
}

impl SnapshotRepositoryPort for SqliteRepository {
    fn get_snapshot(&self, id: &SnapshotId) -> anyhow::Result<Option<Snapshot>> { self.snapshots().get_snapshot(id) }
    fn create_snapshot(&self, snapshot: &Snapshot) -> anyhow::Result<()> { self.snapshots().create_snapshot(snapshot) }
    fn update_snapshot(&self, snapshot: &Snapshot) -> anyhow::Result<()> { self.snapshots().update_snapshot(snapshot) }
    fn list_snapshots(&self, device_id: &DeviceId) -> anyhow::Result<Vec<Snapshot>> { self.snapshots().list_snapshots(device_id) }
    fn list_all_snapshots(&self) -> anyhow::Result<Vec<Snapshot>> { self.snapshots().list_all_snapshots() }
    fn get_latest_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> { self.snapshots().get_latest_snapshot(device_id) }
    fn get_latest_completed_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> { self.snapshots().get_latest_completed_snapshot(device_id) }
    fn get_incomplete_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> { self.snapshots().get_incomplete_snapshot(device_id) }
    fn get_resumable_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> { self.snapshots().get_resumable_snapshot(device_id) }
    fn delete_snapshot(&self, snapshot_id: &SnapshotId) -> anyhow::Result<()> { self.snapshots().delete_snapshot(snapshot_id) }
    fn save_structured_data_ref(&self, snapshot_id: &SnapshotId, data_type: domain::StructuredDataType, object_id: &str) -> anyhow::Result<()> {
        self.snapshots().save_structured_data_ref(snapshot_id, data_type, object_id)
    }
    fn get_structured_data_ref(&self, snapshot_id: &SnapshotId, data_type: domain::StructuredDataType) -> anyhow::Result<Option<String>> {
        self.snapshots().get_structured_data_ref(snapshot_id, data_type)
    }
}

impl FileRepositoryPort for SqliteRepository {
    fn save_file(&self, file: &FileEntry) -> anyhow::Result<()> { self.files().save_file(file) }
    fn save_files_batch(&self, files: &[FileEntry]) -> anyhow::Result<()> { self.files().save_files_batch(files) }
    fn list_files(&self, device_id: &DeviceId) -> anyhow::Result<Vec<FileEntry>> { self.files().list_files(device_id) }
    fn get_snapshot_files(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<FileEntry>> { self.files().get_snapshot_files(snapshot_id) }
    fn link_file_to_snapshot(&self, snapshot_id: &SnapshotId, file_id: &FileId) -> anyhow::Result<()> { self.files().link_file_to_snapshot(snapshot_id, file_id) }
    fn link_files_to_snapshot_batch(&self, snapshot_id: &SnapshotId, file_ids: &[FileId]) -> anyhow::Result<()> { self.files().link_files_to_snapshot_batch(snapshot_id, file_ids) }
    fn search_files(&self, query: &str) -> anyhow::Result<Vec<FileEntry>> { self.files().search_files(query) }
    fn list_media_files(&self, device_id: &DeviceId) -> anyhow::Result<Vec<FileEntry>> { self.files().list_media_files(device_id) }
    fn get_recent_media(&self, limit: u32) -> anyhow::Result<Vec<FileEntry>> { self.files().get_recent_media(limit) }
    fn get_file_diff(&self, old_snapshot_id: &SnapshotId, new_snapshot_id: &SnapshotId) -> anyhow::Result<domain::FileDiff> {
        self.files().get_file_diff(old_snapshot_id, new_snapshot_id)
    }
    fn save_file_chunk(&self, file_id: &FileId, chunk_hash: &str, offset: u64, length: u32, sequence: u32) -> anyhow::Result<()> {
        self.files().save_file_chunk(file_id, chunk_hash, offset, length, sequence)
    }
    fn get_file_chunks(&self, file_id: &FileId) -> anyhow::Result<Vec<(String, u64, u32)>> { self.files().get_file_chunks(file_id) }
}

impl AppRepositoryPort for SqliteRepository {
    fn save_app(&self, app: &AppInfo) -> anyhow::Result<()> { self.apps().save_app(app) }
    fn link_app_to_snapshot(&self, snapshot_id: &SnapshotId, app_id: &AppId) -> anyhow::Result<()> { self.apps().link_app_to_snapshot(snapshot_id, app_id) }
    fn get_snapshot_apps(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<AppInfo>> { self.apps().get_snapshot_apps(snapshot_id) }
}

impl ContactRepositoryPort for SqliteRepository {
    fn save_contact(&self, snapshot_id: &SnapshotId, contact: &Contact) -> anyhow::Result<()> { self.contacts().save_contact(snapshot_id, contact) }
    fn get_snapshot_contacts(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<Contact>> { self.contacts().get_snapshot_contacts(snapshot_id) }
    fn search_contacts(&self, query: &str) -> anyhow::Result<Vec<(SnapshotId, Contact)>> { self.contacts().search_contacts(query) }
    fn get_contact_diff(&self, old_snapshot_id: &SnapshotId, new_snapshot_id: &SnapshotId) -> anyhow::Result<domain::ContactDiff> {
        self.contacts().get_contact_diff(old_snapshot_id, new_snapshot_id)
    }
}

impl ScheduleRepositoryPort for SqliteRepository {
    fn save_schedule(&self, schedule: &BackupSchedule) -> anyhow::Result<()> { self.schedules().save_schedule(schedule) }
    fn get_schedule(&self, device_id: &DeviceId) -> anyhow::Result<Option<BackupSchedule>> { self.schedules().get_schedule(device_id) }
    fn list_schedules(&self) -> anyhow::Result<Vec<BackupSchedule>> { self.schedules().list_schedules() }
}

impl SettingsRepositoryPort for SqliteRepository {
    fn save_settings(&self, settings: &AppSettings) -> anyhow::Result<()> { self.settings().save_settings(settings) }
    fn get_settings(&self) -> anyhow::Result<Option<AppSettings>> { self.settings().get_settings() }
}

impl MaintenanceRepositoryPort for SqliteRepository {
    fn get_all_referenced_hashes(&self) -> anyhow::Result<std::collections::HashSet<String>> { self.maintenance().get_all_referenced_hashes() }
    fn optimize(&self) -> anyhow::Result<()> { self.maintenance().optimize() }
    fn prune_orphans(&self) -> anyhow::Result<u64> { self.maintenance().prune_orphans() }
    fn create_database_backup(&self, destination_path: &str) -> anyhow::Result<()> { self.maintenance().create_database_backup(destination_path) }
}

impl SmsRepositoryPort for SqliteRepository {
    fn save_sms(&self, snapshot_id: &SnapshotId, sms: &domain::Sms) -> anyhow::Result<()> { self.communication().save_sms(snapshot_id, sms) }
    fn save_sms_batch(&self, snapshot_id: &SnapshotId, sms_list: &[domain::Sms]) -> anyhow::Result<()> { self.communication().save_sms_batch(snapshot_id, sms_list) }
    fn get_snapshot_sms(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<domain::Sms>> { self.communication().get_snapshot_sms(snapshot_id) }
    fn search_sms(&self, query: &str) -> anyhow::Result<Vec<(SnapshotId, domain::Sms)>> { self.communication().search_sms(query) }
}

impl CallLogRepositoryPort for SqliteRepository {
    fn save_call_log(&self, snapshot_id: &SnapshotId, log: &domain::CallLog) -> anyhow::Result<()> { self.communication().save_call_log(snapshot_id, log) }
    fn save_call_logs_batch(&self, snapshot_id: &SnapshotId, logs: &[domain::CallLog]) -> anyhow::Result<()> { self.communication().save_call_logs_batch(snapshot_id, logs) }
    fn get_snapshot_call_logs(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<domain::CallLog>> { self.communication().get_snapshot_call_logs(snapshot_id) }
    fn search_call_logs(&self, query: &str) -> anyhow::Result<Vec<(SnapshotId, domain::CallLog)>> { self.communication().search_call_logs(query) }
}

impl RepositoryPort for SqliteRepository {}
