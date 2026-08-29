use anyhow::Result;
use application::BackupService;
use domain::DeviceId;
use number_prefix::NumberPrefix;

fn format_bytes(bytes: u64) -> String {
    match NumberPrefix::binary(bytes as f64) {
        NumberPrefix::Standalone(n) => format!("{:.0} B", n),
        NumberPrefix::Prefixed(prefix, n) => format!("{:.2} {}B", n, prefix),
    }
}

pub fn run_stats<D, S, R, T, A, DP>(service: &BackupService<D, S, R, T, A, DP>) -> Result<()>
where
    D: ports::DevicePort,
    S: ports::ScannerPort,
    R: ports::RepositoryPort,
    T: ports::StoragePort,
    A: ports::AppProviderPort,
    DP: ports::DataProviderPort,
{
    let stats = service.get_storage_stats()?;
    println!("Repository Statistics");
    println!("---------------------");
    println!("Devices tracked:    {}", stats.total_devices);
    println!("Total snapshots:    {}", stats.total_snapshots);
    println!(
        "Total data backed:  {}",
        format_bytes(stats.total_logical_bytes)
    );
    println!(
        "Storage saved:      {} ({:.1}%)",
        format_bytes(stats.total_deduped_bytes),
        stats.efficiency_ratio()
    );
    println!(
        "Physical storage:   {} (estimated)",
        format_bytes(stats.total_logical_bytes - stats.total_deduped_bytes)
    );
    Ok(())
}

pub fn run_search<D, S, R, T, A, DP>(service: &BackupService<D, S, R, T, A, DP>, query: &str) -> Result<()>
where
    D: ports::DevicePort,
    S: ports::ScannerPort,
    R: ports::RepositoryPort,
    T: ports::StoragePort,
    A: ports::AppProviderPort,
    DP: ports::DataProviderPort,
{
    println!("Searching for '{}'...", query);
    let files = service.search_files(query)?;
    println!("\nFound {} matches:", files.len());
    println!("{:<15} {:<40} {:>10} bytes", "DEVICE", "PATH", "SIZE");
    println!("{}", "-".repeat(70));
    for f in files {
        println!("{:<15} {:<40} {:>10}", f.device_id.0, f.path, f.size_bytes);
    }
    Ok(())
}

pub fn run_clone<D, S, R, T, A, DP>(
    service: &BackupService<D, S, R, T, A, DP>,
    source: &str,
    target: &str,
) -> Result<()>
where
    D: ports::DevicePort,
    S: ports::ScannerPort,
    R: ports::RepositoryPort,
    T: ports::StoragePort,
    A: ports::AppProviderPort,
    DP: ports::DataProviderPort,
{
    service.migrate_device(&DeviceId::new(source), &DeviceId::new(target))
}
