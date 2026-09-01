# Phone Backup Engine: Technical Specification v4.1 (Implementation Reference)

Dokumen ini merinci implementasi teknis final dari spesifikasi V4.0 yang telah diterapkan pada modul `phone-backup-chunking` dan `ObjectManager`.

---

## 1. Arsitektur Data Berjenjang (Implemented)

Sistem menggunakan model empat tingkat untuk fleksibilitas maksimal:
1.  **File**: Metadata user (nama, path, permissions).
2.  **FileChunks**: Tabel relasi yang menentukan urutan potongan konten untuk membentuk file kembali.
3.  **Logical Chunks**: Identitas konten unik berbasis **BLAKE3**. Data yang sama hanya diolah satu kali.
4.  **Physical Objects**: File fisik di penyimpanan yang dinamai dengan **UUIDv7** (Obfuscated).

### Jalur Penyimpanan Fisik (Sharding)
Objek disimpan dengan pola: `objects/v4/{prefix1}/{prefix2}/{UUIDv7}`
*Contoh: `objects/v4/01/99/0199c8b2-7bdf-7de6-9f00-8997ebea6864`*

---

## 2. Mesin Chunking Ahli (Expert Chunker)

Terletak di: `libs/storage/chunking`

### Strategi Terpilih (ChunkingPolicy)
| Kategori | Strategi Aktual | Parameter (Min/Avg/Max) |
| :--- | :--- | :--- |
| **Video** | `FixedSize` | 1MB / 4MB / 8MB |
| **SQLite DB** | `FastCDC` | 128KB / 512KB / 2MB |
| **File Kecil** | `FullFile` | N/A (Single Chunk) |
| **Standard** | `FastCDC` | 256KB / 1MB / 2MB |

---

## 3. Protokol Keamanan & Enkripsi

### Convergent Encryption (Message-Locked)
Untuk mendukung deduplikasi pada data terenkripsi:
1.  **Identity**: Hash plaintext (BLAKE3) digunakan sebagai basis identitas konten.
2.  **Key Derivation**: Menggunakan **HKDF-SHA256** dengan konteks `phone-backup-v4-chunk-key` untuk menghasilkan kunci enkripsi unik per chunk.
3.  **Encryption**: Menggunakan **XChaCha20-Poly1305** (AEAD) untuk keamanan tingkat tinggi dan performa streaming.

---

## 4. Pipeline Streaming (Memory Efficiency)

Pipeline diimplementasikan menggunakan **Bounded Channels** untuk menjamin penggunaan memori < 128MB bahkan untuk file berukuran terabyte.

```text
Reader (Device) -> [Channel] -> Chunker -> [Channel] -> Hasher/Dedup -> [Channel] -> Encryptor -> Uploader
```
*Backpressure otomatis akan memperlambat Reader jika Uploader (misal Cloud S3) sedang lambat.*

---

## 5. Status Implementasi & Roadmap

- [x] **Monorepo Reorganization**: Pemisahan `apps/` dan `libs/`.
- [x] **Two-Tier Dedup Schema**: Pemisahan Logikal vs Fisik di SQLite.
- [x] **Expert Chunking Lib**: Library mandiri dengan FastCDC & FixedSize.
- [x] **UUIDv7 Storage Key**: Obfuscation jalur penyimpanan.
- [x] **Snapshot Commit Protocol**: Implementasi Manifest atomic file.
- [ ] **Cloud Storage Adapter**: Integrasi OpenDAL untuk S3/R2.
- [ ] **GC Service**: Background orphan physical object cleaner.

---
*Dokumen ini bersifat otoritatif untuk semua pengembangan modul storage dan backup.*
