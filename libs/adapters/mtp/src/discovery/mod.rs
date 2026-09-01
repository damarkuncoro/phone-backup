use std::path::PathBuf;
use std::sync::Arc;
use tracing::debug;

pub mod macos;
pub mod linux;

pub struct MtpMount {
    pub name: String,
    pub path: PathBuf,
}

pub trait MtpDiscoveryStrategy: Send + Sync {
    fn detect(&self) -> Vec<MtpMount>;
}

pub struct DiscoveryOrchestrator {
    strategies: Vec<Box<dyn MtpDiscoveryStrategy>>,
}

impl DiscoveryOrchestrator {
    pub fn new() -> Self {
        let mut strategies: Vec<Box<dyn MtpDiscoveryStrategy>> = Vec::new();

        #[cfg(target_os = "macos")]
        strategies.push(Box::new(macos::MacosDiscovery::new()));

        #[cfg(target_os = "linux")]
        strategies.push(Box::new(linux::LinuxDiscovery::new()));

        Self { strategies }
    }

    pub fn discover(&self) -> Vec<MtpMount> {
        let mut all_mounts = Vec::new();
        for strategy in &self.strategies {
            all_mounts.extend(strategy.detect());
        }
        debug!("MTP Discovery: Found {} total potential mounts", all_mounts.len());
        all_mounts
    }
}

impl Default for DiscoveryOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}
