# phone-backup 📱

A high-performance, secure, and professional Android backup platform written in Rust.

Designed as a **Backup Engine**, this tool handles discovery, indexing, snapshots, encryption, deduplication, and restoration. It is built using **Clean Architecture** and **Hexagonal Architecture** principles, making the backup logic independent of the transport layer (ADB, MTP, etc.).

## 🚀 Features

- **Multi-Transport**: Supports real Android devices via **ADB** and simulated environments via **Mock** adapters.
- **Intelligent Engine**:
    - **Fast Incremental**: Only copies new or changed files based on size and mtime.
    - **Deduplication**: Content-addressed storage (SHA-256) ensures identical files are stored only once.
    - **Zstd Compression**: High-performance compression for text and data files.
    - **Military-Grade Encryption**: AES-256-GCM authenticated encryption with Argon2 key derivation.
- **Deep Data Support**:
    - **Filesystem**: Recursive scanning of internal storage (`/sdcard`).
    - **Applications**: Metadata tracking and **APK extraction**.
    - **Structured Data**: Backup of **Contacts**, **SMS**, and **Call Logs** (exported as encrypted JSON).
    - **Media Intelligence**: Automatic **EXIF metadata** extraction from photos.
- **Automation & Management**:
    - **Backup Scheduler**: Automate backups (Hourly, Daily, Weekly).
    - **Retention Policy**: Automatically cleans up old snapshots to save space.
    - **Backup Policy**: Fine-grained `include`/`exclude` filtering for folders and file patterns.
- **Safety First**: Pre-backup disk space verification on the host PC.

## 🛠 Project Structure

```text
phone-backup/
├── apps/
│   └── cli/                # Command-line interface (Composition Root)
├── core/
│   ├── domain/             # Entities (Device, Snapshot, FileEntry, etc.)
│   ├── application/        # Use cases (BackupService)
│   └── ports/              # Interfaces (Seams for dependency inversion)
├── adapters/
│   ├── adb/                # Real Android ADB implementation
│   ├── mock/               # Simulation for testing and development
│   └── filesystem/         # Local repository storage
└── infrastructure/
    └── database-sqlite/    # Persistent index and metadata storage
```

## 🚦 Getting Started

### Prerequisites

- **Rust**: Latest stable version.
- **ADB**: Android Debug Bridge installed and available in your `PATH`.

### Installation

```bash
git clone https://github.com/yourusername/phone-backup.git
cd phone-backup
cargo build --release
```

## 📖 Usage

### Device Management

```bash
# List all connected devices
phone-backup --adapter adb devices

# Show detailed device info and storage status
phone-backup --adapter adb device-info <DEVICE_ID>
```

### Backup Operations

```bash
# Perform a full/incremental backup
phone-backup --adapter adb backup <DEVICE_ID>

# Backup with encryption
phone-backup --adapter adb backup <DEVICE_ID> --password "your-secret"

# Backup specific folders only
phone-backup --adapter adb backup <DEVICE_ID> --include /sdcard/DCIM --include /sdcard/WhatsApp
```

### Restoration & Verification

```bash
# List all snapshots for a device
phone-backup snapshots <DEVICE_ID>

# Restore a snapshot to your computer
phone-backup restore <SNAPSHOT_ID> --target ./my_recovered_data --password "your-secret"

# Verify repository integrity
phone-backup verify --password "your-secret"
```

### Automation (Scheduler)

```bash
# Add a daily backup schedule
phone-backup schedule add <DEVICE_ID> --frequency daily

# Run all due backups (add this to your cron/task scheduler)
phone-backup schedule run
```

## 🛡 Security

- No raw data is ever stored without encryption if a password is provided.
- Passwords are never stored; they are used only for key derivation at runtime.
- Deduplication occurs at the encrypted blob level to ensure privacy.

## 📄 License

MIT
