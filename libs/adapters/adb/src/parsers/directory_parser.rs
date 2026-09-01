use domain::{DeviceId, FileEntry, FileId};
use chrono::{TimeZone, Utc};

pub struct DirectoryParser;

impl DirectoryParser {
    pub fn parse(device_id: &DeviceId, stdout: &str) -> Vec<FileEntry> {
        stdout.lines()
            .filter(|line| !line.is_empty())
            .filter_map(|line| {
                // Expected format: path|size|mtime|type_string
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() < 4 { return None; }

                let path = parts[0].to_string();
                let size_bytes = parts[1].parse::<u64>().unwrap_or(0);
                let mtime_unix = parts[2].parse::<i64>().unwrap_or(0);
                let type_str = parts[3].to_lowercase();

                let is_directory = type_str.contains("directory") || type_str.contains("link");

                let modified_at = Utc.timestamp_opt(mtime_unix, 0)
                    .single()
                    .unwrap_or_else(Utc::now);

                let name = path.split('/').last().unwrap_or("").to_string();

                // If it's a directory, we might want to flag it in permissions or mime_type for now
                // until we have an 'is_directory' field in FileEntry.
                let mime_type = if is_directory {
                    "inode/directory".to_string()
                } else {
                    mime_guess::from_path(&path).first_or_octet_stream().to_string()
                };

                Some(FileEntry {
                    id: FileId(path.clone()),
                    device_id: device_id.clone(),
                    path,
                    name,
                    size_bytes,
                    modified_at,
                    mime_type,
                    permissions: if is_directory { "d".to_string() } else { "".to_string() },
                    hash_sha256: None,
                    thumbnail_hash: None,
                    media_info: None,
                })
            })
            .collect()
    }
}
