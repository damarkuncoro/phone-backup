use anyhow::Result;
use application::BackupService;
use ports::{AppProviderPort, DataProviderPort, DevicePort, RepositoryPort, ScannerPort, StoragePort};
use std::process::Command;

pub fn run_doctor<D, S, R, T, A, DP, P>(
    service: &BackupService<D, S, R, T, A, DP, P>,
) -> Result<()>
where
    D: DevicePort,
    S: ScannerPort,
    R: RepositoryPort,
    T: StoragePort,
    A: AppProviderPort,
    DP: DataProviderPort,
    P: ports::ProgressPort,
{
    println!("🩺 Phone Backup Doctor - System Diagnostic");
    println!("-----------------------------------------");

    // 1. Check ADB
    print!("Checking ADB installation... ");
    let adb_check = Command::new("adb").arg("version").output();
    match adb_check {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout).lines().next().unwrap_or("Unknown").to_string();
            println!("✅ FOUND ({})", version);
        }
        _ => println!("❌ NOT FOUND. Please install ADB and add it to your PATH."),
    }

    // 2. Check Device Connectivity
    print!("Checking connected devices... ");
    match service.list_devices() {
        Ok(devices) if !devices.is_empty() => println!("✅ {} device(s) detected", devices.len()),
        Ok(_) => println!("⚠️ NO DEVICES. Connect your phone via USB and enable debugging."),
        Err(e) => println!("❌ ERROR: {}", e),
    }

    // 3. Check Workspace & Database
    print!("Checking workspace integrity... ");
    if std::path::Path::new("workspace/backup.db").exists() {
        println!("✅ backup.db found");
    } else {
        println!("⚠️ backup.db missing. It will be created on first backup.");
    }

    // 4. Check Storage
    print!("Checking storage connectivity... ");
    match service.storage.exists("health-check") {
        Ok(_) => println!("✅ storage reachable"),
        Err(e) => println!("❌ storage error: {}", e),
    }

    println!("\nDiagnostic Complete!");
    Ok(())
}
