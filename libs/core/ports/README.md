# phone-backup-ports

This crate defines the **Ports** (traits) that serve as the boundaries between the application logic and external technical implementations (adapters).

## 🔌 Available Ports

- **DevicePort**: Discovering and communicating with hardware (ADB, Mock). Supports bidirectional transfers (`read_file`/`push_file`).
- **ScannerPort**: Scanning the device filesystem for files.
- **RepositoryPort**: Persisting metadata and indexing snapshots (SQLite). Now includes search and interrupted snapshot tracking.
- **StoragePort**: Physical storage of binary data blobs (Filesystem, OpenDAL/S3).
- **AppProviderPort**: Handling APK extraction and remote installation.
- **DataProviderPort**: Querying structured data like SMS, Contacts, and Call Logs.

## 🛡 Storage Decorators & Middleware

The `ports` crate provides composable **Decorator Patterns** to augment any `StoragePort` implementation transparently:

- **`RetryStorage<S>`**: Automatic retry with exponential backoff for transient I/O and network/cloud timeout errors.
- **`MetricsStorage<S>`**: Transparent telemetry tracking total bytes read/written, write/read operations count, and I/O latency.

## 🏗 Why Ports?

By depending on ports instead of concrete implementations, the `phone-backup-application` layer remains completely decoupled from specific technologies. You can swap ADB for a different transport, or SQLite for a different database, by simply providing a new adapter that implements the corresponding port.

