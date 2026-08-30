pub mod builder;
pub mod command;
pub mod monitor;
pub mod execution;
pub mod io;

pub use builder::AdbClientBuilder;
pub use command::AdbCommandBuilder;
pub use monitor::{AdbMonitor, DeviceEvent};

use anyhow::Result;
use std::process::{Stdio, Child};
use std::time::Duration;

/// Core ADB client providing low-level access to the adb binary
#[derive(Clone)]
pub struct AdbClient {
    pub(crate) adb_path: String,
    #[allow(dead_code)]
    pub(crate) timeout: Duration,
}

impl AdbClient {
    pub fn builder() -> AdbClientBuilder {
        AdbClientBuilder::new()
    }

    pub fn new() -> Self {
        Self::builder().build().unwrap()
    }

    pub fn monitor(&self) -> AdbMonitor {
        AdbMonitor::new(self.adb_path.clone())
    }

    pub(crate) fn find_adb() -> String {
        which::which("adb")
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_default();
                let paths = vec![
                    format!("{}/Library/Android/sdk/platform-tools/adb", home),
                    "/usr/local/bin/adb".to_string(),
                    "/opt/homebrew/bin/adb".to_string(),
                    "/usr/bin/adb".to_string(),
                ];

                for p in paths {
                    if std::path::Path::new(&p).exists() {
                        return p;
                    }
                }
                "adb".to_string()
            })
    }

    pub(crate) fn cmd(&self) -> AdbCommandBuilder<'_> {
        AdbCommandBuilder::new()
    }

    pub fn exec_out(&self, device_serial: &str, command: &str) -> Result<Child> {
        self.cmd()
            .on_device(device_serial)
            .arg("exec-out")
            .arg(command)
            .build(&self.adb_path)
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to start exec-out process: {}", e))
    }

    pub fn get_prop(&self, device_serial: &str, property: &str) -> Result<String> {
        let val = self.shell(device_serial, &format!("getprop {}", property))?;
        Ok(val.trim().to_string())
    }
}

impl Default for AdbClient {
    fn default() -> Self {
        Self::new()
    }
}
