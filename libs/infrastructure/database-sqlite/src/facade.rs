use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::sync::Arc;

use super::connection::{SqliteRepositoryBuilder, SqliteRepositoryFactory};
use crate::repositories::app_repo::AppRepository;
use crate::repositories::communication_repo::CommunicationRepository;
use crate::repositories::contact_repo::ContactRepository;
use crate::repositories::device_repo::DeviceRepository;
use crate::repositories::file_repo::FileRepository;
use crate::repositories::maintenance_repo::MaintenanceRepository;
use crate::repositories::schedule_repo::ScheduleRepository;
use crate::repositories::settings_repo::SettingsRepository;
use crate::repositories::snapshot_repo::SnapshotRepository;

/// FACADE: Implementasi utama RepositoryPort yang mengagregasi sub-repositori
pub struct SqliteRepository {
    pub(crate) pool: Arc<Pool<SqliteConnectionManager>>,
}

impl SqliteRepository {
    pub fn builder() -> SqliteRepositoryBuilder {
        SqliteRepositoryBuilder::new()
    }

    pub fn new(path: &str) -> anyhow::Result<Self> {
        SqliteRepositoryFactory::create_default(path)
    }

    pub fn from_pool(pool: Arc<Pool<SqliteConnectionManager>>) -> Self {
        Self { pool }
    }

    pub(crate) fn devices(&self) -> DeviceRepository {
        DeviceRepository::new(self.pool.clone())
    }
    pub(crate) fn snapshots(&self) -> SnapshotRepository {
        SnapshotRepository::new(self.pool.clone())
    }
    pub(crate) fn files(&self) -> FileRepository {
        FileRepository::new(self.pool.clone())
    }
    pub(crate) fn apps(&self) -> AppRepository {
        AppRepository::new(self.pool.clone())
    }
    pub(crate) fn contacts(&self) -> ContactRepository {
        ContactRepository::new(self.pool.clone())
    }
    pub(crate) fn schedules(&self) -> ScheduleRepository {
        ScheduleRepository::new(self.pool.clone())
    }
    pub(crate) fn maintenance(&self) -> MaintenanceRepository {
        MaintenanceRepository::new(self.pool.clone())
    }
    pub(crate) fn settings(&self) -> SettingsRepository {
        SettingsRepository::new(self.pool.clone())
    }
    pub(crate) fn communication(&self) -> CommunicationRepository {
        CommunicationRepository::new(self.pool.clone())
    }
}
