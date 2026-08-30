use crate::client::AdbClient;
use anyhow::{Context, Result};
use chrono::{TimeZone, Utc};
use domain::{DeviceId, FileEntry, FileId};
use ports::ScannerPort;

/// Default Android paths to scan for media and important data
const DEFAULT_SCAN_ROOTS: &[&str] = &[
    "/storage/emulated/0/DCIM",
    "/storage/emulated/0/Pictures",
    "/storage/emulated/0/Android/media/com.whatsapp/WhatsApp/Media",
    "/storage/emulated/0/WhatsApp/Media",
];

pub struct AdbScannerAdapter {
    client: AdbClient,
}

impl AdbScannerAdapter {
    pub fn new() -> Self {
        Self {
            client: AdbClient::new(),
        }
    }

    /// Resolve which roots to use for the scan
    fn resolve_roots(&self, provided_roots: Vec<String>) -> Vec<String> {
        if provided_roots.is_empty() {
            DEFAULT_SCAN_ROOTS.iter().map(|s| s.to_string()).collect()
        } else {
            provided_roots
        }
    }
}

impl Default for AdbScannerAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ScannerPort for AdbScannerAdapter {
    fn scan(&self, device_id: &DeviceId, roots: Vec<String>) -> Result<Vec<FileEntry>> {
        let scan_roots = self.resolve_roots(roots);

        let script = AdbScriptBuilder::build_find_stat_script(&scan_roots);
        let stdout = self.client.shell(&device_id.0, &script)
            .context("Failed to execute scan script via ADB")?;

        let parser = AdbScannerParser::new(device_id.clone());
        let entries = parser.parse_output(&stdout);

        Ok(entries)
    }
}

/// SRP: Responsible only for building ADB shell scripts
struct AdbScriptBuilder;

impl AdbScriptBuilder {
    fn build_find_stat_script(roots: &[String]) -> String {
        let roots_str = roots.join(" ");
        // %n: file name, %s: size in bytes, %Y: time of last modification (seconds since epoch)
        format!("find {} -type f -exec stat -c '%n|%s|%Y' {{}} + 2>/dev/null", roots_str)
    }
}

/// SRP: Responsible only for parsing ADB shell output into Domain entities
struct AdbScannerParser {
    device_id: DeviceId,
}

impl AdbScannerParser {
    fn new(device_id: DeviceId) -> Self {
        Self { device_id }
    }

    fn parse_output(&self, stdout: &str) -> Vec<FileEntry> {
        stdout.lines()
            .filter(|line| !line.is_empty())
            .filter_map(|line| self.parse_line(line))
            .collect()
    }

    fn parse_line(&self, line: &str) -> Option<FileEntry> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 3 {
            return None;
        }

        let path = parts[0].to_string();
        let size_bytes = parts[1].parse::<u64>().unwrap_or(0);
        let mtime_unix = parts[2].parse::<i64>().unwrap_or(0);

        let modified_at = Utc.timestamp_opt(mtime_unix, 0)
            .single()
            .unwrap_or_else(Utc::now);

        let name = path.split('/').last()
            .unwrap_or("")
            .to_string();

        let mime_type = mime_guess::from_path(&path)
            .first_or_octet_stream()
            .to_string();

        Some(FileEntry {
            id: FileId(path.clone()),
            device_id: self.device_id.clone(),
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
    }
}
