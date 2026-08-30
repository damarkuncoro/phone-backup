# phone-backup-cli

The primary user interface for the phone-backup platform.

## 🛠 Composition Root & Design Patterns

This crate is the **Composition Root** of the application. Its responsibilities include:

1. **CLI Parsing**: Defining the command-line interface using `clap` with support for environment variables.
2. **Facade Integration**: Simplified hardware orchestration using the unified **`AdbAdapter`** facade.
3. **Storage Orchestration**: Constructing storage backends dynamically based on user parameters.
4. **Builder Pattern Usage**: Constructing `BackupPolicy` using `BackupPolicyBuilder` for customized backup inclusions/exclusions.
5. **Formatting**: Presenting backup reports, photo galleries, and statistics in a human-readable format.

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
