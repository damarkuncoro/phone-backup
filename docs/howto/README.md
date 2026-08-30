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
*phone-backup — Engineered with Rust, Clean Architecture, and Military-Grade Security.*
