# How-To Guide: Menggunakan phone-backup 📱

Panduan ini menjelaskan langkah demi langkah cara mengoperasikan engine **phone-backup** dari level dasar hingga fitur keamanan tingkat lanjut.

---

## 1. Persiapan Awal (Doctor Check)
Sebelum memulai, pastikan lingkungan Anda sudah siap. Perintah `doctor` akan memeriksa koneksi ADB, database, dan jangkauan storage.

```bash
# Tambahkan ADB ke path jika belum ada
export PATH=$PATH:/Users/yourname/Library/Android/sdk/platform-tools

# Jalankan diagnosa
./target/debug/phone-backup doctor
```

## 2. Menemukan Perangkat
Sambungkan HP Android Anda melalui kabel USB dan pastikan **USB Debugging** aktif.

```bash
./target/debug/phone-backup --adapter adb devices
```

## 3. Manajemen Keamanan (Asimetris)
Fitur unggulan **phone-backup** adalah dukungan penuh untuk enkripsi asimetris menggunakan format **age (X25519)**.

### Mengapa Asimetris?
Pada backup tradisional (simetris), Anda membutuhkan password untuk melakukan backup DAN restorasi. Jika komputer backup Anda diretas, pelaku bisa mencuri password dan membaca seluruh data backup Anda.

Dengan **Enkripsi Asimetris**:
*   **Public Key**: Hanya bisa digunakan untuk **mengunci (encrypt)** data. Berikan ini ke komputer yang bertugas melakukan backup rutin.
*   **Secret Key**: Satu-satunya kunci yang bisa **membuka (decrypt)** data. Simpan kunci ini di tempat yang sangat aman (misal: Password Manager atau USB drive fisik yang terpisah).

### Langkah 1: Membuat Pasangan Kunci (Public & Secret)
Pasangan kunci ini seperti **Gembok** dan **Kunci Fisik**. Anda membuat gembok (Public Key) untuk mengunci data, dan kunci rahasia (Secret Key) untuk membukanya nanti.

1.  **Jalankan Perintah Pembuat Kunci**:
    Buka terminal/command prompt, lalu ketik perintah berikut:
    ```bash
    ./target/debug/phone-backup keygen
    ```

2.  **Perhatikan Hasil di Layar**:
    Aplikasi akan menampilkan dua baris kode unik. **Jangan tutup terminal dulu!**
    *   **Public Key**: Berawalan `age1...`. Ini aman untuk dibagikan atau ditaruh di server backup.
    *   **Secret Key**: Berawalan `AGE-SECRET-KEY-1...`. Ini **SANGAT RAHASIA**.

3.  **Cara Menyimpan (Sangat Penting!)**:
    *   **Copy Public Key**: Blok kode yang berawalan `age1...`, klik kanan, lalu pilih **Copy**. Simpan di catatan biasa atau kirim ke komputer yang akan melakukan backup.
    *   **Copy Secret Key**: Blok kode yang berawalan `AGE-SECRET-KEY-1...`, klik kanan, pilih **Copy**. 
    *   **Simpan Secret Key di Tempat Aman**: Masukkan kode rahasia ini ke dalam **Password Manager** (seperti Bitwarden, 1Password) atau simpan di file teks di dalam USB Drive yang Anda simpan di laci terkunci. 
    *   *Ingat: Jika Secret Key hilang, data backup Anda TIDAK AKAN PERNAH bisa dibuka kembali.*

### Langkah 2: Distribusi dan Penggunaan
Setelah Anda memiliki kodenya:
1.  **Siapkan Public Key**: Anda butuh kode `age1...` setiap kali ingin menjalankan backup otomatis.
2.  **Amankan Secret Key**: Pastikan kode `AGE-SECRET-KEY-1...` **TIDAK ADA** di komputer yang menjalankan backup harian. Gunakan kunci ini hanya saat Anda butuh mengembalikan data (Restore).

### Langkah 3: Melakukan Backup Terenkripsi
Gunakan parameter `--pubkey` saat menjalankan backup. Engine akan mengenkripsi setiap file/blok data menggunakan kunci tersebut sebelum ditulis ke penyimpanan.
```bash
./target/debug/phone-backup --adapter adb \
  --pubkey "age1..." \
  backup <DEVICE_ID>
```

## 4. Melakukan Backup
Anda dapat melakukan backup penuh atau spesifik ke folder tertentu.

### Backup Folder Foto (Enkripsi Kunci Publik)
```bash
./target/debug/phone-backup --adapter adb \
  --pubkey "age1..." \
  backup <DEVICE_ID> --include /storage/emulated/0/DCIM/Camera
```

### Backup ke Cloud (S3/R2/MinIO)
```bash
./target/debug/phone-backup --storage s3 \
  --s3-bucket my-backups \
  --s3-endpoint https://<id>.r2.cloudflarestorage.com \
  --s3-access-key <key> \
  --s3-secret-key <secret> \
  backup <DEVICE_ID>
```

## 5. Melihat Riwayat (Snapshots)
Dapatkan daftar semua snapshot yang pernah dibuat untuk perangkat tersebut.

```bash
./target/debug/phone-backup snapshots <DEVICE_ID>
```

## 6. Pemulihan Data (Restore)
Gunakan **Kunci Rahasia** untuk mendekripsi data saat restorasi.

```bash
./target/debug/phone-backup --adapter adb \
  --privkey "AGE-SECRET-KEY-1..." \
  restore <SNAPSHOT_ID> --target ./folder_hasil_pulih
```

## 7. Pemeliharaan Repositori
### Cek Statistik Efisiensi (Deduplikasi)
Lihat berapa banyak ruang yang Anda hemat melalui fitur *Content-Addressed Storage* dan *Block-level Deduplication*.
```bash
./target/debug/phone-backup stats
```

### Verifikasi Integritas
Pastikan semua file/blok di repositori masih utuh dan tidak hilang.
```bash
./target/debug/phone-backup verify
```

---

## 💡 Tips Pro:
1.  **Resume Otomatis**: Jika backup terputus (kabel lepas atau disk penuh), cukup jalankan perintah yang sama lagi. Engine akan otomatis melanjutkan dari file terakhir yang gagal.
2.  **Streaming I/O**: Tidak perlu khawatir memori penuh saat backup video 4K berukuran besar. Engine mengalirkan data langsung dari HP ke storage.
3.  **Deduplikasi Blok**: Jika Anda sering melakukan backup pada file database aplikasi, fitur ini hanya akan menyimpan perubahan data yang baru, bukan seluruh file.

---
*Untuk bantuan lebih lanjut, silakan hubungi tim pengembang atau buka issue di GitHub.*
