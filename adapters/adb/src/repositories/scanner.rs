use crate::client::AdbClient;
use crate::scanner::ScannerAggregator;
use anyhow::Result;
use domain::{DeviceId, FileEntry};

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
}
