pub mod audio;
pub mod audit;
pub mod backup;
pub mod bookmarks;
pub mod calendar;
pub mod calls;
pub mod command_trait;
pub mod device;
pub mod diff;
pub mod doctor;
pub mod documents;
pub mod export;
pub mod notes;
pub mod recovery_kit;
pub mod restore;
pub mod scan;
pub mod schedule;
pub mod stats;
pub mod telegram;
pub mod videos;
pub mod whatsapp;
pub mod wifi;

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
    let encryption = if let Some(pk) = cli.pubkey {
        EncryptionMode::PublicKey(pk)
    } else if let Some(sk) = cli.privkey {
        EncryptionMode::PublicKey(sk)
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
        Commands::RecoveryKit { output } => {
            recovery_kit::generate_recovery_kit(&output)?;
        }
        Commands::Devices => device::print_devices(&service)?,
        Commands::DeviceInfo { id } => device::print_device_info(&service, &id)?,
        Commands::Scan {
            id,
            category,
            min_size,
            max_size,
            sort,
            limit,
        } => scan::handle_scan(
            &service,
            &id,
            category,
            min_size,
            max_size,
            &sort,
            limit,
        )?,
        Commands::Diff { id } => diff::handle_diff(&service, &id)?,
        Commands::Apps { id } => device::list_apps(&service, &id)?,
        Commands::Backup {
            id,
            repo: _,
            password,
            include,
            exclude,
            compression,
            medium,
        } => {
            let enc = if let Some(pwd) = password {
                EncryptionMode::Password(pwd)
            } else {
                encryption
            };
            backup::run_backup(&service, &id, enc, include, exclude, &compression, &medium)?
        }
        Commands::Snapshots { id, snapshot } => {
            if let Some(s_id) = snapshot.as_deref() {
                backup::show_snapshot_detail(&service, s_id)?;
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
        Commands::Contacts { query } => stats::run_contact_search(&service, &query)?,
        Commands::Sms { query } => stats::run_sms_search(&service, &query)?,
        Commands::Clone { source, target } => stats::run_clone(&service, &source, &target)?,
        Commands::Photos { id } => device::list_photos(&service, &id)?,
        Commands::Schedule { command } => {
            schedule::handle_schedule(&service, command, EncryptionMode::None)?
        }
        Commands::Export(args) => export::handle_export(args, &service)?,
        Commands::Audit(args) => audit::handle_audit(args)?,
        Commands::Whatsapp(args) => whatsapp::handle_whatsapp(args)?,
        Commands::Audio(args) => audio::handle_audio(args)?,
        Commands::Documents(args) => documents::handle_documents(args, &service)?,
        Commands::Videos(args) => videos::handle_videos(args, &service)?,
        Commands::Calls(args) => calls::handle_calls(args, &service)?,
        Commands::Calendar(args) => calendar::handle_calendar(args, &service)?,
        Commands::Telegram(args) => telegram::handle_telegram(args, &service)?,
        Commands::Notes(args) => notes::handle_notes(args, &service)?,
        Commands::Wifi(args) => wifi::handle_wifi(args, &service)?,
        Commands::Bookmarks(args) => bookmarks::handle_bookmarks(args, &service)?,
    }
    Ok(())
}
