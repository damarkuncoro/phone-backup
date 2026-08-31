# ❓ Troubleshooting & FAQ

Halaman ini berisi kumpulan solusi untuk kendala yang paling sering ditemui saat mengoperasikan **phone-backup**.

---

## 🛠 Panduan Pemecahan Masalah Cepat

### 1. Error: `could not install *smartsocket* listener: Address already in use` atau `ADB server didn't ACK`
- **Penyebab**: Terjadi konflik pada port `5037` karena adanya proses ADB lama yang menggantung (*zombie process*) atau beberapa instance ADB berjalan bersamaan.
- **Solusi**:
  ```bash
  # 1. Hentikan server ADB
  adb kill-server
  
  # 2. Atau matikan paksa proses yang menggunakan port 5037
  kill -9 $(lsof -ti:5037)
  
  # 3. Jalankan kembali daemon ADB
  adb start-server
  ```

---

### 2. Error: `phone-backup doctor` menampilkan `ADB not found`
- **Penyebab**: Direktori `platform-tools` belum didaftarkan di variabel lingkungan `PATH`.
- **Solusi**:
  - **macOS**: `export PATH=$PATH:$HOME/Library/Android/sdk/platform-tools`
  - **Linux**: `sudo apt update && sudo apt install adb -y`
  - **Windows**: Tambahkan folder instalasi Android SDK Platform-Tools ke Environment Variables.

---

### 3. Perangkat Berstatus `unauthorized` saat `phone-backup devices`
- **Penyebab**: Kunci otorisasi RSA komputer belum disetujui di layar smartphone.
- **Solusi**:
  1. Nyalakan dan buka kunci layar smartphone Anda.
  2. Akan muncul dialog pop-up konfirmasi: *"Allow USB debugging?"*.
  3. Centang opsi *"Always allow from this computer"*, lalu ketuk **OK**.

---

### 4. Kontak atau SMS Tidak Terbaca pada HP Xiaomi / Redmi / POCO
- **Penyebab**: Sistem keamanan MIUI / HyperOS membatasi akses pembacaan database sistem melalui kabel data.
- **Solusi**:
  1. Buka **Pengaturan (Settings)** ➔ **Opsi Pengembang (Developer Options)** di HP Anda.
  2. Aktifkan opsi **"USB Debugging (Security settings)"** (memerlukan koneksi internet, kartu SIM aktif, dan login akun Mi).

---

### 5. Bagaimana Cara Melanjutkan Backup yang Terputus (*Interrupted*)?
- **Solusi**: Cukup jalankan kembali perintah backup yang sama. Engine secara otomatis mengenali snapshot yang berstatus `Interrupted` dan menerapkan fitur **Incremental Resume** sehingga berkas dan potongan (*chunks*) yang sudah tersimpan sebelumnya tidak akan ditransfer ulang.

---

### 6. Apakah Data di Cloud S3 Aman dari Kebocoran?
- **Solusi**: **Sangat aman.** Menggunakan prinsip *Zero-Knowledge*, semua berkas dienkripsi secara lokal menggunakan algoritma AES-256-GCM atau kunci asimetris age (X25519) sebelum dikirim ke cloud. Penyedia cloud hanya menerima ciphertext terenkripsi tanpa kunci.

---
*Lanjutkan ke: [Developer Guide & Testing](Developer-Guide-and-Testing.md) atau [Home](Home.md).*
