pub mod backup;
pub mod command_trait;
pub mod device;
pub mod doctor;
pub mod restore;
pub mod schedule;
pub mod stats;

#[allow(unused_imports)]
pub use command_trait::CliCommand;

use crate::cli::{Cli, Commands};
use anyhow::Result;
use application::BackupService;
use domain::EncryptionMode;

pub fn execute_command<D, S, R, T, A, DP, P>(
    cli: Cli,
    service: BackupService<D, S, R, T, A, DP, P>,
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
    // Determine encryption mode from global CLI flags
    let encryption = if let Some(pk) = cli.pubkey {
        EncryptionMode::PublicKey(pk)
    } else if let Some(sk) = cli.privkey {
        EncryptionMode::PublicKey(sk) // Use for decryption
    } else {
        EncryptionMode::None
    };

    match cli.command {
        Commands::Keygen => {
            let (secret, public) = application::EncryptionEngine::generate_keypair();
            println!("New Key Pair Generated!");
            println!("-----------------------");
            println!("Public Key (PB_PUBKEY):  {}", public);
            println!("Secret Key (PB_PRIVKEY): {}", secret);
            println!("\nKeep your Secret Key safe! You will need it to restore your backups.");
        }
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
        } => {
            let enc = if let Some(pwd) = password {
                EncryptionMode::Password(pwd)
            } else {
                encryption
            };
            backup::run_backup(&service, &id, enc, include, exclude)?
        }
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
        } => {
            let enc = if let Some(pwd) = password {
                EncryptionMode::Password(pwd)
            } else {
                encryption
            };
            restore::run_restore(&service, &snapshot_id, target, enc, filter.as_deref())?
        }
        Commands::Verify { password } => {
            let enc = if let Some(pwd) = password {
                EncryptionMode::Password(pwd)
            } else {
                encryption
            };
            restore::run_verify(&service, enc)?
        }
        Commands::Stats => stats::run_stats(&service)?,
        Commands::Gc => {
            println!("🧹 Running Garbage Collection...");
            let deleted = service.garbage_collect()?;
            println!("✅ Done. Removed {} orphaned objects.", deleted);
        }
        Commands::Doctor => doctor::run_doctor(&service)?,
        Commands::Search { query } => stats::run_search(&service, &query)?,
        Commands::Clone { source, target } => stats::run_clone(&service, &source, &target)?,
        Commands::Photos { id } => device::list_photos(&service, &id)?,
        Commands::Schedule { command } => {
            // Schedules might need encryption stored? For now pass None or default.
            schedule::handle_schedule(&service, command, EncryptionMode::None)?
        }
    }
    Ok(())
}
