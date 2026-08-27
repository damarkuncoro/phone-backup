# phone-backup-application

The **Application Layer** of the phone-backup platform. This crate contains the use cases and orchestration logic that powers the backup engine.

## 🚀 Key Component: `BackupService`

The `BackupService` is the primary entry point for all business operations. It coordinates multiple ports to perform complex tasks:

- **Orchestrated Backup**: Discovery -> Scanning -> Policy Filtering -> Deduplication -> Compression -> Encryption -> Metadata Storage.
- **Failure Recovery**: Built-in logic to resume interrupted backups, ensuring reliability over unstable connections.
- **Incremental Logic**: Efficiently detects changes to avoid redundant data transfers.
- **Direct Migration**: High-speed device-to-device cloning (Apps & Files).
- **Integrity Verification**: Scans the repository to detect missing or corrupted data.
- **Media Extraction**: Automatically processes photos to extract EXIF and GPS data using domain logic.
- **Retention Management**: Pruning old snapshots according to user-defined rules.

## 🏗 Modular Architecture

The application layer is divided into specialized internal modules to maintain a clean codebase:
- `security`: AES-256-GCM encryption and Argon2 key derivation.
- `compression`: Zstd-based data compression.
- `media_analysis`: Metadata and GPS extraction from media files.
- `hashing`: SHA-256 integrity calculation.

## 🛡 Security First

All security logic (Key Derivation, Encryption, Decryption) is implemented here to ensure consistent protection regardless of the storage adapter used.
