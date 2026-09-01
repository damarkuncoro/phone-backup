Ya. **Chunking Lab** adalah project khusus untuk **menguji, membandingkan, dan memvalidasi berbagai algoritma chunking** sebelum kita memasukkannya ke Backup Engine utama.

Untuk proyek backup HP berbasis Rust Anda, saya sarankan membuatnya sebagai **laboratorium terpisah**, bukan langsung mencampurkan eksperimen ke production code.

# Chunking Lab for Rust

## Tujuan

```text
INPUT FILES
    │
    ▼
┌──────────────────────┐
│   CHUNKING LAB       │
│                      │
│ Fixed Size           │
│ Rabin CDC            │
│ FastCDC              │
│ BuzHash              │
│ Gear Hash            │
│ Hybrid               │
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ Benchmark Engine     │
│                      │
│ Speed                │
│ Chunk Count          │
│ Average Size         │
│ Deduplication Ratio  │
│ CPU                  │
│ Memory               │
└──────────┬───────────┘
           │
           ▼
       REPORT
```

---

# 1. Struktur Workspace Rust

Saya merekomendasikan Cargo Workspace.

```text
chunking-lab/
│
├── Cargo.toml
│
├── crates/
│   │
│   ├── chunking-core/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── chunk.rs
│   │   │   ├── config.rs
│   │   │   ├── error.rs
│   │   │   └── strategy.rs
│   │   │
│   │   └── Cargo.toml
│   │
│   ├── chunking-fixed/
│   │   ├── src/
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   ├── chunking-rabin/
│   │   ├── src/
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   ├── chunking-gear/
│   │   ├── src/
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   ├── chunking-buzhash/
│   │   ├── src/
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   ├── chunking-fastcdc/
│   │   ├── src/
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   ├── chunking-hybrid/
│   │   ├── src/
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   ├── chunking-dedup/
│   │   ├── src/
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   ├── chunking-benchmark/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── runner.rs
│   │   │   ├── metrics.rs
│   │   │   └── report.rs
│   │   └── Cargo.toml
│   │
│   └── chunking-cli/
│       ├── src/
│       │   └── main.rs
│       └── Cargo.toml
│
├── datasets/
│   ├── images/
│   ├── videos/
│   ├── documents/
│   ├── databases/
│   ├── apk/
│   └── modified/
│
├── reports/
│
└── README.md
```

---

# 2. Core Domain

## Chunk

```rust
pub struct Chunk {
    pub index: u64,
    pub offset: u64,
    pub size: u32,
    pub hash: Option<ChunkHash>,
}
```

Saya sengaja tidak menyimpan `Vec<u8>` di domain metadata.

Untuk file besar:

```text
Chunk Metadata
    │
    ├── index
    ├── offset
    ├── size
    └── hash
```

Data dibaca secara streaming.

---

## ChunkHash

```rust
pub struct ChunkHash(pub [u8; 32]);
```

Nantinya dapat menggunakan:

```text
BLAKE3
```

atau:

```text
SHA-256
```

Untuk Chunking Lab saya menyarankan BLAKE3.

---

# 3. Trait Utama

Ini adalah inti seluruh project.

```rust
use std::io::Read;

pub trait ChunkingStrategy: Send + Sync {
    fn name(&self) -> &'static str;

    fn chunk(
        &self,
        reader: &mut dyn Read,
    ) -> anyhow::Result<Vec<Chunk>>;
}
```

Namun untuk production saya lebih menyukai streaming API.

Versi lebih baik:

```rust
use std::io::Read;

pub trait ChunkingStrategy: Send + Sync {
    fn name(&self) -> &'static str;

    fn next_chunk(
        &mut self,
        reader: &mut dyn Read,
    ) -> anyhow::Result<Option<Chunk>>;
}
```

Atau menggunakan callback:

```rust
pub trait ChunkingStrategy {
    fn chunk(
        &mut self,
        reader: &mut dyn Read,
        on_chunk: &mut dyn FnMut(Chunk),
    ) -> anyhow::Result<()>;
}
```

Dengan ini:

```text
File 20 GB

Chunk
 ↓
Process
 ↓
Release Memory

Chunk berikutnya
 ↓
Process
```

Tidak perlu:

```text
20 GB → RAM
```

---

# 4. Fixed Size Chunker

Implementasi pertama.

```rust
pub struct FixedSizeChunker {
    chunk_size: usize,
}
```

Contoh:

```text
chunk_size = 4 MiB
```

Pipeline:

```text
File
 │
 ├── 4 MiB
 ├── 4 MiB
 ├── 4 MiB
 └── Remaining
```

CLI:

```text
chunklab run \
  --algorithm fixed \
  --chunk-size 4MiB \
  --input datasets/video.mp4
```

---

# 5. Rabin CDC

Konfigurasi:

```rust
pub struct RabinConfig {
    pub min_size: usize,
    pub avg_size: usize,
    pub max_size: usize,
    pub window_size: usize,
}
```

Contoh:

```text
Min     1 MiB
Average 4 MiB
Max     8 MiB
Window  48 bytes
```

Boundary:

```text
Rabin Fingerprint
       │
       ▼
fingerprint & mask == target
       │
       ▼
Chunk Boundary
```

---

# 6. FastCDC

Konfigurasi:

```rust
pub struct FastCdcConfig {
    pub min_size: usize,
    pub avg_size: usize,
    pub max_size: usize,
}
```

Contoh:

```text
Min = 1 MiB
Avg = 4 MiB
Max = 8 MiB
```

Flow:

```text
Start
  │
  ▼
Read until Min
  │
  ▼
Search Normal Boundary
  │
  ├── Found
  │     │
  │     ▼
  │   Chunk
  │
  ▼
Search Max Boundary
  │
  ├── Found
  │
  └── Max → Force Chunk
```

---

# 7. BuzHash

Config:

```rust
pub struct BuzHashConfig {
    pub min_size: usize,
    pub avg_size: usize,
    pub max_size: usize,
    pub window_size: usize,
}
```

Rolling window:

```text
ABCDE
BCDEF
CDEFG
```

Cocok untuk eksperimen perbandingan dengan Rabin.

---

# 8. Gear Hash

```rust
pub struct GearChunker {
    min_size: usize,
    avg_size: usize,
    max_size: usize,
}
```

Pipeline:

```text
Byte
 │
 ▼
Gear Table Lookup
 │
 ▼
Rolling Hash
 │
 ▼
Boundary Test
```

---

# 9. Hybrid Chunker

Hybrid tidak perlu langsung.

Tetapi desainnya:

```rust
pub trait ChunkingPolicy {
    fn select(
        &self,
        file: &FileInfo,
    ) -> Box<dyn ChunkingStrategy>;
}
```

Contoh policy:

```text
File < 1 MB
→ Fixed

Video
→ Fixed 8 MB

Database
→ FastCDC

APK
→ FastCDC

Text
→ FastCDC
```

---

# 10. Deduplication Engine

Chunker menghasilkan:

```text
Chunk Metadata
```

Kemudian:

```text
Chunk Data
    │
    ▼
BLAKE3
    │
    ▼
Hash
    │
    ▼
Hash Index
    │
    ├── Exists
    │     └── Reuse
    │
    └── New
          └── Store
```

Interface:

```rust
pub trait ChunkIndex {
    fn contains(
        &self,
        hash: &ChunkHash,
    ) -> anyhow::Result<bool>;
}
```

Metric:

```text
Total Chunks

Unique Chunks

Duplicate Chunks

Bytes Reused

Dedup Ratio
```

---

# 11. Benchmark Engine

Setiap algoritma harus diuji dengan kondisi sama.

```rust
pub struct BenchmarkResult {
    pub algorithm: String,

    pub input_bytes: u64,

    pub chunk_count: u64,

    pub min_chunk_size: u64,

    pub max_chunk_size: u64,

    pub average_chunk_size: u64,

    pub elapsed_ms: u128,

    pub throughput_mbps: f64,

    pub unique_chunks: u64,

    pub duplicate_chunks: u64,

    pub dedup_ratio: f64,
}
```

---

# 12. Dataset Testing

Jangan benchmark hanya menggunakan satu file.

Gunakan kategori.

```text
datasets/
│
├── photos/
│   ├── jpg/
│   ├── png/
│   └── heic/
│
├── videos/
│   ├── mp4/
│   └── mov/
│
├── documents/
│   ├── pdf/
│   ├── docx/
│   └── txt/
│
├── android/
│   ├── apk/
│   ├── databases/
│   └── cache/
│
├── whatsapp/
│   ├── images/
│   ├── video/
│   └── documents/
│
└── synthetic/
    ├── random.bin
    ├── repetitive.bin
    └── modified.bin
```

Untuk deduplication, dataset harus memiliki versi.

```text
dataset/
│
├── version-1/
│
└── version-2/
    │
    ├── file unchanged
    ├── file modified
    ├── inserted data
    └── deleted data
```

---

# 13. Test paling penting: Boundary Shift

Misalnya file awal:

```text
AAAAAAAA
BBBBBBBB
CCCCCCCC
DDDDDDDD
```

Kemudian tambah:

```text
XYZ
```

di awal.

```text
XYZ
AAAAAAAA
BBBBBBBB
CCCCCCCC
DDDDDDDD
```

Lalu bandingkan:

```text
Fixed Size
vs
FastCDC
vs
Rabin
```

Metrik:

```text
Berapa chunk lama yang masih dapat digunakan?
```

Ini adalah salah satu benchmark paling penting untuk backup incremental.

---

# 14. CLI Design

## List algorithms

```bash
chunklab algorithms
```

Output:

```text
Available Algorithms

fixed
rabin
gear
buzhash
fastcdc
hybrid
```

---

## Test satu file

```bash
chunklab run \
    --algorithm fastcdc \
    --input video.mp4 \
    --min-size 1MiB \
    --avg-size 4MiB \
    --max-size 8MiB
```

---

## Compare

```bash
chunklab compare \
    --input datasets/video.mp4 \
    --algorithms fixed,fastcdc,rabin,gear
```

Output:

```text
Algorithm   Chunks   Avg Size   Speed       Reuse
-------------------------------------------------
Fixed       250      4 MB       900 MB/s    35%
FastCDC     242      4.1 MB     650 MB/s    92%
Rabin       248      4.0 MB     450 MB/s    94%
Gear        245      4.1 MB     720 MB/s    90%
```

Angka di atas hanya contoh; benchmark aktual harus dijalankan pada mesin dan dataset yang sama.

---

# 15. Mutation Lab

Saya sangat menyarankan modul khusus:

```text
chunking-mutation/
```

Fungsinya membuat perubahan pada file.

```text
Original
    │
    ├── Insert Bytes
    ├── Delete Bytes
    ├── Replace Bytes
    ├── Append
    └── Random Modification
```

Contoh CLI:

```bash
chunklab mutate \
    --input original.bin \
    --output modified.bin \
    --insert "HELLO" \
    --offset 100000
```

Kemudian:

```bash
chunklab compare \
    --base original.bin \
    --target modified.bin \
    --algorithms fixed,fastcdc,rabin
```

Hasil:

```text
Fixed:
Reusable Data = 42%

FastCDC:
Reusable Data = 97%

Rabin:
Reusable Data = 98%
```

---

# 16. Fase pembangunan Chunking Lab

## Phase 01 — Core

```text
Chunk
ChunkId
ChunkHash
ChunkingStrategy
ChunkConfig
```

Target:

```text
Semua algoritma dapat menggunakan interface sama.
```

---

## Phase 02 — Fixed Chunker

```text
FixedSizeChunker
```

Target:

```text
Streaming
Memory bounded
Unit tests
```

---

## Phase 03 — Hash + Dedup

```text
BLAKE3
Chunk Index
Dedup Metrics
```

Target:

```text
Mengukur reuse.
```

---

## Phase 04 — Benchmark

```text
Timer
Throughput
Chunk Statistics
Report
```

---

## Phase 05 — CDC

Implementasi:

```text
Rabin
Gear
BuzHash
```

---

## Phase 06 — FastCDC

Fokus:

```text
Performance
Boundary Normalization
Chunk Distribution
```

---

## Phase 07 — Mutation Lab

Test:

```text
Insert
Delete
Replace
Append
```

---

## Phase 08 — Hybrid

```text
File Classification
Chunking Policy
Adaptive Strategy
```

---

# Arsitektur yang paling saya rekomendasikan

```text
                   chunking-cli
                        │
                        ▼
                chunking-benchmark
                        │
                        ▼
                  chunking-core
                        │
        ┌───────────────┼────────────────┐
        │               │                │
        ▼               ▼                ▼
   Fixed Chunker      FastCDC          Rabin
        │               │                │
        └───────────────┼────────────────┘
                        │
                        ▼
                   Chunk Stream
                        │
                        ▼
                     BLAKE3
                        │
                        ▼
                   Dedup Engine
                        │
                        ▼
                  Metrics Engine
                        │
                        ▼
                     Report
```

## Target akhir

Chunking Lab akan menjawab pertanyaan dengan **data nyata**, bukan asumsi:

```text
✓ Algoritma mana paling cepat?

✓ Algoritma mana menghasilkan dedup terbaik?

✓ Chunk size berapa paling optimal?

✓ FastCDC vs Rabin seberapa besar perbedaannya?

✓ Video Android lebih cocok Fixed atau CDC?

✓ Database lebih cocok CDC atau page-aware?

✓ APK memiliki deduplication seperti apa?

✓ Berapa CPU dan memory yang dibutuhkan?

✓ Strategi terbaik untuk Backup HP?
```

Menurut saya, sebelum melanjutkan fitur **encryption, object storage, dan cloud backup**, membangun **Chunking Lab** adalah langkah yang sangat tepat. Hasil benchmark-nya nanti dapat menjadi dasar objektif untuk memilih `ChunkingStrategy` yang masuk ke Backup Engine utama.
