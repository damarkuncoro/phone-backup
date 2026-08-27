# phone-backup-cli

The primary user interface for the phone-backup platform.

## 🛠 Composition Root

This crate is the **Composition Root** of the application. Its responsibilities include:

1.  **CLI Parsing**: Defining the command-line interface using `clap`.
2.  **Dependency Injection**: Constructing the concrete adapters (e.g., `AdbDeviceAdapter`, `SqliteRepository`) and wiring them into the `BackupService`.
3.  **Command Execution**: Mapping user commands to `BackupService` use cases.
4.  **Formatting**: Presenting backup reports, photo galleries, and statistics in a human-readable format.

## 💻 Available Commands

- `devices`: List connected Android devices.
- `backup`: Perform an incremental, encrypted backup.
- `snapshots`: List and inspect previous backup points.
- `restore`: Recover data from the repository (with support for filtering).
- `clone`: Direct device-to-device migration.
- `photos`: View photo gallery metadata and GPS info.
- `stats`: View repository efficiency and storage savings.
- `search`: Find files across all backups.
- `schedule`: Manage automated background backups.
