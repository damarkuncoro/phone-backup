# phone-backup-application

The **Application Layer** of the phone-backup platform. This crate contains the use cases and orchestration logic that powers the backup engine.

## 🚀 Key Component: `BackupService`

The `BackupService` is the primary entry point for all business operations. It coordinates multiple ports to perform complex tasks across modular use-case files:

- **`backup`**: Orchestrated Backup (Discovery -> Scanning -> Policy Filtering -> Deduplication -> Compression -> Encryption -> Metadata Storage).
- **`restore`**: Recover files from content-addressed object storage with filtering support.
- **`verify`**: Scan repository integrity to detect missing objects or corrupted files.
- **`schedule_runner`**: Run automated scheduled backups and apply snapshot retention policies.
- **`device_ops`**: Device info, scanning, app listing, file search, and high-speed device-to-device migration/cloning.

## 🏗 Modular Architecture & Design Patterns

The application layer is divided into specialized internal modules to maintain a clean codebase:

- **`object_store`**: `ObjectStoreKey` helper centralizing 2-level content-addressable storage key & path generation (`objects/ab/cd/...`).
- **`security`**: AES-256-GCM encryption and Argon2 key derivation.
- **`compression`**: Zstd-based data compression.
- **`media_analysis`**: Metadata and GPS extraction from media files.
- **`hashing`**: SHA-256 integrity calculation.

## 🛡 Security First

All security logic (Key Derivation, Encryption, Decryption) is implemented here to ensure consistent protection regardless of the storage adapter used.
