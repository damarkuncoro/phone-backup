use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "phone-backup", about = "Backup platform for Android devices")]
pub struct Cli {
    #[arg(short, long, default_value = "mock")]
    pub adapter: String,

    #[arg(long, default_value = "local")]
    pub storage: String,

    #[arg(long, env = "S3_BUCKET")]
    pub s3_bucket: Option<String>,

    #[arg(long, env = "S3_REGION")]
    pub s3_region: Option<String>,

    #[arg(long, env = "S3_ENDPOINT")]
    pub s3_endpoint: Option<String>,

    #[arg(long, env = "S3_ACCESS_KEY")]
    pub s3_access_key: Option<String>,

    #[arg(long, env = "S3_SECRET_KEY")]
    pub s3_secret_key: Option<String>,

    /// Public key for asymmetric encryption
    #[arg(long, env = "PB_PUBKEY")]
    pub pubkey: Option<String>,

    /// Private key for asymmetric decryption
    #[arg(long, env = "PB_PRIVKEY")]
    pub privkey: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a new key pair for asymmetric encryption
    Keygen,
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
        /// Target directory (defaults to restored_<DEVICE>_<DATE>)
        #[arg(short, long)]
        target: Option<String>,
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
    /// Clean up orphaned objects in the repository
    Gc,
    /// Run system diagnostic to troubleshoot issues
    Doctor,
    /// Search for files in the repository
    Search {
        /// Query pattern
        query: String,
    },
    /// Search for contacts globally
    Contacts {
        /// Query (name, phone, email)
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
pub enum ScheduleCommands {
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
