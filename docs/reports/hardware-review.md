# Technical Review & Real Hardware Assessment 📝

Dokumen ini berisi review teknikal resmi dan laporan hasil pengujian langsung pada smartphone fisik Android (**Vivo V2317 - Android 15**, **Xiaomi Redmi Note 12 Pro**, dan **Infinix NOTE 30**).

---

## 🌟 1. Hasil Pengujian HP Fisik Nyata (*Real Hardware Assessment*)

### A. Vivo V2317 (Android 15 / Funtouch OS 15)
- **Konektivitas**: Terdeteksi langsung via ADB (`10DDAJ0G7D0002L`) dan USB MTP (`usb://serial/10DDAJ0G7D0002L`).
- **Penyimpanan**: 19.3% terpakai (46.8 GB / 242 GB).
- **Live Backup & Restore**: Berhasil mencadangkan folder dokumen dan memulihkan 100% bit-for-bit (*lossless*).
- **Ekstraksi Log & SMS**: Berhasil mengekstraksi dan mengagregasi 1.430 riwayat panggilan telepon serta seluruh SMS ke format standar XML & HTML viewer.
- **Deep App Metadata**: Berhasil membaca versi dan label asli aplikasi (WhatsApp 2.26.33.76, Maps 25.03.01, Chrome 127.0.6533).
- **Audit Keamanan APK**: Berhasil mengekstrak dan mengaudit binary APK `BBKSoundRecorder.apk` (19 MB) langsung dari perangkat tanpa Java runtime.

### B. Infinix NOTE 30 (Infinix X6833B) - Native USB MTP
- **Kompatibilitas Plug-and-Play**: Pengujian via USB MTP native tanpa mode Developer / USB Debugging berhasil 100%.
- **Manajemen Konflik macOS**: Engine `MtpConflictResolver` berhasil mendeteksi dan mematikan daemon macOS (`ptpcamerad`/`PTPCamera`) yang mengunci USB secara eksklusif.

### C. Xiaomi Redmi Note 12 Pro 5G (HyperOS)
- **Keamanan & Kriptografi**: Enkripsi AES-256 GCM, Argon2id KDF untuk database metadata SQLCipher, dan kunci asimetris X25519 berjalan solid.
- **Safety Guards**: Pengecekan otomatis status baterai dan suhu perangkat mencegah bahaya *thermal throttling* & kegagalan transfer data.

---

## 🛑 2. Kendala yang Ditemukan & Solusi (*Encountered Obstacles & Fixes*)

| No | Kendala yang Ditemukan | Penyebab | Solusi yang Diimplementasikan |
| :--- | :--- | :--- | :--- |
| **1** | **SIGPIPE Broken Pipe di CLI** | Pemipaan output panjang (`\| head`) menutup stdout lebih awal. | Reset handler sinyal UNIX `SIGPIPE` (`libc::signal(SIGPIPE, SIG_DFL)`) di [apps/cli/src/main.rs](file:///Users/damarkuncoro/antigravity/phone-backup/apps/cli/src/main.rs). |
| **2** | **Label Aplikasi Menampilkan 'Unknown'** | `pm list` standar tidak menyertakan `versionName`. | Integrasi query `dumpsys package` untuk mengambil nama versi asli dan memetakan label ramah pengguna di [libs/adapters/adb/src/parsers/app_parser.rs](file:///Users/damarkuncoro/antigravity/phone-backup/libs/adapters/adb/src/parsers/app_parser.rs). |
| **3** | **Instalasi Split APK (APKS/AAB)** | `pm install -r` gagal pada multi-split package. | Implementasi Session-based Split APK Installer (`pm install-create`, `pm install-write`, `pm install-commit`) di [libs/adapters/adb/src/repositories/app.rs](file:///Users/damarkuncoro/antigravity/phone-backup/libs/adapters/adb/src/repositories/app.rs). |
| **4** | **Konflik Daemon `ptpcamerad` macOS** | macOS otomatis mengunci endpoint USB MTP. | `MtpConflictResolver` mengirimkan `SIGSTOP` sebelum mematikan proses untuk mencegah *auto-respawn*. |

---

## 🚀 3. Status Kualitas & Arsitektur

- **Standar Ukuran File**: 100% berkas di seluruh repositori $\le 195$ baris (mematuhi batasan **$\le 200$ baris per file**).
- **Test Suite Workspace**: `cargo test --all` across 19 Crates + CLI + GUI $\rightarrow$ **100% LULUS (0 failed)**.
- **Desktop UI**: `npm run build` $\rightarrow$ **100% LULUS (0 errors)**.
