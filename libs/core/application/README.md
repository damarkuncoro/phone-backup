# phone-backup-application

The **Application Layer** of the phone-backup platform. This crate contains the use cases and orchestration logic that powers the backup engine.

## 🚀 Key Component: `BackupService`

The `BackupService` is the primary entry point for all business operations. It coordinates multiple ports to perform complex tasks across modular use-case files:

- **`backup`**: Orchestrated Backup (Discovery -> Scanning -> Policy Filtering -> Deduplication -> Compression -> Encryption -> Metadata Storage).
- **`restore`**: Recover files from content-addressed object storage with filtering support.
- **`verify`**: Scan repository integrity to detect missing objects or corrupted files.
- **`schedule_runner`**: Run automated scheduled backups and apply snapshot retention policies.
- **`device_ops`**: Live device file management (`download_file`, `upload_file`), device scanning, app listing, live search, and file management operations.

## 🏗 Modular Architecture & Design Patterns

The application layer is divided into specialized internal modules to maintain a clean codebase:

- **`object_store`**: `ObjectStoreKey` helper centralizing 2-level content-addressable storage key & path generation (`objects/ab/cd/...`).
- **`security`**: AES-256-GCM symmetric encryption, `age` X25519 asymmetric cryptography, and Argon2id key derivation (`derive_database_key`) for SQLCipher database encryption.
- **`compression`**: Zstd-based data compression.
- **`media_analysis`**: Metadata and GPS extraction from media files.
- **`hashing`**: SHA-256 integrity calculation.

## 🛡 Security & Test Isolation

- **Zero-Knowledge Security**: All security logic (Argon2id KDF, Key Derivation, Encryption, Decryption) is implemented here to ensure consistent protection regardless of the storage adapter used.
- **100% Isolated Tests**: All unit and integration test suites (`security_compression_test.rs`, `backup_integration.rs`) are hosted under `core/application/tests/`.
