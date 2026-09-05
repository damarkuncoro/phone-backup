use super::filesystem_scanner::FileSystemScanner;
use super::mediastore_scanner::MediaStoreScanner;
use crate::client::AdbClient;
use anyhow::Result;
use domain::{DeviceId, FileEntry, ScanFilter, ScanResult, ScanSource, ScanWarning};
use scanner_engine::ScanPipeline;
use std::thread;

pub const DEFAULT_SCAN_ROOTS: &[&str] = &["/storage/emulated/0"];

/// High-performance multi-source concurrent scanner aggregator with deduplication and metrics.
#[derive(Clone)]
pub struct ScannerAggregator {
    mediastore_scanner: MediaStoreScanner,
    filesystem_scanner: FileSystemScanner,
}

impl ScannerAggregator {
    pub fn new(client: AdbClient) -> Self {
        Self {
            mediastore_scanner: MediaStoreScanner::new(client.clone()),
            filesystem_scanner: FileSystemScanner::new(client),
        }
    }

    fn resolve_roots(&self, provided_roots: Vec<String>) -> Vec<String> {
        if provided_roots.is_empty() {
            DEFAULT_SCAN_ROOTS.iter().map(|s| s.to_string()).collect()
        } else {
            provided_roots
        }
    }

    pub fn scan(&self, device_id: &DeviceId, roots: Vec<String>) -> Result<Vec<FileEntry>> {
        let res = self.scan_with_result(device_id, roots)?;
        Ok(res.files)
    }

    pub fn scan_with_result(
        &self,
        device_id: &DeviceId,
        roots: Vec<String>,
    ) -> Result<ScanResult> {
        self.scan_with_filter(device_id, roots, &ScanFilter::default())
    }

    fn handle_worker_result(
        res: thread::Result<Result<Vec<FileEntry>>>,
        source: ScanSource,
        path: String,
        op_name: &str,
        pipeline: &mut ScanPipeline,
    ) -> Vec<FileEntry> {
        match res {
            Ok(Ok(entries)) => entries,
            Ok(Err(e)) => {
                pipeline.add_warning(ScanWarning {
                    source,
                    path,
                    message: format!("{} warning: {}", op_name, e),
                });
                Vec::new()
            }
            Err(_) => {
                pipeline.add_warning(ScanWarning {
                    source,
                    path,
                    message: format!("{} worker thread panicked", op_name),
                });
                Vec::new()
            }
        }
    }

    pub fn scan_with_filter(
        &self,
        device_id: &DeviceId,
        roots: Vec<String>,
        filter: &ScanFilter,
    ) -> Result<ScanResult> {
        let scan_roots = self.resolve_roots(roots);
        let mut pipeline = ScanPipeline::builder()
            .with_filter(filter.clone())
            .with_directory_count(scan_roots.len())
            .build();

        let (media_res, fs_res) = thread::scope(|s| {
            let media_handle = s.spawn(|| self.mediastore_scanner.scan(device_id));
            let fs_handle = s.spawn(|| self.filesystem_scanner.scan(device_id, &scan_roots));
            (media_handle.join(), fs_handle.join())
        });

        let media_entries = Self::handle_worker_result(
            media_res,
            ScanSource::MediaStoreImages,
            "MediaStore".to_string(),
            "MediaStore",
            &mut pipeline,
        );

        let filesystem_entries = Self::handle_worker_result(
            fs_res,
            ScanSource::FileSystem,
            scan_roots.join(", "),
            "Filesystem",
            &mut pipeline,
        );

        Ok(pipeline.process_multi_source(media_entries, filesystem_entries))
    }
}
