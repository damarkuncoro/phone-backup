pub mod builder;
pub mod command;
pub mod monitor;

pub use builder::AdbClientBuilder;
pub use command::AdbCommandBuilder;
pub use monitor::{AdbMonitor, DeviceEvent};

use anyhow::{Context, Result};
use std::fs;
use std::io::Read;
use std::process::{Command, Stdio, Child};
use tracing::debug;
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

    fn cmd(&self) -> AdbCommandBuilder<'_> {
        AdbCommandBuilder::new()
    }

    pub fn run(&self, args: &[&str]) -> Result<String> {
        self.run_with_retry(args, 3)
    }

    pub fn run_with_retry(&self, args: &[&str], max_retries: u32) -> Result<String> {
        let mut last_error = None;
        let mut delay = Duration::from_millis(500);

        for attempt in 0..max_retries {
            debug!("Running adb command (attempt {}): {:?}", attempt + 1, args);
            let mut cmd = Command::new(&self.adb_path);
            cmd.args(args);

            match cmd.output() {
                Ok(output) if output.status.success() => {
                    return Ok(String::from_utf8_lossy(&output.stdout).to_string());
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    last_error = Some(anyhow::anyhow!("ADB command failed: {}", stderr.trim()));
                }
                Err(e) => {
                    last_error = Some(anyhow::Error::from(e).context(format!("Failed to execute adb at {}", self.adb_path)));
                }
            }

            if attempt < max_retries - 1 {
                tracing::warn!("ADB command failed, retrying in {:?}... (Error: {:?})", delay, last_error);
                std::thread::sleep(delay);
                delay *= 2; // Exponential backoff
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("ADB command failed after {} retries", max_retries)))
    }

    pub fn shell(&self, device_serial: &str, script: &str) -> Result<String> {
        self.run_with_retry(&["-s", device_serial, "shell", script], 3)
    }

    pub fn exec_out(&self, device_serial: &str, command: &str) -> Result<Child> {
        self.cmd()
            .on_device(device_serial)
            .arg("exec-out")
            .arg(command)
            .build(&self.adb_path)
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| format!("Failed to start exec-out process for command: {}", command))
    }

    pub fn get_prop(&self, device_serial: &str, property: &str) -> Result<String> {
        let val = self.shell(device_serial, &format!("getprop {}", property))?;
        Ok(val.trim().to_string())
    }

    /// ZERO-COPY STREAMING: Get a reader for a remote file without intermediate temp files.
    pub fn stream_file(&self, device_serial: &str, remote_path: &str) -> Result<Box<dyn Read>> {
        let mut child = self.cmd()
            .on_device(device_serial)
            .arg("exec-out")
            .arg(&format!("cat \"{}\"", remote_path))
            .build(&self.adb_path)
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| format!("Failed to stream file: {}", remote_path))?;

        let stdout = child.stdout.take()
            .ok_or_else(|| anyhow::anyhow!("Failed to capture stdout for streaming"))?;

        Ok(Box::new(stdout))
    }

    pub fn pull_file(&self, device_serial: &str, remote_path: &str) -> Result<Vec<u8>> {
        let temp_dir = std::env::temp_dir().join("phone_backup_pull");
        if !temp_dir.exists() {
            fs::create_dir_all(&temp_dir)?;
        }

        let temp_file = temp_dir.join(uuid::Uuid::new_v4().to_string());
        let temp_path_str = temp_file.to_string_lossy().to_string();

        let status = self.cmd()
            .on_device(device_serial)
            .arg("pull")
            .arg(remote_path)
            .arg(&temp_path_str)
            .build(&self.adb_path)
            .status()
            .context("Failed to run adb pull")?;

        if !status.success() {
            anyhow::bail!("Failed to pull file {} from device", remote_path);
        }

        let content = fs::read(&temp_file).context("Failed to read pulled file")?;
        let _ = fs::remove_file(temp_file);

        Ok(content)
    }

    pub fn push_file(&self, device_serial: &str, data: &[u8], remote_path: &str) -> Result<()> {
        let temp_dir = std::env::temp_dir().join("phone_backup_push");
        if !temp_dir.exists() {
            fs::create_dir_all(&temp_dir)?;
        }
        let temp_file = temp_dir.join(uuid::Uuid::new_v4().to_string());
        let temp_path_str = temp_file.to_string_lossy().to_string();

        fs::write(&temp_file, data).context("Failed to write temp file for adb push")?;

        let status = self.cmd()
            .on_device(device_serial)
            .arg("push")
            .arg(&temp_path_str)
            .arg(remote_path)
            .build(&self.adb_path)
            .status()
            .context("Failed to run adb push")?;

        let _ = fs::remove_file(temp_file);

        if !status.success() {
            anyhow::bail!("Failed to push file to device {}", device_serial);
        }
        Ok(())
    }
}

impl Default for AdbClient {
    fn default() -> Self {
        Self::new()
    }
}
