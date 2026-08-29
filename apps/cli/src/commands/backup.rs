use anyhow::Result;
use application::BackupService;
use domain::{DeviceId, SnapshotId};

pub fn run_backup<D, S, R, T, A, DP, P>(
    service: &BackupService<D, S, R, T, A, DP, P>,
    id: &str,
    encryption: domain::EncryptionMode,
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
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
    println!("Starting backup for device {}...", id);

    let mut builder = domain::BackupPolicy::builder();
    if let Some(inc) = include {
        builder = builder.include_many(inc);
    }
    if let Some(exc) = exclude {
        builder = builder.exclude_many(exc);
    }
    let policy = builder.build();

    let snapshot = service.perform_backup(&device_id, encryption, Some(policy))?;
    println!("\nBackup completed successfully!");
    println!("Snapshot ID: {}", snapshot.id.0);
    println!("Files:       {}", snapshot.total_files);
    println!("Total Size:  {} bytes", snapshot.total_bytes);
    if snapshot.total_bytes > 0 {
        let ratio = (snapshot.deduped_bytes as f64 / snapshot.total_bytes as f64) * 100.0;
        println!("Deduplication: {:.1}% ({} bytes saved)", ratio, snapshot.deduped_bytes);
    }
    Ok(())
}

pub fn list_snapshots<D, S, R, T, A, DP, P>(service: &BackupService<D, S, R, T, A, DP, P>, id: &str) -> Result<()>
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
    let snapshots = service.list_snapshots(&device_id)?;
    println!("Snapshots for device {}\n", id);
    println!("{:<40} {:<20} {:<10} {:>10}", "ID", "STARTED", "STATUS", "FILES");
    println!("{}", "-".repeat(85));
    for s in snapshots {
        println!(
            "{:<40} {:<20} {:<10} {:>10}",
            s.id.0,
            s.started_at.format("%Y-%m-%d %H:%M:%S"),
            format!("{:?}", s.status),
            s.total_files
        );
    }
    Ok(())
}

pub fn show_snapshot_detail<D, S, R, T, A, DP, P>(
    service: &BackupService<D, S, R, T, A, DP, P>,
    snapshot_id: &str,
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
    let s_id = SnapshotId(snapshot_id.to_string());
    let snapshot = service
        .get_snapshot(&s_id)?
        .ok_or_else(|| anyhow::anyhow!("Snapshot not found"))?;

    println!("Snapshot Details");
    println!("----------------");
    println!("ID:          {}", snapshot.id.0);
    println!("Device ID:   {}", snapshot.device_id.0);
    println!("Started:     {}", snapshot.started_at.format("%Y-%m-%d %H:%M:%S"));
    println!(
        "Finished:    {}",
        snapshot
            .finished_at
            .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or("-".into())
    );
    println!("Status:      {:?}", snapshot.status);
    println!("Total files: {}", snapshot.total_files);
    println!("Total size:  {:.2} MB", snapshot.total_bytes as f64 / 1024.0 / 1024.0);
    println!("Saved:       {:.2} MB", snapshot.deduped_bytes as f64 / 1024.0 / 1024.0);

    let apps = service.get_snapshot_apps(&s_id)?;
    println!("\nApplications ({}):", apps.len());
    for app in apps {
        println!("  - {} ({})", app.app_name, app.package_name);
    }

    Ok(())
}
