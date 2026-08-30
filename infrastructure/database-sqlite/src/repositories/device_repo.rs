use rusqlite::{params, Connection};
use domain::{Device, DeviceId};
use crate::mappers::DeviceMapper;

pub struct DeviceRepository;

impl DeviceRepository {
    pub fn save(conn: &Connection, device: &Device) -> anyhow::Result<()> {
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

    pub fn list(conn: &Connection) -> anyhow::Result<Vec<Device>> {
        let mut stmt = conn.prepare("SELECT id, manufacturer, model, serial, os_version, storage_total_bytes, storage_used_bytes, connection_type FROM devices")?;
        let device_iter = stmt.query_map([], DeviceMapper::to_device)?;
        let mut devices = Vec::new();
        for d in device_iter { devices.push(d?); }
        Ok(devices)
    }

    pub fn get_by_id(_conn: &Connection, _id: &DeviceId) -> anyhow::Result<Option<Device>> {
        Ok(None)
    }
}
