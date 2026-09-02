use domain::AppSettings;
use ports::SettingsRepositoryPort;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use std::sync::Arc;

pub struct SettingsRepository {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl SettingsRepository {
    pub fn new(pool: Arc<Pool<SqliteConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl SettingsRepositoryPort for SettingsRepository {
    fn save_settings(&self, settings: &AppSettings) -> anyhow::Result<()> {
        let conn = self.pool.get()?;
        let json = serde_json::to_string(settings)?;
        conn.execute(
            "INSERT OR REPLACE INTO settings (id, json_data, updated_at) VALUES (1, ?1, CURRENT_TIMESTAMP)",
            params![json],
        )?;
        Ok(())
    }

    fn get_settings(&self) -> anyhow::Result<Option<AppSettings>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT json_data FROM settings WHERE id = 1")?;
        let mut rows = stmt.query([])?;
        if let Some(row) = rows.next()? {
            let json: String = row.get(0)?;
            let settings: AppSettings = serde_json::from_str(&json)?;
            Ok(Some(settings))
        } else {
            Ok(None)
        }
    }
}
