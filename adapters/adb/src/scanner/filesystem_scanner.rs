use crate::client::AdbClient;
use crate::parsers::media_parser::MediaParser;
use crate::scripts::AndroidScripts;
use anyhow::{Context, Result};
use domain::{DeviceId, FileEntry};

/// Sub-scanner dedicated to POSIX filesystem scanning (`find`) on target roots.
#[derive(Clone)]
pub struct FileSystemScanner {
    client: AdbClient,
}

impl FileSystemScanner {
    pub fn new(client: AdbClient) -> Self {
        Self { client }
    }

    pub fn scan(&self, device_id: &DeviceId, roots: &[String]) -> Result<Vec<FileEntry>> {
        let script = AndroidScripts::find_files(roots);
        let stdout = self
            .client
            .shell(&device_id.0, &script)
            .context("Failed to execute Android filesystem scan")?;

        Ok(MediaParser::parse_filesystem_scan(device_id, &stdout))
    }
}
