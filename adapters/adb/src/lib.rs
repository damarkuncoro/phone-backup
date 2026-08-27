use anyhow::{Context, Result};
use domain::{CapabilityMatrix, ConnectionType, Device, DeviceId, FileEntry, FileId, AppInfo, AppId, Contact, Sms, CallLog};
use ports::{DevicePort, ScannerPort, AppProviderPort, DataProviderPort};
use std::process::Command;
use chrono::{Utc, TimeZone};
use std::fs;

pub struct AdbDeviceAdapter {
    adb_path: String,
}

impl AdbDeviceAdapter {
    pub fn new() -> Self {
        let adb_path = which::which("adb")
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "adb".to_string());
        Self { adb_path }
    }

    fn run_adb(&self, args: &[&str]) -> Result<String> {
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
}

impl DevicePort for AdbDeviceAdapter {
    fn discover(&self) -> Result<Vec<Device>> {
        let output = self.run_adb(&["devices", "-l"])?;
        let mut devices = Vec::new();

        for line in output.lines().skip(1) {
            if line.is_empty() { continue; }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 || parts[1] != "device" { continue; }

            let id = DeviceId::new(parts[0]);
            let mut model = "Unknown".to_string();

            for part in &parts[2..] {
                if part.starts_with("model:") {
                    model = part.replace("model:", "");
                }
            }

            devices.push(Device {
                id,
                manufacturer: "Unknown".into(),
                model,
                serial: parts[0].to_string(),
                os_version: "Unknown".into(),
                sdk_version: None,
                storage_total_bytes: 0,
                storage_used_bytes: 0,
                storage_free_bytes: 0,
                connection_type: if line.contains("usb:") { ConnectionType::Usb } else { ConnectionType::Wifi },
            });
        }

        Ok(devices)
    }

    fn info(&self, id: &DeviceId) -> Result<Device> {
        let manufacturer = self.run_adb(&["-s", &id.0, "shell", "getprop", "ro.product.manufacturer"])?.trim().to_string();
        let model = self.run_adb(&["-s", &id.0, "shell", "getprop", "ro.product.model"])?.trim().to_string();
        let os_version = self.run_adb(&["-s", &id.0, "shell", "getprop", "ro.build.version.release"])?.trim().to_string();
        let sdk_version = self.run_adb(&["-s", &id.0, "shell", "getprop", "ro.build.version.sdk"])?.trim().parse().ok();

        let df_output = self.run_adb(&["-s", &id.0, "shell", "df", "/data"])?;
        let mut storage_total_bytes = 0;
        let mut storage_used_bytes = 0;
        let mut storage_free_bytes = 0;

        if let Some(line) = df_output.lines().nth(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                storage_total_bytes = parts[1].parse::<u64>().unwrap_or(0) * 1024;
                storage_used_bytes = parts[2].parse::<u64>().unwrap_or(0) * 1024;
                storage_free_bytes = parts[3].parse::<u64>().unwrap_or(0) * 1024;
            }
        }

        Ok(Device {
            id: id.clone(),
            manufacturer,
            model,
            serial: id.0.clone(),
            os_version,
            sdk_version,
            storage_total_bytes,
            storage_used_bytes,
            storage_free_bytes,
            connection_type: ConnectionType::Usb,
        })
    }

    fn capabilities(&self, _id: &DeviceId) -> Result<CapabilityMatrix> {
        Ok(CapabilityMatrix::new())
    }

    fn read_file(&self, id: &DeviceId, path: &str) -> Result<Box<dyn std::io::Read>> {
        // Gunakan adb pull untuk integritas binary data yang lebih baik daripada shell cat
        let temp_dir = std::env::temp_dir().join("phone_backup_pull");
        if !temp_dir.exists() {
            fs::create_dir_all(&temp_dir)?;
        }

        let temp_file = temp_dir.join(uuid::Uuid::new_v4().to_string());

        let status = Command::new(&self.adb_path)
            .args(&["-s", &id.0, "pull", path, temp_file.to_str().unwrap()])
            .status()?;

        if !status.success() {
            anyhow::bail!("Failed to pull file {} from device", path);
        }

        let content = fs::read(&temp_file)?;
        let _ = fs::remove_file(temp_file); // Cleanup

        Ok(Box::new(std::io::Cursor::new(content)))
    }
}

pub struct AdbScannerAdapter {
    adb_path: String,
}

impl AdbScannerAdapter {
    pub fn new() -> Self {
        let adb_path = which::which("adb")
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "adb".to_string());
        Self { adb_path }
    }
}

impl ScannerPort for AdbScannerAdapter {
    fn scan(&self, device_id: &DeviceId) -> Result<Vec<FileEntry>> {
        // Optimasi: Gunakan find + stat untuk mendapatkan path, size, dan mtime sekaligus
        // Format: path|size|mtime_unix
        let script = "find /sdcard/ -type f -maxdepth 4 2>/dev/null | xargs stat -c '%n|%s|%Y' 2>/dev/null";

        let output = Command::new(&self.adb_path)
            .args(&["-s", &device_id.0, "shell", script])
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut entries = Vec::new();

        for line in stdout.lines() {
            if line.is_empty() { continue; }
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() < 3 { continue; }

            let path = parts[0].to_string();
            let size_bytes = parts[1].parse::<u64>().unwrap_or(0);
            let mtime_unix = parts[2].parse::<i64>().unwrap_or(0);

            let modified_at = Utc.timestamp_opt(mtime_unix, 0).single().unwrap_or_else(Utc::now);

            entries.push(FileEntry {
                id: FileId(path.clone()),
                device_id: device_id.clone(),
                path: path.clone(),
                name: path.split('/').last().unwrap_or("").to_string(),
                size_bytes,
                modified_at,
                mime_type: "application/octet-stream".into(),
                permissions: "".into(),
                hash_sha256: None,
                media_info: None,
            });
        }

        Ok(entries)
    }
}

pub struct AdbAppProvider {
    adb_path: String,
}

impl AdbAppProvider {
    pub fn new() -> Self {
        let adb_path = which::which("adb")
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "adb".to_string());
        Self { adb_path }
    }
}

impl AppProviderPort for AdbAppProvider {
    fn list_apps(&self, device_id: &DeviceId) -> Result<Vec<AppInfo>> {
        let output = Command::new(&self.adb_path)
            .args(&["-s", &device_id.0, "shell", "pm", "list", "packages", "-f", "--user", "0"])
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut apps = Vec::new();

        for line in stdout.lines() {
            if let Some(stripped) = line.strip_prefix("package:") {
                if let Some((_path, pkg)) = stripped.rsplit_once('=') {
                    apps.push(AppInfo {
                        id: AppId(pkg.to_string()),
                        device_id: device_id.clone(),
                        package_name: pkg.to_string(),
                        version_name: "Unknown".into(),
                        version_code: 0,
                        installer: None,
                        app_name: pkg.to_string(),
                    });
                }
            }
        }
        Ok(apps)
    }

    fn get_apk(&self, device_id: &DeviceId, package_name: &str) -> Result<Box<dyn std::io::Read>> {
        let output = Command::new(&self.adb_path)
            .args(&["-s", &device_id.0, "shell", "pm", "path", package_name])
            .output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(path) = stdout.lines().next().and_then(|l| l.strip_prefix("package:")) {
            let apk_output = Command::new(&self.adb_path)
                .args(&["-s", &device_id.0, "shell", "cat", path])
                .output()?;
            Ok(Box::new(std::io::Cursor::new(apk_output.stdout)))
        } else {
            anyhow::bail!("Package not found")
        }
    }
}

pub struct AdbDataProvider {
    adb_path: String,
}

impl AdbDataProvider {
    pub fn new() -> Self {
        let adb_path = which::which("adb")
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "adb".to_string());
        Self { adb_path }
    }
}

impl DataProviderPort for AdbDataProvider {
    fn list_contacts(&self, device_id: &DeviceId) -> Result<Vec<Contact>> {
        // Query content provider for contacts
        // content query --uri content://com.android.contacts/data --projection display_name:data1:data4
        let output = self.run_adb_shell(device_id, "content query --uri content://com.android.contacts/data --projection display_name:data1:data4")?;

        let mut contacts = Vec::new();
        for line in output.lines() {
            if line.contains("display_name=") {
                // Sangat disederhanakan: parsing manual output ADB
                contacts.push(Contact {
                    name: line.to_string(),
                    phones: vec![],
                    emails: vec![],
                });
            }
        }
        Ok(contacts)
    }

    fn list_sms(&self, device_id: &DeviceId) -> Result<Vec<Sms>> {
        let output = self.run_adb_shell(device_id, "content query --uri content://sms --projection address:body:date:type")?;
        let mut messages = Vec::new();
        for line in output.lines() {
            if line.contains("body=") {
                messages.push(Sms {
                    address: "Unknown".into(),
                    body: line.to_string(),
                    date: Utc::now(),
                    type_code: 1,
                });
            }
        }
        Ok(messages)
    }

    fn list_call_logs(&self, device_id: &DeviceId) -> Result<Vec<CallLog>> {
        let output = self.run_adb_shell(device_id, "content query --uri content://call_log/calls --projection number:date:duration:type")?;
        let mut logs = Vec::new();
        for line in output.lines() {
            if line.contains("number=") {
                logs.push(CallLog {
                    number: "Unknown".into(),
                    date: Utc::now(),
                    duration_seconds: 0,
                    type_code: 1,
                });
            }
        }
        Ok(logs)
    }
}

impl AdbDataProvider {
    fn run_adb_shell(&self, device_id: &DeviceId, script: &str) -> Result<String> {
        let output = Command::new(&self.adb_path)
            .args(&["-s", &device_id.0, "shell", script])
            .output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
