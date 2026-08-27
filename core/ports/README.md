# phone-backup-ports

This crate defines the **Ports** (traits) that serve as the boundaries between the application logic and external technical implementations (adapters).

## 🔌 Available Ports

- **DevicePort**: Discovering and communicating with hardware (ADB, Mock).
- **ScannerPort**: Scanning the device filesystem for files.
- **RepositoryPort**: Persisting metadata and indexing snapshots (SQLite).
- **StoragePort**: Physical storage of binary data blobs (Filesystem, Cloud).
- **AppProviderPort**: Handling APK extraction and installation.
- **DataProviderPort**: Querying structured data like SMS, Contacts, and Call Logs.

## 🏗 Why Ports?

By depending on ports instead of concrete implementations, the `phone-backup-application` layer remains completely decoupled from specific technologies. You can swap ADB for a different transport, or SQLite for a different database, by simply providing a new adapter that implements the corresponding port.
