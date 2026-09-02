use crate::backup::BackupService;
use anyhow::Result;
use domain::{CapabilityMatrix, Device, DeviceId, FileEntry};
use ports::{
    AppProviderPort, DataProviderPort, DevicePort, ProgressPort, RepositoryPort, ScannerPort,
    StoragePort,
};
use tracing::{info, instrument};

impl<D, S, R, T, A, DP, P> BackupService<D, S, R, T, A, DP, P>
where
    D: DevicePort,
    S: ScannerPort,
    R: RepositoryPort,
    T: StoragePort,
    A: AppProviderPort,
    DP: DataProviderPort,
    P: ProgressPort,
{
    #[instrument(skip(self))]
    pub fn list_devices(&self) -> Result<Vec<Device>> {
        self.device_adapter.discover()
    }

    #[instrument(skip(self))]
    pub fn list_all_known_devices(&self) -> Result<Vec<Device>> {
        self.repository.list_devices()
    }

    #[instrument(skip(self))]
    pub fn device_info(&self, id: &DeviceId) -> Result<Device> {
        self.device_adapter.info(id)
    }

    #[instrument(skip(self))]
    pub fn device_capabilities(&self, id: &DeviceId) -> Result<CapabilityMatrix> {
        // Wait, CapabilityMatrix is in domain
        self.device_adapter.capabilities(id)
    }

    #[instrument(skip(self))]
    pub fn get_device_battery(&self, id: &DeviceId) -> Result<(u32, f32)> {
        self.device_adapter.battery_status(id)
    }

    #[instrument(skip(self))]
    pub fn list_directory(&self, id: &DeviceId, path: &str) -> Result<Vec<FileEntry>> {
        self.device_adapter.list_directory(id, path)
    }

    #[instrument(skip(self))]
    pub fn delete_remote(&self, id: &DeviceId, path: &str) -> Result<()> {
        self.device_adapter.delete_remote(id, path)
    }

    #[instrument(skip(self))]
    pub fn rename_remote(&self, id: &DeviceId, old_path: &str, new_path: &str) -> Result<()> {
        self.device_adapter.rename_remote(id, old_path, new_path)
    }

    #[instrument(skip(self))]
    pub fn copy_remote(&self, id: &DeviceId, source_path: &str, target_path: &str) -> Result<()> {
        self.device_adapter
            .copy_remote(id, source_path, target_path)
    }

    #[instrument(skip(self))]
    pub fn calculate_hash(&self, id: &DeviceId, path: &str) -> Result<String> {
        self.device_adapter.calculate_hash(id, path)
    }

    #[instrument(skip(self))]
    pub fn upload_file(&self, id: &DeviceId, source_path: &str, target_path: &str) -> Result<()> {
        let mut file = std::fs::File::open(source_path)?;
        self.device_adapter.push_file(id, &mut file, target_path)
    }

    #[instrument(skip(self))]
    pub fn download_file(&self, id: &DeviceId, remote_path: &str, local_path: &str) -> Result<()> {
        let mut reader = self.device_adapter.read_file(id, remote_path)?;

        let path = std::path::Path::new(local_path);
        let target_file_path =
            if path.is_dir() || local_path.ends_with('/') || path.extension().is_none() {
                let filename = std::path::Path::new(remote_path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("downloaded_file");
                std::fs::create_dir_all(path)?;
                path.join(filename)
            } else {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                path.to_path_buf()
            };

        let mut file = std::fs::File::create(&target_file_path)?;
        std::io::copy(&mut reader, &mut file)?;
        info!("Downloaded {} to {:?}", remote_path, target_file_path);
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn scan_device(&self, id: &DeviceId) -> Result<Vec<FileEntry>> {
        self.scanner_adapter.scan(id, vec![])
    }
}
