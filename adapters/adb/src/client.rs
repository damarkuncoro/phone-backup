use anyhow::{Context, Result};
use std::fs;
use std::process::Command;

pub struct AdbClient {
    adb_path: String,
}

impl AdbClient {
    pub fn new() -> Self {
        let adb_path = which::which("adb")
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| {
                // Lokasi standar macOS Android SDK dan Homebrew
                let home = std::env::var("HOME").unwrap_or_default();
                let paths = vec![
                    format!("{}/Library/Android/sdk/platform-tools/adb", home),
                    "/usr/local/bin/adb".to_string(),
                    "/opt/homebrew/bin/adb".to_string(),
                ];

                for p in paths {
                    if std::path::Path::new(&p).exists() {
                        return p;
                    }
                }
                "adb".to_string()
            });
        Self { adb_path }
    }

    pub fn run(&self, args: &[&str]) -> Result<String> {
        let output = Command::new(&self.adb_path)
            .args(args)
            .output()
            .with_context(|| format!("Failed to execute adb at {}", self.adb_path))?;

        if !output.status.success() {
            anyhow::bail!(
                "ADB command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn shell(&self, device_id: &str, script: &str) -> Result<String> {
        let output = Command::new(&self.adb_path)
            .args(&["-s", device_id, "shell", script])
            .output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn exec_out(&self, device_id: &str, command: &str) -> Result<std::process::Child> {
        let child = Command::new(&self.adb_path)
            .args(&["-s", device_id, "exec-out", command])
            .stdout(std::process::Stdio::piped())
            .spawn()?;
        Ok(child)
    }

    pub fn pull_file(&self, device_id: &str, remote_path: &str) -> Result<Vec<u8>> {
        let temp_dir = std::env::temp_dir().join("phone_backup_pull");
        if !temp_dir.exists() {
            fs::create_dir_all(&temp_dir)?;
        }

        let temp_file = temp_dir.join(uuid::Uuid::new_v4().to_string());

        let status = Command::new(&self.adb_path)
            .args(&["-s", device_id, "pull", remote_path, temp_file.to_str().unwrap()])
            .status()?;

        if !status.success() {
            anyhow::bail!("Failed to pull file {} from device", remote_path);
        }

        let content = fs::read(&temp_file)?;
        let _ = fs::remove_file(temp_file);

        Ok(content)
    }

    pub fn push_file(&self, device_id: &str, data: &[u8], remote_path: &str) -> Result<()> {
        let temp_dir = std::env::temp_dir().join("phone_backup_push");
        if !temp_dir.exists() {
            fs::create_dir_all(&temp_dir)?;
        }
        let temp_file = temp_dir.join(uuid::Uuid::new_v4().to_string());
        fs::write(&temp_file, data)?;

        let status = Command::new(&self.adb_path)
            .args(&["-s", device_id, "push", temp_file.to_str().unwrap(), remote_path])
            .status()?;

        let _ = fs::remove_file(temp_file);

        if !status.success() {
            anyhow::bail!("Failed to push file to device {}", device_id);
        }
        Ok(())
    }
}

impl Default for AdbClient {
    fn default() -> Self {
        Self::new()
    }
}
