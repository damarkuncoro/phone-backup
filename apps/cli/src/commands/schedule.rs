use crate::cli::ScheduleCommands;
use anyhow::Result;
use application::BackupService;
use domain::{EncryptionMode, ScheduleFrequency};

pub fn handle_schedule<D, S, R, T, A, DP, P>(
    service: &BackupService<D, S, R, T, A, DP, P>,
    command: ScheduleCommands,
    encryption: EncryptionMode,
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
    match command {
        ScheduleCommands::Add { id, frequency } => {
            let freq = match frequency.to_lowercase().as_str() {
                "hourly" => ScheduleFrequency::Hourly,
                "weekly" => ScheduleFrequency::Weekly,
                _ => ScheduleFrequency::Daily,
            };
            service.add_schedule(domain::DeviceId(id), freq)?;
            println!("Schedule added.");
        }
        ScheduleCommands::List => {
            let schedules = service.list_schedules()?;
            println!("{:<15} {:<10} {:<20}", "DEVICE ID", "FREQ", "LAST RUN");
            println!("{}", "-".repeat(45));
            for s in schedules {
                let last_run = s
                    .last_run_at
                    .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or("Never".into());
                println!(
                    "{:<15} {:<10?} {:<20}",
                    s.device_id.0, s.frequency, last_run
                );
            }
        }
        ScheduleCommands::Run { password } => {
            let enc = if let Some(pwd) = password {
                EncryptionMode::Password(pwd)
            } else {
                encryption
            };
            service.run_pending_backups(enc)?;
        }
    }
    Ok(())
}
