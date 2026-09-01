# Analisis Kompetitif: Phone Backup Engine vs. Restic & Borg Backup

Dokumen ini membandingkan arsitektur **Phone Backup Engine (v4.0)** dengan dua standar industri backup open-source yang paling dihormati: **Restic** dan **Borg Backup**. Analisis ini bertujuan untuk memvalidasi apakah spesifikasi v4.0 sudah setara dengan tool kelas profesional.

---

## 1. Tabel Perbandingan Fitur

| Fitur | Restic | Borg Backup | Current Engine | **Engine v4.0 (Target)** |
| :--- | :--- | :--- | :--- | :--- |
| **Bahasa Pemrograman** | Go | Python / C | Rust / TypeScript | **Rust** |
| **Unit Deduplikasi** | CDC (BuzHash) | CDC (BuzHash/Rabin) | File-level | **CDC (FastCDC)** |
| **Model Storage** | CAS (Content Addressable) | Repository + Archive | File-based | **CAS (UUIDv7 Obfuscated)** |
| **Kompresi** | Zstd (Baru) | lz4, zstd, zlib, lzma | None | **Zstd** |
| **Enkripsi** | AES-256-CTR + Poly1305 | AES-256-GCM | Basic Deterministic | **Convergent (XChaCha20)** |
| **Integritas Data** | Hashing (SHA-256) | Hashing + HMAC | SHA-256 | **BLAKE3 (Multi-level)** |
| **Resume Backup** | Sangat Baik (Partial) | Sangat Baik | File-level | **Chunk-level (Atomic)** |
| **Cloud Support** | Native (S3, B2, etc) | Via Rclone/SSH | Local/Custom | **Plugable Adapters** |

---

## 2. Analisis Arsitektur: Menuju Standar Profesional

### A. Deduplikasi: Menghilangkan Redundansi
*   **Restic & Borg:** Keduanya menggunakan **Content Defined Chunking (CDC)**. Ini adalah "rahasia" mengapa mereka bisa menyimpan ratusan snapshot dengan penggunaan disk yang minimal. Mereka melihat data sebagai aliran byte, bukan sebagai file.
*   **Engine v4.0:** Memilih **FastCDC**. Ini memberikan efisiensi deduplikasi yang setara dengan Borg/Restic tetapi dengan performa yang lebih cepat (dioptimalkan untuk CPU modern dan Rust), yang sangat penting untuk aplikasi mobile/desktop GUI.

### B. Keamanan: Zero-Knowledge
*   **Restic:** Sangat kuat di sisi keamanan. Menggunakan model repositori terenkripsi penuh. Tanpa kunci, penyerang tidak tahu nama file, ukuran, atau struktur direktori.
*   **Engine v4.0:** Mengadopsi prinsip yang sama melalui **Obfuscated Storage (UUIDv7)** dan **Convergent Encryption**. Targetnya adalah *Zero-Knowledge Metadata*, di mana penyedia storage (misal: Google Drive atau S3) tidak bisa mengintip isi atau struktur backup Anda.

### C. Kecepatan & Resource: Keunggulan Rust
*   **Borg:** Karena sebagian besar Python, Borg kadang mengalami hambatan performa pada dataset jutaan file kecil, meskipun inti C-nya sangat cepat.
*   **Restic:** Sangat efisien dalam memori, namun penggunaan Go terkadang menyebabkan *Garbage Collection pause* pada dataset masif.
*   **Engine v4.0 (Rust):** Dengan menggunakan Rust, engine Anda memiliki potensi untuk **mengalahkan Borg dan Restic** dalam hal manajemen memori (zero-cost abstractions) dan kecepatan I/O mentah, terutama dengan integrasi `io_uring` atau `tokio` di masa depan.

---

## 3. Mengapa Spesifikasi v4.0 Adalah "Must-Have"?

Tanpa migrasi ke v4.0, Phone Backup Engine akan tetap menjadi "tool copy-paste yang canggih". Dengan v4.0, Engine Anda masuk ke liga yang sama dengan Restic dan Borg karena:

1.  **Deduplikasi Sub-file**: Esensial untuk menangani database WhatsApp, log sistem Android, dan database media yang terus berubah namun hanya sedikit.
2.  **Integritas BLAKE3**: Menggunakan hashing tercepat di dunia saat ini, jauh melampaui SHA-256 yang digunakan Restic/Borg dalam hal throughput mentah.
3.  **Atomic Snapshot**: Memungkinkan backup HP yang stabil meskipun koneksi USB goyang atau baterai habis, mirip dengan ketangguhan Borg.

---

## 4. Kesimpulan

Spesifikasi **v4.0** adalah jembatan yang mengubah Phone Backup Engine dari sekadar "aplikasi utilitas" menjadi **Engine Backup Kelas Dunia**. Desain v4.0 tidak hanya meniru fitur terbaik dari Restic dan Borg, tetapi juga mengoptimalkannya menggunakan ekosistem Rust modern untuk kasus penggunaan spesifik perangkat mobile (high latency, unstable connection, flash storage).
