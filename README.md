# phone-backup 📱

A high-performance, secure, and professional Android backup platform written in Rust.

**phone-backup** is more than just a file copying tool; it's a comprehensive backup engine designed with **Clean Architecture** and **Hexagonal Architecture** principles. It handles device discovery, intelligent indexing, versioned snapshots, military-grade encryption, and storage-efficient deduplication.

---

## 🚀 Features

### 🧠 Intelligent Engine
- **Fast Incremental Backup**: Scans device state and only transfers new or modified files based on size and mtime.
- **Content-Addressed Storage (Deduplication)**: Ensures identical files (across different folders, snapshots, or even devices) are stored only once.
- **Zstd Compression**: High-performance compression for logs, JSON, and text.
- **Military-Grade Security**: AES-256-GCM authenticated encryption with keys derived using Argon2.

### 📱 Deep Android Support
- **Native ADB Integration**: Reliable communication with real Android devices.
- **Direct Migration (Cloning)**: Transfer apps and files directly from HP A to HP B with a single command.
- **App Management**: Tracks installed applications and performs automatic **APK extraction**.
- **Structured Data**: Backs up **Contacts**, **SMS**, and **Call Logs** into secure JSON blobs.
- **Media Intelligence**: Extracts **EXIF metadata** (Resolution, Camera, GPS) from photos.

### 🛠 Management & UX
- **Interactive UI**: Beautiful progress bars (via `indicatif`) for long-running operations.
- **Global Search**: Search for any file across all devices and snapshots in the repository.
- **Retention Policies**: Automatically prunes old snapshots to save space.
- **Storage Statistics**: Detailed reports on deduplication efficiency and physical disk usage.

---

## 🏗 Architecture

The project follows a strict Hexagonal Architecture to ensure business logic is decoupled from external tools.

```text
phone-backup/
├── apps/
│   └── cli/                # Composition Root (Wiring adapters)
├── core/
│   ├── domain/             # Core Entities (Device, Snapshot, File, App)
│   ├── application/        # Use Cases & Orchestration (BackupService)
│   └── ports/              # Interface definitions (Dependency Inversion)
├── adapters/
│   ├── adb/                # Real Android ADB communication
│   ├── mock/               # Simulation for dev/test
│   └── filesystem/         # Local object storage
└── infrastructure/
    └── database-sqlite/    # Persistent index (SQLite)
```

---

## 🚦 Getting Started

### Prerequisites
- **Rust**: Latest stable toolchain.
- **ADB**: Android Debug Bridge installed and in your `PATH`.

### Installation
```bash
# Install via Cargo
cargo install phone-backup

# Or build from source
git clone https://github.com/damarkuncoro/phone-backup.git
cd phone-backup
cargo build --release
```

---

## 📖 Usage Guide

### 1. Device Discovery
```bash
phone-backup --adapter adb devices
```

### 2. Backup & Migration
```bash
# Backup with Encryption
phone-backup --adapter adb backup <DEVICE_ID> --password "your-secret"

# Direct Device-to-Device Cloning
phone-backup --adapter adb clone <SOURCE_ID> <TARGET_ID>
```

### 3. Restore & Analysis
```bash
# Selective Restore (Filter by keyword)
phone-backup restore <SNAPSHOT_ID> --target ./restore --filter "WhatsApp"

# View Photo Gallery with GPS/Camera info
phone-backup photos <DEVICE_ID>

# Search for a file anywhere in the repository
phone-backup search "resume.pdf"

# View storage efficiency report
phone-backup stats
```

---

## 🛡 Security & Privacy
- **Zero-Knowledge Storage**: No plain-text data is stored if encryption is enabled.
- **Authenticated Encryption**: GCM tags ensure data hasn't been tampered with.
- **Metadata Protection**: File catalogs are stored in a local SQLite database, protected by system permissions.

## 📄 License
MIT

---
*Developed with ❤️ in Rust for the Android Community.*
