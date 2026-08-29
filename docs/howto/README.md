# How-To Guide: Menggunakan phone-backup 📱

Panduan ini menjelaskan langkah demi langkah cara mengoperasikan engine **phone-backup** melalui antarmuka **Desktop GUI** (Dashboard) maupun **CLI** (Terminal).

---

## 🎮 Opsi A: Menggunakan Desktop GUI (Rekomendasi)
Dashboard visual memberikan kemudahan untuk memantau kapasitas penyimpanan dan riwayat backup secara langsung.

### 1. Menjalankan Aplikasi
Masuk ke folder GUI dan jalankan perintah pengembangan:
```bash
cd apps/gui/src-tauri
cargo tauri dev
```

### 2. Memahami Dashboard
*   **Storage Efficiency**: Menampilkan persentase ruang yang berhasil dihemat melalui deduplikasi.
*   **Engine Health**: Status indikator "Active" yang berkedip menandakan engine Rust siap bekerja.
*   **Connected Devices**: Menampilkan daftar HP yang terhubung via ADB secara real-time.

### 3. Melakukan Backup & Restore
*   **Backup**: Klik tombol **"Backup Now"** pada perangkat yang diinginkan. Sebuah jendela melayang (*Progress HUD*) akan muncul di pojok bawah untuk menunjukkan progress.
*   **History**: Klik tombol **"History"** untuk melihat daftar snapshot yang pernah dibuat.
*   **Restore**: Klik ikon **Unduh (Restore)** pada baris snapshot di dalam riwayat. Anda akan diminta memasukkan lokasi folder tujuan restorasi.

---

## 💻 Opsi B: Menggunakan CLI (Untuk Expert)
CLI sangat kuat untuk otomatisasi dan integrasi server.

### 1. Persiapan Awal (Doctor Check)
Pastikan lingkungan Anda sudah siap sebelum melakukan backup nyata.
```bash
phone-backup doctor
```

### 2. Manajemen Keamanan (Asimetris)
Fitur ini memungkinkan backup tanpa menyimpan password di komputer backup.

#### Langkah 1: Membuat Pasangan Kunci (Public & Secret)
1.  **Jalankan Perintah**: `./target/debug/phone-backup keygen`
2.  **Public Key**: Berawalan `age1...`. Ini digunakan untuk mengunci data saat backup.
3.  **Secret Key**: Berawalan `AGE-SECRET-KEY-1...`. Ini **SANGAT RAHASIA**, gunakan hanya saat restorasi.

#### Langkah 2: Backup Terenkripsi
```bash
phone-backup --adapter adb --pubkey "age1..." backup <DEVICE_ID>
```

---

## 🧹 Pemeliharaan (Maintenance)
Baik di GUI maupun CLI, Anda bisa melakukan pembersihan repositori:

### Mengapa butuh Maintenance?
Karena kita menggunakan sistem deduplikasi tingkat blok, ada kalanya sisa-sisa data lama tidak lagi terpakai oleh snapshot mana pun.

*   **GUI**: Klik tombol **"Clean Orphans"** di bagian atas aplikasi.
*   **CLI**: Jalankan perintah `phone-backup gc`.

---

## 💡 Tips Pro:
1.  **Resume Otomatis**: Jika kabel USB terlepas di tengah jalan, jangan panik. Jalankan lagi backup, engine akan melanjutkan dari file terakhir.
2.  **Streaming I/O**: Engine ini tidak menggunakan folder `/tmp` di disk Anda untuk file sementara, sehingga sangat aman untuk SSD dan cepat untuk video besar.
3.  **Deduplikasi Blok**: Jika Anda mengubah satu foto (misal: rotasi), engine hanya akan menyimpan bagian kecil yang berubah di dalam file tersebut.

---
*Untuk bantuan lebih lanjut, silakan hubungi tim pengembang atau buka issue di GitHub.*
