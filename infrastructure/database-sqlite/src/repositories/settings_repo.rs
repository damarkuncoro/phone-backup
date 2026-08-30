use rusqlite::{params, Connection};
use domain::AppSettings;

pub struct SettingsRepository;

impl SettingsRepository {
    pub fn save(conn: &Connection, settings: &AppSettings) -> anyhow::Result<()> {
        let json = serde_json::to_string(settings)?;
        conn.execute(
            "INSERT OR REPLACE INTO settings (id, json_data, updated_at) VALUES (1, ?1, CURRENT_TIMESTAMP)",
            params![json],
        )?;
        Ok(())
    }

    pub fn get(conn: &Connection) -> anyhow::Result<Option<AppSettings>> {
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
