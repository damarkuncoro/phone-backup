use rusqlite::params;
use std::sync::Arc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use domain::{SnapshotId, Sms, CallLog};
use ports::{SmsRepositoryPort, CallLogRepositoryPort};
use crate::mappers::parse_date;

pub struct CommunicationRepository {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl CommunicationRepository {
    pub fn new(pool: Arc<Pool<SqliteConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl SmsRepositoryPort for CommunicationRepository {
    fn save_sms(&self, snapshot_id: &SnapshotId, sms: &Sms) -> anyhow::Result<()> {
        self.save_sms_batch(snapshot_id, std::slice::from_ref(sms))
    }

    fn save_sms_batch(&self, snapshot_id: &SnapshotId, sms_list: &[Sms]) -> anyhow::Result<()> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;

        {
            let mut stmt = tx.prepare_cached(
                "INSERT INTO messages (id, snapshot_id, address, body, date, type_code)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
            )?;

            for sms in sms_list {
                stmt.execute(params![
                    uuid::Uuid::new_v4().to_string(), snapshot_id.0,
                    sms.address, sms.body, sms.date.to_rfc3339(), sms.type_code
                ])?;
            }
        }

        tx.commit()?;
        Ok(())
    }

    fn get_snapshot_sms(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<Sms>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT address, body, date, type_code FROM messages WHERE snapshot_id = ?1")?;
        let sms_iter = stmt.query_map([&snapshot_id.0], |row| {
            Ok(Sms {
                address: row.get(0)?,
                body: row.get(1)?,
                date: parse_date(&row.get::<_, String>(2)?).unwrap_or_default(),
                type_code: row.get(3)?,
            })
        })?;

        let mut results = Vec::new();
        for s in sms_iter { results.push(s?); }
        Ok(results)
    }

    fn search_sms(&self, query: &str) -> anyhow::Result<Vec<(SnapshotId, Sms)>> {
        let conn = self.pool.get()?;
        let fts_query = format!("\"{}\"*", query.replace("\"", "\"\""));

        let mut stmt = conn.prepare(
            "SELECT m.snapshot_id, m.address, m.body, m.date, m.type_code
             FROM messages m
             JOIN messages_fts fts ON m.rowid = fts.rowid
             WHERE messages_fts MATCH ?1 ORDER BY rank"
        )?;

        let sms_iter = stmt.query_map([fts_query], |row| {
            let snap_id: String = row.get(0)?;
            Ok((SnapshotId(snap_id), Sms {
                address: row.get(1)?,
                body: row.get(2)?,
                date: parse_date(&row.get::<_, String>(3)?).unwrap_or_default(),
                type_code: row.get(4)?,
            }))
        })?;

        let mut results = Vec::new();
        for s in sms_iter { results.push(s?); }
        Ok(results)
    }
}

impl CallLogRepositoryPort for CommunicationRepository {
    fn save_call_log(&self, snapshot_id: &SnapshotId, log: &CallLog) -> anyhow::Result<()> {
        self.save_call_logs_batch(snapshot_id, std::slice::from_ref(log))
    }

    fn save_call_logs_batch(&self, snapshot_id: &SnapshotId, logs: &[CallLog]) -> anyhow::Result<()> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;

        {
            let mut stmt = tx.prepare_cached(
                "INSERT INTO call_logs (id, snapshot_id, number, name, date, duration_seconds, type_code, location)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"
            )?;

            for log in logs {
                stmt.execute(params![
                    uuid::Uuid::new_v4().to_string(), snapshot_id.0,
                    log.number, log.name, log.date.to_rfc3339(),
                    log.duration_seconds, log.type_code, log.location
                ])?;
            }
        }

        tx.commit()?;
        Ok(())
    }

    fn get_snapshot_call_logs(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<CallLog>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT number, name, date, duration_seconds, type_code, location FROM call_logs WHERE snapshot_id = ?1")?;
        let log_iter = stmt.query_map([&snapshot_id.0], |row| {
            Ok(CallLog {
                number: row.get(0)?,
                name: row.get(1)?,
                date: parse_date(&row.get::<_, String>(2)?).unwrap_or_default(),
                duration_seconds: row.get(3)?,
                type_code: row.get(4)?,
                location: row.get(5)?,
            })
        })?;

        let mut results = Vec::new();
        for l in log_iter { results.push(l?); }
        Ok(results)
    }

    fn search_call_logs(&self, query: &str) -> anyhow::Result<Vec<(SnapshotId, CallLog)>> {
        let conn = self.pool.get()?;
        let pattern = format!("%{}%", query);

        let mut stmt = conn.prepare(
            "SELECT snapshot_id, number, name, date, duration_seconds, type_code, location
             FROM call_logs
             WHERE number LIKE ?1 OR name LIKE ?1 OR location LIKE ?1"
        )?;

        let log_iter = stmt.query_map([&pattern], |row| {
            let snap_id: String = row.get(0)?;
            Ok((SnapshotId(snap_id), CallLog {
                number: row.get(1)?,
                name: row.get(2)?,
                date: parse_date(&row.get::<_, String>(3)?).unwrap_or_default(),
                duration_seconds: row.get(4)?,
                type_code: row.get(5)?,
                location: row.get(6)?,
            }))
        })?;

        let mut results = Vec::new();
        for l in log_iter { results.push(l?); }
        Ok(results)
    }
}
