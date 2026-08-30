use rusqlite::{params, Connection};
use domain::{AppId, AppInfo, SnapshotId};
use crate::mappers::AndroidMapper;

pub struct AppRepository;

impl AppRepository {
    pub fn save(conn: &Connection, app: &AppInfo) -> anyhow::Result<()> {
        conn.execute(
            "INSERT OR REPLACE INTO apps (id, device_id, package_name, version_name, version_code, installer, app_name)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![app.id.0, app.device_id.0, app.package_name, app.version_name, app.version_code, app.installer, app.app_name],
        )?;
        Ok(())
    }

    pub fn link_to_snapshot(conn: &Connection, snapshot_id: &SnapshotId, app_id: &AppId) -> anyhow::Result<()> {
        conn.execute(
            "INSERT OR IGNORE INTO snapshot_apps (snapshot_id, app_id) VALUES (?1, ?2)",
            params![snapshot_id.0, app_id.0],
        )?;
        Ok(())
    }

    pub fn get_by_snapshot(conn: &Connection, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<AppInfo>> {
        let mut stmt = conn.prepare(
            "SELECT a.* FROM apps a JOIN snapshot_apps sa ON a.id = sa.app_id WHERE sa.snapshot_id = ?1"
        )?;
        let app_iter = stmt.query_map([&snapshot_id.0], AndroidMapper::to_app)?;
        let mut apps = Vec::new();
        for a in app_iter { apps.push(a?); }
        Ok(apps)
    }
}
