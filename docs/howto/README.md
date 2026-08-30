# Panduan Operasional Lengkap: phone-backup 📱

Dokumen ini berisi panduan penggunaan mendalam untuk mengoperasikan platform **phone-backup** melalui **Desktop GUI (Tauri)** dan **CLI (Command Line Interface)**, lengkap dengan konfigurasi lingkungan ADB, enkripsi tingkat lanjut, dan verifikasi integritas data.

---

## 🛠 1. Persiapan Lingkungan & Konfigurasi ADB

Sebelum menghubungkan smartphone Android, pastikan `adb` (Android Debug Bridge) telah terinstal dan terdaftar dalam `PATH`.

### Konfigurasi PATH (macOS / Linux)
Tambahkan lokasi `platform-tools` ke file `.zshrc` atau `.bashrc`:
```bash
export PATH=$PATH:$HOME/Library/Android/sdk/platform-tools
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

CLI menyediakan perintah yang cepat dan dapat diotomatisasi melalui skrip shell.

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
```

### 3.2 Deteksi Perangkat & Matriks Kapabilitas
Tampilkan daftar HP yang terhubung via USB/Wi-Fi:
```bash
phone-backup --adapter adb devices
```

Tampilkan informasi detail perangkat dan matriks kapabilitas izin:
```bash
phone-backup --adapter adb device-info <DEVICE_ID>
```
*Contoh:* `phone-backup --adapter adb device-info fynrorjncy6x4xib`

### 3.3 Inspeksi Aplikasi & Ekspor APK
Tampilkan seluruh aplikasi Android terinstal di HP:
```bash
phone-backup --adapter adb apps <DEVICE_ID>
```

### 3.4 Pemindaian Sistem File Real-Time (Scan Dry-Run)
Pindai isi memori internal HP tanpa menyimpan ke backup:
```bash
phone-backup --adapter adb scan <DEVICE_ID>
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

### 3.7 Verifikasi Integritas Repository (Verify)
Pastikan tidak ada file terenkripsi yang rusak atau hilang:
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

### 3.8 Restorasi Snapshot (Restore)
Pulihkan data dari snapshot terenkripsi ke lokasi direktori lokal:
```bash
phone-backup restore -p "KataSandiSuperKuat" <SNAPSHOT_ID>
```
*Contoh:* `phone-backup restore -p "my_secret_pass" 504f46de-2e86-4fcf-af29-68570cf8d68f`

### 3.9 Pencadangan, Pencarian & Ekspor Kontak (Contacts Management)

Platform **phone-backup** secara otomatis mengekstrak seluruh data buku telepon Android melalui Content Provider (`content://com.android.contacts/data`) pada setiap sesi backup penuh maupun terstruktur.

#### A. Data Kontak yang Didukung & Diamankan:
- **Identitas**: Nama lengkap (*Display Name*), Nama depan, Nama belakang, Gelar (*Prefix/Suffix*).
- **Nomor Telepon**: Multi-nomor (*Mobile, Home, Work, WhatsApp*) dengan normalisasi nomor internasional (E.164).
- **Email & Alamat**: Email pribadi/kantor serta alamat fisik lengkap.
- **Organisasi & Jabatan**: Nama perusahaan, departemen, dan judul pekerjaan.
- **Catatan (Notes) & Label**: Label kustom dan catatan yang tersimpan pada kontak.

#### B. Pencarian Kontak Instan (FTS5 Search):
Cari kontak berdasarkan nama atau potongan nomor telepon langsung dari terminal:
```bash
# Cari berdasarkan nama
phone-backup contacts "Damar"

# Cari berdasarkan awalan kode negara atau nomor
phone-backup contacts "+62"
```
*Contoh Output:*
```text
Searching for contact 'Damar'...

Found 1 matches:
SNAPSHOT        NAME                      PHONES                        
----------------------------------------------------------------------
b21c6f3c        damarkuncoro              +6285921495599                
```

#### C. Format Standar vCard (`.vcf`) & Ekspor:
Data kontak yang telah di-backup dapat diekspor langsung ke format standar **vCard 4.0 / 3.0 (RFC 6350)** melalui antarmuka GUI atau API Tauri `export_contacts_vcard`. Berkas `.vcf` yang dihasilkan kompatibel 100% untuk diimpor kembali ke:
- Google Contacts (Android baru)
- Apple Contacts (iPhone / iPad / macOS)
- Microsoft Outlook / Mozilla Thunderbird

#### D. Visual Contact Diffing (GUI Matrix):
Pada Desktop GUI, fitur *Visual Contact Diffing* membandingkan dua snapshot untuk menampilkan:
- 🟢 **Kontak Baru**: Kontak yang baru ditambahkan sejak backup terakhir.
- 🟡 **Kontak Berubah**: Kontak yang nomor telepon atau alamat emailnya dimodifikasi.
- 🔴 **Kontak Terhapus**: Kontak yang sudah tidak ada di HP.

---

### 3.10 Pemeliharaan & Garbage Collection (GC)
Bersihkan data *orphan* yang tidak lagi dirujuk oleh snapshot manapun:
```bash
phone-backup gc
```

---

## 🔒 4. Enkripsi Metadata Database (SQLCipher + Argon2id)

Metadata katalog disembunyikan di dalam database SQLite terenkripsi **SQLCipher AES-256** menggunakan derivasi kunci **Argon2id**:
- **Argon2id Key Derivation**: Menghasilkan kunci 256-bit aman dari kata sandi master.
- **Connection Customizer**: Eksekusi otomatis `PRAGMA key` saat r2d2 connection pool memperoleh koneksi database.

Untuk menjalankan unit & integration test terisolasi di seluruh workspace:
```bash
cargo test --workspace
```

---

## 💡 5. Checksing Khusus Pengguna Xiaomi / Redmi / POCO (MIUI / HyperOS)

Jika Anda menggunakan perangkat Xiaomi/Redmi/POCO, pastikan opsi berikut aktif di **Developer Options**:
1. **USB Debugging**: Aktifkan.
2. **USB Debugging (Security settings)**: **Wajib aktif** agar ADB diizinkan membaca database SMS, Kontak, dan Call Logs.
3. **Install via USB**: Aktifkan untuk pengujian migrasi/ekspor APK.

---

## 👥 6. Panduan & Rekomendasi: Pencadangan Khusus Kontak Saja (Contacts-Only Backup)

Jika tujuan utama Anda adalah **hanya mengamankan buku telepon (kontak)** tanpa ingin menyalin puluhan gigabyte file media/video yang memakan waktu dan kapasitas disk, ikuti rekomendasi dan strategi berikut:

### 6.1 Strategi Pencadangan Instan (< 5 Detik)
Pada arsitektur `phone-backup`, data terstruktur (**Kontak, SMS, Riwayat Panggilan, dan Metadata Aplikasi**) akan **selalu diekstraksi secara otomatis** pada setiap sesi backup.

Untuk mencegah sistem memindai seluruh memori internal HP yang besar (50 GB - 200 GB), gunakan filter `-i` (*include*) ke folder atau file kecil:

```bash
# Backup kontak + metadata terenkripsi super cepat (hanya 3 - 5 detik)
phone-backup --adapter adb backup -i /storage/emulated/0/Download/ -p "KataSandiKontak123" <DEVICE_ID>
```

> **Keuntungan Strategi Ini:**
> - **Super Cepat**: Selesai dalam hitungan detik karena tidak menyalin video/foto besar.
> - **Hemat Storage**: Hanya memakan ruang beberapa ratus KB di komputer.
> - **Terenkripsi Penuh**: Seluruh data kontak dienkripsi dengan AES-256 / age dan diindeks ke SQLite FTS5.

### 6.2 Ekspor Kontak ke Format Standar Universal (vCard `.vcf`)
Jika Anda mencadangkan kontak untuk keperluan migrasi ke HP baru atau sinkronisasi dengan platform lain, gunakan format standar **vCard (RFC 6350)**:
- **Kompatibilitas Penuh**: Dapat langsung diimpor ke **Google Contacts**, **iPhone (Apple Contacts)**, **Samsung**, **Microsoft Outlook**, atau **Mozilla Thunderbird**.
- **Cara Ekspor di GUI**: Buka tab **Android Data Explorer (👥 CONTACTS)** pada Desktop GUI (`cargo tauri dev`), lalu klik tombol **Export vCard** untuk menyimpan file `.vcf` ke komputer.

### 6.3 Pencarian Instan Nomor Telepon Tanpa Perlu Restore
Anda tidak perlu mengekstrak seluruh backup hanya untuk mencari 1 nomor telepon. Gunakan Full-Text Search (FTS5) langsung dari terminal:
```bash
# Cari berdasarkan nama kontak
phone-backup contacts "Damar"

# Cari berdasarkan potongan nomor atau kode negara
phone-backup contacts "+62"
```

### 6.4 Otomatisasi Sinkronisasi Kontak Harian (Zero-Knowledge Age Key)
1. **Buat Kunci Kriptografi Sekali Saja**:
   ```bash
   phone-backup keygen
   ```
2. **Jadwalkan Backup Harian**:
   ```bash
   phone-backup schedule add <DEVICE_ID> --frequency daily
   ```
   *Setiap kali HP terhubung ke komputer, data kontak terbaru otomatis tersinkronisasi tanpa perlu memasukkan password manual.*

---

## 🔌 7. Analisis & Panduan: Pencadangan Tanpa USB Debugging (Non-USB Debugging Alternatives)

Banyak pengguna bertanya: *"Apakah kita bisa melakukan backup tanpa mengaktifkan USB Debugging?"*

Secara arsitektur keamanan Android:
- **Untuk Berkas Media (Foto/Video/Dokumen)**: **BISA** menggunakan protokol transfer berkas standar (MTP).
- **Untuk Kontak, SMS, Call Logs & App Data**: **TIDAK BISA** via kabel biasa tanpa USB Debugging, kecuali menggunakan metode ekspor manual atau aplikasi pendamping (*Companion App*).

### 7.1 Matriks Perbandingan Kemampuan

| Tipe Data / Fitur | 🔌 Jalur ADB (USB Debugging) | 📁 Jalur Kabel Biasa (MTP / File Transfer) | 📱 Jalur Aplikasi Pendamping (*Companion App*) |
| :--- | :---: | :---: | :---: |
| **Foto, Video, Musik (DCIM)** | ✅ Cepat & Terenkripsi | ✅ Bisa (Salin Manual) | ✅ Penuh (Nirkabel) |
| **Dokumen & Berkas Download** | ✅ Penuh | ✅ Bisa | ✅ Penuh |
| **Buku Telepon (Kontak)** | ✅ Otomatis (Content Provider) | ❌ **TIDAK BISA** (Diblokir Android) | ✅ Bisa (Izin Runtime Android) |
| **Pesan SMS & Log Panggilan** | ✅ Otomatis | ❌ **TIDAK BISA** (Diblokir Android) | ✅ Bisa |
| **Daftar Aplikasi & Ekspor APK** | ✅ Penuh | ❌ **TIDAK BISA** | ✅ Penuh |
| **Deduplikasi Blok (FastCDC)** | ✅ Aktif (CAS) | ⚠️ Terbatas | ✅ Aktif |

### 7.2 Mengapa Android Memblokir Kontak & SMS pada Kabel Biasa (MTP)?
Sistem operasi Android mengisolasi database sistem (**`contacts2.db`**, **`mmssms.db`**) di dalam direktori terlindungi. Protokol MTP (*Media Transfer Protocol*) hanya diizinkan membaca folder publik (`/storage/emulated/0`) dan **tidak memiliki instruksi query database sistem**. Oleh karena itu, komputer luar tidak dapat membaca kontak/SMS tanpa perantara **ADB** atau izin **Aplikasi Android**.

### 7.3 Solusi Praktis Tanpa USB Debugging

#### A. Ekspor Manual vCard di HP (Khusus Kontak):
1. Buka aplikasi **Kontak (Contacts)** bawaan di smartphone Android Anda.
2. Buka menu **Pengaturan (Settings / Kelola Kontak)**.
3. Pilih opsi **Ekspor Kontak ke file `.vcf`** (Simpan ke memori internal).
4. Hubungkan HP ke komputer dengan kabel USB biasa (Mode *File Transfer / MTP*).
5. Salin berkas `.vcf` tersebut ke komputer Anda untuk diarsipkan.

#### B. Solusi Nirkabel: Android Companion App (Wi-Fi Local Backup):
Solusi jangka panjang terbaik agar pengguna tidak perlu repot mengaktifkan USB Debugging adalah arsitektur **Companion Agent APK**:
- Pengguna menginstal aplikasi kecil `phone-backup.apk` di HP.
- Aplikasi meminta izin runtime standar (*"Izinkan akses Kontak & SMS"*).
- HP dan Komputer berkomunikasi langsung melalui jaringan Wi-Fi lokal via WebSockets/gRPC (**100% nirkabel tanpa kabel & tanpa USB Debugging**).

### 7.4 Mengoperasikan Adapter Agen Nirkabel (`--adapter agent`)
Anda dapat menjalankan CLI `phone-backup` dengan adapter agen nirkabel menggunakan flag `--adapter agent`:

```bash
# 1. Deteksi agen Android nirkabel yang aktif
phone-backup --adapter agent devices

# 2. Periksa detail spesifikasi dan matriks perizinan agen nirkabel
phone-backup --adapter agent device-info AGENT_WIRELESS_01

# 3. Pindai berkas jarak jauh via agen nirkabel
phone-backup --adapter agent scan AGENT_WIRELESS_01

# 4. Tampilkan aplikasi terpasang di HP via agen
phone-backup --adapter agent apps AGENT_WIRELESS_01

# 5. Jalankan pencadangan terenkripsi penuh nirkabel
phone-backup --adapter agent backup -p "KataSandiNirkabel123" AGENT_WIRELESS_01
```

---
*phone-backup — Engineered with Rust, Clean Architecture, and Military-Grade Security.*
