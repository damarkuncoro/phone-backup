# phone-backup 📱

A high-performance, secure, and professional Android backup platform written in Rust.

**phone-backup** is a comprehensive backup engine designed with **Clean Architecture**, **Hexagonal Architecture**, and **SOLID Design Principles**. It handles device discovery, intelligent indexing, versioned snapshots, military-grade encryption, and storage-efficient deduplication.

---

## 🚀 Features

### 🧠 Intelligent Engine
- **Block-level Deduplication**: Uses Content-Defined Chunking (FastCDC) to deduplicate large files at the block level, saving massive storage for frequently modified large files.
- **Fast Incremental Backup**: Scans device state and only transfers new or modified files based on size and mtime.
- **Streaming I/O**: Direct data transfer from ADB (`exec-out`) to the backup engine without temporary files, maximizing performance and reducing disk wear.
- **Parallel Processing**: Utilizes `Rayon` for multi-threaded hashing, compression, and encryption.
- **Failure Recovery (Resume)**: Automatically detects interrupted backups and resumes from the last successful file.
- **Content-Addressed Storage (CAS)**: Ensures identical files/blocks are stored only once using SHA-256 keying.
- **Zstd Compression**: High-performance compression for text-based data.

### 🛡 Security & Privacy
- **Asymmetric Encryption**: Support for **age (X25519)** public-key encryption. Perform password-less backups while keeping the secret key safe elsewhere.
- **Zero-Knowledge Storage**: No plain-text data is stored if encryption is enabled.
- **Authenticated Encryption**: AES-256-GCM or age-authenticated blobs ensure data integrity.
- **Metadata Protection**: File catalogs are stored in a local SQLite database in the `workspace/` directory.

### 📱 Deep Android Support
- **Native ADB Integration**: Reliable communication with real Android devices via a custom `AdbClient`.
- **Direct Migration (Cloning)**: Transfer apps and files directly from Device A to Device B.
- **App Management**: Tracks installed applications and performs automatic **APK extraction**.
- **Structured Data**: Backs up **Contacts**, **SMS**, and **Call Logs** into secure JSON objects.
- **Media Intelligence**: Extracts **EXIF metadata** from photos.

---

## 🏗 Architecture

The project follows strict **Clean Architecture** and **Hexagonal Architecture**:

```text
phone-backup/
├── apps/               # UI implementations (CLI, GUI-Tauri)
├── core/               # Domain & Application logic (Pure Rust)
├── adapters/           # Technical adapters (ADB, Cloud S3, Filesystem)
├── infrastructure/     # Database implementations (SQLite)
├── docs/               # Detailed documentation & roadmaps
└── workspace/          # Local storage (Database, Objects, Logs)
```

---

## 🚦 Getting Started

### Prerequisites
- **Rust**: Latest stable toolchain.
- **ADB**: Android Debug Bridge installed and in your `PATH`.

### Installation
```bash
git clone https://github.com/damarkuncoro/phone-backup.git
cd phone-backup
cargo build --release
```

---

## 📖 Usage Guide

### 1. System Health Check
Run diagnostic to ensure ADB and workspace are ready:
```bash
./target/release/phone-backup doctor
```

### 2. Backup using Public Key (Asymmetric)
```bash
# Generate a keypair first (stored in age format)
# Then backup using the public key
./target/release/phone-backup --adapter adb --pubkey "age1..." backup <DEVICE_ID>
```

### 3. Backup to Cloud (S3/R2/MinIO)
```bash
./target/release/phone-backup --storage s3 \
  --s3-bucket my-backup \
  --s3-endpoint https://<id>.r2.cloudflarestorage.com \
  --s3-access-key <key> \
  --s3-secret-key <secret> \
  backup <DEVICE_ID>
```

### 4. Restore & Management
```bash
# Restore snapshot using Secret Key
./target/release/phone-backup --privkey "AGE-SECRET-KEY-1..." restore <SNAPSHOT_ID> --target ./restore

# View snapshots for a device
./target/release/phone-backup snapshots <DEVICE_ID>

# Search for a file anywhere in the repository
./target/release/phone-backup search "resume.pdf"
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
