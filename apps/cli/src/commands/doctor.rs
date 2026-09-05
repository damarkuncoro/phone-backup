use anyhow::Result;
use application::BackupService;
use ports::{
    AppProviderPort, DataProviderPort, DevicePort, RepositoryPort, ScannerPort, StoragePort,
};
use std::process::Command;

pub fn run_doctor<D, S, R, T, A, DP, P>(service: &BackupService<D, S, R, T, A, DP, P>) -> Result<()>
where
    D: DevicePort,
    S: ScannerPort,
    R: RepositoryPort,
    T: StoragePort,
    A: AppProviderPort,
    DP: DataProviderPort,
    P: ports::ProgressPort,
{
    println!("🩺 Phone Backup Doctor - Comprehensive System Diagnostic");
    println!("-------------------------------------------------------");

    // 1. Check Android ADB
    print!("Checking Android ADB installation... ");
    let adb_path = adapter_adb::AdbClient::find_adb();
    let adb_check = Command::new(&adb_path).arg("version").output();
    match adb_check {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout)
                .lines()
                .next()
                .unwrap_or("Unknown")
                .to_string();
            println!("✅ FOUND ({}) at {}", version, adb_path);
        }
        _ => println!("❌ NOT FOUND. Please install ADB or set ANDROID_HOME."),
    }

    // 2. Check USB MTP Subsystem & Conflicts
    print!("Checking USB MTP subsystem... ");
    let conflicts = adapter_mtp::MtpConflictResolver::find_conflicts();
    if conflicts.is_empty() {
        println!("✅ READY (No exclusive USB locking processes detected)");
    } else {
        println!("⚠️ CONFLICT: {} process(es) locking USB: {:?}", conflicts.len(), conflicts);
        println!("   Tip: Run `phone-backup fix-mtp` or use the GUI troubleshooting wizard.");
    }

    // 3. Check Apple iOS libimobiledevice Tools
    print!("Checking Apple iOS bridge (ideviceinfo)... ");
    let ios_check = Command::new("ideviceinfo").arg("-h").output();
    if ios_check.is_ok() {
        println!("✅ FOUND (Apple device tethering available)");
    } else {
        println!("ℹ️ NOT INSTALLED (Optional for iOS backup. Install via `brew install libimobiledevice`)");
    }

    // 4. Check Device Connectivity & Battery/Thermal Health
    print!("Checking connected devices... ");
    match service.list_devices() {
        Ok(devices) if !devices.is_empty() => {
            println!("✅ {} device(s) detected", devices.len());
            for dev in &devices {
                let bat_info = service.get_device_battery(&dev.id);
                match bat_info {
                    Ok((level, temp)) => {
                        println!(
                            "   📱 [{}] {} (Android {}) | 🔋 Battery: {}% | 🌡️ Temp: {:.1}°C",
                            dev.id.0, dev.model, dev.os_version, level, temp
                        );
                    }
                    Err(_) => {
                        println!("   📱 [{}] {} (Android {})", dev.id.0, dev.model, dev.os_version);
                    }
                }
            }
        }
        Ok(_) => println!("⚠️ NO DEVICES. Connect your phone via USB and enable debugging/MTP."),
        Err(e) => println!("❌ ERROR: {}", e),
    }

    // 5. Check Workspace & Database
    print!("Checking workspace integrity... ");
    if std::path::Path::new("workspace/backup.db").exists() {
        println!("✅ backup.db found");
    } else {
        println!("⚠️ backup.db missing. It will be created on first backup.");
    }

    // 6. Check Storage Connectivity & Available Free Space
    print!("Checking storage connectivity... ");
    match service.storage.available_space() {
        Ok(bytes) => {
            let gb = bytes as f64 / 1024.0 / 1024.0 / 1024.0;
            println!("✅ Storage reachable ({:.2} GB available)", gb);
        }
        Err(e) => println!("❌ Storage error: {}", e),
    }

    println!("\nDiagnostic Complete!");
    Ok(())
}

