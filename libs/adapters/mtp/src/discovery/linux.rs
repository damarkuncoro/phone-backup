use std::path::PathBuf;
use super::{MtpDiscoveryStrategy, MtpMount};

pub struct LinuxDiscovery;

impl LinuxDiscovery {
    pub fn new() -> Self {
        Self
    }
}

impl MtpDiscoveryStrategy for LinuxDiscovery {
    fn detect(&self) -> Vec<MtpMount> {
        let mut mounts = Vec::new();

        // Check Linux GVFS / MTP mount directories
        if let Ok(user_id) = std::env::var("UID") {
            let gvfs_dir = PathBuf::from(format!("/run/user/{}/gvfs", user_id));
            if gvfs_dir.exists() {
                if let Ok(entries) = std::fs::read_dir(gvfs_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        let name = entry.file_name().to_string_lossy().to_string();
                        if name.starts_with("mtp:") {
                            mounts.push(MtpMount { name, path });
                        }
                    }
                }
            }
        }
        mounts
    }
}
