# phone-backup-adapter-database-sqlite

An infrastructure adapter that implements the `RepositoryPort` using **SQLite**.

## 📊 Database Schema

This crate manages the persistent metadata for the entire backup system, including:

- **Devices**: Tracks all phones ever backed up.
- **Files**: A global catalog of all unique files discovered across all devices.
- **Snapshots**: History of backup runs, their status, and performance metrics.
- **Links**: Relational tables linking specific files and apps to specific snapshots.
- **Schedules**: Persistent configuration for automated backup tasks.

## ⚡ Performance

- **Deduplication Lookup**: Efficiently finds existing file hashes to avoid redundant uploads.
- **Global Search**: Supports fast `LIKE` queries across paths and filenames.
- **Relational Integrity**: Uses Foreign Key constraints to ensure the repository remains consistent during deletions (Retention cleanup).
