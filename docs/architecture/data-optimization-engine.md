# Phone Backup Data Optimization & Compression Engine

Dokumen ini memetakan arsitektur optimasi data, kompresi, deduplikasi, dan strategi enkripsi yang diterapkan pada platform **Phone Backup**, mengintegrasikan 19 pilar kompresi modern.

---

## 🏗️ 1. Pipeline Alur Data (*Data Lifecycle Pipeline*)

Phone Backup bukan sekadar pembungkus (*wrapper*) library kompresi, melainkan sebuah **Data Optimization Engine**:

```text
                        SOURCE FILE / STREAM
                                 │
                                 ▼
                     ┌───────────────────────┐
                     │   Mime & Entropy Sniff │ (Lewati jika JPEG, MP4, APK, ZIP)
                     └───────────┬───────────┘
                                 │
                                 ▼
                     ┌───────────────────────┐
                     │ FastCDC Dynamic Chunk │ (Potong chunk variabel 1KB-64KB)
                     └───────────┬───────────┘
                                 │
                                 ▼
                     ┌───────────────────────┐
                     │   CAS Deduplication   │ (Cek SHA-256 di SQLite, skip jika ada)
                     └───────────┬───────────┘
                                 │ (Hanya chunk unik baru)
                                 ▼
                     ┌───────────────────────┐
                     │ Adaptive Zstd/LZ4 +   │ (Kamus terlatih & level adaptif 1-19)
                     │  Dictionary Training  │
                     └───────────┬───────────┘
                                 │
                                 ▼
                     ┌───────────────────────┐
                     │ Age X25519 Encryption │ (Enkripsi asimetris zero-knowledge)
                     └───────────┬───────────┘
                                 │
                                 ▼
                     ┌───────────────────────┐
                     │  Storage Port Write   │ (Local Disk / S3 / MinIO / WebDAV NAS)
                     └───────────────────────┘
```

---

## 📊 2. Pemetaan Fitur ke Codebase

| Pilar Kompresi | Komponen di Codebase | Lokasi File Implementasi |
| :--- | :--- | :--- |
| **Content-Defined Chunking** | `FastCdcChunker` | [fastcdc.rs](file:///Users/damarkuncoro/antigravity/phone-backup/libs/storage/chunking/src/fastcdc.rs) |
| **CAS Cross-File Deduplication** | `ContentAddressableStorage` | [cas.rs](file:///Users/damarkuncoro/antigravity/phone-backup/libs/core/application/src/backup/cas.rs) |
| **Zstd Trained Dictionaries** | `AutoDictionaryTrainer` | [auto_dictionary.rs](file:///Users/damarkuncoro/antigravity/phone-backup/libs/storage/compression/src/auto_dictionary.rs) |
| **Media & Entropy Bypass** | `MimeBypassDetector` | [mime_bypass.rs](file:///Users/damarkuncoro/antigravity/phone-backup/libs/storage/compression/src/mime_bypass.rs) |
| **Adaptive Compression** | `AdaptiveCompressionEngine` | [adaptive.rs](file:///Users/damarkuncoro/antigravity/phone-backup/libs/storage/compression/src/adaptive.rs) |
| **Specialized Binary Parsers** | AXML Pure-Rust & Audio Sniffer | [axml/](file:///Users/damarkuncoro/antigravity/phone-backup/libs/data/apps/src/axml/) & [audio/](file:///Users/damarkuncoro/antigravity/phone-backup/libs/media/audio/) |
| **Entity Delta Differencing** | `ContactDiffEngine` | [diff.rs](file:///Users/damarkuncoro/antigravity/phone-backup/libs/data/contacts/src/diff.rs) |
| **Parallel Multi-threading** | Rayon Parallel Thread Pool | [compressor.rs](file:///Users/damarkuncoro/antigravity/phone-backup/libs/storage/compression/src/compressor.rs) |
| **Safety & Strict Sequencing** | Thermal Guard & X25519 Age | [uploader.rs](file:///Users/damarkuncoro/antigravity/phone-backup/libs/core/application/src/backup/uploader.rs) |

---

## 🚀 3. Rencana Inovasi & Pengembangan Lanjutan (Next Horizons)

Berikut adalah inisiatif teknis yang dapat kita kembangkan lebih lanjut untuk meningkatkan efisiensi:

### A. Page-Aware SQLite Zero-Fill Preconditioning
Database SQLite sering mengandung *free pages* (ruang kosong sisa penghapusan) yang berantakan:
- **Konsep**: Membuat zero-copy filter yang mendeteksi header halaman kosong SQLite dan menggantinya dengan byte `0x00` sebelum kompresi tanpa merusak struktur database.
- **Dampak**: Kompresibilitas file `.db` WhatsApp / Kontak meningkat drastis ($>40\%$).

### B. Categorized Zstd Dictionary Pools
Saat ini dictionary dilatih secara umum:
- **Konsep**: Membuat kamus terlatih spesifik kategori:
  1. `dict_android_manifest.zstd` (spesifik XML Android & permissions)
  2. `dict_json_vcard.zstd` (spesifik kontak vCard & JSON payload)
  3. `dict_chat_whatsapp.zstd` (spesifik teks pesan & emoji header)
- **Dampak**: Kompresi data berukuran kecil ($<4\text{ KB}$) mencapai rasio hingga $5.5\times$ lebih padat.

### C. SIMD-Accelerated Rolling Hash (AVX-512 / ARM Neon)
- **Konsep**: Optimasi algoritma hashing FastCDC v3 menggunakan instruksi SIMD hardware pada macOS Apple Silicon dan prosesor Intel/AMD x86_64.
- **Dampak**: Throughput chunking melonjak dari $800\text{ MB/s}$ ke $>2.5\text{ GB/s}$.

### D. Dynamic CDC Window Adaptation
- **Konsep**: Menyesuaikan rata-rata ukuran chunk ($4\text{ KB}$ vs $16\text{ KB}$ vs $64\text{ KB}$) secara otomatis berdasarkan bandwidth koneksi (Wi-Fi 6 vs USB 3.2 vs WebDAV Cloud).
