use domain::{Device, DeviceId, ConnectionType, FileEntry, FileId, Snapshot, SnapshotId, SnapshotStatus, AppInfo, AppId, BackupSchedule, ScheduleFrequency};
use chrono::{DateTime, Utc};
use rusqlite::Row;

pub struct RowMapper;

impl RowMapper {
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

    pub fn to_file(row: &Row) -> rusqlite::Result<FileEntry> {
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

    pub fn to_snapshot(row: &Row) -> rusqlite::Result<Snapshot> {
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

    pub fn to_app(row: &Row) -> rusqlite::Result<AppInfo> {
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

    pub fn to_contact_name(row: &Row) -> rusqlite::Result<domain::ContactName> {
        Ok(domain::ContactName {
            display_name: row.get(2)?,
            given_name: row.get(3)?,
            middle_name: row.get(4)?,
            family_name: row.get(5)?,
            prefix: row.get(6)?,
            suffix: row.get(7)?,
        })
    }

    pub fn to_contact_phone(row: &Row) -> rusqlite::Result<domain::ContactPhone> {
        Ok(domain::ContactPhone {
            raw_value: row.get(2)?,
            normalized_value: row.get(3)?,
            phone_type: row.get(4)?,
            label: row.get(5)?,
            is_primary: row.get::<_, i32>(6)? == 1,
        })
    }

    pub fn to_contact_email(row: &Row) -> rusqlite::Result<domain::ContactEmail> {
        Ok(domain::ContactEmail {
            value: row.get(2)?,
            email_type: row.get(3)?,
            label: row.get(4)?,
            is_primary: row.get::<_, i32>(5)? == 1,
        })
    }

    pub fn to_contact_address(row: &Row) -> rusqlite::Result<domain::ContactAddress> {
        Ok(domain::ContactAddress {
            formatted_address: row.get(2)?,
            street: row.get(3)?,
            city: row.get(4)?,
            region: row.get(5)?,
            postal_code: row.get(6)?,
            country: row.get(7)?,
            country_code: row.get(8)?,
            address_type: row.get(9)?,
            label: row.get(10)?,
        })
    }

    pub fn to_contact_organization(row: &Row) -> rusqlite::Result<domain::ContactOrganization> {
        Ok(domain::ContactOrganization {
            company_name: row.get(2)?,
            department: row.get(3)?,
            title: row.get(4)?,
            job_description: row.get(5)?,
            org_type: row.get(6)?,
            label: row.get(7)?,
        })
    }

    pub fn to_contact_url(row: &Row) -> rusqlite::Result<domain::ContactUrl> {
        Ok(domain::ContactUrl {
            url: row.get(2)?,
            url_type: row.get(3)?,
            label: row.get(4)?,
        })
    }

    pub fn to_contact_event(row: &Row) -> rusqlite::Result<domain::ContactEvent> {
        Ok(domain::ContactEvent {
            event_type: row.get(2)?,
            event_date: row.get(3)?,
            label: row.get(4)?,
        })
    }

    pub fn to_contact_photo(row: &Row) -> rusqlite::Result<domain::ContactPhoto> {
        Ok(domain::ContactPhoto {
            file_id: row.get(2)?,
            photo_hash: row.get(3)?,
            mime_type: row.get(4)?,
            is_primary: row.get::<_, i32>(5)? == 1,
        })
    }
}
