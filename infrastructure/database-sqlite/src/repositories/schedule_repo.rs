use rusqlite::{params, Connection};
use domain::{DeviceId, BackupSchedule};
use crate::mappers::DeviceMapper;

pub struct ScheduleRepository;

impl ScheduleRepository {
    pub fn save(conn: &Connection, schedule: &BackupSchedule) -> anyhow::Result<()> {
        conn.execute(
            "INSERT OR REPLACE INTO schedules (device_id, frequency, last_run_at, enabled) VALUES (?1, ?2, ?3, ?4)",
            params![schedule.device_id.0, format!("{:?}", schedule.frequency), schedule.last_run_at.map(|t| t.to_rfc3339()), if schedule.enabled { 1 } else { 0 }],
        )?;
        Ok(())
    }

    pub fn get_by_device(conn: &Connection, device_id: &DeviceId) -> anyhow::Result<Option<BackupSchedule>> {
        let mut stmt = conn.prepare("SELECT * FROM schedules WHERE device_id = ?1")?;
        let mut schedule_iter = stmt.query_map([&device_id.0], DeviceMapper::to_schedule)?;
        if let Some(s) = schedule_iter.next() { Ok(Some(s?)) } else { Ok(None) }
    }

    pub fn list_enabled(conn: &Connection) -> anyhow::Result<Vec<BackupSchedule>> {
        let mut stmt = conn.prepare("SELECT * FROM schedules WHERE enabled = 1")?;
        let schedule_iter = stmt.query_map([], DeviceMapper::to_schedule)?;
        let mut schedules = Vec::new();
        for s in schedule_iter { schedules.push(s?); }
        Ok(schedules)
    }
}
