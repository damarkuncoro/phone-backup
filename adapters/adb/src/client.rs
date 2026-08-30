use anyhow::{Context, Result};
use std::fs;
use std::process::{Command, Stdio, Child};
use tracing::debug;

pub struct AdbCommandBuilder<'a> {
    serial: Option<&'a str>,
    args: Vec<&'a str>,
}

impl<'a> AdbCommandBuilder<'a> {
    pub fn new() -> Self {
        Self { serial: None, args: Vec::new() }
    }

    pub fn on_device(mut self, serial: &'a str) -> Self {
        self.serial = Some(serial);
        self
    }

    pub fn arg(mut self, arg: &'a str) -> Self {
        self.args.push(arg);
        self
    }

    pub fn shell(mut self, script: &'a str) -> Self {
        self.args.push("shell");
        self.args.push(script);
        self
    }

    pub fn build(self, adb_path: &str) -> Command {
        let mut cmd = Command::new(adb_path);
        if let Some(s) = self.serial {
            cmd.arg("-s").arg(s);
        }
        cmd.args(&self.args);
        cmd
    }
}

/// Core ADB client providing low-level access to the adb binary
#[derive(Clone)]
pub struct AdbClient {
    adb_path: String,
}

impl AdbClient {
    pub fn new() -> Self {
        let adb_path = Self::find_adb();
        Self { adb_path }
    }

    fn find_adb() -> String {
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

    /// Run a raw ADB command
    pub fn run(&self, args: &[&str]) -> Result<String> {
        debug!("Running adb command: {:?}", args);
        let mut cmd = Command::new(&self.adb_path);
        cmd.args(args);

        let output = cmd.output()
            .with_context(|| format!("Failed to execute adb at {}", self.adb_path))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("ADB command failed: {}", stderr.trim());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Run a shell command on a specific device
    pub fn shell(&self, device_serial: &str, script: &str) -> Result<String> {
        let output = self.cmd()
            .on_device(device_serial)
            .shell(script)
            .build(&self.adb_path)
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Execute a command and get a Child process for streaming (exec-out)
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

    /// Get a property value from the device
    pub fn get_prop(&self, device_serial: &str, property: &str) -> Result<String> {
        let val = self.shell(device_serial, &format!("getprop {}", property))?;
        Ok(val.trim().to_string())
    }

    /// Pull a file from the device into memory
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

    /// Push data to a file on the device
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
