use rusqlite::params;
use std::sync::Arc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use domain::{DeviceId, BackupSchedule};
use ports::ScheduleRepositoryPort;
use crate::mappers::DeviceMapper;

pub struct ScheduleRepository {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl ScheduleRepository {
    pub fn new(pool: Arc<Pool<SqliteConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl ScheduleRepositoryPort for ScheduleRepository {
    fn save_schedule(&self, schedule: &BackupSchedule) -> anyhow::Result<()> {
        let conn = self.pool.get()?;
        conn.execute(
            "INSERT OR REPLACE INTO schedules (device_id, frequency, last_run_at, enabled) VALUES (?1, ?2, ?3, ?4)",
            params![schedule.device_id.0, format!("{:?}", schedule.frequency), schedule.last_run_at.map(|t| t.to_rfc3339()), if schedule.enabled { 1 } else { 0 }],
        )?;
        Ok(())
    }

    fn get_schedule(&self, device_id: &DeviceId) -> anyhow::Result<Option<BackupSchedule>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT * FROM schedules WHERE device_id = ?1")?;
        let mut schedule_iter = stmt.query_map([&device_id.0], DeviceMapper::to_schedule)?;
        if let Some(s) = schedule_iter.next() { Ok(Some(s?)) } else { Ok(None) }
    }

    fn list_schedules(&self) -> anyhow::Result<Vec<BackupSchedule>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT * FROM schedules WHERE enabled = 1")?;
        let schedule_iter = stmt.query_map([], DeviceMapper::to_schedule)?;
        let mut schedules = Vec::new();
        for s in schedule_iter { schedules.push(s?); }
        Ok(schedules)
    }
}
