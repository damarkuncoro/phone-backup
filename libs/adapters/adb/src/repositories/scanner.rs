use crate::client::AdbClient;
use crate::scanner::ScannerAggregator;
use anyhow::Result;
use domain::{DeviceId, FileEntry, ScanFilter, ScanResult};

/// Repository delegating ADB scanning operations to ScannerAggregator.
#[derive(Clone)]
pub struct AdbScannerRepository {
    aggregator: ScannerAggregator,
}

impl AdbScannerRepository {
    pub fn new(client: AdbClient) -> Self {
        Self {
            aggregator: ScannerAggregator::new(client),
        }
    }

    pub fn scan(&self, device_id: &DeviceId, roots: Vec<String>) -> Result<Vec<FileEntry>> {
        self.aggregator.scan(device_id, roots)
    }

    pub fn scan_detailed(
        &self,
        device_id: &DeviceId,
        roots: Vec<String>,
        filter: Option<&ScanFilter>,
    ) -> Result<ScanResult> {
        if let Some(f) = filter {
            self.aggregator.scan_with_filter(device_id, roots, f)
        } else {
            self.aggregator.scan_with_result(device_id, roots)
        }
    }
}
