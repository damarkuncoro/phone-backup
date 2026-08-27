# phone-backup 📱

A high-performance, secure, and professional Android backup platform written in Rust.

**phone-backup** is more than just a file copying tool; it's a comprehensive backup engine designed with **Clean Architecture** and **Hexagonal Architecture** principles. It handles device discovery, intelligent indexing, versioned snapshots, military-grade encryption, and storage-efficient deduplication.

---

## 🚀 Features

### 🧠 Intelligent Engine
- **Fast Incremental Backup**: Scans device state and only transfers new or modified files based on size and mtime.
- **Content-Addressed Storage (Deduplication)**: Uses SHA-256 hashing to ensure identical files (across different folders, snapshots, or even devices) are stored only once.
- **Zstd Compression**: High-performance compression for compressible data (logs, json, text).
- **Military-Grade Security**: AES-256-GCM authenticated encryption with keys derived using the Argon2 memory-hard function.

### 📱 Deep Android Support
- **Multi-Transport**: Native support for **ADB** (real devices) and **Mock** adapters (testing).
- **App Management**: Tracks installed applications and performs **APK extraction** for backup.
- **Structured Data**: Backs up **Contacts**, **SMS**, and **Call Logs** into secure JSON blobs.
- **Media Intelligence**: Automatically extracts **EXIF metadata** (Resolution, Camera Model, GPS location) from photos during backup.

### 🛠 Management & Safety
- **Backup Scheduler**: Built-in logic for Hourly, Daily, and Weekly automated backups.
- **Retention Policies**: Automatically prunes old snapshots while keeping important milestones.
- **Selective Backup**: Fine-grained `include` and `exclude` path filtering.
- **Smart Safety Check**: Pre-calculates required backup size and verifies available disk space on the host computer before starting.

---

## 🛠 Project Structure

The project follows a strict Hexagonal Architecture to ensure business logic is decoupled from external tools like ADB or SQLite.

```text
phone-backup/
├── apps/
│   └── cli/                # Command-line interface (Composition Root)
├── core/
│   ├── domain/             # Core Entities (Device, Snapshot, File, App, etc.)
│   ├── application/        # Use Cases & Orchestration (BackupService)
│   └── ports/              # Trait definitions (Dependency Inversion seams)
├── adapters/
│   ├── adb/                # Real Android ADB communication
│   ├── mock/               # Simulation for dev/test
│   └── filesystem/         # Physical storage management
└── infrastructure/
    └── database-sqlite/    # Persistent index & metadata repository
```

---

## 🚦 Getting Started

### Prerequisites
- **Rust**: Latest stable toolchain.
- **ADB**: Android Debug Bridge installed and in your `PATH`.
- **Android Device**: Developer Options enabled with USB Debugging on.

### Installation
```bash
git clone https://github.com/damarkuncoro/phone-backup.git
cd phone-backup
cargo build --release
```

---

## 📖 Usage Guide

### 1. Device Discovery
List all connected devices through the ADB adapter:
```bash
phone-backup --adapter adb devices
```

Inspect a specific device's storage and hardware info:
```bash
phone-backup --adapter adb device-info <DEVICE_ID>
```

### 2. Backup Operations
Perform a full backup (first run) or incremental backup (subsequent runs):
```bash
phone-backup --adapter adb backup <DEVICE_ID>
```

Backup with encryption (recommended):
```bash
phone-backup --adapter adb backup <DEVICE_ID> --password "your-secret"
```

Selective folder backup:
```bash
phone-backup --adapter adb backup <DEVICE_ID> --include /sdcard/DCIM --include /sdcard/WhatsApp
```

### 3. Restore & Maintenance
List all available snapshots for a device:
```bash
phone-backup snapshots <DEVICE_ID>
```

Restore a snapshot to your computer:
```bash
phone-backup restore <SNAPSHOT_ID> --target ./my_recovered_data --password "your-secret"
```

Verify repository integrity:
```bash
phone-backup verify --password "your-secret"
```

---

## 🛡 Security Architecture
- **Zero Raw Storage**: No data is stored in plain text if a password is provided.
- **Key Derivation**: Uses Argon2id to protect against brute-force attacks.
- **Deduplication Privacy**: Deduplication occurs at the encrypted blob level to prevent metadata leakage.
- **Integrity**: Every object is verified using authenticated encryption (GCM tag).

## 📄 License
MIT

---
*Developed with ❤️ in Rust for the Android Community.*
