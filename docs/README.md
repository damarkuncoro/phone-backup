# 📚 Phone Backup Documentation

Welcome to the technical documentation for the **phone-backup** platform. This directory contains detailed guides, architecture diagrams, and development roadmaps.

## 🧭 Navigation

- **[Project Roadmap & Phases](phase.md)**: A detailed breakdown of the 34 development phases.
- **[Known Limitations](LIMITATIONS.md)**: Important information about what cannot be backed up from Android devices.
- **[Architecture Overview](#architecture-overview)**: Deep dive into the Hexagonal/Clean Architecture implementation.
- **[Development Guide](#development-guide)**: How to set up the environment and run tests.

---

## 🏗 Architecture Overview

The system is built using **Hexagonal Architecture** (Ports & Adapters) to ensure that the core backup logic is independent of external technical details.

### 1. Core Domain (`core/domain`)
Contains the business entities and rules. It has zero dependencies on other crates in the workspace.
- **Entities**: `Device`, `Snapshot`, `FileEntry`, `AppInfo`.
- **Logic**: `BackupPolicy` (filtering), `RetentionPolicy`.

### 2. Application Layer (`core/application`)
The "Brain" of the system. Implements use cases via the `BackupService`.
- **Modules**:
    - `security`: AES-256-GCM encryption & Argon2 KDF.
    - `compression`: Zstd orchestration.
    - `media_analysis`: EXIF/GPS extraction logic.
    - `hashing`: SHA-256 integrity.
- **Key Use Case**: `perform_backup` with failure recovery (resume) support.

### 3. Ports (`core/ports`)
Interface definitions that the application layer uses to talk to the outside world.
- `DevicePort`, `ScannerPort`, `StoragePort`, `RepositoryPort`.

### 4. Adapters
Concrete implementations of the Ports.
- **`adapter-adb`**: Real Android communication.
- **`adapter-opendal`**: Cloud storage (S3/R2) support.
- **`adapter-filesystem`**: Local disk storage.
- **`adapter-database-sqlite`**: Metadata indexing.
- **`adapter-mock`**: Testing simulation.

---

## 🧪 Development Guide

### Running the Test Suite
We maintain a strict testing policy. Always run tests before pushing:
```bash
# Run unit and integration tests
cargo test

# Run a specific integration test
cargo test --test backup_integration
```

### Versioning Policy
This project follows [Semantic Versioning](https://semver.org/).
- **v0.1.x**: Initial MVP (Local backup).
- **v0.2.x**: Modular engine, Cloud support, and Failure Recovery.
- **v1.0.0**: Stable release with GUI support.

---
*For user-facing installation and usage guides, please refer to the [Root README](../README.md).*
