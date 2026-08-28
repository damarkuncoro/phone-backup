# 🏗 Software Architecture Document (SAD)

## 1. Introduction
This document describes the high-level architecture of the **phone-backup** platform. It is designed to be a secure, storage-efficient, and multi-transport backup engine for Android devices.

## 2. Architectural Goals
- **Decoupling**: Business logic must be independent of external transports (ADB) and storage (Disk/Cloud).
- **Security**: Data must be encrypted before leaving the device's volatile memory context.
- **Efficiency**: Identical files should only be stored once across all backups (Deduplication).
- **Reliability**: Backups must be resumable after any technical failure.

## 3. The Hexagonal Pattern
The system follows a strict **Hexagonal Architecture** (Ports & Adapters):

### 🧅 The Core (Domain & Application)
- **Domain**: Pure business entities (`Snapshot`, `Device`, `FileEntry`). No dependencies.
- **Application**: The `BackupService` orchestrates the backup pipeline. It depends only on **Ports**.

### 🔌 Ports (Interfaces)
- `DevicePort`: Abstraction for hardware communication.
- `StoragePort`: Abstraction for physical data persistence.
- `RepositoryPort`: Abstraction for metadata and indexing.
- `AppProviderPort`: Abstraction for APK management.

### ⚙️ Adapters (Implementations)
- `AdbDeviceAdapter`: Communicates with real devices.
- `LocalStorage`: Persists objects to the local filesystem.
- `CloudStorage (OpenDAL)`: Persists objects to S3/R2.
- `SqliteRepository`: Manages the metadata index.

---

## 4. Backup Pipeline (Data Flow)

```mermaid
graph TD
    A[Start Backup] --> B[Device Discovery]
    B --> C[Metadata Scanning]
    C --> D[Policy Filtering]
    D --> E[Incremental Diffing]
    E --> F[Object Hashing]
    F --> G{Exists in Storage?}
    G -- No --> H[Compression & Encryption]
    H --> I[Write to Storage]
    G -- Yes --> J[Link Existing Object]
    I --> K[Update Metadata Index]
    J --> K
    K --> L[Finish Snapshot]
```

---

## 5. Security Architecture
We implement a **Zero-Knowledge** capable security model:
1.  **Key Derivation**: User passwords are never stored. We use **Argon2id** with a salt to derive a 256-bit encryption key.
2.  **Encryption**: All data blobs are encrypted using **AES-256-GCM** (Authenticated Encryption).
3.  **Integrity**: The GCM authentication tag ensures that objects have not been tampered with or corrupted in storage.

---

## 6. Storage Strategy
### Content-Addressed Storage (CAS)
Objects are stored based on their SHA-256 hash.
- **Deduplication**: If two files have the same hash, they share the same physical object.
- **Path Sharding**: Objects are stored in subdirectories like `objects/ab/cd/...` to prevent directory performance degradation.

### Failure Recovery (Resume)
The engine tracks `Interrupted` snapshots in the `RepositoryPort`. Upon restarting, it identifies already-processed files by querying the partial snapshot manifest, allowing it to skip successful transfers and resume exactly where it left off.

---

## 7. Quality Attributes
- **Testability**: High. Every component is mockable via its Port.
- **Portability**: High. Written in Rust with minimal OS-specific dependencies (ADB being the primary external tool).
- **Extensibility**: Adding a new cloud provider or a new device type (e.g., iOS) only requires a new Adapter implementation.
