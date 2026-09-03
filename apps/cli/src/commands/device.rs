use anyhow::Result;
use application::BackupService;
use domain::DeviceId;

pub fn print_devices<D, S, R, T, A, DP, P>(
    service: &BackupService<D, S, R, T, A, DP, P>,
) -> Result<()>
where
    D: ports::DevicePort,
    S: ports::ScannerPort,
    R: ports::RepositoryPort,
    T: ports::StoragePort,
    A: ports::AppProviderPort,
    DP: ports::DataProviderPort,
    P: ports::ProgressPort,
{
    let devices = service.list_devices()?;
    println!("Connected Devices\n");
    println!("{:<40} {:<15} {:<12} STATUS", "ID", "MODEL", "OS");
    println!("{}", "-".repeat(80));
    for d in devices {
        let os_short = if d.os_version.len() > 10 {
            "MTP".to_string()
        } else {
            d.os_version.clone()
        };
        println!("{:<40} {:<15} {:<12} Ready", d.id, d.model, os_short);
    }
    Ok(())
}

pub fn print_device_info<D, S, R, T, A, DP, P>(
    service: &BackupService<D, S, R, T, A, DP, P>,
    id: &str,
) -> Result<()>
where
    D: ports::DevicePort,
    S: ports::ScannerPort,
    R: ports::RepositoryPort,
    T: ports::StoragePort,
    A: ports::AppProviderPort,
    DP: ports::DataProviderPort,
    P: ports::ProgressPort,
{
    let device_id = DeviceId::new(id);
    let device = service.device_info(&device_id)?;
    let capabilities = service.device_capabilities(&device_id)?;

    println!("Device");
    println!("├── id: {}", device.id);
    println!("├── manufacturer: {}", device.manufacturer);
    println!("├── model: {}", device.model);
    println!("├── android_version: {}", device.os_version);
    println!(
        "├── storage: {:.1}% used ({} / {} bytes)",
        device.storage_used_percent(),
        device.storage_used_bytes,
        device.storage_total_bytes
    );
    println!("└── capabilities:");
    for (capability, status) in capabilities.iter() {
        println!("      {:?} -> {:?}", capability, status);
    }
    Ok(())
}

pub fn scan_device<D, S, R, T, A, DP, P>(
    service: &BackupService<D, S, R, T, A, DP, P>,
    id: &str,
) -> Result<()>
where
    D: ports::DevicePort,
    S: ports::ScannerPort,
    R: ports::RepositoryPort,
    T: ports::StoragePort,
    A: ports::AppProviderPort,
    DP: ports::DataProviderPort,
    P: ports::ProgressPort,
{
    let device_id = DeviceId::new(id);
    println!("Scanning device {}...", id);

    let result = service.scan_device(&device_id);

    let files = match result {
        Ok(f) => f,
        Err(e)
            if e.to_string().contains("exclusively")
                || e.to_string().contains("timed out")
                || e.to_string().contains("SessionAlreadyOpen") =>
        {
            println!("\n⚠️  Gagal mengakses HP: Perangkat sibuk atau sesi menggantung.");
            println!("Penyebab: {} ", e);
            print!("Apakah Anda ingin saya mencoba membersihkan agen macOS pengganggu? (y/n): ");
            use std::io::{self, Write};
            std::io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if input.trim().to_lowercase() == "y" {
                println!("🚀 Mencoba menutup aplikasi pengganggu...");
                let killed = if id.starts_with("usb://serial/") {
                    let serial = id.trim_start_matches("usb://serial/");
                    adapter_mtp::MtpConflictResolver::resolve_conflicts(serial)
                        .map_err(|e| anyhow::anyhow!(e))?
                } else {
                    adapter_mtp::MtpConflictResolver::kill_conflicts()?
                };
                println!(
                    "✅ Berhasil menutup {} aplikasi. Mencoba memindai ulang...",
                    killed
                );
                service.scan_device(&device_id)?
            } else {
                return Err(e);
            }
        }
        Err(e) => return Err(e),
    };

    println!("\nFound {} files:", files.len());
    for f in files {
        println!(
            "{:<40} {:>10} bytes  {}",
            f.path,
            f.size_bytes,
            f.modified_at.format("%Y-%m-%d %H:%M:%S")
        );
    }
    Ok(())
}

pub fn list_apps<D, S, R, T, A, DP, P>(
    service: &BackupService<D, S, R, T, A, DP, P>,
    id: &str,
) -> Result<()>
where
    D: ports::DevicePort,
    S: ports::ScannerPort,
    R: ports::RepositoryPort,
    T: ports::StoragePort,
    A: ports::AppProviderPort,
    DP: ports::DataProviderPort,
    P: ports::ProgressPort,
{
    let device_id = DeviceId::new(id);
    println!("Listing apps for device {}...", id);
    let apps = service.list_apps(&device_id)?;
    println!("\nInstalled Applications:");
    println!("{:<30} {:<15} PACKAGE", "APP NAME", "VERSION");
    println!("{}", "-".repeat(80));
    for app in apps {
        println!(
            "{:<30} {:<15} {}",
            app.app_name, app.version_name, app.package_name
        );
    }
    Ok(())
}

pub fn list_photos<D, S, R, T, A, DP, P>(
    service: &BackupService<D, S, R, T, A, DP, P>,
    id: &str,
) -> Result<()>
where
    D: ports::DevicePort,
    S: ports::ScannerPort,
    R: ports::RepositoryPort,
    T: ports::StoragePort,
    A: ports::AppProviderPort,
    DP: ports::DataProviderPort,
    P: ports::ProgressPort,
{
    let device_id = DeviceId::new(id);
    let files = service.scan_device(&device_id)?;
    println!("Photo Gallery for device {}\n", id);
    println!(
        "{:<30} {:<15} {:<15} LOCATION",
        "FILE", "CAMERA", "TAKEN AT"
    );
    println!("{}", "-".repeat(90));

    for f in files {
        if f.mime_type.starts_with("image/") {
            let camera = f.media_info.as_ref().and_then(|m| m.camera_model.clone()).unwrap_or_else(|| "-".into());
            let taken = f.media_info.as_ref().and_then(|m| m.taken_at).map(|t| t.format("%Y-%m-%d").to_string()).unwrap_or_else(|| "-".into());
            let loc = f.media_info.as_ref().and_then(|m| m.latitude.zip(m.longitude)).map(|(lat, lon)| format!("{:.4}, {:.4}", lat, lon)).unwrap_or_else(|| "-".into());
            println!("{:<30} {:<15} {:<15} {}", f.name, camera, taken, loc);
        }
    }
    Ok(())
}
