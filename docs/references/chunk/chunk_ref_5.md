Dokumen **v2.0 ini sudah sangat kuat secara arsitektur**. Untuk Phone Backup Engine berbasis Rust, saya menilai arahnya sudah mendekati desain production-grade. Namun ada beberapa bagian yang perlu **dikoreksi secara teknis**, terutama urutan pipeline, Convergent Encryption, metadata, dan konsistensi database.

## Review singkat

| Area                  | Status | Catatan                                            |
| --------------------- | ------ | -------------------------------------------------- |
| Streaming pipeline    | ✅      | Sangat baik                                        |
| Bounded channels      | ✅      | Tepat                                              |
| Small-file grouping   | ✅      | Penting untuk mobile                               |
| Fixed-size + FastCDC  | ✅      | Kombinasi bagus                                    |
| BLAKE3                | ✅      | Sangat cocok                                       |
| Zstd                  | ⚠️     | Urutan pipeline perlu diperbaiki                   |
| Convergent encryption | ⚠️     | Ada risiko keamanan yang perlu ditangani           |
| `ref_count`           | ⚠️     | Jangan dijadikan satu-satunya sumber kebenaran     |
| SQLite active backup  | ⚠️     | Tidak selalu bisa dilakukan pada Android eksternal |
| GC                    | ⚠️     | Perlu strategi crash-safe                          |
| Metadata schema       | ⚠️     | Perlu snapshot/container/object layer              |

---

# 1. Perbaikan paling penting: urutan Pipeline

Pipeline Anda sekarang:

```text
Scanner
→ Grouping
→ Streaming Reader
→ Chunking
→ Hashing
→ Convergent Encryption
→ Dedup
→ Compression
→ Storage
```

Ada masalah pada urutan tersebut.

Jika tujuan deduplication adalah menemukan data identik, deduplication harus dilakukan berdasarkan representasi data yang konsisten.

Saya lebih menyarankan:

```text
Scanner
   │
   ▼
Grouping / File Classification
   │
   ▼
Streaming Reader
   │
   ▼
Chunking Engine
   │
   ▼
Plaintext Hash (BLAKE3)
   │
   ▼
Dedup Lookup
   │
   ├── Existing
   │      │
   │      └── Reuse Chunk Reference
   │
   └── New
          │
          ▼
       Compression
          │
          ▼
       Encryption
          │
          ▼
       Ciphertext Hash
          │
          ▼
       Storage
```

Secara umum:

```text
CHUNK
  ↓
HASH
  ↓
DEDUP
  ↓
COMPRESS
  ↓
ENCRYPT
  ↓
STORE
```

Ini menghindari CPU terbuang untuk:

* kompresi chunk yang ternyata sudah ada;
* enkripsi chunk yang ternyata sudah ada;
* upload data duplikat.

---

# 2. Catatan penting mengenai Convergent Encryption

Konsep Anda:

```text
Plaintext
   ↓
BLAKE3
   ↓
Derived Key
   ↓
Encrypt
```

secara konsep adalah **deterministic/message-locked encryption**.

Namun ada risiko penting:

```text
Same Plaintext
       =
Same Key
       =
Potential Equality Leakage
```

Artinya, deduplication sendiri dapat mengungkap bahwa dua backup memiliki konten yang sama.

Selain itu, skema naif:

```text
key = BLAKE3(chunk)
```

sebaiknya tidak langsung digunakan sebagai desain production tanpa domain separation dan KDF.

Lebih aman secara desain:

```text
plaintext_hash = BLAKE3(chunk)

chunk_key =
    HKDF(
        plaintext_hash,
        context = "phone-backup-chunk-v1"
    )
```

Atau secara konseptual:

```text
Chunk
  │
  ▼
BLAKE3 Hash
  │
  ▼
Key Derivation
  │
  ├── Encryption Key
  └── Storage Identity
```

Saya juga menyarankan membedakan:

```text
Content Hash
Storage Object ID
Encryption Key ID
```

Jangan semuanya memakai satu hash yang sama.

---

# 3. Domain Model perlu sedikit diperbaiki

Model saat ini:

```rust
pub struct ChunkMetadata {
    pub hash: ChunkHash,
    pub enc_hash: ChunkHash,
    pub size: u32,
    pub compressed_size: u32,
    pub storage_key: String,
}
```

Masalahnya:

* belum ada versi encoding;
* belum ada compression algorithm;
* belum ada encryption version;
* belum ada nonce/IV;
* `u32` membatasi ukuran sekitar 4 GiB;
* belum jelas apakah `enc_hash` adalah hash ciphertext atau ID object.

Saya menyarankan:

```rust
pub struct ChunkMetadata {
    pub content_hash: ChunkHash,
    pub object_hash: ChunkHash,

    pub plaintext_size: u64,
    pub stored_size: u64,

    pub compression: CompressionAlgorithm,
    pub encryption: EncryptionAlgorithm,

    pub format_version: u16,

    pub storage_key: StorageKey,
}
```

Kemudian metadata encryption dipisahkan:

```rust
pub struct EncryptionMetadata {
    pub algorithm: EncryptionAlgorithm,
    pub key_version: u32,
    pub nonce: Vec<u8>,
}
```

Ini membuat migrasi di masa depan lebih mudah.

Misalnya:

```text
v1
→ Zstd + XChaCha20

v2
→ Different Compression

v3
→ New Encryption
```

Backup lama masih dapat direstore.

---

# 4. Small Files: Virtual Container sangat bagus

Ini salah satu bagian terbaik dari desain Anda.

Saya menyebutnya:

```text
Small File Packing
```

Arsitektur:

```text
file_a.txt      2 KB
file_b.json     5 KB
file_c.log      1 KB
file_d.jpg      8 KB

        │
        ▼

Small File Container

┌──────────────────────┐
│ Container Metadata   │
├──────────────────────┤
│ File A               │
│ File B               │
│ File C               │
│ File D               │
└──────────────────────┘

Target: 4 MiB
```

Tetapi container harus memiliki internal index.

```text
Container

Offset      Length      File
--------------------------------
0           2048        a.txt
2048        5120        b.json
7168        1024        c.log
8192        8192        d.jpg
```

Domain:

```rust
pub struct PackedFileEntry {
    pub file_id: FileId,
    pub offset: u64,
    pub length: u64,
}
```

Ini penting saat restore satu file kecil.

---

# 5. Taksonomi Chunking saya sedikit revisi

Tabel Anda sudah bagus, tetapi untuk backup HP saya menyarankan:

| File                  | Strategi                   |
| --------------------- | -------------------------- |
| < 64 KB               | Small File Packing         |
| Foto JPG/HEIC         | File-Level Dedup           |
| Video MP4/MOV         | Fixed-size                 |
| APK                   | FastCDC atau File-Level    |
| SQLite DB             | FastCDC / snapshot-aware   |
| PDF/DOCX              | FastCDC                    |
| ZIP/RAR               | File-Level atau Fixed-size |
| Random encrypted data | Fixed-size                 |
| Log/Text              | FastCDC                    |

Kenapa ZIP/RAR tidak selalu cocok FastCDC?

Karena data terkompresi memiliki entropi tinggi. Deduplication pada level byte sering memberikan manfaat kecil.

---

# 6. Database aktif: batasan penting pada Android

Bagian ini:

```text
SQLite Online Backup API
```

bagus jika aplikasi backup memiliki akses langsung ke SQLite database.

Namun untuk **backup Android melalui ADB atau filesystem biasa**, Anda belum tentu dapat:

```text
Open SQLite database
```

karena:

* sandbox Android;
* permission;
* aplikasi sedang berjalan;
* database mungkin tidak dapat diakses;
* Online Backup API harus dijalankan melalui SQLite connection yang valid.

Strategi harus dibagi:

```text
Access Available?
      │
 ┌────┴─────┐
 YES         NO
 │           │
 ▼           ▼
SQLite API   Filesystem Snapshot
```

Jika hanya akses file:

```text
database.db
database.db-wal
database.db-shm
```

harus diperlakukan sebagai satu consistency group.

Namun menyalin `.db`, `.db-wal`, dan `.db-shm` **tidak otomatis menjamin snapshot konsisten** jika database aktif berubah selama proses penyalinan.

Ini harus menjadi bagian dari desain `ConsistencyStrategy`.

---

# 7. Ref Count jangan menjadi sumber kebenaran tunggal

Schema:

```sql
ref_count INTEGER DEFAULT 1
```

memang cepat.

Tetapi ada risiko:

```text
Snapshot dibuat
→ ref_count +1

Crash

Snapshot rollback gagal
→ ref_count salah
```

Saya menyarankan:

```text
Source of Truth
=
Snapshot/File/Chunk Relationship
```

Sedangkan:

```text
ref_count
=
Derived Optimization
```

Jadi GC dapat melakukan recovery.

Model:

```text
snapshots
    │
    ▼
files
    │
    ▼
file_chunks
    │
    ▼
chunks
```

GC bisa menghitung ulang referensi jika terjadi inconsistency.

---

# 8. Schema perlu Snapshot Layer

Schema Anda saat ini:

```text
chunks
   ↑
file_chunks
   ↑
files
```

Untuk backup system, saya sarankan:

```text
snapshots
    │
    ▼
snapshot_files
    │
    ▼
file_chunks
    │
    ▼
chunks
```

Contoh:

```sql
CREATE TABLE snapshots (
    id BLOB PRIMARY KEY,
    device_id BLOB NOT NULL,

    created_at INTEGER NOT NULL,
    status TEXT NOT NULL,

    format_version INTEGER NOT NULL
);
```

File metadata:

```sql
CREATE TABLE files (
    id BLOB PRIMARY KEY,

    path TEXT NOT NULL,
    size_bytes INTEGER NOT NULL,

    modified_at INTEGER,
    content_hash BLOB
);
```

Relasi snapshot:

```sql
CREATE TABLE snapshot_files (
    snapshot_id BLOB NOT NULL,
    file_id BLOB NOT NULL,

    PRIMARY KEY (snapshot_id, file_id)
);
```

Kemudian:

```sql
CREATE TABLE file_chunks (
    file_id BLOB NOT NULL,
    chunk_hash BLOB NOT NULL,

    position INTEGER NOT NULL,
    offset_bytes INTEGER NOT NULL,

    PRIMARY KEY (file_id, position)
);
```

---

# 9. Garbage Collection: gunakan state machine

Jangan langsung:

```text
No Reference
→ Delete
```

Karena bisa terjadi race condition dengan backup baru.

Saya menyarankan:

```text
ACTIVE
   │
   ▼
CANDIDATE
   │
   ▼
VERIFY
   │
   ├── Referenced
   │      └── ACTIVE
   │
   └── Unreferenced
          │
          ▼
        DELETE
```

Untuk storage:

```text
Chunk Object

ACTIVE
  ↓
GC_MARKED
  ↓
GC_PENDING
  ↓
DELETED
```

Ini lebih aman untuk crash recovery.

---

# 10. Data Scrubbing

Bagian ini sangat bagus, tetapi sebaiknya hash diverifikasi berdasarkan bentuk data.

Jika storage menyimpan:

```text
Compressed + Encrypted Data
```

maka proses:

```text
Storage Object
      │
      ▼
Verify Object Hash
      │
      ▼
Decrypt
      │
      ▼
Decompress
      │
      ▼
Verify Content Hash
```

Ada dua level integritas:

```text
object_hash
```

untuk mendeteksi kerusakan object yang tersimpan.

Dan:

```text
content_hash
```

untuk memastikan plaintext hasil restore benar.

---

# 11. Pipeline Concurrency yang saya rekomendasikan

```text
                     Bounded Channel
Scanner ──────────────────────────────────┐
                                         ▼
                                  File Classifier
                                         │
                                         ▼
                               Streaming Reader
                                         │
                                         ▼
                                  Chunk Workers
                                         │
                         ┌───────────────┴───────────────┐
                         ▼                               ▼
                   Fixed Chunker                      FastCDC
                         │                               │
                         └───────────────┬───────────────┘
                                         ▼
                                     BLAKE3
                                         │
                                         ▼
                                  Dedup Lookup
                                         │
                           ┌─────────────┴─────────────┐
                           ▼                           ▼
                       Reuse                      New Chunk
                                                       │
                                                       ▼
                                                    Zstd
                                                       │
                                                       ▼
                                                   Encrypt
                                                       │
                                                       ▼
                                                    Store
```

Backpressure:

```text
Uploader Slow
      │
      ▼
Channel Full
      │
      ▼
Hasher Waits
      │
      ▼
Chunker Waits
      │
      ▼
Reader Slows
```

Ini tepat untuk menjaga RAM.

---

# 12. Crate Architecture yang saya sarankan

```text
phone-backup/
│
├── domain/
│   ├── snapshot/
│   ├── file/
│   ├── chunk/
│   └── storage/
│
├── application/
│   ├── backup/
│   ├── restore/
│   ├── gc/
│   └── scrub/
│
├── ports/
│   ├── scanner_port.rs
│   ├── chunker_port.rs
│   ├── storage_port.rs
│   ├── dedup_port.rs
│   └── repository_port.rs
│
├── adapters/
│   ├── adb/
│   ├── sqlite/
│   ├── filesystem/
│   └── object_storage/
│
└── infrastructure/
    ├── crypto/
    ├── compression/
    └── hashing/
```

Ini sangat cocok dengan pendekatan Clean Architecture/DDD yang Anda gunakan.

---

# Verdict untuk v2.0

Saya akan menilai:

```text
Architecture       █████████░  9/10
Performance        █████████░  9/10
Scalability        ████████░░  8/10
Deduplication      █████████░  9/10
Security           ███████░░░  7/10
Operability        ████████░░  8/10
```

## Hal yang paling perlu diperbaiki sebelum coding

```text
1. Tentukan exact encryption threat model
2. Perbaiki urutan Dedup → Compress → Encrypt
3. Pisahkan content hash dan storage object hash
4. Tambahkan snapshot layer
5. Jadikan ref_count sebagai cache/optimization
6. Buat crash-safe GC
7. Buat consistency strategy untuk database aktif
8. Tambahkan format/version metadata
```

### Arsitektur final yang saya rekomendasikan

```text
SCAN
 │
 ▼
CLASSIFY
 │
 ▼
PACK SMALL FILES
 │
 ▼
STREAM
 │
 ▼
SELECT CHUNK STRATEGY
 │
 ├── FILE LEVEL
 ├── FIXED SIZE
 └── FASTCDC
 │
 ▼
CONTENT HASH (BLAKE3)
 │
 ▼
DEDUP LOOKUP
 │
 ├── HIT ──────────────► REFERENCE EXISTING OBJECT
 │
 └── MISS
       │
       ▼
     ZSTD
       │
       ▼
   ENCRYPT
       │
       ▼
   OBJECT HASH
       │
       ▼
      STORE
       │
       ▼
SNAPSHOT COMMIT
```

Menurut saya, dokumen berikutnya yang paling penting adalah **v3.0: “Chunk Lifecycle & Storage Transaction Design”**. Itu akan mendefinisikan secara detail lifecycle `Chunk → Pending → Stored → Referenced → Orphan → GC`, termasuk transaction SQLite, crash recovery, dedup race condition, dan atomic snapshot commit.
