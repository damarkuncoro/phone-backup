use crate::mappers::parse_date;
use domain::{BackupSchedule, ConnectionType, Device, DeviceId, ScheduleFrequency};
use rusqlite::Row;

pub struct DeviceMapper;

impl DeviceMapper {
    pub fn to_device(row: &Row) -> rusqlite::Result<Device> {
        let total: u64 = row.get(5)?;
        let used: u64 = row.get(6)?;
        let conn_type_str: String = row.get(7)?;

        Ok(Device {
            id: DeviceId(row.get(0)?),
            manufacturer: row.get(1)?,
            model: row.get(2)?,
            serial: row.get(3)?,
            os_version: row.get(4)?,
            sdk_version: None, // Optional: add to DB if needed
            storage_total_bytes: total,
            storage_used_bytes: used,
            storage_free_bytes: total.saturating_sub(used),
            connection_type: match conn_type_str.as_str() {
                "Usb" => ConnectionType::Usb,
                "Wifi" => ConnectionType::Wifi,
                _ => ConnectionType::Unknown,
            },
        })
    }

    pub fn to_schedule(row: &Row) -> rusqlite::Result<BackupSchedule> {
        let frequency_str: String = row.get(1)?;
        let last_run_at_str: Option<String> = row.get(2)?;
        let enabled: i32 = row.get(3)?;

        Ok(BackupSchedule {
            device_id: DeviceId(row.get(0)?),
            frequency: match frequency_str.as_str() {
                "Hourly" => ScheduleFrequency::Hourly,
                "Weekly" => ScheduleFrequency::Weekly,
                _ => ScheduleFrequency::Daily,
            },
            last_run_at: last_run_at_str.and_then(|s| parse_date(&s).ok()),
            enabled: enabled == 1,
        })
    }
}
