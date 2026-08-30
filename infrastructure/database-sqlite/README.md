# phone-backup-adapter-database-sqlite 🗄️

High-performance persistence layer implementing the `RepositoryPort` using **SQLite** and **SQLCipher**.

## 🏗 Modular Architecture (Facade Pattern)

This crate is organized into domain-specific repositories aggregated by a central `SqliteRepository` Facade:

- **`repositories/`**: Modular implementations for each domain:
  - `device_repo.rs`: Device registration and storage metrics.
  - `file_repo.rs`: Global file catalog with **FTS5 search** and **Batch Indexing**.
  - `contact_repo.rs`: Deep contact storage using **JSON Aggregation** to solve N+1 query problems.
  - `communication_repo.rs`: SMS and Call Log indexing.
  - `snapshot_repo.rs`: Snapshot lifecycle management and resume support.
  - `maintenance_repo.rs`: Garbage collection, orphan pruning, and online backups.
- **`schema/`**: Evolutionary schema management with a robust **Migration System**.
- **`mappers/`**: Clean translation between database rows and Domain Entities.

## 🚀 Advanced Features

- **At-Rest Encryption**: Integrated **SQLCipher** for military-grade AES-256 encryption of the entire metadata database.
- **Full-Text Search (FTS5)**: Dedicated virtual tables and triggers for instant search across files, contacts, and SMS.
- **Concurrency & Reliability**: 
  - **WAL (Write-Ahead Logging)** mode for better concurrent read/write performance.
  - **Connection Pooling** via `r2d2` for thread-safe access.
  - **Busy Timeout (5s)** to gracefully handle lock contention.
- **Atomic Batch Processing**: Efficient bulk ingestion of thousands of records (files, SMS) in a single transaction.
- **Maintenance Suite**: Built-in support for `VACUUM`, `PRAGMA integrity_check`, and automatic pruning of orphaned data.

## 📊 Relational Integrity

The schema enforces strict referential integrity using **Foreign Key Cascades**:
- Deleting a device removes all its snapshots.
- Deleting a snapshot removes all its file links and structured data references.
- Ensures the `workspace/` metadata stays clean and consistent.

## 🧪 Quality Assurance

Comprehensive integration testing suite covers:
- Relational integrity and constraints.
- Multi-threaded concurrent write stability.
- Large-scale data ingestion performance.
- Database migration paths.

---
*The brains of the backup system, optimized for Rust and SQLite.*
