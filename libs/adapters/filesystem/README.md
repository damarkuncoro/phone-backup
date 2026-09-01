# phone-backup-adapter-filesystem

A `StoragePort` implementation that uses the local filesystem as the physical target for backup data.

## 📁 Storage Strategy

- **Object Store**: Files are stored in an `objects/` directory using a content-addressed structure (e.g., `objects/ab/cd/ef...`).
- **Path Sharding**: Uses the first 4 characters of the file hash to create subdirectories, preventing performance issues with thousands of files in a single directory.
- **Atomic Writes**: Ensures data integrity by writing to temporary files before moving them to the final destination.

## 🛡 Features

- Transparent handling of compressed (`.zst`) and encrypted (`.enc`) data blobs.
- Simple, directory-based structure compatible with standard backup tools (Rsync, Backblaze B2, etc.) for off-site mirroring.
