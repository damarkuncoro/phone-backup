use anyhow::Result;
use application::BackupService;
use domain::SnapshotId;

pub fn run_restore<D, S, R, T, A, DP>(
    service: &BackupService<D, S, R, T, A, DP>,
    snapshot_id: &str,
    target: &str,
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
{
    if let Some(f) = filter {
        println!(
            "Restoring files matching '{}' from snapshot {} to {}...",
            f, snapshot_id, target
        );
    } else {
        println!("Restoring snapshot {} to {}...", snapshot_id, target);
    }
    service.perform_restore(&SnapshotId(snapshot_id.to_string()), target, encryption, filter)?;
    println!("\nRestore completed successfully!");
    Ok(())
}

pub fn run_verify<D, S, R, T, A, DP>(
    service: &BackupService<D, S, R, T, A, DP>,
    encryption: domain::EncryptionMode,
) -> Result<()>
where
    D: ports::DevicePort,
    S: ports::ScannerPort,
    R: ports::RepositoryPort,
    T: ports::StoragePort,
    A: ports::AppProviderPort,
    DP: ports::DataProviderPort,
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
