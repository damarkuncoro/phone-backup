# Arsitektur & Roadmap Pengembangan 📱

Roadmap ini mendokumentasikan evolusi **phone-backup** dari sebuah skrip sederhana menjadi engine backup Android kelas dunia.

---

# STATUS PROYEK: v0.3.5-stable 🚀

## ✅ PHASE 01 — Project Foundation
*   Struktur Workspace (10 package).
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
*   MP4/Video metadata processing.

## ✅ PHASE 20-22 — Scheduler & Retention
*   Background schedule runner.
*   Retention Strategies (Keep Daily/Count).

## ✅ PHASE 23 — Backup Integrity
*   Perintah `verify` untuk cek objek hilang/rusak.

## ✅ PHASE 24 — Desktop GUI (Tauri Dashboard)
*   Inisialisasi Tauri project.
*   Backend Bridge (Rust Commands -> JavaScript).
*   Event-driven real-time progress reporting.

## ✅ PHASE 25 — Modular GUI Architecture
*   **Atomic Design Implementation**: Komponen Web Native (Atom, Molecule, Organism).
*   **Reactive State Management**: Centralized Store untuk konsistensi data.
*   **Service Layer Pattern**: Decoupling API logic dari UI logic.

## ✅ PHASE 26 — Android Data Explorer
*   Visualisasi data terstruktur (SMS, Contacts) langsung di Dashboard.
*   Tab-based navigation antara Files dan Android Data.

## ✅ PHASE 27 — Smart Retention (Auto-Pruning)
*   Otomatis menghapus snapshot lama jika snapshot terbaru 100% identik (redundan).
*   Menjaga timeline backup tetap bersih dan bermakna.

## ✅ PHASE 28 — Dynamic Infrastructure
*   **Switchable Storage**: Berpindah provider storage (Local/Mock) secara runtime.
*   Implementasi SOLID (Liskov Substitution Principle) pada layer infrastruktur.

## ✅ PHASE 30 — Failure Recovery
*   **Resume Logic**: Melanjutkan backup yang terputus secara otomatis.

## ✅ PHASE 31 — CLI Final
*   Clean CLI interface dengan subcommand lengkap.
*   **Doctor Command**: Diagnosa kesehatan sistem.

## ✅ PHASE 32 — Packaging
*   Binary build untuk macOS & Linux.
*   Published v0.3.1.

## ✅ PHASE 33 — Observability
*   Structured logging (`tracing`).
*   Rolling file logs harian di `workspace/logs`.

## ✅ PHASE 34 — Relational Contact Engine
*   Migrasi dari format JSON ke **Full Relational Schema** di SQLite.
*   Deep Extraction: Mendukung multiple phones, emails, addresses, organizations, dan events (Birthday).
*   Constraint Enforcement: Penjaminan integritas data (Unique primary phones/emails).

## ✅ PHASE 35 — Global Search & Advanced Navigation
*   **Global Contact Search**: Pencarian lintas snapshot dan perangkat secara instan via SQL.
*   **Drawer Sidebar Navigation**: Layout modern dengan sidebar tetap dan active state tracking.
*   **Full-Page Views**: Migrasi dari modal-based UI ke full-page explorer untuk ruang kerja yang lebih luas.

## ✅ PHASE 36 — Live Device File Manager & On-Device Operations
*   **Live Device File Explorer**: Navigasi direktori HP secara real-time via ADB gateway.
*   **File Transfer Pipeline**: Fitur `download_from_device` (`download_file`) dan upload langsung dari/ke HP.
*   **File Operations**: Search, rename, copy, move, delete, view metadata, dan kalkulasi SHA-256 hash langsung di HP.

## ✅ PHASE 37 — Visual Snapshot Diffing Engine
*   **Visual Diff Matrix**: Membandingkan perubahan file dan kontak antara dua snapshot.
*   **Status Indicators**: Penanda visual intuitif untuk status **New**, **Modified**, **Deleted**, dan **Unchanged**.

## ✅ PHASE 38 — Installed App / APK Manager
*   **Live App Explorer**: Menampilkan daftar aplikasi terinstall di HP beserta nama paket dan versi.
*   **Snapshot App Inspection**: Dukungan filter tipe data `apps` di snapshot browser.

## ✅ PHASE 39 — Tauri Capabilities & ACL Permission Standardization
*   **ACL Manifests**: Standardisasi perintah Tauri (`snake_case`) dan penyusunan permission manifests (`autogenerated.toml`, `acl-manifests.json`).
*   **Security Enforcement**: Memastikan seluruh perintah hardware dan file manager terlindungi permission ACL.

---

# NEXT GOALS (v1.0.0 Roadmap)

1.  **Cloud Sync GUI**: Pengaturan S3/Google Drive langsung dari panel Settings.
2.  **iOS Support**: Eksplorasi adapter Apple via `libimobiledevice`.
3.  **Encrypted Metadata**: Enkripsi database SQLite (`SQLCipher`).
4.  **Auto-Backup Daemon**: Service yang jalan di background saat USB dicolok (Plug & Forget).
