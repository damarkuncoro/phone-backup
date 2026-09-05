use crate::repositories::AdbScannerRepository;
use anyhow::Result;
use domain::{DeviceId, FileEntry, ScanFilter, ScanResult};
use ports::ScannerPort;

#[derive(Clone)]
pub struct AdbScannerGateway {
    repo: AdbScannerRepository,
}

impl AdbScannerGateway {
    pub fn new(repo: AdbScannerRepository) -> Self {
        Self { repo }
    }
}

impl ScannerPort for AdbScannerGateway {
    fn scan(&self, device_id: &DeviceId, roots: Vec<String>) -> Result<Vec<FileEntry>> {
        self.repo.scan(device_id, roots)
    }

    fn scan_detailed(
        &self,
        device_id: &DeviceId,
        roots: Vec<String>,
        filter: Option<&ScanFilter>,
    ) -> Result<ScanResult> {
        self.repo.scan_detailed(device_id, roots, filter)
    }
}
