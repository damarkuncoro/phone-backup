use anyhow::Result;
use domain::{DeviceId, FileEntry};
use ports::ScannerPort;
use crate::repositories::AdbScannerRepository;

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
}
