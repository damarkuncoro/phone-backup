# 🚀 Getting Started

Panduan ini akan memandu Anda mulai dari persiapan lingkungan, instalasi dependensi, build binary, hingga verifikasi kesehatan sistem.

---

## 1. Prasyarat Sistem

Sebelum memulai, pastikan komputer Anda telah terinstal:
- **Rust toolchain** (versi 1.75+): [https://rustup.rs](https://rustup.rs)
- **Android Debug Bridge (`adb`)**: Bagian dari Android SDK Platform-Tools.
- **Node.js (Opsional, untuk Desktop GUI)**: Versi 18+ untuk pengembangan Tauri frontend.

### A. Memasang & Mengonfigurasi ADB

#### macOS (Homebrew):
```bash
brew install android-platform-tools
```
Tambahkan ke `~/.zshrc`:
```bash
export PATH=$PATH:$HOME/Library/Android/sdk/platform-tools
```

#### Linux (Ubuntu / Debian):
```bash
sudo apt update && sudo apt install adb -y
```

#### Windows (PowerShell):
1. Unduh [Android SDK Platform-Tools for Windows](https://developer.android.com/tools/releases/platform-tools).
2. Ekstrak dan tambahkan path ke System Environment Variables `PATH`.

---

## 2. Persiapan Smartphone Android

### A. Mengaktifkan USB Debugging
1. Buka **Pengaturan (Settings)** ➔ **Tentang Ponsel (About Phone)**.
2. Ketuk **Nomor Bentukan (Build Number)** sebanyak **7 kali** hingga muncul notifikasi pengembang.
3. Masuk ke **Opsi Pengembang (Developer Options)** dan aktifkan **USB Debugging**.
4. Hubungkan HP ke komputer dengan kabel USB.
5. Saat pop-up *"Allow USB Debugging?"* muncul, centang *"Always allow from this computer"* lalu klik **OK**.

### B. Perhatian Khusus Perangkat Xiaomi / Redmi / POCO
Pada perangkat berbasis MIUI / HyperOS, Anda wajib mengaktifkan:
- **USB Debugging (Security settings)**: Diperlukan agar ADB diizinkan membaca database Kontak & SMS.
- **Install via USB**: Diperlukan jika ingin mengekspor aplikasi / APK.

---

## 3. Mengunduh & Membangun Proyek (Build)

```bash
# 1. Kloning repositori
git clone https://github.com/damarkuncoro/phone-backup.git
cd phone-backup

# 2. Build CLI binary versi release
cargo build --release -p phone-backup
```
Binary executable akan berada di `./target/release/phone-backup`.

---

## 4. Diagnostik Sistem (Doctor Check)

Jalankan perintah `doctor` untuk memastikan seluruh komponen siap:
```bash
./target/release/phone-backup doctor
```

**Contoh Output Sehat:**
```text
🩺 Phone Backup Doctor - System Diagnostic
-----------------------------------------
Checking ADB installation... ✅ FOUND (Android Debug Bridge version 1.0.41)
Checking connected devices... ✅ 1 device(s) detected
Checking workspace integrity... ✅ backup.db found
Checking storage connectivity... ✅ storage reachable

System is ready for backup operations!
```

---

## 5. Menjalankan Backup Pertama Anda

```bash
# 1. Lihat ID perangkat yang terhubung
./target/release/phone-backup --adapter adb devices

# 2. Eksekusi backup penuh terenkripsi
./target/release/phone-backup --adapter adb backup -p "SandiRahasia123" <DEVICE_ID>
```

---
*Lanjutkan ke: [CLI Reference](CLI-Reference.md) atau [Desktop GUI Guide](Desktop-GUI-Guide.md).*
