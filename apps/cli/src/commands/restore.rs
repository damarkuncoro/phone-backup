use anyhow::Result;
use application::BackupService;
use domain::SnapshotId;

pub fn run_restore<D, S, R, T, A, DP, P>(
    service: &BackupService<D, S, R, T, A, DP, P>,
    snapshot_id: &str,
    target: Option<String>,
    encryption: domain::EncryptionMode,
    filter: Option<&str>,
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
    let snapshot = if snapshot_id == "last" {
        service
            .get_latest_snapshot_any_device()?
            .ok_or_else(|| anyhow::anyhow!("No snapshots found in repository"))?
    } else {
        let id = SnapshotId(snapshot_id.to_string());
        service
            .get_snapshot(&id)?
            .ok_or_else(|| anyhow::anyhow!("Snapshot {} not found", snapshot_id))?
    };

    let target_dir = match target {
        Some(t) => t,
        None => {
            let date_str = snapshot.started_at.format("%Y%m%d_%H%M%S").to_string();
            format!("workspace/restored_{}_{}", snapshot.device_id.0, date_str)
        }
    };

    if let Some(f) = filter {
        println!(
            "Restoring files matching '{}' from snapshot {} to {}...",
            f, snapshot.id.0, target_dir
        );
    } else {
        println!("Restoring snapshot {} to {}...", snapshot.id.0, target_dir);
    }

    let mut options_builder =
        domain::RestoreOptions::builder(&target_dir).with_encryption(encryption);
    if let Some(f) = filter {
        options_builder = options_builder.with_filter(f);
    }
    let options = options_builder.build();

    service.perform_restore_with_options(&snapshot.id, &options)?;
    println!("\nRestore completed successfully to: {}", target_dir);
    Ok(())
}

pub fn run_verify<D, S, R, T, A, DP, P>(
    service: &BackupService<D, S, R, T, A, DP, P>,
    encryption: domain::EncryptionMode,
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
    println!("Verifying repository integrity...");
    let report = service.verify_repository(encryption)?;
    println!("\nRepository Verification Report");
    println!("------------------------------");
    println!("Total files in index:  {}", report.total_files);
    println!("Verified objects:      {}", report.verified_files);
    println!("Missing objects:       {}", report.missing_objects.len());
    println!("Corrupted files:       {}", report.corrupted_files.len());
    if report.is_healthy() {
        println!("\nSTATUS: HEALTHY");
    } else {
        println!("\nSTATUS: UNHEALTHY");
    }
    Ok(())
}
