# Arsitektur & Roadmap Pengembangan 📱

Roadmap ini mendokumentasikan evolusi **phone-backup** dari sebuah skrip sederhana menjadi engine backup Android kelas dunia.

---

# STATUS PROYEK: v0.3.0-alpha 🚀

## ✅ PHASE 01 — Project Foundation
*   Struktur Workspace (9 package).
*   Hexagonal Architecture (Ports & Adapters).

## ✅ PHASE 02 — Device Discovery
*   ADB & Mock device discovery.
*   Command: `phone-backup devices`.

## ✅ PHASE 03 — Permission & Capability
*   Capability Matrix (Files, SMS, Contacts).

## ✅ PHASE 04 — File Scanner
*   Recursive scanner dengan metadata (size, mtime, mime).

## ✅ PHASE 05 — File Index Database
*   SQLite Metadata Catalog terpusat.
*   Modular SQL Repository (Mappers & Schema).

## ✅ PHASE 06 — Backup Snapshot
*   Snapshot-based backup system.
*   Track: Pending, Running, Completed, Interrupted.

## ✅ PHASE 07 — Backup Engine
*   Parallel Processing dengan **Rayon**.
*   **Streaming I/O ADB** (Bypass Temp Files).

## ✅ PHASE 08 — Storage Backend
*   Local Storage & S3-Compatible Storage (OpenDAL).

## ✅ PHASE 09 — Deduplication (Advanced)
*   Content-Addressed Storage (CAS).
*   **Block-level Deduplication** (FastCDC).

## ✅ PHASE 10 — Compression
*   Zstd High-speed compression.
*   MIME-based compression policy.

## ✅ PHASE 11 — Encryption
*   AES-256-GCM (Password-based).
*   **Asymmetric X25519 (age)** public-key encryption.

## ✅ PHASE 12-14 — Incremental & Manifest
*   Metadata-only scan untuk file yang sudah ada.
*   Snapshot integrity manifest.

## ✅ PHASE 15-16 — Restore Engine
*   Full & Selective Restore.
*   Chunk Reassembly (Re-assembling fragmented files).

## ✅ PHASE 17-18 — Apps & Structured Data
*   APK Backup support.
*   Contacts, SMS, Call History (via ADB content query).

## ✅ PHASE 19 — Media Intelligence
*   EXIF metadata extraction (Resolution, Camera).

## ✅ PHASE 20-22 — Scheduler & Retention
*   Background schedule runner.
*   Retention Strategies (Keep Daily/Count).

## ✅ PHASE 23 — Backup Integrity
*   Perintah `verify` untuk cek objek hilang/rusak.

## 🚧 PHASE 24 — Desktop GUI
*   [In-Progress] Inisialisasi Tauri project.
*   [TODO] Visual Dashboard.

## ✅ PHASE 30 — Failure Recovery
*   **Resume Logic**: Melanjutkan backup yang terputus secara otomatis.

## ✅ PHASE 31 — CLI Final
*   Clean CLI interface dengan subcommand lengkap.
*   **Doctor Command**: Diagnosa kesehatan sistem.

## ✅ PHASE 32 — Packaging
*   Binary build untuk macOS & Linux.
*   Published v0.2.0.

## ✅ PHASE 33 — Observability
*   Structured logging (`tracing`).
*   Rolling file logs harian di `workspace/logs`.

---

# NEXT GOALS (v1.0.0 Roadmap)

1.  **GUI Dashboard**: Tampilan visual untuk memantau storage.
2.  **iOS Support**: Eksplorasi adapter Apple via `libimobiledevice`.
3.  **Encrypted Metadata**: Enkripsi database SQLite (`SQLCipher`).
4.  **Auto-Backup Daemon**: Service yang jalan di background saat USB dicolok.
