# How-To Guide: Menggunakan phone-backup 📱

Panduan ini menjelaskan langkah demi langkah cara mengoperasikan engine **phone-backup** melalui antarmuka **Desktop GUI** (Dashboard) maupun **CLI** (Terminal).

---

## 🎮 Opsi A: Menggunakan Desktop GUI (Rekomendasi)
Dashboard visual memberikan kemudahan untuk memantau kapasitas penyimpanan dan riwayat backup secara langsung dengan arsitektur modular yang responsif.

### 1. Menjalankan Aplikasi
Masuk ke root project dan jalankan perintah:
```bash
cargo tauri dev
```

### 2. Memahami Dashboard
*   **Storage Efficiency**: Menampilkan persentase ruang yang berhasil dihemat melalui deduplikasi (Content-Addressed Storage).
*   **System Health**: Memastikan koneksi ke engine Rust dan status ADB (Android Debug Bridge) aktif.
*   **Connected Devices**: Menampilkan daftar HP yang terhubung. Jika tidak muncul, klik **"RESCAN"**.

### 3. Fitur Utama GUI
*   **Live Device File Manager**: Akses langsung sistem file HP yang terhubung. Anda dapat menjelajahi folder, mengunduh file langsung ke komputer local via `download_from_device`, mengunggah file baru, mengganti nama, menyalin/memindahkan, menghapus file, serta menghitung hash SHA-256 file secara instan.
*   **Scan (Dry Run)**: Klik tombol **"SCAN"** pada perangkat. Anda bisa melihat daftar file di HP tanpa mendownloadnya, lalu memilih file tertentu saja yang ingin di-backup.
*   **Backup All**: Melakukan backup menyeluruh untuk seluruh file media di perangkat.
*   **History & Browser**: Klik **"HISTORY"** untuk melihat riwayat backup. Anda bisa mengklik setiap baris riwayat untuk membuka **File Browser** dan melihat isi file di dalam backup tersebut.
*   **Visual Snapshot Diffing**: Bandingkan dua snapshot secara visual. Sistem akan menandai perubahan secara otomatis dengan status **New** (Hijau), **Modified** (Kuning), **Deleted** (Merah), dan **Unchanged**.
*   **Android Data & Apps Explorer**: Di dalam Explorer, pilih tab **"ANDROID DATA"** (Kontak, SMS, Call Logs) atau tab **"APPS"** untuk memeriksa aplikasi terinstall di HP.
*   **Restore**: Klik ikon **Unduh (Restore)** di daftar riwayat. Sistem akan otomatis membuat folder unik di `workspace/` untuk hasil restorasi Anda.

---

## 💻 Opsi B: Menggunakan CLI (Untuk Expert)
CLI sangat kuat untuk otomatisasi dan integrasi server.

### 1. Persiapan Awal (Doctor Check)
Pastikan lingkungan Anda sudah siap sebelum melakukan backup nyata.
```bash
phone-backup doctor
```

### 2. Manajemen Keamanan (Asimetris)
Fitur ini memungkinkan backup tanpa menyimpan password di komputer backup (Zero-Knowledge).

#### Langkah 1: Membuat Pasangan Kunci (Public & Secret)
1.  **Jalankan Perintah**: `phone-backup keygen`
2.  **Public Key**: Digunakan untuk mengunci data saat backup.
3.  **Secret Key**: Digunakan untuk membuka data saat restorasi. **SIMPAN DI TEMPAT AMAN!**

#### Langkah 2: Backup Terenkripsi
```bash
phone-backup --adapter adb --pubkey "age1..." backup <DEVICE_ID>
```

#### Langkah 3: Restore Cerdas
```bash
# Otomatis mencari snapshot terbaru dan memulihkannya ke folder versi
phone-backup --privkey "AGE-SECRET-KEY-1..." restore last
```

---

## 🧹 Pemeliharaan & Efisiensi
*   **Smart Retention**: Engine secara otomatis menghapus snapshot lama yang identik dengan snapshot terbaru agar riwayat Anda tidak penuh dengan data duplikat.
*   **Garbage Collection (GC)**: 
    *   **GUI**: Buka menu **Settings (ikon gerigi)** dan klik **"Run Garbage Collection"**.
    *   **CLI**: Jalankan `phone-backup gc`.

---

## 💡 Tips untuk Pengguna Xiaomi:
Agar fitur **Scan**, **Contacts**, dan **SMS** berjalan lancar, pastikan Anda telah mengaktifkan:
1.  **USB Debugging**.
2.  **USB Debugging (Security settings)** — *Wajib di Xiaomi agar ADB bisa membaca database SMS/Kontak.*

---
*Dikembangkan dengan standar kualitas tinggi untuk komunitas Android.*
