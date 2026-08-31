# 💾 Storage & Deduplication

Platform **phone-backup** menggunakan strategi penyimpanan modern yang menggabungkan **Content-Addressed Storage (CAS)**, pemotongan blok dinamis **FastCDC**, dan abstraksi multi-backend melalui **OpenDAL**.

---

## 1. Content-Addressed Storage (CAS)

Dalam penyimpanan berbasis konten (CAS), setiap berkas dan potongan data tidak diidentifikasi berdasarkan nama aslinya, melainkan berdasarkan **hash kriptografis SHA-256**.

### Keunggulan CAS:
1. **Deduplikasi Global Otomatis**: Jika dua foto berukuran 50 MB memiliki isi yang sama (meskipun berada di folder berbeda atau di HP yang berbeda), hanya satu objek 50 MB yang disimpan di disk.
2. **Integritas Bawaan**: Kerusakan berkas (*bit rot*) dapat dideteksi secara langsung dengan menghitung ulang hash dan membandingkannya dengan ID objek.

### Struktur Sharding Direktori:
Untuk mencegah penurunan performa sistem berkas saat menyimpan jutaan objek, CAS menerapkan sharding 2 tingkat:
```text
workspace/
└── objects/
    ├── a1/
    │   └── b2/
    │       └── a1b2c3d4e5f6... (blob terenkripsi)
    └── ff/
        └── 09/
            └── ff0978abc123... (blob terenkripsi)
```

---

## 2. Deduplikasi Tingkat Blok (FastCDC)

Untuk berkas besar seperti database atau arsip, perubahan kecil di tengah berkas biasanya membuat seluruh hash berubah.
- **FastCDC (Fast Content-Defined Chunking)**: Membagi berkas besar menjadi potongan-potongan (*chunks*) berukuran variabel berdasarkan pola konten data.
- **Efisiensi**: Saat berkas dimodifikasi, hanya potongan (*chunk*) yang berubah yang perlu dienkripsi dan disimpan ulang, menghemat ruang penyimpanan hingga 80%.

---

## 3. Backend Cloud Storage (OpenDAL)

Melalui adapter `adapters/opendal`, platform dapat menyimpan objek langsung ke penyedia cloud yang kompatibel dengan protokol S3:
- **Amazon AWS S3**
- **Cloudflare R2**
- **MinIO (Self-hosted S3)**
- **Wasabi / Backblaze B2**

### Contoh Penggunaan Cloud Storage:
```bash
phone-backup --storage opendal \
  --s3-bucket "my-backups" \
  --s3-endpoint "https://<account-id>.r2.cloudflarestorage.com" \
  --s3-access-key "KEY_ID" \
  --s3-secret-key "SECRET_KEY" \
  --adapter adb backup -p "Password123" <DEVICE_ID>
```

---

## 4. Pemeliharaan & Garbage Collection (GC)

Saat snapshot lama dihapus atau di-pruning, beberapa objek di direktori `objects/` mungkin tidak lagi dirujuk oleh snapshot mana pun (*orphan objects*).

Jalankan perintah `gc` untuk membersihkan objek sampah dan mengembalikan ruang disk:
```bash
phone-backup gc
```

---
*Lanjutkan ke: [Wireless Companion Agent](Wireless-Companion-Agent.md) atau [Contacts & Data Management](Contacts-and-Data-Management.md).*
