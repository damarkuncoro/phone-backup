# phone-backup 📱

A high-performance, secure, and professional Android backup platform written in Rust.

**phone-backup** is a comprehensive backup engine designed with **Clean Architecture**, **Hexagonal Architecture**, and **SOLID Design Patterns** (Builder, Factory, Strategy). It handles device discovery, intelligent indexing, versioned snapshots, military-grade encryption, and storage-efficient deduplication.

---

## 🚀 Features

### 🧠 Intelligent Engine
- **Fast Incremental Backup**: Scans device state and only transfers new or modified files based on size and mtime.
- **Failure Recovery (Resume)**: Automatically detects interrupted backups and resumes from the last successful file.
- **Content-Addressed Storage (Deduplication)**: Ensures identical files (across different folders, snapshots, or even devices) are stored only once using SHA-256 keying.
- **Zstd Compression**: High-performance compression for logs, JSON, and text.
- **Military-Grade Security**: AES-256-GCM authenticated encryption with keys derived using Argon2.

### 📱 Deep Android Support
- **Native ADB Integration**: Reliable communication with real Android devices via `AdbClient`.
- **Direct Migration (Cloning)**: Transfer apps and files directly from HP A to HP B with a single command.
- **App Management**: Tracks installed applications and performs automatic **APK extraction**.
- **Structured Data**: Backs up **Contacts**, **SMS**, and **Call Logs** into secure JSON blobs.
- **Media Intelligence**: Extracts **EXIF metadata** (Resolution, Camera, GPS) from photos.

### 🛠 Management & UX
- **Interactive UI**: Progress bars (via `indicatif`) for long-running operations.
- **Global Search**: Search for any file across all devices and snapshots in the repository.
- **Retention Policies**: Automatically prunes old snapshots to save space.
- **Storage Statistics**: Detailed reports on deduplication efficiency and physical disk usage.

---

## 🏗 Architecture & Design Patterns

The project follows strict **Clean Architecture** and **Hexagonal Architecture** with modular design patterns:

```text
phone-backup/
├── apps/
│   └── cli/                # Composition Root (Cli, Commands, StorageFactory)
├── core/
│   ├── domain/             # Core Entities (Device, Snapshot, File, BackupPolicyBuilder)
│   ├── application/        # Use Cases & Orchestration (BackupService submodules & ObjectStoreKey)
│   └── ports/              # Interface definitions (StoragePort, DevicePort, RepositoryPort)
├── adapters/
│   ├── adb/                # Real Android ADB communication (AdbClient, Device, Scanner, App, Data)
│   ├── mock/               # Simulation adapters for dev/test (Device, Scanner, App, Data)
│   └── filesystem/         # Local object storage
└── infrastructure/
    └── database-sqlite/    # Persistent SQLite catalog (Schema, Device, File, Snapshot, App, Mappers)
```

### 🧩 Applied Design Patterns
- **Builder Pattern**: Used in `BackupPolicy::builder()` (`core/domain/src/policy.rs`) for fluent, step-by-step policy construction (`.include(...)`, `.exclude(...)`).
- **Factory Pattern**: Used in `StorageFactory::create_storage(&cli)` (`apps/cli/src/factory.rs`) for creating `Box<dyn StoragePort>` dynamically (`"local"` vs `"s3"`).
- **Adapter / Hexagonal Pattern**: Decouples application logic from external I/O using trait ports (`DevicePort`, `ScannerPort`, `RepositoryPort`, `StoragePort`, `AppProviderPort`, `DataProviderPort`).
- **Content-Addressable Storage (ObjectStoreKey)**: Centralizes 2-level content-addressable storage paths (`objects/ab/cd/...`) in `core/application/src/object_store.rs`.
- **Row Mapper Pattern**: Centralizes database row parsing into domain models in `infrastructure/database-sqlite/src/mappers.rs`.

---

## 🚦 Getting Started

### Prerequisites
- **Rust**: Latest stable toolchain.
- **ADB**: Android Debug Bridge installed and in your `PATH`.

### Installation
```bash
# Clone source repository
git clone https://github.com/damarkuncoro/phone-backup.git
cd phone-backup

# Build release binary
cargo build --release
```

---

## 📖 Usage Guide

### 1. Device Discovery
```bash
cargo run -- --adapter adb devices
```

### 2. Backup & Migration
```bash
# Backup to Local Storage
cargo run -- --adapter adb backup <DEVICE_ID>

# Backup to Cloud (S3/R2/MinIO) using StorageFactory
cargo run -- --storage s3 \
  --s3-bucket my-backup \
  --s3-region auto \
  --s3-endpoint https://<id>.r2.cloudflarestorage.com \
  --s3-access-key <key> \
  --s3-secret-key <secret> \
  backup <DEVICE_ID>

# Direct Device-to-Device Cloning
cargo run -- --adapter adb clone <SOURCE_ID> <TARGET_ID>
```

### 3. Restore & Analysis
```bash
# Selective Restore (Filter by keyword)
cargo run -- restore <SNAPSHOT_ID> --target ./restore --filter "WhatsApp"

# View Photo Gallery with GPS/Camera info
cargo run -- photos <DEVICE_ID>

# Search for a file anywhere in the repository
cargo run -- search "resume.pdf"

# View storage efficiency report
cargo run -- stats
```

---

## 🧪 Testing

Run all unit and integration tests across all workspace crates:
```bash
cargo test
```

---

## 🛡 Security & Privacy
- **Zero-Knowledge Storage**: No plain-text data is stored if encryption is enabled.
- **Authenticated Encryption**: AES-256-GCM tags ensure data integrity.
- **Metadata Protection**: File catalogs are stored in a local SQLite database with strict permissions.

## 📄 License
MIT

---
*Developed with ❤️ in Rust for the Android Community.*
