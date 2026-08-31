# Panduan Operasional Lengkap: phone-backup 📱

Dokumen ini berisi panduan penggunaan mendalam untuk mengoperasikan platform **phone-backup** melalui **Desktop GUI (Tauri)** dan **CLI (Command Line Interface)**, lengkap dengan konfigurasi lingkungan ADB, konektivitas Cloud S3 (OpenDAL), agen Android nirkabel (*Wireless Companion Agent*), enkripsi tingkat lanjut, penjadwalan otomatis, dan pemecahan masalah.

---

## 📑 Daftar Isi
1. [Persiapan Lingkungan & Konfigurasi ADB](#1-persiapan-lingkungan--konfigurasi-adb)
2. [Panduan Penggunaan Desktop GUI (Tauri Dashboard)](#2-panduan-penggunaan-desktop-gui-tauri-dashboard)
3. [Panduan Penggunaan CLI (Master Command Line)](#3-panduan-penggunaan-cli-master-command-line)
   - [3.1 Diagnostik Sistem (Doctor Check)](#31-diagnostik-sistem-doctor-check)
   - [3.2 Deteksi Perangkat & Matriks Kapabilitas](#32-deteksi-perangkat--matriks-kapabilitas)
   - [3.3 Pemindaian Sistem File Real-Time (Scan)](#33-pemindaian-sistem-file-real-time-scan)
   - [3.4 Inspeksi Aplikasi & Ekspor APK](#34-inspeksi-aplikasi--ekspor-apk)
   - [3.5 Eksekusi Backup (Penuh & Selektif)](#35-eksekusi-backup-penuh--selektif)
   - [3.6 Manajemen Keamanan Kunci Asimetris (age X25519)](#36-manajemen-keamanan-kunci-asimetris-age-x25519)
   - [3.7 Penyimpanan Cloud S3 / Cloudflare R2 / MinIO (OpenDAL)](#37-penyimpanan-cloud-s3--cloudflare-r2--minio-opendal)
   - [3.8 Pencarian Global (FTS5 Files, Contacts & SMS)](#38-pencarian-global-fts5-files-contacts--sms)
   - [3.9 Ekspor & Manajemen Kontak (vCard RFC 6350)](#39-ekspor--manajemen-kontak-vcard-rfc-6350)
   - [3.10 Inspeksi Foto & Metadata EXIF / Kamera](#310-inspeksi-foto--metadata-exif--kamera)
   - [3.11 Kloning Antar Perangkat (Direct Device Clone)](#311-kloning-antar-perangkat-direct-device-clone)
   - [3.12 Penjadwalan Backup Otomatis & Daemon (Schedule)](#312-penjadwalan-backup-otomatis--daemon-schedule)
   - [3.13 Verifikasi Integritas & Pemeliharaan (Verify & GC)](#313-verifikasi-integritas--pemeliharaan-verify--gc)
   - [3.14 Restorasi Snapshot (Full & Filtered)](#314-restorasi-snapshot-full--filtered)
4. [Pencadangan Nirkabel: Android Companion Agent (Wi-Fi)](#4-pencadangan-nirkabel-android-companion-agent-wi-fi)
5. [Enkripsi Metadata Database (SQLCipher + Argon2id)](#5-enkripsi-metadata-database-sqlcipher--argon2id)
6. [Catatan Khusus Pengguna Xiaomi / Redmi / POCO (MIUI / HyperOS)](#6-catatan-khusus-pengguna-xiaomi--redmi--poco-miui--hyperos)
7. [Panduan Strategis: Backup Khusus Kontak (< 5 Detik)](#7-panduan-strategis-backup-khusus-kontak--5-detik)
8. [Panduan & Analisis: Pencadangan Tanpa USB Debugging](#8-panduan--analisis-pencadangan-tanpa-usb-debugging)
9. [Panduan Pemecahan Masalah (Troubleshooting & FAQ)](#9-panduan-pemecahan-masalah-troubleshooting--faq)

---

## 🛠 1. Persiapan Lingkungan & Konfigurasi ADB

Sebelum menghubungkan smartphone Android via kabel USB, pastikan `adb` (Android Debug Bridge) telah terinstal dan terdaftar dalam `PATH`.

### Konfigurasi PATH (macOS / Linux)
Tambahkan lokasi `platform-tools` ke file `.zshrc` atau `.bashrc`:
```bash
export PATH=$PATH:$HOME/Library/Android/sdk/platform-tools
```

### Konfigurasi PATH (Windows PowerShell)
```powershell
$env:Path += ";C:\Users\<Username>\AppData\Local\Android\Sdk\platform-tools"
```

Verifikasi koneksi fisik perangkat Android:
```bash
adb devices -l
```
*Contoh Output:*
```text
List of devices attached
fynrorjncy6x4xib       device usb:20-2.4 product:ruby_id model:22101316G device:ruby transport_id:1
```

---

## 🖥 2. Panduan Penggunaan Desktop GUI (Tauri Dashboard)

Desktop GUI dikembangkan menggunakan **Tauri**, **Tailwind CSS**, dan **Native Web Components** modular.

### 2.1 Menjalankan Aplikasi GUI
Masuk ke root repository dan jalankan perintah:
```bash
cargo tauri dev
```

### 2.2 Fitur Utama GUI
1. **Live Device File Manager**:
   - Jelajahi folder memori internal HP secara real-time.
   - **Download File**: Klik tombol **Download** untuk mengunduh file remote dari HP langsung ke komputer lokal via command `download_from_device`.
   - **Upload File**: Unggah file lokal dari komputer ke lokasi direktori HP.
   - **File Operations**: Rename (ubah nama), Copy/Move (salin/pindah), Delete (hapus), dan hitung hash SHA-256 instan.
2. **Visual Snapshot Diffing Matrix**:
   - Bandingkan dua snapshot secara visual.
   - Matriks akan secara otomatis membedakan status entri:
     - 🟢 **New** (File baru ditambahkan).
     - 🟡 **Modified** (File diubah atau ukuran berubah).
     - 🔴 **Deleted** (File dihapus).
     - ⚪ **Unchanged** (File tidak berubah).
3. **Installed Apps Explorer & APK Exporter**:
   - Inspeksi seluruh aplikasi terinstal di smartphone Android.
   - Ekspor file `.apk` secara individu atau batch langsung ke folder lokal.
4. **Android Data Explorer**:
   - Jelajahi database **Contacts**, **SMS Messages**, dan **Call Logs** yang telah di-backup dengan dukungan pencarian cepat.
5. **Real-time Progress HUD & Safety Guard**:
   - Progress bar melayang (HUD) menampilkan status transfer file secara real-time.
   - Peringatan otomatis jika baterai HP $< 10\%$ atau suhu perangkat $> 45^\circ\text{C}$.

---

## 💻 3. Panduan Penggunaan CLI (Master Command Line)

CLI menyediakan perintah yang cepat, andal, dan dapat diotomatisasi melalui skrip shell atau cron job.

### 3.1 Diagnostik Sistem (Doctor Check)
Periksa kesehatan ADB, database SQLite, dan konektivitas storage:
```bash
phone-backup doctor
```
*Output:*
```text
🩺 Phone Backup Doctor - System Diagnostic
-----------------------------------------
Checking ADB installation... ✅ FOUND (Android Debug Bridge version 1.0.41)
Checking connected devices... ✅ 1 device(s) detected
Checking workspace integrity... ✅ backup.db found
Checking storage connectivity... ✅ storage reachable

System is ready for backup operations!
```

### 3.2 Deteksi Perangkat & Matriks Kapabilitas
Tampilkan daftar HP yang terhubung:
```bash
phone-backup --adapter adb devices
```

Tampilkan informasi detail perangkat dan matriks kapabilitas izin:
```bash
phone-backup --adapter adb device-info <DEVICE_ID>
```
*Contoh:* `phone-backup --adapter adb device-info fynrorjncy6x4xib`

### 3.3 Pemindaian Sistem File Real-Time (Scan)
Pindai isi memori internal HP tanpa menyimpan ke backup (*Dry-Run*):
```bash
phone-backup --adapter adb scan <DEVICE_ID>
```

### 3.4 Inspeksi Aplikasi & Ekspor APK
Tampilkan seluruh aplikasi Android terinstal di HP:
```bash
phone-backup --adapter adb apps <DEVICE_ID>
```

### 3.5 Eksekusi Backup (Penuh & Selektif)
#### A. Backup Penuh (Terenkripsi Kata Sandi)
```bash
phone-backup --adapter adb backup -p "KataSandiSuperKuat" <DEVICE_ID>
```

#### B. Backup Selektif Folder Tertentu
Hanya mem-backup folder tertentu (misal: `/storage/emulated/0/DCIM/Screenshots`):
```bash
phone-backup --adapter adb backup -i /storage/emulated/0/DCIM/Screenshots -p "KataSandiSuperKuat" <DEVICE_ID>
```

#### C. Mengecualikan Folder / Pola Tertentu (Exclude)
```bash
phone-backup --adapter adb backup -e "*.tmp" -e "*/cache/*" -p "KataSandiSuperKuat" <DEVICE_ID>
```

---

### 3.6 Manajemen Keamanan Kunci Asimetris (age X25519)
Memungkinkan backup otomatis tanpa menyimpan kata sandi master di komputer backup (*Zero-Knowledge Storage*):

1. **Generate Pasangan Kunci**:
   ```bash
   phone-backup keygen
   ```
   *Output:*
   - **Public Key**: `age1...` (Gunakan untuk mengunci backup).
   - **Secret Key**: `AGE-SECRET-KEY-1...` (Gunakan untuk memulihkan/dekripsi data).

2. **Jalankan Backup dengan Public Key**:
   ```bash
   phone-backup --adapter adb backup --pubkey "age1..." <DEVICE_ID>
   ```

3. **Restorasi dengan Secret Key**:
   ```bash
   phone-backup --privkey "AGE-SECRET-KEY-1..." restore <SNAPSHOT_ID>
   ```

---

### 3.7 Penyimpanan Cloud S3 / Cloudflare R2 / MinIO (OpenDAL)

Platform **phone-backup** mendukung penyimpanan objek langsung ke backend cloud berbasis S3-compatible melalui flag `--storage opendal` atau *environment variables*.

#### A. Konfigurasi via CLI Flags:
```bash
phone-backup --storage opendal \
  --s3-bucket "my-phone-backups" \
  --s3-region "auto" \
  --s3-endpoint "https://<account-id>.r2.cloudflarestorage.com" \
  --s3-access-key "<ACCESS_KEY_ID>" \
  --s3-secret-key "<SECRET_ACCESS_KEY>" \
  --adapter adb backup -p "KataSandiSuperKuat" <DEVICE_ID>
```

#### B. Konfigurasi via Environment Variables:
```bash
export S3_BUCKET="my-phone-backups"
export S3_REGION="us-east-1"
export S3_ENDPOINT="https://s3.amazonaws.com"
export S3_ACCESS_KEY="AKIAIOSFODNN7EXAMPLE"
export S3_SECRET_KEY="wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"

# Jalankan backup langsung ke Cloud
phone-backup --storage opendal --adapter adb backup -p "KataSandiSuperKuat" <DEVICE_ID>
```

---

### 3.8 Pencarian Global (FTS5 Files, Contacts & SMS)

Anda dapat mencari berkas, kontak telepon, atau isi pesan SMS secara instan menggunakan pencarian Full-Text Search (FTS5):

#### A. Pencarian Berkas (Files):
```bash
phone-backup search "laporan_keuangan.pdf"
```

#### B. Pencarian Kontak (Contacts):
```bash
phone-backup contacts "Damar"
phone-backup contacts "+62"
```

#### C. Pencarian Pesan SMS (SMS / OTP):
```bash
phone-backup sms "Bank"
phone-backup sms "Verification"
```

---

### 3.9 Ekspor & Manajemen Kontak (vCard RFC 6350)

Platform secara otomatis mengekstrak seluruh data buku telepon Android melalui Content Provider (`content://com.android.contacts/data`) pada setiap sesi backup.

- **Format Standar**: Kompatibel dengan **vCard 4.0 / 3.0 (RFC 6350)**.
- **Dukungan Bidang Data**: Nama lengkap, multi-nomor telepon (E.164), email, alamat rumah/kantor, organisasi/jabatan, dan catatan kontak.
- **Ekspor vCard**: Melalui Desktop GUI (Tab **👥 Android Data Explorer**) klik tombol **Export vCard** untuk mendapatkan file `.vcf`.

---

### 3.10 Inspeksi Foto & Metadata EXIF / Kamera

Lihat galeri foto dan metadata detail kamera (resolusi, aperture, ISO, timestamp) yang telah di-backup:
```bash
phone-backup photos <DEVICE_ID>
```

---

### 3.11 Kloning Antar Perangkat (Direct Device Clone)

Salin data langsung dari HP sumber ke HP tujuan melalui CLI:
```bash
phone-backup clone <SOURCE_DEVICE_ID> <TARGET_DEVICE_ID>
```

---

### 3.12 Penjadwalan Backup Otomatis & Daemon (Schedule)

Kelola backup berkala atau otomatis saat HP terhubung:

#### A. Menambahkan Jadwal Backup:
```bash
# Jadwal harian (daily)
phone-backup schedule add <DEVICE_ID> --frequency daily

# Jadwal mingguan (weekly)
phone-backup schedule add <DEVICE_ID> --frequency weekly

# Jadwal otomatis setiap kali HP dicolok (OnConnect)
phone-backup schedule add <DEVICE_ID> --frequency onconnect
```

#### B. Menampilkan Daftar Jadwal:
```bash
phone-backup schedule list
```

#### C. Mengeksekusi Semua Jadwal Tertunda:
```bash
phone-backup schedule run -p "KataSandiSuperKuat"
```

---

### 3.13 Verifikasi Integritas & Pemeliharaan (Verify & GC)

#### A. Verifikasi Integritas Repository (Verify)
Pastikan seluruh blok terenkripsi utuh dan tidak mengalami *bit rot* atau korupsi:
```bash
phone-backup verify -p "KataSandiSuperKuat"
```
*Output:*
```text
Repository Verification Report
------------------------------
Total files in index:  142
Verified objects:      142
Missing objects:       0
Corrupted files:       0

STATUS: HEALTHY
```

#### B. Garbage Collection (GC)
Hapus objek orphan yang tidak lagi dirujuk oleh snapshot manapun:
```bash
phone-backup gc
```

#### C. Menampilkan Statistik Repository:
```bash
phone-backup stats
```

---

### 3.14 Restorasi Snapshot (Full & Filtered)

#### A. Restore Penuh ke Folder Lokal:
```bash
phone-backup restore -p "KataSandiSuperKuat" -t ./hasil_restore <SNAPSHOT_ID>
```

#### B. Restore Selektif dengan Filter Pola (Glob):
```bash
# Hanya memulihkan file gambar JPG
phone-backup restore -p "KataSandiSuperKuat" --filter "*.jpg" -t ./foto_restore <SNAPSHOT_ID>

# Hanya memulihkan dokumen PDF
phone-backup restore -p "KataSandiSuperKuat" --filter "*.pdf" -t ./dokumen_restore <SNAPSHOT_ID>
```

---

## 📱 4. Pencadangan Nirkabel: Android Companion Agent (Wi-Fi)

Mulai dari arsitektur nirkabel (`adapters/agent` dan `apps/android-agent`), Anda dapat mencadangkan smartphone Android melalui jaringan Wi-Fi lokal **tanpa kabel USB dan tanpa mode pengembang/USB Debugging**.

### 4.1 Instalasi Companion APK di Android
1. Build atau pasang APK dari `apps/android-agent/` pada smartphone Android.
2. Buka aplikasi **Phone Backup Agent** di Android.
3. Berikan izin runtime standar saat diminta:
   - Akses Kontak (`READ_CONTACTS`)
   - Akses SMS & Riwayat Telepon (`READ_SMS`, `READ_CALL_LOG`)
   - Akses Media & File (`READ_MEDIA_IMAGES`, `READ_MEDIA_VIDEO`, `READ_EXTERNAL_STORAGE`)
4. Jalankan layanan agen (klik **Start Agent Service**). Aplikasi akan membuka port layanan lokal dan menampilkan ID Perangkat Nirkabel (misal: `AGENT_WIRELESS_01`).

### 4.2 Menjalankan Perintah via Adapter Agen Nirkabel
Gunakan flag `--adapter agent` pada CLI `phone-backup`:

```bash
# 1. Deteksi agen Android nirkabel yang aktif
phone-backup --adapter agent devices

# 2. Periksa detail kapabilitas & status perizinan agen
phone-backup --adapter agent device-info AGENT_WIRELESS_01

# 3. Pindai berkas jarak jauh via Wi-Fi
phone-backup --adapter agent scan AGENT_WIRELESS_01

# 4. Tampilkan daftar aplikasi terpasang di HP
phone-backup --adapter agent apps AGENT_WIRELESS_01

# 5. Jalankan pencadangan terenkripsi penuh via Wi-Fi
phone-backup --adapter agent backup -p "KataSandiNirkabel123" AGENT_WIRELESS_01
```

---

## 🔒 5. Enkripsi Metadata Database (SQLCipher + Argon2id)

Metadata katalog disembunyikan di dalam database SQLite terenkripsi **SQLCipher AES-256** menggunakan derivasi kunci **Argon2id**:
- **Argon2id Key Derivation**: Menghasilkan kunci 256-bit aman dari kata sandi master (`derive_database_key`).
- **Connection Customizer**: Eksekusi otomatis `PRAGMA key = '<hex_key>';` saat r2d2 connection pool memperoleh koneksi database.

Untuk menjalankan unit & integration test terisolasi di seluruh workspace:
```bash
cargo test --workspace
```

---

## 💡 6. Catatan Khusus Pengguna Xiaomi / Redmi / POCO (MIUI / HyperOS)

Jika Anda menggunakan perangkat Xiaomi/Redmi/POCO melalui kabel ADB, pastikan opsi berikut aktif di **Developer Options**:
1. **USB Debugging**: Aktifkan.
2. **USB Debugging (Security settings)**: **Wajib aktif** (memerlukan login akun Mi & kartu SIM terpasang) agar ADB diizinkan membaca database SMS, Kontak, dan Call Logs.
3. **Install via USB**: Aktifkan untuk pengujian migrasi/ekspor APK.

---

## 👥 7. Panduan Strategis: Backup Khusus Kontak (< 5 Detik)

Jika tujuan utama Anda adalah **hanya mengamankan buku telepon (kontak)** tanpa menyalin puluhan gigabyte file media/video yang memakan waktu dan kapasitas disk:

```bash
# Backup kontak + metadata terenkripsi super cepat (3 - 5 detik)
phone-backup --adapter adb backup -i /storage/emulated/0/Download/ -p "KataSandiKontak123" <DEVICE_ID>
```

> **Keuntungan Strategi Ini:**
> - **Super Cepat**: Selesai dalam hitungan detik.
> - **Hemat Storage**: Hanya memakan ruang beberapa ratus KB di komputer.
> - **Terenkripsi Penuh**: Seluruh data kontak dienkripsi dengan AES-256 / age dan diindeks ke SQLite FTS5.

---

## 🔌 8. Panduan & Analisis: Pencadangan Tanpa USB Debugging

| Tipe Data / Fitur | 🔌 Jalur ADB (USB Debugging) | 📁 Jalur Kabel Biasa (MTP / File Transfer) | 📱 Jalur Aplikasi Pendamping (*Companion App*) |
| :--- | :---: | :---: | :---: |
| **Foto, Video, Musik (DCIM)** | ✅ Cepat & Terenkripsi | ✅ Bisa (Salin Manual) | ✅ Penuh (Nirkabel) |
| **Dokumen & Berkas Download** | ✅ Penuh | ✅ Bisa | ✅ Penuh |
| **Buku Telepon (Kontak)** | ✅ Otomatis (Content Provider) | ❌ **TIDAK BISA** (Diblokir Android) | ✅ Bisa (Izin Runtime Android) |
| **Pesan SMS & Log Panggilan** | ✅ Otomatis | ❌ **TIDAK BISA** (Diblokir Android) | ✅ Bisa |
| **Daftar Aplikasi & Ekspor APK** | ✅ Penuh | ❌ **TIDAK BISA** | ✅ Penuh |
| **Deduplikasi Blok (FastCDC)** | ✅ Aktif (CAS) | ⚠️ Terbatas | ✅ Aktif |

---

## ❓ 9. Panduan Pemecahan Masalah (Troubleshooting & FAQ)

### Q1: `phone-backup doctor` menampilkan error "ADB not found"?
**Solusi:** Pastikan direktori Android SDK platform-tools terdaftar di variabel lingkungan `PATH`.
- macOS: `export PATH=$PATH:$HOME/Library/Android/sdk/platform-tools`
- Linux: `sudo apt install adb`

### Q2: Perangkat berstatus `unauthorized` saat menjalankan `phone-backup devices`?
**Solusi:** Buka layar smartphone Anda. Akan muncul dialog pop-up konfirmasi *"Allow USB debugging?"*. Centang opsi *"Always allow from this computer"* lalu tekan **OK**.

### Q3: Kenapa kontak dan SMS tidak terbaca pada HP Xiaomi / Redmi?
**Solusi:** Buka **Developer Options** di HP Xiaomi, lalu aktifkan opsi **"USB Debugging (Security settings)"**.

### Q4: Apakah backup aman jika disimpan di Google Drive / AWS S3?
**Solusi:** **Sangat aman.** Data dienkripsi secara lokal (*Client-Side Zero-Knowledge*) dengan algoritma AES-256-GCM / X25519 sebelum dikirim ke cloud. Penyedia cloud hanya menyimpan ciphertext yang tidak dapat dibuka tanpa kunci Anda.

### Q5: Bagaimana cara melanjutkan backup yang terputus di tengah jalan?
**Solusi:** Jalankan kembali perintah backup yang sama. Engine secara otomatis mendeteksi snapshot status `Interrupted` dan menerapkan fitur **Incremental Resume** sehingga file yang sudah berhasil disimpan tidak akan diunduh ulang.

### Q6: Error "could not install \*smartsocket\* listener: Address already in use" atau "ADB server didn't ACK"?
**Penyebab:** Terjadi konflik pada port `5037` (port standar daemon ADB) karena ada proses ADB gantung (*zombie process*) atau beberapa instance ADB mencoba dijalankan bersamaan.
**Solusi:**
1. Hentikan server ADB yang sedang berjalan:
   ```bash
   adb kill-server
   ```
2. Atau matikan proses yang menggunakan port 5037 secara paksa:
   ```bash
   kill -9 $(lsof -ti:5037)
   ```
3. Nyalakan kembali server ADB:
   ```bash
   adb start-server
   ```

---
*phone-backup — Engineered with Rust, Clean Architecture, and Military-Grade Security.*

