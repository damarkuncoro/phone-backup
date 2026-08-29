# phone-backup 📱

A high-performance, secure, and professional Android backup platform written in Rust.

**phone-backup** is a comprehensive backup engine designed with **Clean Architecture**, **Hexagonal Architecture**, and **SOLID Design Principles**. It handles device discovery, intelligent indexing, versioned snapshots, military-grade encryption, and storage-efficient deduplication.

---

## 🚀 Features

### 🖥 Desktop GUI & Dashboard
- **Modern Dashboard**: Visual summary of storage efficiency, engine health, and snapshot history built with Tauri, Tailwind CSS, and Chart.js.
- **Modular Architecture**: Built using **Atomic Design** principles with Native Web Components for high maintainability.
- **Drawer Sidebar Navigation**: Modern, fixed side navigation with persistent search and engine status.
- **Full-Page Explorers**: Comprehensive views for File Browsing and Android Data (Contacts, SMS, Call Logs) without limiting modals.
- **Global Contact Search**: Instantly find people across all snapshots and devices using a high-performance relational search engine.
- **Real-time Progress HUD**: Floating status window with animated progress for long-running operations.
- **Selective Backup (Dry Run)**: Scan device files first, then select specifically what you want to protect.

### 🧠 Intelligent Engine
- **Block-level Deduplication**: Uses Content-Defined Chunking (FastCDC) to deduplicate large files.
- **Relational Data Engine**: Full SQLite relational schema for Contacts, supporting deep extraction of multiple phones, emails, and organizations with transactional integrity.
- **Smart Retention**: Automatically prunes redundant snapshots if no data has changed.
- **Fast Incremental Backup**: Scans device state and only transfers new or modified files.
- **Streaming I/O**: Direct data transfer from ADB (`exec-out`) to the backup engine without temporary files.
- **Parallel Processing**: multi-threaded hashing, compression, and encryption using `Rayon`.

### 🛡 Security & Privacy
- **Asymmetric Encryption**: Support for **age (X25519)** public-key encryption. Perform password-less backups while keeping the secret key safe.
- **Zero-Knowledge Storage**: No plain-text data is stored; your data is encrypted before it hits the disk.
- **Authenticated Integrity**: Every object is hashed and verified to prevent silent data corruption.

---

## 🏗 Architecture

The project follows strict **Clean Architecture** and **Hexagonal Architecture**:

```text
phone-backup/
├── apps/
│   ├── cli/            # Professional Command Line Interface
│   └── gui/            # Desktop Dashboard (Modular Web Components)
├── core/
│   ├── domain/         # Pure business logic & entities
│   ├── application/    # Use cases, ObjectManager, and Engine
│   └── ports/          # Port definitions (Repository, Storage, etc.)
├── adapters/           # ADB, Cloud Storage (OpenDAL), Filesystem, Mock
├── infrastructure/     # Persistence (SQLite modular repository)
└── workspace/          # Centralized data (DB, Objects, Logs)
```

---

## 🚦 Getting Started

### Prerequisites
- **Rust**: Latest stable toolchain.
- **ADB**: Android Debug Bridge installed and authorized on your device.
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
```bash
cd apps/gui/src-tauri
cargo tauri dev
```

### 2. System Diagnostic (CLI)
```bash
phone-backup doctor
```

### 3. Backup with Encryption (CLI)
```bash
phone-backup --adapter adb --pubkey "age1..." backup <DEVICE_ID>
```

### 4. Smart Restore
```bash
# Automatically restores to a versioned folder in your workspace
phone-backup restore last
```

---

## 🧪 Testing & Quality
The modular architecture allows for easy unit testing of both Rust and JavaScript components:
```bash
cargo test             # Rust Core Tests
# (Future) npm test    # Frontend Atomic Component Tests
```

---

## 📄 License
MIT

---
*Developed with ❤️ for the Android Community.*
