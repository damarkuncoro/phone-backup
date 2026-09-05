use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a new key pair for asymmetric encryption
    Keygen,
    /// Export zero-knowledge printable emergency recovery kit (HTML)
    RecoveryKit {
        /// Target HTML file output path
        #[arg(short, long, default_value = "emergency_recovery_kit.html")]
        output: String,
    },
    /// List connected devices
    Devices,
    /// Show detailed info + capability matrix for one device
    DeviceInfo {
        /// Device id, e.g. A1B2C3D4
        id: String,
    },
    /// Scan device filesystem with specialist categorization and filtering
    Scan {
        /// Device id, e.g. A1B2C3D4
        id: String,
        /// Optional category filter (e.g. photos, videos, whatsapp, documents, apks, audio)
        #[arg(short, long)]
        category: Option<String>,
        /// Optional minimum size filter in bytes
        #[arg(long)]
        min_size: Option<u64>,
        /// Optional maximum size filter in bytes
        #[arg(long)]
        max_size: Option<u64>,
        /// Sort results by: name, size, or date
        #[arg(short, long, default_value = "name")]
        sort: String,
        /// Maximum number of discovered files to display
        #[arg(short, long, default_value_t = 20)]
        limit: usize,
    },
    /// Compare current device state with the latest backup snapshot
    Diff {
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
        /// Compression mode: auto, fast, max, none
        #[arg(short = 'c', long, default_value = "auto")]
        compression: String,
        /// Dynamic CDC medium profile: auto, local, wifi, cloud, thermal
        #[arg(short = 'm', long, default_value = "auto")]
        medium: String,
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
    /// Search for SMS messages globally
    Sms {
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
    /// Export structured data (contacts, SMS, call logs)
    Export(crate::commands::export::ExportArgs),
    /// Audit application permissions and security risk
    Audit(crate::commands::audit::AuditArgs),
    /// WhatsApp backup tools and HTML offline archive
    Whatsapp(crate::commands::whatsapp::WhatsAppArgs),
    /// Audio inspection, tags, and waveform peaks
    Audio(crate::commands::audio::AudioArgs),
    /// Document intelligence, PDF/Office inspection, and snippet preview
    Documents(crate::commands::documents::DocumentsArgs),
    /// Video inspection, container formats, and quality resolution tiering
    Videos(crate::commands::videos::VideosArgs),
    /// Call log history inspection, talk time analytics, and export
    Calls(crate::commands::calls::CallsArgs),
    /// Calendar schedule inspection, recurrence, conflict detection, and export
    Calendar(crate::commands::calendar::CalendarArgs),
    /// Telegram backup discovery, voice/video note classification, and offline HTML export
    Telegram(crate::commands::telegram::TelegramArgs),
    /// Notes, checklists, memos, and Google Keep archive backup
    Notes(crate::commands::notes::NotesArgs),
    /// Wi-Fi credentials, config store parser, and QR code connection generator
    Wifi(crate::commands::wifi::WifiArgs),
    /// Browser bookmarks, reading lists, and universal HTML/Markdown exporter
    Bookmarks(crate::commands::bookmarks::BookmarksArgs),
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
