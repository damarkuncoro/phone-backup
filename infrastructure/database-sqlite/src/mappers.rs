use chrono::{DateTime, Utc};
use domain::{
    AppId, AppInfo, BackupSchedule, ConnectionType, Device, DeviceId, FileEntry, FileId, ScheduleFrequency,
    Snapshot, SnapshotId, SnapshotStatus,
};
use rusqlite::Row;

pub fn map_row_to_device(row: &Row) -> rusqlite::Result<Device> {
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

pub fn map_row_to_file(row: &Row) -> rusqlite::Result<FileEntry> {
    let modified_at_str: String = row.get(5)?;
    let media_info_str: Option<String> = row.get(9)?;
    let media_info = media_info_str.map(|s| serde_json::from_str(&s).unwrap());

    Ok(FileEntry {
        id: FileId(row.get(0)?),
        device_id: DeviceId(row.get(1)?),
        path: row.get(2)?,
        name: row.get(3)?,
        size_bytes: row.get(4)?,
        modified_at: DateTime::parse_from_rfc3339(&modified_at_str).unwrap().with_timezone(&Utc),
        mime_type: row.get(6)?,
        permissions: row.get(7)?,
        hash_sha256: row.get(8)?,
        media_info,
    })
}

pub fn map_row_to_snapshot(row: &Row) -> rusqlite::Result<Snapshot> {
    let started_at_str: String = row.get(2)?;
    let finished_at_str: Option<String> = row.get(3)?;
    let status_str: String = row.get(4)?;

    Ok(Snapshot {
        id: SnapshotId(row.get(0)?),
        device_id: DeviceId(row.get(1)?),
        started_at: DateTime::parse_from_rfc3339(&started_at_str).unwrap().with_timezone(&Utc),
        finished_at: finished_at_str.map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
        status: match status_str.as_str() {
            "Pending" => SnapshotStatus::Pending,
            "Running" => SnapshotStatus::Running,
            "Completed" => SnapshotStatus::Completed,
            "Interrupted" => SnapshotStatus::Interrupted,
            _ => SnapshotStatus::Failed,
        },
        total_files: row.get(5)?,
        total_bytes: row.get(6)?,
        deduped_bytes: row.get(7)?,
    })
}

pub fn map_row_to_app(row: &Row) -> rusqlite::Result<AppInfo> {
    Ok(AppInfo {
        id: AppId(row.get(0)?),
        device_id: DeviceId(row.get(1)?),
        package_name: row.get(2)?,
        version_name: row.get(3)?,
        version_code: row.get(4)?,
        installer: row.get(5)?,
        app_name: row.get(6)?,
    })
}

pub fn map_row_to_schedule(row: &Row) -> rusqlite::Result<BackupSchedule> {
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
