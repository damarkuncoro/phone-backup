# ⚠️ Known Limitations & Boundaries

Dokumen ini menjelaskan batasan teknis platform **phone-backup** berdasarkan model keamanan Android OS, hak akses perizinan, dan perbedaan antara jalur transport (**ADB USB**, **MTP Cable**, dan **Companion Agent Wi-Fi**).

---

## 1. Data Internal Aplikasi Privat (`/data/data/`)
Android mengisolasi database internal dan shared preferences setiap aplikasi di direktori sandbox terisolasi.
- **Data yang Tidak Dapat Dicadangkan Langsung**: Riwayat chat internal (WhatsApp / Telegram lokal), save game, dan session login aplikasi perbankan.
- **Penyebab**: Akses ke folder `/data/data/` membutuhkan hak akses **Root**.
- **Solusi Rekomendasi**: Gunakan fitur backup in-app bawaan masing-masing aplikasi (misal: WhatsApp Backup ke Google Drive).

---

## 2. Pengaturan Sistem & Konfigurasi OS
- **Data yang Tidak Dapat Dicadangkan**: Sandi Wi-Fi tersimpan, pairing Bluetooth, dan tata letak icon home screen.
- **Penyebab**: Tersimpan di partisi sistem terlindungi yang dikelola eksklusif oleh Android System Server.
- **Solusi Rekomendasi**: Gunakan sinkronisasi akun Google / OEM Cloud bawaan smartphone (Xiaomi Cloud, Samsung Cloud, Google One).

---

## 3. Secure Element & Data Biometrik
- **Data yang Tidak Dapat Dicadangkan**: Sidik jari (Fingerprints), data Face ID, dan kunci kriptografi perangkat keras (Android Keystore / TEE).
- **Penyebab**: Data ini tidak pernah keluar dari **TEE (Trusted Execution Environment)** atau hardware Secure Element. Secara fisik mustahil untuk diekstrak demi keamanan pengguna.

---

## 4. Konten Terproteksi DRM
- **Data yang Tidak Dapat Dicadangkan**: Unduhan offline dari Netflix, Spotify, Disney+, YouTube Premium.
- **Penyebab**: Berkas dienkripsi dengan kunci hardware Widevine DRM yang terikat pada ID perangkat fisik.

---

## 5. Berkas Cloud-Only
- **Data yang Tidak Dapat Dicadangkan**: Foto atau dokumen di Google Photos / Google Drive yang belum diunduh / tidak tersimpan di memori lokal smartphone.
- **Penyebab**: Scanner hanya dapat membaca data yang secara fisik berada di media penyimpanan lokal HP.

---

## 6. Ringkasan Matriks Jalur Akses

| Kategori Data | 🔌 Jalur ADB (USB) | 📁 Jalur Kabel Biasa (MTP) | 📱 Jalur Companion Agent (Wi-Fi) |
| :--- | :---: | :---: | :---: |
| **Foto, Video, Musik (DCIM)** | ✅ Cepat & Terenkripsi | ✅ Bisa (Salin Manual) | ✅ Penuh (Nirkabel) |
| **Dokumen & Berkas Download** | ✅ Penuh | ✅ Bisa | ✅ Penuh |
| **Buku Telepon (Kontak)** | ✅ Otomatis (Content Provider) | ❌ Diblokir Android OS | ✅ Bisa (Izin Runtime Android) |
| **Pesan SMS & Log Panggilan** | ✅ Otomatis | ❌ Diblokir Android OS | ✅ Bisa (Izin Runtime Android) |
| **Daftar Aplikasi & Ekspor APK** | ✅ Penuh | ❌ Tidak Bisa | ✅ Penuh |
| **Deduplikasi FastCDC (CAS)** | ✅ Aktif | ⚠️ Terbatas | ✅ Aktif |
| **Tanpa USB Debugging / Kabel** | ❌ Butuh Debugging | ✅ Tanpa Debugging | ✅ **100% Nirkabel & Tanpa Debugging** |

---

### 💡 Rekomendasi Strategi Migrasi Penuh:
1. Gunakan **phone-backup** untuk seluruh Foto, Video, Dokumen, Buku Telepon (Kontak), Pesan SMS, Riwayat Telepon, dan Berkas APK.
2. Gunakan **Google One / OEM Cloud** untuk sinkronisasi pengaturan sistem dan sandi Wi-Fi.
3. Gunakan **In-App Cloud Backup** untuk aplikasi chatting (WhatsApp, Telegram).

---
*phone-backup — Engineered with Rust, Clean Architecture, and Military-Grade Security.*
