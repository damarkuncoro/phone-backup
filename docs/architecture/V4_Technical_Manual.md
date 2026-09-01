# Manual Teknis V4.0: Deduplikasi Expert & Penyimpanan Ter-obfuscate

Dokumen ini menjelaskan implementasi teknis dari Spesifikasi V4.0 yang telah diterapkan pada Phone Backup Engine. Fokus utama adalah pada pemisahan identitas data dari representasi fisiknya.

---

## 1. Pemisahan Logikal vs Fisik (Two-Tier Architecture)

Berbeda dengan sistem backup tradisional, V4.0 memisahkan data menjadi dua lapisan:

### Layer 1: Logical Chunks (Identitas Konten)
Disimpan dalam tabel `chunks`.
*   **Kunci Utama**: BLAKE3 hash dari konten asli (plaintext).
*   **Fungsi**: Menjamin bahwa konten yang sama hanya diproses satu kali di seluruh repositori (Global Deduplication).
*   **Keuntungan**: Efisiensi ruang hingga 90% pada data berulang.

### Layer 2: Physical Objects (Representasi Penyimpanan)
Disimpan dalam tabel `chunk_objects`.
*   **Kunci Utama**: UUIDv7 (Acak dan ter-obfuscate).
*   **Fungsi**: Mengelola bagaimana data disimpan di disk (setelah dikompresi Zstd dan dienkripsi).
*   **Keuntungan**: Memungkinkan penggantian algoritma enkripsi/kompresi di masa depan tanpa merusak referensi file asli.

---

## 2. Expert Chunking Engine

Sistem menggunakan **Expert Chunker** yang memilih strategi secara cerdas berdasarkan tipe file:

| Tipe File | Metode | Konfigurasi | Alasan |
| :--- | :--- | :--- | :--- |
| **Video (MP4/MOV)** | `FixedSize` | 4MB Avg | Performa I/O maksimal, streaming cepat. |
| **Database (SQLite)** | `FastCDC` | 512KB Avg | Sensitif terhadap penyisipan data kecil. |
| **File Kecil (<128KB)** | `FullFile` | N/A | Mengurangi overhead metadata database. |
| **Umum (APK/Doc)** | `FastCDC` | 1MB Avg | Keseimbangan terbaik performa vs dedup. |

---

## 3. Aliran Data (Pipeline)

Pipeline diimplementasikan menggunakan **Bounded Channels** (Tokio/Rayon) untuk menjamin penggunaan memori tetap rendah:

1.  **Scanner**: Mendeteksi file di perangkat Android.
2.  **Streaming Reader**: Membaca byte dari USB/Wi-Fi tanpa memuat semuanya ke RAM.
3.  **Chunker**: Memecah aliran byte menjadi potongan-potongan kecil.
4.  **Logical Dedup**: Mengecek apakah hash konten sudah ada di database.
5.  **Processor (New Object)**: Jika konten baru -> Kompresi (Zstd) -> Enkripsi (XChaCha20) -> Hitung Hash Ciphertext.
6.  **Storage**: Menulis ke disk menggunakan jalur ter-obfuscate: `objects/v4/aa/bb/UUIDv7`.

---

## 4. Keamanan & Privasi (Obfuscation)

Untuk meningkatkan privasi, V4.0 menggunakan skema penyimpanan **Zero-Knowledge Pathing**:
*   Nama file di storage tidak lagi berupa hash plaintext.
*   Digunakan **UUIDv7** sebagai nama file fisik.
*   Struktur folder dipisahkan berdasarkan 4 karakter pertama UUID untuk performa filesystem.
*   Tanpa akses ke database metadata (SQLCipher), penyerang tidak dapat mengetahui isi atau struktur file hanya dengan melihat folder `objects/`.

---

## 5. Pemeliharaan (GC & Scrubbing)

*   **Garbage Collection**: Menghapus entri di `chunk_objects` yang tidak lagi dirujuk oleh tabel `file_chunks`.
*   **Data Scrubbing**: Memvalidasi integritas fisik objek menggunakan `object_hash` (ciphertext hash) tanpa perlu melakukan dekripsi (cepat dan efisien).
