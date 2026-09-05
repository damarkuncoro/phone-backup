use anyhow::{bail, Result};
use application::BackupService;
use domain::DeviceId;
use scanner_engine::IncrementalScanner;
use std::collections::HashMap;

fn format_bytes(bytes: u64) -> String {
    match number_prefix::NumberPrefix::decimal(bytes as f64) {
        number_prefix::NumberPrefix::Standalone(b) => format!("{:.0} B", b),
        number_prefix::NumberPrefix::Prefixed(p, n) => format!("{:.2} {}B", n, p),
    }
}

pub fn handle_diff<D, S, R, T, A, DP, P>(
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
    println!("🔍 Comparing live state of device {} against latest snapshot...", id);

    let snapshots = service.list_snapshots(&device_id)?;
    if snapshots.is_empty() {
        bail!("No previous snapshots found for device {}. Run a backup first!", id);
    }

    let latest_snapshot = &snapshots[0];
    println!("📅 Baseline Snapshot: {} ({})", latest_snapshot.id.0, latest_snapshot.started_at.format("%Y-%m-%d %H:%M:%S"));

    let previous_files = service.get_snapshot_files(&latest_snapshot.id)?;
    let mut prev_map = HashMap::new();
    for f in previous_files {
        prev_map.insert(f.path.clone(), f);
    }

    let current_files = service.scan_device(&device_id)?;
    let diff = IncrementalScanner::diff(&current_files, &prev_map);

    println!("\n📊 Delta Summary:");
    println!("  🟢 Added (New):     {} files ({})", diff.added.len(), format_bytes(diff.added.iter().map(|f| f.size_bytes).sum()));
    println!("  🟡 Modified:        {} files ({})", diff.modified.len(), format_bytes(diff.modified.iter().map(|f| f.size_bytes).sum()));
    println!("  🔴 Removed:         {} files ({})", diff.removed.len(), format_bytes(diff.removed.iter().map(|f| f.size_bytes).sum()));

    if !diff.added.is_empty() {
        println!("\n🟢 New Files (Sample 10):");
        for f in diff.added.iter().take(10) {
            println!("  + {:<50} {:>10}", f.path, format_bytes(f.size_bytes));
        }
    }

    if !diff.modified.is_empty() {
        println!("\n🟡 Modified Files (Sample 10):");
        for f in diff.modified.iter().take(10) {
            println!("  ~ {:<50} {:>10}", f.path, format_bytes(f.size_bytes));
        }
    }

    if !diff.removed.is_empty() {
        println!("\n🔴 Removed Files (Sample 10):");
        for f in diff.removed.iter().take(10) {
            println!("  - {:<50} {:>10}", f.path, format_bytes(f.size_bytes));
        }
    }

    if diff.added.is_empty() && diff.modified.is_empty() && diff.removed.is_empty() {
        println!("\n✨ 100% In Sync: Device is completely identical to the latest snapshot!");
    }

    Ok(())
}
