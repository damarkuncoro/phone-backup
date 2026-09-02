use crate::client::AdbClient;
use anyhow::{Context, Result};
use std::fs;
use std::io::Read;
use std::process::Stdio;

impl AdbClient {
    /// ZERO-COPY STREAMING: Get a reader for a remote file without intermediate temp files.
    pub fn stream_file(&self, device_serial: &str, remote_path: &str) -> Result<Box<dyn Read>> {
        let mut child = self
            .cmd()
            .on_device(device_serial)
            .arg("exec-out")
            .arg(&format!("cat \"{}\"", remote_path))
            .build(&self.adb_path)
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| format!("Failed to stream file: {}", remote_path))?;

        let stdout = child
            .stdout
            .take()
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

        let status = self
            .cmd()
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

        let status = self
            .cmd()
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
