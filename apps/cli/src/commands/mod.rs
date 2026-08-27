pub mod backup;
pub mod command_trait;
pub mod device;
pub mod restore;
pub mod schedule;
pub mod stats;

#[allow(unused_imports)]
pub use command_trait::CliCommand;

use crate::cli::Commands;
use anyhow::Result;
use application::BackupService;

pub fn execute_command<D, S, R, T, A, DP>(
    command: Commands,
    service: BackupService<D, S, R, T, A, DP>,
) -> Result<()>
where
    D: ports::DevicePort,
    S: ports::ScannerPort,
    R: ports::RepositoryPort,
    T: ports::StoragePort,
    A: ports::AppProviderPort,
    DP: ports::DataProviderPort,
{
    match command {
        Commands::Devices => device::print_devices(&service)?,
        Commands::DeviceInfo { id } => device::print_device_info(&service, &id)?,
        Commands::Scan { id } => device::scan_device(&service, &id)?,
        Commands::Apps { id } => device::list_apps(&service, &id)?,
        Commands::Backup {
            id,
            repo: _,
            password,
            include,
            exclude,
        } => backup::run_backup(&service, &id, password.as_deref(), include, exclude)?,
        Commands::Snapshots { id, snapshot } => {
            if let Some(s_id) = snapshot {
                backup::show_snapshot_detail(&service, &s_id)?;
            } else {
                backup::list_snapshots(&service, &id)?;
            }
        }
        Commands::Restore {
            snapshot_id,
            target,
            password,
            filter,
        } => restore::run_restore(&service, &snapshot_id, &target, password.as_deref(), filter.as_deref())?,
        Commands::Verify { password } => restore::run_verify(&service, password.as_deref())?,
        Commands::Stats => stats::run_stats(&service)?,
        Commands::Search { query } => stats::run_search(&service, &query)?,
        Commands::Clone { source, target } => stats::run_clone(&service, &source, &target)?,
        Commands::Photos { id } => device::list_photos(&service, &id)?,
        Commands::Schedule { command } => schedule::handle_schedule(&service, command)?,
    }
    Ok(())
}
