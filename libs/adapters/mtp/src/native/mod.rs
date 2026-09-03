pub mod directory;
pub mod paths;
pub mod reader;
pub mod session;
pub mod transfer;

use anyhow::Result;
use domain::{DeviceId, FileEntry};
use std::io::Read;

pub use directory::MtpDirectoryOps;
pub use paths::MtpPathResolver;
pub use reader::{MtpReaderOps, MtpStreamingReader};
pub use session::NativeMtpOperations;
pub use transfer::MtpTransferOps;

impl NativeMtpOperations {
    pub fn list_directory(&self, device_id: &DeviceId, path: &str) -> Result<Vec<FileEntry>> {
        MtpDirectoryOps::list_directory(self, device_id, path)
    }

    pub fn scan_recursive(
        &self,
        device_id: &DeviceId,
        target_paths: Vec<String>,
    ) -> Result<Vec<FileEntry>> {
        MtpDirectoryOps::scan_recursive(self, device_id, target_paths)
    }

    pub fn push_file(&self, source: &mut dyn Read, target_path: &str) -> Result<()> {
        MtpTransferOps::push_file(self, source, target_path)
    }

    pub fn delete_object(&self, path: &str) -> Result<()> {
        MtpTransferOps::delete_object(self, path)
    }

    pub fn rename_object(&self, old_path: &str, new_path: &str) -> Result<()> {
        MtpTransferOps::rename_object(self, old_path, new_path)
    }

    pub fn read_file(&self, path: &str) -> Result<Box<dyn Read>> {
        MtpReaderOps::read_file(self, path)
    }

    pub fn calculate_quick_hash(&self, path: &str) -> Result<String> {
        MtpReaderOps::calculate_quick_hash(self, path)
    }
}
