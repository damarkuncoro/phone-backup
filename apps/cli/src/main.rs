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

    #[arg(long, default_value = "local")]
    storage: String,

    #[arg(long, env = "S3_BUCKET")]
    s3_bucket: Option<String>,

    #[arg(long, env = "S3_REGION")]
    s3_region: Option<String>,

    #[arg(long, env = "S3_ENDPOINT")]
    s3_endpoint: Option<String>,

    #[arg(long, env = "S3_ACCESS_KEY")]
    s3_access_key: Option<String>,

    #[arg(long, env = "S3_SECRET_KEY")]
    s3_secret_key: Option<String>,

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
        /// Optional snapshot ID to show details
        #[arg(short, long)]
        snapshot: Option<String>,
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
        /// Optional filter pattern (restore only matching files)
        #[arg(short, long)]
        filter: Option<String>,
    },
    /// Verify repository integrity
    Verify {
        /// Optional password if backup is encrypted
        #[arg(short, long)]
        password: Option<String>,
    },
    /// Show repository statistics
    Stats,
    /// Search for files in the repository
    Search {
        /// Query pattern
        query: String,
    },
    /// Direct transfer from one device to another
    Clone {
        /// Source device id
        source: String,
        /// Target device id
        target: String,
    },
    /// List all photos with metadata
    Photos {
        /// Device id
        id: String,
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

    // Initialize storage
    let storage_type = cli.storage.as_str();

    macro_rules! run_with_storage {
        ($storage:expr) => {
            match cli.adapter.as_str() {
                "adb" => {
                    let service = BackupService::new(
                        AdbDeviceAdapter::new(),
                        AdbScannerAdapter::new(),
                        repository,
                        $storage,
                        AdbAppProvider::new(),
                        AdbDataProvider::new(),
                    );
                    execute_command(cli.command, service)
                }
                _ => {
                    let service = BackupService::new(
                        MockDeviceAdapter::default(),
                        MockScannerAdapter::default(),
                        repository,
                        $storage,
                        MockAppProvider,
                        MockDataProvider,
                    );
                    execute_command(cli.command, service)
                }
            }
        };
    }

    match storage_type {
        "s3" => {
            use adapter_opendal::CloudStorage;
            let bucket = cli.s3_bucket.as_deref().unwrap_or("");
            let region = cli.s3_region.as_deref().unwrap_or("us-east-1");
            let endpoint = cli.s3_endpoint.as_deref().unwrap_or("");
            let access = cli.s3_access_key.as_deref().unwrap_or("");
            let secret = cli.s3_secret_key.as_deref().unwrap_or("");

            let storage = CloudStorage::new_s3(bucket, region, endpoint, access, secret)?;
            run_with_storage!(storage)
        }
        _ => {
            let storage = LocalStorage::new("backups")?;
            run_with_storage!(storage)
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
        Commands::Snapshots { id, snapshot } => {
            if let Some(s_id) = snapshot {
                show_snapshot_detail(&service, &s_id)?;
            } else {
                list_snapshots(&service, &id)?;
            }
        }
        Commands::Restore { snapshot_id, target, password, filter } => run_restore(&service, &snapshot_id, &target, password.as_deref(), filter.as_deref())?,
        Commands::Verify { password } => run_verify(&service, password.as_deref())?,
        Commands::Stats => run_stats(&service)?,
        Commands::Search { query } => run_search(&service, &query)?,
        Commands::Clone { source, target } => service.migrate_device(&DeviceId::new(&source), &DeviceId::new(&target))?,
        Commands::Photos { id } => list_photos(&service, &id)?,
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
    if snapshot.total_bytes > 0 {
        let ratio = (snapshot.deduped_bytes as f64 / snapshot.total_bytes as f64) * 100.0;
        println!("Deduplication: {:.1}% ({} bytes saved)", ratio, snapshot.deduped_bytes);
    }
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

fn show_snapshot_detail<D, S, R, T, A, DP>(service: &BackupService<D, S, R, T, A, DP>, snapshot_id: &str) -> Result<()>
where D: ports::DevicePort, S: ports::ScannerPort, R: ports::RepositoryPort, T: ports::StoragePort, A: ports::AppProviderPort, DP: ports::DataProviderPort
{
    use domain::SnapshotId;
    let s_id = SnapshotId(snapshot_id.to_string());
    let snapshot = service.get_snapshot(&s_id)?
        .ok_or_else(|| anyhow::anyhow!("Snapshot not found"))?;

    println!("Snapshot Details");
    println!("----------------");
    println!("ID:          {}", snapshot.id.0);
    println!("Device ID:   {}", snapshot.device_id.0);
    println!("Started:     {}", snapshot.started_at.format("%Y-%m-%d %H:%M:%S"));
    println!("Finished:    {}", snapshot.finished_at.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or("-".into()));
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

fn run_restore<D, S, R, T, A, DP>(service: &BackupService<D, S, R, T, A, DP>, snapshot_id: &str, target: &str, password: Option<&str>, filter: Option<&str>) -> Result<()>
where D: ports::DevicePort, S: ports::ScannerPort, R: ports::RepositoryPort, T: ports::StoragePort, A: ports::AppProviderPort, DP: ports::DataProviderPort
{
    use domain::SnapshotId;
    if let Some(f) = filter {
        println!("Restoring files matching '{}' from snapshot {} to {}...", f, snapshot_id, target);
    } else {
        println!("Restoring snapshot {} to {}...", snapshot_id, target);
    }
    service.perform_restore(&SnapshotId(snapshot_id.to_string()), target, password, filter)?;
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

fn run_stats<D, S, R, T, A, DP>(service: &BackupService<D, S, R, T, A, DP>) -> Result<()>
where D: ports::DevicePort, S: ports::ScannerPort, R: ports::RepositoryPort, T: ports::StoragePort, A: ports::AppProviderPort, DP: ports::DataProviderPort
{
    let stats = service.get_storage_stats()?;
    println!("Repository Statistics");
    println!("---------------------");
    println!("Devices tracked:    {}", stats.total_devices);
    println!("Total snapshots:    {}", stats.total_snapshots);
    println!("Total data backed:  {:.2} GB", stats.total_logical_bytes as f64 / 1024.0 / 1024.0 / 1024.0);
    println!("Storage saved:      {:.2} GB ({:.1}%)",
        stats.total_deduped_bytes as f64 / 1024.0 / 1024.0 / 1024.0,
        stats.efficiency_ratio()
    );
    println!("Physical storage:   {:.2} GB (estimated)",
        (stats.total_logical_bytes - stats.total_deduped_bytes) as f64 / 1024.0 / 1024.0 / 1024.0
    );
    Ok(())
}

fn run_search<D, S, R, T, A, DP>(service: &BackupService<D, S, R, T, A, DP>, query: &str) -> Result<()>
where D: ports::DevicePort, S: ports::ScannerPort, R: ports::RepositoryPort, T: ports::StoragePort, A: ports::AppProviderPort, DP: ports::DataProviderPort
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

fn list_photos<D, S, R, T, A, DP>(service: &BackupService<D, S, R, T, A, DP>, id: &str) -> Result<()>
where D: ports::DevicePort, S: ports::ScannerPort, R: ports::RepositoryPort, T: ports::StoragePort, A: ports::AppProviderPort, DP: ports::DataProviderPort
{
    let device_id = DeviceId::new(id);
    let files = service.scan_device(&device_id)?;
    println!("Photo Gallery for device {}\n", id);
    println!("{:<30} {:<15} {:<15} {}", "FILE", "CAMERA", "TAKEN AT", "LOCATION");
    println!("{}", "-".repeat(90));

    for f in files {
        if f.mime_type.starts_with("image/") {
            let camera = f.media_info.as_ref()
                .and_then(|m| m.camera_model.clone())
                .unwrap_or("-".into());
            let taken = f.media_info.as_ref()
                .and_then(|m| m.taken_at)
                .map(|t| t.format("%Y-%m-%d").to_string())
                .unwrap_or("-".into());
            let loc = f.media_info.as_ref()
                .and_then(|m| m.latitude.zip(m.longitude))
                .map(|(lat, lon)| format!("{:.4}, {:.4}", lat, lon))
                .unwrap_or("-".into());

            println!("{:<30} {:<15} {:<15} {}", f.name, camera, taken, loc);
        }
    }
    Ok(())
}
