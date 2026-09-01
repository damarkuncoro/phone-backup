# Analisis Perbandingan: Implementasi Saat Ini vs. Spesifikasi v4.0

Dokumen ini memetakan posisi teknis kode sumber **Phone Backup Engine** saat ini terhadap target **Spesifikasi Teknis v4.0 (Final)**. Analisis ini berfungsi sebagai panduan migrasi dan prioritas pengembangan.

---

## 1. Tabel Perbandingan Ringkas

| Fitur | Implementasi Saat Ini (Current) | Spesifikasi v4.0 (Target) | Status Gaps |
| :--- | :--- | :--- | :--- |
| **Unit Deduplikasi** | File-level (Seluruh file) | Chunk-level (Sub-file / FastCDC) | 🔴 Major Gap |
| **Urutan Pipeline** | Hash -> Encrypt -> Upload | Hash -> Dedup -> Compress -> Encrypt | 🟡 Moderate |
| **Keamanan** | Basic Encrypt (Deterministic) | HKDF Convergent Encryption | 🟡 Moderate |
| **Reliabilitas** | File-level Resume | Chunk-level Resume & Commit Protocol | 🔴 Major Gap |
| **Storage Identity** | Hash-based Filename | UUIDv7 (Obfuscated Storage) | 🟢 Minor |
| **Metadata Schema** | File-centric | Logical vs Physical Separation | 🔴 Major Gap |

---

## 2. Analisis Mendalam (Gaps & Impact)

### A. Deduplikasi & Efisiensi Storage
*   **Kondisi Sekarang (`uploader.rs`):** Menggunakan `hash_sha256` dari seluruh file. Jika file database SQLite sebesar 100MB berubah 1 baris, sistem akan mengupload ulang 100MB dan menyimpan 2 versi penuh.
*   **V4.0 Target:** Menggunakan `FastCDC`. Hanya chunk yang berubah (misal 4KB) yang akan diupload. 
*   **Dampak:** Penghematan storage hingga 90% pada backup berulang (incremental) dan efisiensi bandwidth yang masif.

### B. Arsitektur Pipeline & Performa CPU
*   **Kondisi Sekarang (`backup.rs`):** Logika pengecekan duplikasi terkadang dilakukan setelah beberapa proses awal.
*   **V4.0 Target:** **Strict Pipeline**. Deduplikasi (lookup hash plaintext) dilakukan di awal. Jika data sudah ada, sistem langsung berhenti untuk chunk tersebut.
*   **Dampak:** Mengurangi beban CPU secara signifikan karena menghindari kompresi dan enkripsi pada data duplikat.

### C. Keamanan & Metadata
*   **Kondisi Sekarang:** Nama objek di storage seringkali merupakan hash langsung dari konten. Ini memungkinkan "Known-hash attack" di mana penyerang bisa tahu file apa yang Anda miliki meskipun terenkripsi.
*   **V4.0 Target:** Menggunakan **UUIDv7** untuk nama objek fisik dan **HKDF** untuk kunci enkripsi. Identitas fisik dan logikal dipisahkan sepenuhnya di database.
*   **Dampak:** Privasi data jauh lebih tinggi (*Zero-knowledge metadata*).

### D. Penanganan Interupsi (Reliability)
*   **Kondisi Sekarang:** Jika backup terputus di tengah file 1GB, saat di-*resume*, sistem harus mengulang dari awal file tersebut (karena hash belum terbentuk).
*   **V4.0 Target:** **Snapshot Commit Protocol**. Karena setiap chunk diverifikasi dan dicatat secara atomik, sistem bisa melanjutkan tepat dari byte terakhir yang berhasil di-chunk.
*   **Dampak:** Pengalaman pengguna yang jauh lebih stabil, terutama pada koneksi USB/ADB yang tidak stabil.

---

## 3. Prioritas Migrasi (Next Steps)

Berdasarkan analisis di atas, berikut adalah urutan modul yang perlu diperbarui:

1.  **Refaktor Database Schema (Priority 1)**:
    Ubah skema SQLite untuk mendukung tabel `chunks` (logikal) dan `chunk_objects` (fisik) sesuai v4.0.
    
2.  **Implementasi Chunking Engine (Priority 2)**:
    Integrasikan library `fastcdc` ke dalam `processor.rs` untuk menggantikan pemrosesan satu-file-utuh.

3.  **Update Security Module (Priority 3)**:
    Tambahkan fungsi derivasi kunci menggunakan `HKDF` dan enkripsi yang mendukung *deterministic output* untuk dedup.

4.  **Implementasi Recovery Manager (Priority 4)**:
    Tambahkan logika status state-machine pada snapshot untuk menangani *crash recovery* secara otomatis.

---

## 4. Kesimpulan
Implementasi saat ini adalah fondasi yang solid untuk backup file tradisional. Namun, untuk menjadi **Engine Backup Professional** yang kompetitif (seperti Restic atau Borg), transisi ke **Spesifikasi v4.0** (Deduplicated Chunk-based Engine) adalah langkah yang wajib dilakukan.
