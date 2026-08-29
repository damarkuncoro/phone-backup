use anyhow::Result;
use application::BackupService;
use domain::DeviceId;

pub fn print_devices<D, S, R, T, A, DP, P>(service: &BackupService<D, S, R, T, A, DP, P>) -> Result<()>
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
    println!("{:<15} {:<15} {:<12} {}", "ID", "MODEL", "OS", "STATUS");
    println!("{}", "-".repeat(50));
    for d in devices {
        println!("{:<15} {:<15} {:<12} {}", d.id, d.model, d.os_version, "Ready");
    }
    Ok(())
}

pub fn print_device_info<D, S, R, T, A, DP, P>(service: &BackupService<D, S, R, T, A, DP, P>, id: &str) -> Result<()>
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

pub fn scan_device<D, S, R, T, A, DP, P>(service: &BackupService<D, S, R, T, A, DP, P>, id: &str) -> Result<()>
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
    let files = service.scan_device(&device_id)?;
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

pub fn list_apps<D, S, R, T, A, DP, P>(service: &BackupService<D, S, R, T, A, DP, P>, id: &str) -> Result<()>
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
    println!("{:<30} {:<15} {}", "APP NAME", "VERSION", "PACKAGE");
    println!("{}", "-".repeat(80));
    for app in apps {
        println!("{:<30} {:<15} {}", app.app_name, app.version_name, app.package_name);
    }
    Ok(())
}

pub fn list_photos<D, S, R, T, A, DP, P>(service: &BackupService<D, S, R, T, A, DP, P>, id: &str) -> Result<()>
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
    println!("{:<30} {:<15} {:<15} {}", "FILE", "CAMERA", "TAKEN AT", "LOCATION");
    println!("{}", "-".repeat(90));

    for f in files {
        if f.mime_type.starts_with("image/") {
            let camera = f
                .media_info
                .as_ref()
                .and_then(|m| m.camera_model.clone())
                .unwrap_or("-".into());
            let taken = f
                .media_info
                .as_ref()
                .and_then(|m| m.taken_at)
                .map(|t| t.format("%Y-%m-%d").to_string())
                .unwrap_or("-".into());
            let loc = f
                .media_info
                .as_ref()
                .and_then(|m| m.latitude.zip(m.longitude))
                .map(|(lat, lon)| format!("{:.4}, {:.4}", lat, lon))
                .unwrap_or("-".into());

            println!("{:<30} {:<15} {:<15} {}", f.name, camera, taken, loc);
        }
    }
    Ok(())
}
