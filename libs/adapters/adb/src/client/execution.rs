use crate::client::AdbClient;
use anyhow::Result;
use std::process::Command;
use std::time::Duration;
use tracing::debug;

impl AdbClient {
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
                    last_error = Some(
                        anyhow::Error::from(e)
                            .context(format!("Failed to execute adb at {}", self.adb_path)),
                    );
                }
            }

            if attempt < max_retries - 1 {
                tracing::warn!(
                    "ADB command failed, retrying in {:?}... (Error: {:?})",
                    delay,
                    last_error
                );
                std::thread::sleep(delay);
                delay *= 2; // Exponential backoff
            }
        }

        Err(last_error
            .unwrap_or_else(|| anyhow::anyhow!("ADB command failed after {} retries", max_retries)))
    }

    pub fn shell(&self, device_serial: &str, script: &str) -> Result<String> {
        self.run_with_retry(&["-s", device_serial, "shell", script], 3)
    }
}
