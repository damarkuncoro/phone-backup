use crate::mappers::AndroidMapper;
use domain::{AppId, AppInfo, SnapshotId};
use ports::AppRepositoryPort;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use std::sync::Arc;

pub struct AppRepository {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl AppRepository {
    pub fn new(pool: Arc<Pool<SqliteConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl AppRepositoryPort for AppRepository {
    fn save_app(&self, app: &AppInfo) -> anyhow::Result<()> {
        let conn = self.pool.get()?;
        // Use true UPSERT to avoid triggering ON DELETE CASCADE on child tables
        conn.execute(
            "INSERT INTO apps (id, device_id, package_name, version_name, version_code, installer, app_name)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(id) DO UPDATE SET
                version_name = excluded.version_name,
                version_code = excluded.version_code,
                installer = excluded.installer,
                app_name = excluded.app_name",
            params![app.id.0, app.device_id.0, app.package_name, app.version_name, app.version_code, app.installer, app.app_name],
        )?;
        Ok(())
    }

    fn link_app_to_snapshot(&self, snapshot_id: &SnapshotId, app_id: &AppId) -> anyhow::Result<()> {
        let conn = self.pool.get()?;
        conn.execute(
            "INSERT OR IGNORE INTO snapshot_apps (snapshot_id, app_id) VALUES (?1, ?2)",
            params![snapshot_id.0, app_id.0],
        )?;
        Ok(())
    }

    fn get_snapshot_apps(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<AppInfo>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT a.* FROM apps a JOIN snapshot_apps sa ON a.id = sa.app_id WHERE sa.snapshot_id = ?1"
        )?;
        let app_iter = stmt.query_map([&snapshot_id.0], AndroidMapper::to_app)?;
        let mut apps = Vec::new();
        for a in app_iter {
            apps.push(a?);
        }
        Ok(apps)
    }
}
