use domain::{FileEntry, FileId, DeviceId, Snapshot, SnapshotId, SnapshotStatus};
use rusqlite::Row;
use crate::mappers::parse_date;

pub struct BackupMapper;

impl BackupMapper {
    pub fn to_file(row: &Row) -> rusqlite::Result<FileEntry> {
        let modified_at_str: String = row.get(5)?;
        let media_info_str: Option<String> = row.get(9)?;

        let media_info = media_info_str.and_then(|s| {
            serde_json::from_str(&s).ok()
        });

        Ok(FileEntry {
            id: FileId(row.get(0)?),
            device_id: DeviceId(row.get(1)?),
            path: row.get(2)?,
            name: row.get(3)?,
            size_bytes: row.get(4)?,
            modified_at: parse_date(&modified_at_str)?,
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
            started_at: parse_date(&started_at_str)?,
            finished_at: finished_at_str.and_then(|s| parse_date(&s).ok()),
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
}
