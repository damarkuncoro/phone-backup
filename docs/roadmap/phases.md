# Arsitektur & Roadmap Pengembangan 📱

Roadmap ini mendokumentasikan evolusi **phone-backup** dari sebuah skrip sederhana menjadi engine backup Android kelas dunia.

---

# STATUS PROYEK: v0.4.1-stable (Final V4.0 Specifications & Native MTP) 🚀

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

## ✅ PHASE 40 — Auto-Backup Daemon (Plug & Forget)
*   **OnConnect Frequency**: Dukungan opsi penjadwalan `ScheduleFrequency::OnConnect`.
*   **Reactive Background Trigger**: Otomatis mendeteksi saat HP dicolok via USB dan memicu `trigger_on_connect_backup` tanpa intervensi pengguna.
*   **Tauri Event Toasts**: Emisi event `"auto-backup-started"` dan `"auto-backup-finished"` untuk notifikasi visual di GUI.

## ✅ PHASE 41 — Encrypted Metadata Engine (SQLCipher + Argon2id)
*   **Argon2id Key Derivation**: Fungsi `derive_database_key` pada `EncryptionEngine` untuk menghasilkan kunci enkripsi 256-bit dari kata sandi pengguna.
*   **Encrypted Repository Factory**: `SqliteRepositoryFactory::create_encrypted` dengan inisialisasi `PRAGMA key` otomatis pada connection customizer pool.

## ✅ PHASE 42 — Workspace Source & Test Isolation (src/ vs tests/)
*   **Pure Production Code**: Menghapus seluruh blok `#[cfg(test)]` dari folder `src/` seluruh crate workspace (`core/domain`, `core/application`, `adapters/filesystem`, `adapters/mock`, `infrastructure/database-sqlite`).
*   **Dedicated Test Suites**: Memisahkan test suite terisolasi ke direktori `tests/` (`domain_tests.rs`, `security_compression_test.rs`, `filesystem_adapter_test.rs`, `mock_adapter_test.rs`, `encrypted_repo_test.rs`).

## ✅ PHASE 43 — Wireless Companion Agent Protocol & Rust Adapter (`adapters/agent`)
*   **Crate Baru**: Pembuatan `adapters/agent` (`phone-backup-adapter-agent`) yang mengimplementasikan `ports::DevicePort`, `ports::ScannerPort`, `ports::DataProviderPort`, dan `ports::AppProviderPort`.
*   **Protokol Nirkabel**: Definisi kontrak data nirkabel (`AgentHandshake`, `AgentFileScanResponse`, `AgentStructuredDataResponse`, `AgentHeartbeat`).
*   **CLI Integration**: Dukungan penuh flag `--adapter agent` pada CLI `phone-backup`.
*   **Scaffolding Android APK**: Inisialisasi struktur proyek native `apps/android-agent/` (Kotlin + Jetpack Compose + CameraX + `AndroidManifest.xml`).
*   **Isolated Integration Tests**: Test suite terisolasi di `adapters/agent/tests/agent_adapter_test.rs`.

## ✅ PHASE 44 — Monorepo Architecture & Expert Storage Foundation
*   **Reorganisasi Monorepo**: Pemisahan yang bersih antara aplikasi (`apps/`) dan pustaka internal (`libs/`).
*   **Expert Chunking Library**: Implementasi `phone-backup-chunking` dengan strategi polimorfik (FastCDC v2020, FixedSize, FullFile).
*   **True Streaming Pipeline**: Pemrosesan data berukuran besar tanpa membebani RAM melalui bounded channels.

## ✅ PHASE 45 — V4.0 Specification: Logical & Physical Separation
*   **Two-Tier Deduplication**: Pemisahan Identitas Konten (Logical Chunks) dari Representasi Penyimpanan (Physical Objects).
*   **UUIDv7 Storage Key**: Penggunaan identitas acak untuk file fisik guna obfuscation dan privasi data di sistem file.
*   **Snapshot Commit Protocol**: Implementasi Manifest JSON immutable sebagai "Root of Trust" untuk integritas snapshot.
*   **Cloud Storage Multi-Provider**: Dukungan native untuk GCS, Azure Blob, dan S3 via integrasi OpenDAL yang diperluas.

## ✅ PHASE 46 — Native Pure-Rust MTP Adapter & macOS Conflict Resolution
*   **Native MTP Engine (`adapters/mtp`)**: Komunikasi low-level USB MTP via `mtp-rs` tanpa memerlukan mode Developer atau USB Debugging.
*   **macOS Conflict Resolver (`MtpConflictResolver`)**: Otomatis mendeteksi dan menghentikan daemon pengunci eksklusif (`ptpcamerad` / `PTPCamera` LaunchAgents) dengan penanganan sinyal `SIGSTOP`.
*   **Multi-Storage & Partition Scanner**: Pemindaian rekursif partisi `Internal shared storage` dengan isolasi otomatis folder privat Android 11+ (`Android/data`, `Android/obb`).
*   **Real Hardware Verification**: Teruji dan terverifikasi fungsional 100% pada smartphone fisik `Infinix NOTE 30 (Infinix X6833B)`.
*   **Modular Automation Suite**: Restrukturisasi direktori `scripts/` (DRY & SOLID) dengan unified runner CLI (`./scripts/run.sh`).

## ✅ PHASE 47 — Specialist Data & Media Domain Crates
*   **Contacts Specialist Engine (`libs/data/contacts`)**: Parser & writer standar vCard RFC 6350, fuzzy duplicate merger (Jaro-Winkler), dan contact diff matrix.
*   **Messages & Call Analytics Engine (`libs/data/messages`)**: Formatter XML standar Android (*SMS Backup & Restore*), HTML chat transcript viewer, ekstraktor kode OTP, klasifikasi pesan perbankan/promo, dan metrik statistik panggilan.
*   **App Security & Split APK Engine (`libs/data/apps`)**: Parser AXML (binary AndroidManifest.xml) murni dalam Rust, auditor izin berbahaya (*Dangerous Permissions*), penilaian skor risiko privasi, dan assembler Split APK bundle.
*   **WhatsApp Specialist Provider (`libs/apps/whatsapp`)**: Pemindai Scoped Storage Android 11–15 & Legacy, pengindeks media berkas bertanda waktu, dan pembuat arsip HTML offline interaktif.
*   **Image Intelligence Engine (`libs/media/image`)**: Piramida thumbnail multi-resolusi (*micro, thumb, preview*), deteksi keburaman (*Laplacian sharpness*), dan perceptual hashing (*dHash & aHash*).
*   **Audio Intelligence & Waveform Engine (`libs/media/audio`)**: Sniffer format audio (MP3, Opus, Ogg, M4A, FLAC, AMR), parser ID3/Vorbis, klasifikasi Voice Notes/Call Recordings, dan generator kurva gelombang amplitudo (*normalized waveform peaks*).

## ✅ PHASE 48 — Specialist CLI Subcommands Integration
*   `phone-backup export`: Ekspor kontak ke vCard/CSV, SMS ke XML/HTML/CSV/JSON, dan log panggilan ke JSON/Stats.
*   `phone-backup audit`: Audit keamanan dan penilaian risiko aplikasi/APK.
*   `phone-backup whatsapp`: Menampilkan lokasi penyimpanan WhatsApp dan mengekspor arsip chat HTML offline.
*   `phone-backup audio`: Memeriksa metadata audio dan menghasilkan visualisasi ASCII grafik gelombang waveform.

## ✅ PHASE 49 — Desktop GUI Multimedia & Security Lab
*   **WhatsApp Archive Explorer**: Pratinjau interaktif arsip chat WhatsApp dengan live iframe dan download HTML offline.
*   **App Security Risk Auditor**: Antarmuka analisis izin berbahaya APK dan kalkulasi skor risiko keamanan.
*   **Media Lab**: Visualisasi 60-point waveform peaks audio dan evaluasi tingkat ketajaman citra foto.
*   **100% Code Quality**: Mematuhi aturan $\le 200$ baris per file di seluruh workspace dan 100% test isolation lulus.

## ✅ PHASE 50 — Production Hardening & Real-Device Safety Engine
*   **Graceful SIGPIPE Handling**: Reset signal handler UNIX `SIGPIPE` pada CLI untuk mencegah crash `Broken pipe (os error 32)`.
*   **Deep App Metadata Resolver**: Integrasi kueri `dumpsys package` untuk membaca nama aplikasi ramah pengguna dan `versionName` akurat.
*   **Continuous Thermal Safety Guard**: Pemantauan berkala suhu baterai dan daya pada setiap batch pemrosesan file untuk mencegah *overheating* HP.
*   **MediaStore Fast Scanner**: Kueri multi-kategori MediaStore Android (`image`, `video`, `audio`, `file`) untuk pemindaian instan dalam milidetik.
*   **Direct Contact Restorer**: Injeksi kontak langsung ke `content://com.android.contacts/data` via ADB Content Provider.
*   **Session-based Split APKs Installer**: Pemasangan paket aplikasi modern (*App Bundles / APKS*) dengan sesi atomik (`pm install-create`, `pm install-write`, `pm install-commit`).
*   **Live Cloud Connection Verification GUI**: Uji konektivitas live ke AWS S3/MinIO via OpenDAL Operator dari Settings Tab.

## ✅ PHASE 51 — Wireless QR Pairing & Multi-Socket ADB Streamer Engine
*   **Wireless Companion Agent QR Pairing GUI**: Dashboard interaktif perender QR Code pairing token dan deteksi IP LAN lokal (`phonebackup://pair?ip=...&port=...&token=...`).
*   **Multi-Socket ADB Worker Pool (`libs/adapters/adb`)**: Pool soket multi-stream berkinerja tinggi dengan manajemen konkurensi RAII (`AdbWorkerGuard`).
*   **Desktop System Tray Daemon (`apps/gui/src-tauri`)**: Integrasi `TrayManager` Tauri 2.0 untuk background daemon dan auto-backup USB connect.
*   **Emergency Recovery Kit Generator (`apps/cli`)**: Command `phone-backup recovery-kit` penghasil dokumen cetak mandiri pemulihan offline zero-knowledge.
*   **Production Release Packaging Pipeline**: Script `./scripts/build_release.sh` penghasil binary stripped LTO dan checksum SHA-256 terverifikasi.

## ✅ PHASE 52 — Data Optimization Engine & Native WebDAV Storage
*   **WebDAV & Nextcloud Storage Adapter (`libs/adapters/opendal`)**: Integrasi OpenDAL WebDAV provider (`services-webdav`) dengan kredensial fleksibel.
*   **WebDAV Desktop GUI & CLI Integration**: Tab konfigurasi multi-cloud (Local, S3, WebDAV, Mock) dan flags `--webdav-endpoint`, `--webdav-user`, `--webdav-password`.
*   **Categorized Zstd Dictionary Pools (`libs/storage/compression`)**: Kamus terlatih spesifik domain (`android-vcard-v1`, `android-whatsapp-v1`, `android-xml-v1`, `android-sqlite-v1`, `android-json-v1`).
*   **Page-Aware SQLite Zero-Fill Preconditioning**: Filter *lossless* yang men-zeroing halaman *freelist* SQLite untuk meningkatkan kompresibilitas $>40\%$.
*   **Dynamic FastCDC TransferMedium Tuning (`libs/storage/chunking`)**: Pengaturan otomatis target chunk FastCDC (`HighSpeedLocal`, `WirelessAgent`, `CloudWebDav`, `ThermalConstrained`) dengan flag `-m/--medium`.

## ✅ PHASE 53 — WhatsApp Live QR Sync, Granular Selective Restore & USB Troubleshooting Wizard
*   **WhatsApp Live Multi-Device Sync & Offline Viewer GUI**: Integrasi penuh protokol sinkronisasi chat WhatsApp multi-device (Baileys) langsung ke antarmuka Desktop GUI, dilengkapi monitor QR code, status live sync, dan HTML offline chat archive reader.
*   **Interactive USB Connection & Troubleshooting Wizard**: Panduan visual interaktif langkah-demi-langkah per merk smartphone (Vivo, Xiaomi, Samsung, Oppo) dan tombol aksi satu klik **Auto-Fix macOS USB Conflict** (`ptpcamerad`).
*   **Granular & Selective Category Restore**: Modal pemulihan fleksibel yang memungkinkan pengguna memilih kategori data spesifik (Kontak, SMS, Riwayat Telepon, Media Foto/Video, atau APK) sebelum restore dijalankan.
*   **Cloud Storage 1-Click Presets**: Tombol preset konfigurasi instan untuk **AWS S3**, **Cloudflare R2**, **MinIO Local**, dan **Backblaze B2** di tab pengaturan Storage.
*   **History UI Modular Refactoring**: Pemecahan antarmuka riwayat snapshot menjadi subkomponen independen (`DevicePickerBar`, `SnapshotCard`, `SelectiveRestoreModal`) yang memastikan 100% file strictly $\le 200$ baris.

## ✅ PHASE 54 — Android Companion Agent APK (Kotlin & Jetpack Compose)
*   **Modern Jetpack Compose UI**: Tampilan antarmuka native Android modern dengan pemantauan status koneksi real-time (`AgentHomeScreen`, `MainActivity.kt`).
*   **CameraX & ML Kit QR Code Scanner**: Scanner kamera live (`QrScannerView.kt`) untuk memindai token pairing nirkabel dari desktop (`phonebackup://pair?...`).
*   **High-Speed Ktor WebSocket Streaming Client**: Komunikasi real-time dua arah (`AgentNetworkClient.kt`) untuk transmisi metadata perangkat dan streaming data ke desktop.
*   **Unified Android Content Resolver Extractor**: Ekstraksi terpusat untuk Kontak, SMS, Call Logs, dan metrik storage Android (`AndroidDataExtractor.kt`).
*   **Foreground Service Execution**: Layanan latar belakang (`BackupAgentService.kt`) dengan notifikasi persisten agar backup tetap berjalan lancar saat layar mati.
*   **Gradle Packaging & CI/CD Pipeline**: Konfigurasi `settings.gradle.kts`, `gradle-wrapper.properties`, dan `./scripts/build_android_agent.sh` untuk otomasi kompilasi APK.

## ✅ PHASE 55 — Deep iOS Sync & Apple File Conduit (AFC) Engine
*   **Apple File Conduit (AFC) Integration (`libs/adapters/ios/src/afc.rs`)**: Komunikasi low-level dengan subsistem file Apple via protocol AFC untuk membaca direktori DCIM dan media iPhone.
*   **Zero-Jailbreak Media Scanner**: Pemindaian rekursif foto & video iPhone (`IMG_*.JPG`, `IMG_*.HEIC`, `IMG_*.MOV`, `IMG_*.PNG`) tanpa memerlukan jailbreak.
*   **Streaming File Reader & Hardware Fallback**: Ekstraksi streaming byte media iOS langsung via buffer channel RAII.
*   **Device Marketing Identifier Mapping**: Konversi kode model hardware Apple internal (e.g. `iPhone15,3` $\rightarrow$ `iPhone 14 Pro Max`, `iPhone17,1` $\rightarrow$ `iPhone 16 Pro`).
*   **Isolated Integration Test Suite (`tests/ios_adapter_test.rs`)**: Pengujian otomatis 100% lulus untuk metadata lockdown, capability matrix, dan scanning DCIM.

## ✅ PHASE 57 — Specialist Scanner Crate (`libs/scanner` -> `phone-backup-scanner`)
*   **Dedicated Specialist Library (`libs/scanner`)**: Pemisahan seluruh engine scanner menjadi crate independen (`phone-backup-scanner`) yang dapat digunakan lintas adapter (ADB, MTP, Filesystem).
*   **Multi-Threaded Concurrent Scanner (`libs/adapters/adb`)**: Pemindaian paralel menggunakan `std::thread::scope` yang mengeksekusi MediaStore queries (Gambar, Video, Audio) dan FileSystem Crawler secara bersamaan dengan deduplikasi deterministik `BTreeMap`.
*   **Smart Noise & Junk Filter (`NoiseFilter`)**: Filter pintar berkecepatan tinggi yang mengabaikan direktori sampah Android (`.thumbnails`, `.cache`, `cache/`, `.Trash`, `.recycle`, `lost+found`, `.nomedia`, `*.tmp`, `*.part`, `*.crdownload`).
*   **Intelligent File Classifier (`FileClassifier`)**: Pengklasifikasi otomatis berkas ke kategori terstruktur (`Photos`, `Videos`, `Audio`, `Documents`, `WhatsApp`, `APKs`, `Downloads`, `System`, `Other`) berbasis path, MIME, dan ekstensi.
*   **Real-time Scan Metrics Tracker (`ScanMetrics`)**: Pelacakan waktu pemindaian, jumlah direktori yang dipindai, serta throughput kecepatan pemindaian (*files/sec*).
*   **Deterministic File Merger & Incremental Diffing**: `FileMerger` dan `IncrementalScanner` untuk pemrosesan snapshot cerdas.
*   **Rich Desktop Scanner HUD (`apps/gui/ui`)**: Visualisasi live scan status, breakdown badges per kategori berkas, indikator throughput, dan alert banner peringatan non-fatal Scoped Storage.
*   **Tauri IPC Bridge & Isolated Tests**: Command baru `scan_device_detailed` dengan 100% test passing pada `scanner_specialist_test.rs`, `scanner_enhancement_test.rs`, dan seluruh test suite monorepo.

---

# STATUS: v1.0.0 Production Ready 🏆
Seluruh fitur pada roadmap dari Phase 01 hingga Phase 57 telah selesai diimplementasikan, diuji, dan dipaketkan.


