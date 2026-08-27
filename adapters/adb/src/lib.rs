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

    fn capabilities(&self, id: &DeviceId) -> Result<CapabilityMatrix> {
        let mut matrix = CapabilityMatrix::new();
        matrix.set(domain::Capability::ReadFiles, domain::CapabilityStatus::Available);

        // Check SMS access
        if self.run_adb(&["-s", &id.0, "shell", "content", "query", "--uri", "content://sms", "--limit", "1"]).is_ok() {
            matrix.set(domain::Capability::ReadSms, domain::CapabilityStatus::Available);
        } else {
            matrix.set(domain::Capability::ReadSms, domain::CapabilityStatus::Denied);
        }

        // Check Contacts access
        if self.run_adb(&["-s", &id.0, "shell", "content", "query", "--uri", "content://com.android.contacts/data", "--limit", "1"]).is_ok() {
            matrix.set(domain::Capability::ReadContacts, domain::CapabilityStatus::Available);
        } else {
            matrix.set(domain::Capability::ReadContacts, domain::CapabilityStatus::RequiresUserAction);
        }

        Ok(matrix)
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

    fn push_file(&self, id: &DeviceId, source: &mut dyn std::io::Read, target_path: &str) -> Result<()> {
        let temp_dir = std::env::temp_dir().join("phone_backup_push");
        if !temp_dir.exists() {
            fs::create_dir_all(&temp_dir)?;
        }
        let temp_file = temp_dir.join(uuid::Uuid::new_v4().to_string());
        let mut buffer = Vec::new();
        source.read_to_end(&mut buffer)?;
        fs::write(&temp_file, buffer)?;

        let status = Command::new(&self.adb_path)
            .args(&["-s", &id.0, "push", temp_file.to_str().unwrap(), target_path])
            .status()?;

        let _ = fs::remove_file(temp_file);

        if !status.success() {
            anyhow::bail!("Failed to push file to device {}", id.0);
        }
        Ok(())
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
            let mime_type = mime_guess::from_path(&path).first_or_octet_stream().to_string();

            entries.push(FileEntry {
                id: FileId(path.clone()),
                device_id: device_id.clone(),
                path: path.clone(),
                name: path.split('/').last().unwrap_or("").to_string(),
                size_bytes,
                modified_at,
                mime_type,
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
            let temp_dir = std::env::temp_dir().join("phone_backup_apk");
            if !temp_dir.exists() {
                fs::create_dir_all(&temp_dir)?;
            }
            let temp_file = temp_dir.join(format!("{}.apk", uuid::Uuid::new_v4()));

            let status = Command::new(&self.adb_path)
                .args(&["-s", &device_id.0, "pull", path, temp_file.to_str().unwrap()])
                .status()?;

            if !status.success() {
                anyhow::bail!("Failed to pull APK for package {}", package_name);
            }

            let content = fs::read(&temp_file)?;
            let _ = fs::remove_file(temp_file);
            Ok(Box::new(std::io::Cursor::new(content)))
        } else {
            anyhow::bail!("Package not found")
        }
    }

    fn install_app(&self, device_id: &DeviceId, apk_data: &mut dyn std::io::Read) -> Result<()> {
        let temp_dir = std::env::temp_dir().join("phone_backup_install");
        if !temp_dir.exists() {
            fs::create_dir_all(&temp_dir)?;
        }
        let temp_file = temp_dir.join(format!("{}.apk", uuid::Uuid::new_v4()));
        let mut buffer = Vec::new();
        apk_data.read_to_end(&mut buffer)?;
        fs::write(&temp_file, buffer)?;

        let status = Command::new(&self.adb_path)
            .args(&["-s", &device_id.0, "install", "-r", temp_file.to_str().unwrap()])
            .status()?;

        let _ = fs::remove_file(temp_file);

        if !status.success() {
            anyhow::bail!("Failed to install APK to device {}", device_id.0);
        }
        Ok(())
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
        let output = self.run_adb_shell(device_id, "content query --uri content://com.android.contacts/data --projection display_name:data1")?;

        let mut contacts = std::collections::HashMap::new();
        for line in output.lines() {
            if let (Some(name), Some(phone)) = (Self::extract_value(line, "display_name"), Self::extract_value(line, "data1")) {
                let contact = contacts.entry(name.clone()).or_insert(Contact {
                    name,
                    phones: vec![],
                    emails: vec![],
                });
                if !contact.phones.contains(&phone) {
                    contact.phones.push(phone);
                }
            }
        }
        Ok(contacts.into_values().collect())
    }

    fn list_sms(&self, device_id: &DeviceId) -> Result<Vec<Sms>> {
        let output = self.run_adb_shell(device_id, "content query --uri content://sms --projection address:body:date:type")?;
        let mut messages = Vec::new();
        for line in output.lines() {
            if let (Some(address), Some(body), Some(date_str)) = (
                Self::extract_value(line, "address"),
                Self::extract_value(line, "body"),
                Self::extract_value(line, "date")
            ) {
                let timestamp = date_str.parse::<i64>().unwrap_or(0);
                messages.push(Sms {
                    address,
                    body,
                    date: Utc.timestamp_opt(timestamp / 1000, 0).single().unwrap_or_else(Utc::now),
                    type_code: Self::extract_value(line, "type").and_then(|s| s.parse().ok()).unwrap_or(1),
                });
            }
        }
        Ok(messages)
    }

    fn list_call_logs(&self, device_id: &DeviceId) -> Result<Vec<CallLog>> {
        let output = self.run_adb_shell(device_id, "content query --uri content://call_log/calls --projection number:date:duration:type")?;
        let mut logs = Vec::new();
        for line in output.lines() {
            if let (Some(number), Some(date_str), Some(duration_str)) = (
                Self::extract_value(line, "number"),
                Self::extract_value(line, "date"),
                Self::extract_value(line, "duration")
            ) {
                let timestamp = date_str.parse::<i64>().unwrap_or(0);
                logs.push(CallLog {
                    number,
                    date: Utc.timestamp_opt(timestamp / 1000, 0).single().unwrap_or_else(Utc::now),
                    duration_seconds: duration_str.parse().unwrap_or(0),
                    type_code: Self::extract_value(line, "type").and_then(|s| s.parse().ok()).unwrap_or(1),
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

    fn extract_value(line: &str, key: &str) -> Option<String> {
        let key_with_eq = format!("{}=", key);
        if let Some(start) = line.find(&key_with_eq) {
            let value_part = &line[start + key_with_eq.len()..];
            // ADB output uses comma as separator, but values might contain commas if not escaped
            // This is a simple heuristic: split by comma-space or end of string
            if let Some(end) = value_part.find(", ") {
                return Some(value_part[..end].trim().to_string());
            } else {
                return Some(value_part.trim().to_string());
            }
        }
        None
    }
}
