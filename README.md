# phone-backup 📱

A high-performance, secure, and professional Android backup platform written in Rust.

**phone-backup** is an enterprise-grade backup engine designed with **Clean Architecture**, **Hexagonal Architecture**, and **SOLID Design Principles**. It handles USB & Wi-Fi device discovery, intelligent indexing, versioned snapshots, military-grade encryption, and storage-efficient deduplication.

---

## 📑 Documentation Index

| Guide / Document | Description |
| :--- | :--- |
| 📖 **[Project Wiki](wiki/Home.md)** | Basis pengetahuan lengkap modular (Getting Started, CLI, GUI, Architecture, Security, Storage, Wireless Agent, FAQ, Testing). |
| 🛠 **[Complete How-To Guide](docs/guides/README.md)** | Panduan operasional CLI, Tauri Desktop GUI, S3/R2 Cloud, Wireless Agent, Scheduler, dan Troubleshooting FAQ. |
| 🏗 **[Software Architecture Document (SAD)](docs/architecture/README.md)** | Hexagonal ports & adapters, CAS deduplication pipeline, FastCDC, and security architecture. |
| 🚀 **[Project Roadmap & Phases](docs/roadmap/phases.md)** | Detailed changelog from Phase 01 to Phase 50+ (*Final V4.0 Specifications*). |
| 🧪 **[Comprehensive Feature Verification Report](docs/reports/comprehensive-feature-verification-report.md)** | Laporan resmi pengujian end-to-end seluruh 19 crates & fitur platform. |
| 📝 **[Technical Review & Hardware Assessment](docs/reports/hardware-review.md)** | Laporan pengujian langsung pada HP fisik nyata (Vivo V2317 Android 15, Infinix NOTE 30, Xiaomi). |
| ⚠️ **[Known Limitations](docs/guides/limitations.md)** | Boundaries of ADB vs MTP vs Companion Agent. |

---

## 🚀 Key Capabilities & Specialist Engines

### 🧠 Core Storage, Deduplication & Safety
- **Content-Addressed Storage (CAS)**: Sub-file FastCDC chunking with dynamic Zstd compression.
- **Continuous Thermal Safety Guard**: Pemantauan real-time suhu baterai dan daya ponsel di setiap batch upload.
- **Zero-Knowledge Encryption**: X25519 asimetris (*age*) dan SQLCipher AES-256 GCM (Argon2id KDF).
- **Emergency Recovery Kit**: Lembar dokumen cetak mandiri untuk pemulihan dingin (*Cold Storage*).

### 📱 Specialist Domain Crates
- **Contacts (`phone-backup-contacts`)**: Parser & writer vCard RFC 6350, CSV export, dan direct ADB injection.
- **Messages & Calls (`phone-backup-messages`)**: Ekspor standar XML (*SMS Backup & Restore*), HTML viewer, dan agregasi analitik riwayat panggilan.
- **WhatsApp (`phone-backup-whatsapp`)**: Live Multi-Device QR Sync, pemindai Scoped Storage Android 11–15 & generator arsip chat HTML offline.
- **App Security Audit (`phone-backup-apps`)**: Pure-Rust AXML parser, evaluator izin berbahaya, dan Session-based Split APK Installer.
- **Media Lab (`phone-backup-image` & `phone-backup-audio`)**: Deteksi keburaman foto (*Laplacian sharpness*), perceptual hash (*dHash/aHash*), dan visualisasi kurva 60-point waveform audio.

---

## 🛠 Command Line Interface (CLI)

```bash
# 1. Diagnostik & Deteksi Perangkat
phone-backup doctor
phone-backup -a adb devices
phone-backup -a adb device-info <DEVICE_ID>

# 2. Backup & Restore Data
phone-backup -a adb backup <DEVICE_ID> -i /sdcard/Documents
phone-backup restore <SNAPSHOT_ID> -t ./restored_folder

# 3. Ekspor Data Spesialis
phone-backup -a adb export contacts <SNAPSHOT_ID> --format vcard --output contacts.vcf
phone-backup -a adb export sms <SNAPSHOT_ID> --format xml --output sms_backup.xml
phone-backup -a adb export calls <SNAPSHOT_ID> --format stats

# 4. WhatsApp Archive & App Audit
phone-backup whatsapp paths
phone-backup whatsapp export --output whatsapp_archive.html
phone-backup audit --apk app_to_check.apk

# 5. Media Lab & Emergency Recovery Kit
phone-backup audio waveform voice_note.opus
phone-backup recovery-kit --output emergency_recovery_kit.html
```

---

## 🖥 Desktop GUI (Tauri + React)

```bash
# Jalankan Desktop GUI mode pengembangan
cargo tauri dev
```

---

## 🧪 Testing & Quality Standards

- **100% Modularity & Clean Architecture**: Setiap file di seluruh repositori strictly $\le 200$ baris.
- **Workspace Test Isolation**: Seluruh test suites tersimpan mandiri di direktori `tests/` di setiap crate.
- **Pengujian Keseluruhan**: `cargo test --all` across 19 Crates + CLI + GUI $\rightarrow$ **100% LULUS (0 failed)**.
- **UI TypeScript Build**: `npm run build` $\rightarrow$ **100% LULUS (0 errors)**.

---

## 📄 License
MIT License. Developed with ❤️ for the Android Community.
