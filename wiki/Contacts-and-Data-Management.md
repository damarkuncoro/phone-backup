# 👥 Contacts & Data Management

Platform **phone-backup** memperlakukan data terstruktur smartphone (**Kontak, SMS, Riwayat Telepon, dan Metadata Aplikasi**) sebagai entitas relasional kelas satu (*first-class relational entities*).

---

## 1. Ekstraksi Data Kontak Mendalam

Saat sesi backup berlangsung, engine mengekstrak data buku telepon secara menyeluruh melalui Android Content Provider (`content://com.android.contacts/data`) ke dalam skema relasional SQLite:

### Bidang Data yang Didukung:
- **Nama & Identitas**: Display Name, Given Name, Family Name, Prefix, Suffix.
- **Multi-Nomor Telepon**: Mobile, Home, Work, WhatsApp dengan normalisasi format internasional E.164.
- **Email & Alamat**: Email kantor/pribadi dan alamat fisik lengkap.
- **Organisasi**: Perusahaan, departemen, dan jabatan.
- **Catatan & Label Kustom**: Catatan khusus yang tersimpan pada kontak.

---

## 2. Format Standar Universal vCard (RFC 6350)

Data kontak dapat diekspor langsung ke format standar industri **vCard 4.0 / 3.0 (`.vcf`)**.

### Kompatibilitas Impor 100%:
- **Google Contacts** (Android baru / Web)
- **Apple Contacts** (iPhone / iPad / macOS)
- **Microsoft Outlook** & **Mozilla Thunderbird**

### Cara Ekspor vCard:
- Buka **Desktop GUI** (`cargo tauri dev`) ➔ Masuk ke tab **👥 Android Data Explorer** ➔ Klik tombol **Export vCard**.

---

## 3. Pencarian Cepat Global (Full-Text Search FTS5)

Anda dapat menemukan kontak, berkas, atau isi pesan SMS secara instan tanpa perlu memulihkan seluruh arsip backup:

```bash
# Cari kontak berdasarkan nama atau potongan nomor
phone-backup contacts "Damar"
phone-backup contacts "+62859"

# Cari pesan SMS atau kode OTP
phone-backup sms "Bank"
phone-backup sms "Verification"

# Cari berkas atau foto
phone-backup search "screenshot_2026"
```

---

## 4. Strategi Backup Khusus Kontak (< 5 Detik)

Jika Anda hanya ingin mencadangkan buku telepon secara instan tanpa menyalin puluhan gigabyte foto atau video besar:

```bash
phone-backup --adapter adb backup -i /storage/emulated/0/Download/ -p "PasswordKontak" <DEVICE_ID>
```
*Seluruh data kontak dan SMS otomatis tercadangkan dan terindeks dalam hitungan detik.*

---
*Lanjutkan ke: [Troubleshooting & FAQ](Troubleshooting-and-FAQ.md) atau [Developer Guide & Testing](Developer-Guide-and-Testing.md).*
