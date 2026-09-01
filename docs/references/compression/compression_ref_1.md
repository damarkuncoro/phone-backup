Kalau yang dimaksud **metode kompresi data/file**, secara umum ada **2 kategori utama**:

### 1. Lossless Compression

Data setelah dikompresi bisa dikembalikan **100% sama persis** seperti aslinya.

Contoh metode:

* **Huffman Coding**
* **LZ77**
* **LZ78**
* **LZW**
* **LZMA / LZMA2**
* **DEFLATE** → kombinasi LZ77 + Huffman
* **Brotli**
* **Zstandard (Zstd)**
* **Snappy**
* **LZ4**
* **BWT (Burrows-Wheeler Transform)**

Contoh penggunaan:
`ZIP`, `GZIP`, `7z`, `PNG`, database backup, source code, dan **backup HP**.

### 2. Lossy Compression

Sebagian informasi dibuang agar ukuran jauh lebih kecil. Hasilnya **tidak identik 100%** dengan file asli.

Contoh:

* **JPEG** → foto
* **WebP lossy** → gambar
* **AVIF lossy** → gambar
* **MP3 / AAC / Opus** → audio
* **H.264 / H.265 / AV1** → video

---

### Kalau untuk Phone Backup Engine

Untuk proyek backup HP yang sedang kita rancang, saya sarankan jangan hanya menggunakan satu metode:

```text
                 BACKUP DATA
                     │
        ┌────────────┴────────────┐
        │                         │
     LOSSLESS                    LOSSY
        │                         │
  ┌─────┼─────┐              ┌────┼────┐
  │     │     │              │    │    │
 Zstd  LZ4  ZIP             JPEG WebP AVIF
  │     │     │
  │     │     └─ Compatibility
  │     └─────── High Speed
  └───────────── Best Compression
```

Untuk **backup**, pilihan utama saya:

**Zstandard (Zstd)** → default
**LZ4** → mode sangat cepat
**GZIP/ZIP** → kompatibilitas
**No compression** → file yang memang sudah compressed

Dan penting: **Jangan mengompresi JPEG, MP4, MP3, APK, ZIP, 7z, dll. secara membabi buta**, karena biasanya ukuran hampir tidak berkurang tetapi CPU dan waktu backup bertambah.

Kalau yang Anda maksud adalah **berapa banyak metode kompresi yang bisa kita implementasikan di Phone Backup Engine**, saya bisa buatkan **Compression Engine lengkap (Zstd + LZ4 + Deflate + Brotli + auto-detection + chunk compression)** beserta desain Rust-nya.
