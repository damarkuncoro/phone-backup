# 💡 Feature Requests & Future Ideas

This document tracks completed features, active initiatives, and future roadmap ideas for the **phone-backup** platform.

---

## 🚀 Implemented Features (v0.3.5)

- [x] **Desktop GUI (Tauri Dashboard)**: Modular UI with Tailwind CSS, Chart.js, and Native Web Components.
- [x] **Live Device File Manager**: Real-time browsing, remote download, upload, rename, move, delete, and on-device SHA-256 calculation.
- [x] **Visual Snapshot Diffing**: Side-by-side snapshot comparison matrix (**New**, **Modified**, **Deleted**, **Unchanged**).
- [x] **Automatic Periodic Backups & Daemon**: Scheduled backup runner (`daily`, `weekly`) and reactive `OnConnect` trigger upon USB connection.
- [x] **Asymmetric Public-Key Encryption (`age` X25519)**: Zero-knowledge automated backups with public key encryption and offline private key decryption.
- [x] **Encrypted Catalog Database (SQLCipher + Argon2id)**: Full database-level encryption for `backup.db` using 256-bit keys derived via Argon2id KDF.
- [x] **Block-level Deduplication (FastCDC)**: Content-defined chunking for variable-sized sub-file deduplication.
- [x] **Full-Text Search (FTS5)**: Instant global search across backed-up files, contacts, and SMS messages (`search`, `contacts`, `sms`).
- [x] **vCard RFC 6350 Contacts Export**: Universal format export supporting Google Contacts, Apple Contacts, Outlook, and Thunderbird.
- [x] **Wireless Companion Agent Protocol (`adapters/agent`)**: Extensible adapter and Android APK scaffolding (`apps/android-agent`) for wireless backups over Wi-Fi.
- [x] **Cloud Object Storage (OpenDAL)**: Native support for AWS S3, Cloudflare R2, and MinIO backends.
- [x] **Pure Production & Isolated Test Architecture**: 100% clean `src/` modules with dedicated `tests/` directories across the workspace.

---

## 📱 Active & Future Initiatives (v1.0.0+)

### 🔌 Connectivity & Transports
- [ ] **Android Companion Agent APK (Full Wi-Fi Client)**: Complete on-device Jetpack Compose UI with CameraX QR pairing and gRPC/WebSocket streaming.
- [ ] **MTP Adapter**: Direct file access via Media Transfer Protocol for basic media copy without requiring ADB/Developer Mode.
- [ ] **iOS Adapter**: Photo and contact backup for iPhones and iPads using `libimobiledevice` bindings.

### ☁️ Cloud & Synchronization
- [ ] **GUI Cloud Sync Settings Panel**: Configure S3, Cloudflare R2, and Google Drive directly from the Desktop GUI Settings tab.
- [ ] **Remote Repository Sync**: Bidirectional sync for the local `objects/` directory with a remote repository (rsync-like).

### 🧠 Intelligence & UX
- [ ] **Duplicate Finder & Cleanup UI**: Interactive interface to inspect and prune duplicate media files across multiple smartphones.
- [ ] **Built-in Media Player / Viewer**: Preview encrypted photos, audio, and videos directly inside the GUI without restoring to disk.
- [ ] **Tauri System Tray Daemon**: Background tray indicator for seamless plug-and-forget backup automation.

---
*phone-backup — Engineered with Rust, Clean Architecture, and Military-Grade Security.*
