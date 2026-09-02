use crate::parsers::common::ParserUtils;
use chrono::{TimeZone, Utc};
use domain::{DeviceId, FileEntry, FileId, MediaInfo};

pub struct MediaParser;

impl MediaParser {
    pub fn parse_filesystem_scan(device_id: &DeviceId, stdout: &str) -> Vec<FileEntry> {
        stdout
            .lines()
            .filter(|line| !line.is_empty())
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() < 3 {
                    return None;
                }

                let path = parts[0].to_string();
                let size_bytes = parts[1].parse::<u64>().unwrap_or(0);
                let mtime_unix = parts[2].parse::<i64>().unwrap_or(0);

                let modified_at = Utc
                    .timestamp_opt(mtime_unix, 0)
                    .single()
                    .unwrap_or_else(Utc::now);

                let name = path.split('/').next_back().unwrap_or("").to_string();
                let mime_type = mime_guess::from_path(&path)
                    .first_or_octet_stream()
                    .to_string();

                Some(FileEntry {
                    id: FileId(path.clone()),
                    device_id: device_id.clone(),
                    path,
                    name,
                    size_bytes,
                    modified_at,
                    mime_type,
                    permissions: String::new(),
                    hash_sha256: None,
                    thumbnail_hash: None,
                    media_info: None,
                })
            })
            .collect()
    }

    pub fn parse_mediastore(device_id: &DeviceId, output: &str) -> Vec<FileEntry> {
        output
            .lines()
            .filter(|line| !line.is_empty())
            .filter_map(|line| {
                let path = ParserUtils::extract_value(line, "_data")?;
                let size = ParserUtils::extract_value(line, "_size")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                let mtime = ParserUtils::extract_value(line, "date_modified")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                let mime = ParserUtils::extract_value(line, "mime_type").unwrap_or_default();

                let width = ParserUtils::extract_value(line, "width").and_then(|s| s.parse().ok());
                let height =
                    ParserUtils::extract_value(line, "height").and_then(|s| s.parse().ok());
                let taken_at_ms = ParserUtils::extract_value(line, "datetaken")
                    .and_then(|s| s.parse::<i64>().ok());
                let lat = ParserUtils::extract_value(line, "latitude").and_then(|s| s.parse().ok());
                let lon =
                    ParserUtils::extract_value(line, "longitude").and_then(|s| s.parse().ok());

                let modified_at = Utc
                    .timestamp_opt(mtime, 0)
                    .single()
                    .unwrap_or_else(Utc::now);
                let taken_at = taken_at_ms.and_then(|ms| Utc.timestamp_opt(ms / 1000, 0).single());

                let media_info =
                    if width.is_some() || height.is_some() || taken_at.is_some() || lat.is_some() {
                        Some(MediaInfo {
                            width,
                            height,
                            taken_at,
                            latitude: lat,
                            longitude: lon,
                            ..Default::default()
                        })
                    } else {
                        None
                    };

                Some(FileEntry {
                    id: FileId(path.clone()),
                    device_id: device_id.clone(),
                    path: path.clone(),
                    name: path.split('/').next_back().unwrap_or("").to_string(),
                    size_bytes: size,
                    modified_at,
                    mime_type: mime,
                    permissions: String::new(),
                    hash_sha256: None,
                    thumbnail_hash: None,
                    media_info,
                })
            })
            .collect()
    }
}
