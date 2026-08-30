use domain::{Device, DeviceId, ConnectionType, BackupSchedule, ScheduleFrequency};
use rusqlite::Row;
use chrono::{DateTime, Utc};

pub struct DeviceMapper;

impl DeviceMapper {
    pub fn to_device(row: &Row) -> rusqlite::Result<Device> {
        Ok(Device {
            id: DeviceId(row.get(0)?),
            manufacturer: row.get(1)?,
            model: row.get(2)?,
            serial: row.get(3)?,
            os_version: row.get(4)?,
            sdk_version: None,
            storage_total_bytes: row.get(5)?,
            storage_used_bytes: row.get(6)?,
            storage_free_bytes: row.get::<_, u64>(5)? - row.get::<_, u64>(6)?,
            connection_type: ConnectionType::Usb,
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
            last_run_at: last_run_at_str.map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
            enabled: enabled == 1,
        })
    }
}
