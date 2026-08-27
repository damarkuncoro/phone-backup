//! Composition root.
//!
//! This is the ONLY place in the workspace that is allowed to know
//! about a concrete adapter (`adapter_mock::MockDeviceAdapter` today,
//! `adapter_adb::AdbDeviceAdapter` from Phase 02 onward) and wire it
//! into `application::BackupService`. Everything above this file
//! programs against the `ports::DevicePort` trait only.

use adapter_database_sqlite::SqliteRepository;
use adapter_filesystem::LocalStorage;
use adapter_mock::{MockDeviceAdapter, MockScannerAdapter, MockAppProvider, MockDataProvider};
use adapter_adb::{AdbDeviceAdapter, AdbScannerAdapter, AdbAppProvider, AdbDataProvider};
use anyhow::Result;
use application::BackupService;
use clap::{Parser, Subcommand};
use domain::DeviceId;

#[derive(Parser)]
#[command(name = "phone-backup", about = "Backup platform for Android devices")]
struct Cli {
    #[arg(short, long, default_value = "mock")]
    adapter: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List connected devices
    Devices,
    /// Show detailed info + capability matrix for one device
    DeviceInfo {
        /// Device id, e.g. A1B2C3D4
        id: String,
    },
    /// Scan device filesystem
    Scan {
        /// Device id, e.g. A1B2C3D4
        id: String,
    },
    /// List installed applications
    Apps {
        /// Device id, e.g. A1B2C3D4
        id: String,
    },
    /// Run a backup for a device
    Backup {
        /// Device id, e.g. A1B2C3D4
        id: String,
        /// Target repository path
        #[arg(short, long, default_value = "backups")]
        repo: String,
        /// Optional password for encryption
        #[arg(short, long)]
        password: Option<String>,
        /// Folders to include (e.g. /sdcard/DCIM)
        #[arg(short, long)]
        include: Option<Vec<String>>,
        /// Patterns to exclude (e.g. *.tmp)
        #[arg(short, long)]
        exclude: Option<Vec<String>>,
    },
    /// List snapshots for a device
    Snapshots {
        /// Device id, e.g. A1B2C3D4
        id: String,
    },
    /// Restore a snapshot to a local directory
    Restore {
        /// Snapshot ID
        snapshot_id: String,
        /// Target directory
        #[arg(short, long, default_value = "restore")]
        target: String,
        /// Optional password for encrypted backups
        #[arg(short, long)]
        password: Option<String>,
    },
    /// Verify repository integrity
    Verify {
        /// Optional password if backup is encrypted
        #[arg(short, long)]
        password: Option<String>,
    },
    /// Manage backup schedules
    Schedule {
        #[command(subcommand)]
        command: ScheduleCommands,
    },
}

#[derive(Subcommand)]
enum ScheduleCommands {
    /// Add a new schedule
    Add {
        /// Device id
        id: String,
        /// Frequency (hourly, daily, weekly)
        #[arg(short, long, default_value = "daily")]
        frequency: String,
    },
    /// List all schedules
    List,
    /// Run all pending scheduled backups
    Run {
        /// Optional password for encrypted backups
        #[arg(short, long)]
        password: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize repository (metadata)
    let repository = SqliteRepository::new("backup.db")?;

    match cli.adapter.as_str() {
        "adb" => {
            let storage = LocalStorage::new("backups")?;
            let service = BackupService::new(
                AdbDeviceAdapter::new(),
                AdbScannerAdapter::new(),
                repository,
                storage,
                AdbAppProvider::new(),
                AdbDataProvider::new(),
            );
            execute_command(cli.command, service)
        }
        _ => {
            let storage = LocalStorage::new("backups")?;
            let service = BackupService::new(
                MockDeviceAdapter::default(),
                MockScannerAdapter::default(),
                repository,
                storage,
                MockAppProvider,
                MockDataProvider,
            );
            execute_command(cli.command, service)
        }
    }
}

fn execute_command<D, S, R, T, A, DP>(
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
        Commands::Devices => print_devices(&service)?,
        Commands::DeviceInfo { id } => print_device_info(&service, &id)?,
        Commands::Scan { id } => scan_device(&service, &id)?,
        Commands::Apps { id } => list_apps(&service, &id)?,
        Commands::Backup { id, repo: _, password, include, exclude } => run_backup(&service, &id, password.as_deref(), include, exclude)?,
        Commands::Snapshots { id } => list_snapshots(&service, &id)?,
        Commands::Restore { snapshot_id, target, password } => run_restore(&service, &snapshot_id, &target, password.as_deref())?,
        Commands::Verify { password } => run_verify(&service, password.as_deref())?,
        Commands::Schedule { command } => {
            match command {
                ScheduleCommands::Add { id, frequency } => {
                    use domain::ScheduleFrequency;
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
                        let last_run = s.last_run_at
                            .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
                            .unwrap_or("Never".into());
                        println!("{:<15} {:<10?} {:<20}", s.device_id.0, s.frequency, last_run);
                    }
                }
                ScheduleCommands::Run { password } => {
                    service.run_pending_backups(password.as_deref())?;
                }
            }
        }
    }
    Ok(())
}

fn print_devices<D, S, R, T, A, DP>(service: &BackupService<D, S, R, T, A, DP>) -> Result<()>
where D: ports::DevicePort, S: ports::ScannerPort, R: ports::RepositoryPort, T: ports::StoragePort, A: ports::AppProviderPort, DP: ports::DataProviderPort
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

fn print_device_info<D, S, R, T, A, DP>(service: &BackupService<D, S, R, T, A, DP>, id: &str) -> Result<()>
where D: ports::DevicePort, S: ports::ScannerPort, R: ports::RepositoryPort, T: ports::StoragePort, A: ports::AppProviderPort, DP: ports::DataProviderPort
{
    let device_id = DeviceId::new(id);
    let device = service.device_info(&device_id)?;
    let capabilities = service.device_capabilities(&device_id)?;

    println!("Device");
    println!("├── id: {}", device.id);
    println!("├── manufacturer: {}", device.manufacturer);
    println!("├── model: {}", device.model);
    println!("├── android_version: {}", device.os_version);
    println!("├── storage: {:.1}% used ({} / {} bytes)", device.storage_used_percent(), device.storage_used_bytes, device.storage_total_bytes);
    println!("└── capabilities:");
    for (capability, status) in capabilities.iter() {
        println!("      {:?} -> {:?}", capability, status);
    }
    Ok(())
}

fn scan_device<D, S, R, T, A, DP>(service: &BackupService<D, S, R, T, A, DP>, id: &str) -> Result<()>
where D: ports::DevicePort, S: ports::ScannerPort, R: ports::RepositoryPort, T: ports::StoragePort, A: ports::AppProviderPort, DP: ports::DataProviderPort
{
    let device_id = DeviceId::new(id);
    println!("Scanning device {}...", id);
    let files = service.scan_device(&device_id)?;
    println!("\nFound {} files:", files.len());
    for f in files {
        println!("{:<40} {:>10} bytes  {}", f.path, f.size_bytes, f.modified_at.format("%Y-%m-%d %H:%M:%S"));
    }
    Ok(())
}

fn list_apps<D, S, R, T, A, DP>(service: &BackupService<D, S, R, T, A, DP>, id: &str) -> Result<()>
where D: ports::DevicePort, S: ports::ScannerPort, R: ports::RepositoryPort, T: ports::StoragePort, A: ports::AppProviderPort, DP: ports::DataProviderPort
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

fn run_backup<D, S, R, T, A, DP>(
    service: &BackupService<D, S, R, T, A, DP>,
    id: &str,
    password: Option<&str>,
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
{
    let device_id = DeviceId::new(id);
    println!("Starting backup for device {}...", id);

    let mut policy = domain::BackupPolicy::default();
    if let Some(inc) = include {
        policy.include_paths = inc;
    }
    if let Some(exc) = exclude {
        policy.exclude_patterns.extend(exc);
    }

    let snapshot = service.perform_backup(&device_id, password, Some(policy))?;
    println!("\nBackup completed successfully!");
    println!("Snapshot ID: {}", snapshot.id.0);
    println!("Files:       {}", snapshot.total_files);
    println!("Total Size:  {} bytes", snapshot.total_bytes);
    Ok(())
}

fn list_snapshots<D, S, R, T, A, DP>(service: &BackupService<D, S, R, T, A, DP>, id: &str) -> Result<()>
where D: ports::DevicePort, S: ports::ScannerPort, R: ports::RepositoryPort, T: ports::StoragePort, A: ports::AppProviderPort, DP: ports::DataProviderPort
{
    let device_id = DeviceId::new(id);
    let snapshots = service.list_snapshots(&device_id)?;
    println!("Snapshots for device {}\n", id);
    println!("{:<40} {:<20} {:<10} {:>10}", "ID", "STARTED", "STATUS", "FILES");
    println!("{}", "-".repeat(85));
    for s in snapshots {
        println!("{:<40} {:<20} {:<10} {:>10}", s.id.0, s.started_at.format("%Y-%m-%d %H:%M:%S"), format!("{:?}", s.status), s.total_files);
    }
    Ok(())
}

fn run_restore<D, S, R, T, A, DP>(service: &BackupService<D, S, R, T, A, DP>, snapshot_id: &str, target: &str, password: Option<&str>) -> Result<()>
where D: ports::DevicePort, S: ports::ScannerPort, R: ports::RepositoryPort, T: ports::StoragePort, A: ports::AppProviderPort, DP: ports::DataProviderPort
{
    use domain::SnapshotId;
    println!("Restoring snapshot {} to {}...", snapshot_id, target);
    service.perform_restore(&SnapshotId(snapshot_id.to_string()), target, password)?;
    println!("\nRestore completed successfully!");
    Ok(())
}

fn run_verify<D, S, R, T, A, DP>(service: &BackupService<D, S, R, T, A, DP>, password: Option<&str>) -> Result<()>
where D: ports::DevicePort, S: ports::ScannerPort, R: ports::RepositoryPort, T: ports::StoragePort, A: ports::AppProviderPort, DP: ports::DataProviderPort
{
    println!("Verifying repository integrity...");
    let report = service.verify_repository(password)?;
    println!("\nRepository Verification Report");
    println!("------------------------------");
    println!("Total files in index:  {}", report.total_files);
    println!("Verified objects:      {}", report.verified_files);
    println!("Missing objects:       {}", report.missing_objects.len());
    println!("Corrupted files:       {}", report.corrupted_files.len());
    if report.is_healthy() { println!("\nSTATUS: HEALTHY"); } else { println!("\nSTATUS: UNHEALTHY"); }
    Ok(())
}
