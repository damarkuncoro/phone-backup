# phone-backup-scanner 🔍

High-speed device filesystem scanner, directory hierarchy crawler, and incremental change detection engine.

## 🏗 Architecture & Modules

- **`incremental.rs`**: Fast metadata diffing engine comparing previous snapshots with current device state. Includes Android path canonicalizer (`/sdcard/` vs `/storage/emulated/0/`).
- **`partition.rs`**: Partition-level state tracking and fast rolling signature computation.
- **`manifest.rs`**: Immutable manifest compiler generating Merkle-tree rooted file indexes.

## 🚀 Key Features

- **Microsecond Path Canonicalization**: Normalizes aliased Android storage mountpoints to prevent false-positive file removals.
- **Incremental Diff Engine**: Evaluates `(path, size, mtime)` tuples across hundreds of thousands of files in milliseconds.
- **Parallel Category Aggregation**: Automatically classifies discovered files into standard categories (Images, Audio, Video, Documents, APKs, System).
