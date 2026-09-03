# phone-backup-domain

This crate contains the core domain models and business entities for the **phone-backup** platform. It is a pure library with no external I/O dependencies, serving as the inner-most layer of the Hexagonal Architecture.

## 🧱 Core Entities & Builder Pattern

- **Device**: Represents an Android device (id, model, manufacturer, storage stats).
- **Snapshot**: A versioned point-in-time backup. Supports states: `Pending`, `Running`, `Completed`, `Failed`, and `Interrupted` (for resume support).
- **FileEntry**: Metadata for a single file on the device, including path, size, and hash.
- **AppInfo**: Information about an installed Android application (package name, version).
- **MediaInfo**: Extracted metadata from media files (EXIF, GPS, resolution).
- **BackupPolicy**: Configuration for what to include or exclude during a backup. Implements **`BackupPolicyBuilder`** (`BackupPolicy::builder()`) for fluent construction.
- **RestoreOptions**: Granular configuration for restoring snapshots. Implements **`RestoreOptionsBuilder`** (`RestoreOptions::builder()`) with path filters and overwrite options.
- **CancellationToken**: Thread-safe cooperative cancellation token for responsive GUI and CLI job cancellation.
- **DomainEventBus**: In-memory observer/event bus for decoupling domain lifecycle events from UI/audit listeners.
- **RetentionPolicy**: Rules for how long to keep old snapshots.

## 🛠 Usage

This crate is used by all other crates in the workspace to ensure a consistent data model across the entire system.
