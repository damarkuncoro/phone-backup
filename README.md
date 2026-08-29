# phone-backup 📱

A high-performance, secure, and professional Android backup platform written in Rust.

**phone-backup** is a comprehensive backup engine designed with **Clean Architecture**, **Hexagonal Architecture**, and **SOLID Design Principles**. It handles device discovery, intelligent indexing, versioned snapshots, military-grade encryption, and storage-efficient deduplication.

---

## 🚀 Features

### 🖥 Desktop GUI & Dashboard
- **Modern Dashboard**: Visual summary of storage efficiency, engine health, and snapshot history built with Tauri, Tailwind CSS, and Chart.js.
- **Real-time Progress HUD**: Floating status window with animated progress for long-running backup and restore operations.
- **Device Management**: One-click backup and history viewing for all connected ADB devices.

### 🧠 Intelligent Engine
- **Block-level Deduplication**: Uses Content-Defined Chunking (FastCDC) to deduplicate large files at the block level, saving massive storage for frequently modified large files.
- **Fast Incremental Backup**: Scans device state and only transfers new or modified files based on size and mtime.
- **Streaming I/O**: Direct data transfer from ADB (`exec-out`) to the backup engine without temporary files, maximizing performance and reducing disk wear.
- **Parallel Processing**: Utilizes `Rayon` for multi-threaded hashing, compression, and encryption.
- **Failure Recovery (Resume)**: Automatically detects interrupted backups and resumes from the last successful file.

### 🛡 Security & Privacy
- **Asymmetric Encryption**: Support for **age (X25519)** public-key encryption. Perform password-less backups while keeping the secret key safe elsewhere.
- **Zero-Knowledge Storage**: No plain-text data is stored if encryption is enabled.
- **Authenticated Encryption**: AES-256-GCM or age-authenticated blobs ensure data integrity.

---

## 🏗 Architecture

The project follows strict **Clean Architecture** and **Hexagonal Architecture**:

```text
phone-backup/
├── apps/
│   ├── cli/            # Professional Command Line Interface
│   └── gui/            # Desktop Dashboard (Tauri + Tailwind)
├── core/
│   ├── domain/         # Pure business logic & entities
│   └── application/    # Use cases, ObjectManager, and Engine
├── adapters/           # ADB, Cloud Storage (OpenDAL), Filesystem
├── infrastructure/     # Persistence (SQLite modular repository)
└── workspace/          # Centralized data (DB, Objects, Logs)
```

---

## 🚦 Getting Started

### Prerequisites
- **Rust**: Latest stable toolchain.
- **ADB**: Android Debug Bridge installed and in your `PATH`.
- **Node.js**: Required only for GUI development.

### Installation
```bash
git clone https://github.com/damarkuncoro/phone-backup.git
cd phone-backup

# Install CLI globally
cargo install --path apps/cli
```

---

## 📖 Usage Guide

### 1. Launching the Desktop GUI
For a visual experience, run the Tauri dashboard:
```bash
cd apps/gui/src-tauri
cargo tauri dev
```

### 2. System Health Check (CLI)
Run diagnostic to ensure ADB and workspace are ready:
```bash
phone-backup doctor
```

### 3. Backup using Public Key (CLI)
```bash
phone-backup --adapter adb --pubkey "age1..." backup <DEVICE_ID>
```

---

## 🧪 Testing & Quality
Run the full test suite and quality checks:
```bash
cargo test
cargo clippy
cargo fmt --check
```

---

## 📄 License
MIT

---
*Developed with ❤️ for the Android Community.*
