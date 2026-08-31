# 🖥 Desktop GUI Guide

Platform **phone-backup** dilengkapi dengan antarmuka grafis desktop modern berbasis **Tauri**, **Tailwind CSS**, **Chart.js**, dan **Native Web Components**.

---

## 1. Menjalankan Desktop GUI

Dari root direktori repositori, jalankan:
```bash
cargo tauri dev
```

---

## 2. Navigasi & Arsitektur Antarmuka

Dashboard didesain dengan layout sidebar tetap yang membagi ruang kerja menjadi beberapa modul utama:

```text
+-----------------------------------------------------------------------+
|  [Sidebar Navigation]   |  [Main Workspace Panel]                     |
|  - 📊 Dashboard         |                                             |
|  - 📂 Device Explorer   |  [Live Device File Manager]                 |
|  - 🔄 Snapshot Diffing  |  - Breadcrumb Directory Navigator           |
|  - 📦 Apps / APKs       |  - Real-time Remote Download & Upload       |
|  - 👥 Android Data      |  - Instant SHA-256 Hash Calculation         |
|  - ⚙️ Settings          |                                             |
+-------------------------+---------------------------------------------+
```

---

## 3. Fitur Unggulan Desktop GUI

### 📂 A. Live Device File Manager
- **Navigasi Real-Time**: Telusuri folder penyimpanan internal HP secara interaktif.
- **Download File**: Klik tombol **Download** untuk menyalin berkas langsung dari HP ke komputer lokal via command `download_from_device`.
- **Upload File**: Unggah berkas lokal dari komputer ke lokasi folder mana pun di HP.
- **Operasi Berkas**: Ubah nama (*Rename*), salin/pindah (*Copy/Move*), dan hapus berkas secara instan.
- **Kalkulasi SHA-256**: Menghitung checksum integritas berkas langsung di hardware Android sebelum transfer.

---

### 🔄 B. Visual Snapshot Diffing Matrix
Fitur komparasi snapshot membandingkan dua titik waktu backup secara visual untuk mendeteksi:
- 🟢 **New**: Berkas atau kontak yang baru ditambahkan.
- 🟡 **Modified**: Berkas yang ukurannya berubah atau kontak yang dimodifikasi.
- 🔴 **Deleted**: Berkas atau kontak yang telah dihapus sejak backup terakhir.
- ⚪ **Unchanged**: Data yang identik 100% (tidak memakan ruang storage tambahan berkat CAS deduplikasi).

---

### 📦 C. Installed Apps & APK Exporter
- Menampilkan seluruh aplikasi sistem dan aplikasi pihak ketiga yang terpasang di HP.
- Ekspor berkas master `.apk` secara individu atau batch langsung ke folder lokal untuk keperluan migrasi atau pengarsipan.

---

### 👥 D. Android Data Explorer
- **Buku Telepon (Contacts)**: Visualisasi kontak dengan dukungan multi-nomor telepon, email, alamat, dan tombol **Export vCard (.vcf)**.
- **Pesan SMS**: Tampilan thread pesan dan kode OTP dengan filter pencarian instan.
- **Log Panggilan**: Riwayat panggilan masuk, keluar, dan tidak terjawab.

---

### 🛡 E. Safety Guard & Real-Time Progress HUD
- **HUD Melayang**: Menampilkan status progres operasi yang sedang berjalan secara animasi dan non-blocking.
- **Peringatan Keselamatan Hardware**: Peringatan otomatis jika kapasitas baterai HP $< 10\%$ atau suhu perangkat $> 45^\circ\text{C}$ sebelum memulai transfer.

---
*Lanjutkan ke: [Architecture & Design](Architecture-and-Design.md) atau [Contacts & Data Management](Contacts-and-Data-Management.md).*
