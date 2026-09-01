use std::path::PathBuf;
use tracing::{info, debug};
use super::{MtpDiscoveryStrategy, MtpMount};

pub struct MacosDiscovery;

impl MacosDiscovery {
    pub fn new() -> Self {
        Self
    }

    fn is_android_volume(&self, name: &str, path: &std::path::Path) -> bool {
        let name_lower = name.to_lowercase();

        // Keywords check
        let has_keyword = name_lower.contains("android")
            || name_lower.contains("phone")
            || name_lower.contains("mtp")
            || name_lower.contains("pixel")
            || name_lower.contains("samsung")
            || name_lower.contains("storage");

        if has_keyword { return true; }

        // Structure check
        let has_structure = path.join("DCIM").exists()
            || path.join("Internal storage").exists()
            || path.join("Internal Storage").exists()
            || path.join("sdcard").exists();

        if has_structure { return true; }

        // Deeper check
        std::fs::read_dir(path).map(|dir| {
            dir.flatten().any(|e| {
                let n = e.file_name().to_string_lossy().to_lowercase();
                n.contains("internal") || n.contains("storage") || n.contains("sdcard")
            })
        }).unwrap_or(false)
    }
}

impl MtpDiscoveryStrategy for MacosDiscovery {
    fn detect(&self) -> Vec<MtpMount> {
        let mut mounts = Vec::new();

        if let Ok(entries) = std::fs::read_dir("/Volumes") {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();

                debug!("MacosDiscovery: checking volume '{}'", name);

                if self.is_android_volume(&name, &path) {
                    info!("MacosDiscovery: ✅ Android device detected at {:?}", path);
                    mounts.push(MtpMount { name, path });
                }
            }
        }
        mounts
    }
}
