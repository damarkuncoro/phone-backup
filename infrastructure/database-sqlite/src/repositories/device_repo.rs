use rusqlite::params;
use std::sync::Arc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use domain::{Device, DeviceId};
use ports::DeviceRepositoryPort;
use crate::mappers::DeviceMapper;

pub struct DeviceRepository {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl DeviceRepository {
    pub fn new(pool: Arc<Pool<SqliteConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl DeviceRepositoryPort for DeviceRepository {
    fn save_device(&self, device: &Device) -> anyhow::Result<()> {
        let conn = self.pool.get()?;
        conn.execute(
            "INSERT OR REPLACE INTO devices
            (id, manufacturer, model, serial, os_version, storage_total_bytes, storage_used_bytes, connection_type)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                device.id.0, device.manufacturer, device.model, device.serial,
                device.os_version, device.storage_total_bytes, device.storage_used_bytes,
                format!("{:?}", device.connection_type)
            ],
        )?;
        Ok(())
    }

    fn list_devices(&self) -> anyhow::Result<Vec<Device>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT id, manufacturer, model, serial, os_version, storage_total_bytes, storage_used_bytes, connection_type FROM devices")?;
        let device_iter = stmt.query_map([], DeviceMapper::to_device)?;
        let mut devices = Vec::new();
        for d in device_iter { devices.push(d?); }
        Ok(devices)
    }

    fn get_device(&self, id: &DeviceId) -> anyhow::Result<Option<Device>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT id, manufacturer, model, serial, os_version, storage_total_bytes, storage_used_bytes, connection_type FROM devices WHERE id = ?1")?;
        let mut device_iter = stmt.query_map(params![id.0], DeviceMapper::to_device)?;

        if let Some(res) = device_iter.next() {
            Ok(Some(res?))
        } else {
            Ok(None)
        }
    }

    fn get_storage_usage_by_device(&self, device_id: &DeviceId) -> anyhow::Result<u64> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT SUM(total_bytes) FROM snapshots WHERE device_id = ?1")?;
        let usage: Option<u64> = stmt.query_row(params![device_id.0], |row| row.get(0))?;
        Ok(usage.unwrap_or(0))
    }
}
