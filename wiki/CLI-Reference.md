# 💻 CLI Reference

Dokumentasi lengkap seluruh perintah, flag, dan opsi baris perintah (**CLI**) untuk binary `phone-backup`.

---

## 📌 Global Options & Flags

| Flag | Tipe | Default | Deskripsi |
| :--- | :--- | :--- | :--- |
| `-a, --adapter` | String | `mock` | Jenis adapter hardware: `adb`, `agent` (Wi-Fi), `mock`. |
| `--storage` | String | `local` | Backend penyimpanan: `local`, `opendal` (Cloud S3/R2). |
| `--pubkey` | String | - | Kunci publik asimetris `age` (`age1...`) untuk enkripsi otomatis tanpa sandi. |
| `--privkey` | String | - | Kunci privat asimetris `age` (`AGE-SECRET-KEY-1...`) untuk dekripsi data. |
| `--s3-bucket` | String | - | Nama bucket S3 (Environment: `S3_BUCKET`). |
| `--s3-region` | String | - | Region S3 (Environment: `S3_REGION`). |
| `--s3-endpoint` | String | - | Custom endpoint S3 / Cloudflare R2 (Environment: `S3_ENDPOINT`). |
| `--s3-access-key`| String | - | Access key S3 (Environment: `S3_ACCESS_KEY`). |
| `--s3-secret-key`| String | - | Secret key S3 (Environment: `S3_SECRET_KEY`). |

---

## 🛠 Subcommands Reference

### 1. `doctor`
Melakukan diagnostik kesehatan menyeluruh pada instalasi ADB, perangkat yang terpasang, integritas database SQLite, dan konektivitas storage.
```bash
phone-backup doctor
```

---

### 2. `devices`
Menampilkan daftar smartphone Android yang terdeteksi via USB atau Wi-Fi.
```bash
phone-backup --adapter adb devices
phone-backup --adapter agent devices
```

---

### 3. `device-info <ID>`
Menampilkan rincian teknis perangkat keras, model, level baterai, suhu, dan matriks izin akses data.
```bash
phone-backup --adapter adb device-info <DEVICE_ID>
```

---

### 4. `scan <ID>`
Memindai berkas di dalam smartphone secara real-time tanpa menyimpan ke repository (*Dry Run*).
```bash
phone-backup --adapter adb scan <DEVICE_ID>
```

---

### 5. `apps <ID>`
Mengekstrak dan menampilkan daftar seluruh aplikasi Android yang terinstal beserta versi dan package name.
```bash
phone-backup --adapter adb apps <DEVICE_ID>
```

---

### 6. `backup <ID>`
Mengeksekusi proses pencadangan data dari perangkat.

```bash
# Backup Penuh
phone-backup --adapter adb backup -p "Password123" <DEVICE_ID>

# Backup Selektif Folder Tertentu (Include)
phone-backup --adapter adb backup -i /storage/emulated/0/DCIM -p "Password123" <DEVICE_ID>

# Mengecualikan Folder / Pola (Exclude)
phone-backup --adapter adb backup -e "*.tmp" -p "Password123" <DEVICE_ID>

# Backup dengan Kunci Publik Asimetris (age)
phone-backup --adapter adb backup --pubkey "age1..." <DEVICE_ID>
```

---

### 7. `snapshots <ID>`
Melihat riwayat snapshot backup untuk perangkat tertentu.
```bash
# Daftar semua snapshot
phone-backup snapshots <DEVICE_ID>

# Rincian isi file snapshot tertentu
phone-backup snapshots <DEVICE_ID> -s <SNAPSHOT_ID>
```

---

### 8. `restore <SNAPSHOT_ID>`
Memulihkan data dari snapshot terenkripsi ke folder lokal komputer.

```bash
# Restore Penuh
phone-backup restore -p "Password123" -t ./output_folder <SNAPSHOT_ID>

# Restore Selektif dengan Filter Pola (Glob)
phone-backup restore -p "Password123" --filter "*.pdf" -t ./dokumen <SNAPSHOT_ID>

# Restore dengan Kunci Privat Asimetris (age)
phone-backup --privkey "AGE-SECRET-KEY-1..." restore -t ./output_folder <SNAPSHOT_ID>
```

---

### 9. `verify`
Memverifikasi integritas seluruh objek dan chunk di dalam repository untuk mendeteksi data yang hilang atau rusak.
```bash
phone-backup verify -p "Password123"
```

---

### 10. `search <QUERY>`
Pencarian cepat nama file atau path di seluruh snapshot menggunakan index FTS5.
```bash
phone-backup search "invoice_2026.pdf"
```

---

### 11. `contacts <QUERY>`
Pencarian instan data buku telepon (nama, nomor telepon, email) di seluruh riwayat backup.
```bash
phone-backup contacts "Damar"
phone-backup contacts "+62"
```

---

### 12. `sms <QUERY>`
Pencarian cepat pesan SMS atau kode OTP di seluruh snapshot.
```bash
phone-backup sms "Bank"
```

---

### 13. `photos <ID>`
Menampilkan galeri foto beserta metadata EXIF mendalam (kamera, aperture, ISO, timestamp).
```bash
phone-backup photos <DEVICE_ID>
```

---

### 14. `clone <SOURCE_ID> <TARGET_ID>`
Melakukan kloning dan migrasi data langsung dari HP sumber ke HP tujuan.
```bash
phone-backup clone fynrorjncy6x4xib TARGET_DEVICE_ID
```

---

### 15. `schedule`
Manajemen penjadwalan backup otomatis dan background daemon.

```bash
# Menambah jadwal harian
phone-backup schedule add <DEVICE_ID> --frequency daily

# Menambah jadwal reaktif saat HP dicolok USB
phone-backup schedule add <DEVICE_ID> --frequency onconnect

# Menampilkan daftar jadwal
phone-backup schedule list

# Menjalankan seluruh jadwal tertunda
phone-backup schedule run -p "Password123"
```

---

### 16. `keygen`
Membuat pasangan kunci asimetris X25519 (`age`) untuk backup zero-knowledge otomatis.
```bash
phone-backup keygen
```

---

### 17. `stats` & `gc`
- `phone-backup stats`: Menampilkan ringkasan statistik efisiensi deduplikasi dan total objek.
- `phone-backup gc`: Membersihkan objek sampah (*orphan objects*) yang tidak lagi dirujuk oleh snapshot manapun.

---
*Lanjutkan ke: [Desktop GUI Guide](Desktop-GUI-Guide.md) atau [Security & Encryption](Security-and-Encryption.md).*
