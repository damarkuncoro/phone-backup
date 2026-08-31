# Phone Backup Documentation 📚

Selamat datang di pusat dokumentasi resmi **phone-backup**. Seluruh dokumentasi telah diorganisasikan ke dalam kategori terstruktur untuk memudahkan navigasi bagi pengguna, arsitek perangkat lunak, dan pengembang.

---

## 🗂 Struktur & Kategori Dokumentasi

```text
docs/
├── 📖 README.md                            # Master Index Dokumentasi
├── 🏗 architecture/                         # Arsitektur Sistem & Spesifikasi Teknis
│   ├── README.md                           # Arsitektur Heksagonal, Ports & Adapters, CAS & FastCDC
│   └── companion-agent-roadmap.md          # Spesifikasi & Protokol Agen Android Nirkabel (Wi-Fi)
├── 🛠 guides/                              # Panduan Penggunaan & Batasan Platform
│   ├── README.md                           # Panduan Operasional Lengkap (CLI, GUI, S3/R2, FAQ)
│   └── limitations.md                      # Batasan Teknis & Sandbox Keamanan Android OS
├── 🧪 reports/                             # Laporan Pengujian & Audit Hardware Fisik
│   ├── test-cases-report.md                # Laporan Verifikasi & Pengujian Multi-Skenario
│   └── hardware-review.md                  # Review Teknis & Pengujian HP Android Fisik (Xiaomi/Redmi)
├── 🚀 roadmap/                             # Perjalanan Pengembangan & Pelacak Fitur
│   ├── phases.md                           # Kronologi Implementasi Phase 01 hingga Phase 43+
│   └── feature-requests.md                 # Pelacak Fitur Selesai & Inisiatif Masa Depan
└── 📑 references/                          # Spesifikasi Desain & Dokumen RFC
    ├── backup-specification.md             # Spesifikasi Teknis Pipeline Pencadangan
    ├── contact-schema-design.md            # Desain Skema Relasional Buku Telepon (Kontak)
    ├── contact-update-rfc.md               # RFC Pembaruan Ekstraksi Kontak Multi-Kolom
    ├── database-schema-rfc.md              # RFC Migrasi Database SQLite & FTS5 Indexing
    ├── scanner-specification.md            # Spesifikasi Engine Pemindai Berkas & MediaStore
    └── ui-design-system.md                 # Desain Antarmuka & Wireframe Desktop GUI
```

---

## 🧭 Panduan Cepat Navigasi

| Kategori | Dokumen Utama | Deskripsi |
| :--- | :--- | :--- |
| **🌐 Wiki Penuh** | [📖 Project Wiki](../wiki/Home.md) | Basis pengetahuan lengkap terintegrasi (Getting Started, CLI, GUI, Security, Storage, Testing). |
| **🛠 Panduan Operasional** | [Panduan Lengkap (How-To)](guides/README.md) | Panduan langkah-demi-langkah CLI, GUI Tauri, Cloud S3/R2 (OpenDAL), Wireless Companion Agent, dan FAQ. |
| **🏗 Arsitektur Sistem** | [System Architecture (SAD)](architecture/README.md) | Desain Heksagonal, Ports & Adapters, pipeline CAS deduplikasi, dan enkripsi SQLCipher + Argon2id. |
| **📱 Agen Nirkabel** | [Companion Agent Roadmap](architecture/companion-agent-roadmap.md) | Blueprint agen Android APK nirkabel, mDNS discovery, dan streaming transfer data via Wi-Fi. |
| **🧪 Laporan Pengujian** | [Laporan Uji Coba & Benchmarks](reports/test-cases-report.md) | Hasil pengujian multi-skenario, integritas data, dan benchmarking kecepatan enkripsi. |
| **📝 Audit Perangkat Keras** | [Review Hardware Nyata](reports/hardware-review.md) | Evaluasi pengujian langsung pada smartphone Xiaomi/Redmi dan solusi hambatan sistem. |
| **⚠️ Batasan Sistem** | [Known Limitations](guides/limitations.md) | Batasan ADB vs MTP vs Companion Agent terkait izin sandbox Android OS. |
| **🚀 Roadmap & Status** | [Project Phases (v0.3.5)](roadmap/phases.md) | Kronologi implementasi fase 01 sampai fase 43+ beserta milestone v1.0.0. |

---

## 🏛 Prinsip Rekayasa Perangkat Lunak

1. **Hexagonal Architecture (Ports & Adapters)**: Seluruh ketergantungan eksternal (ADB, SQLite SQLCipher, OpenDAL S3, Android Wireless Agent) diabstraksikan di balik port (`core/ports`). Inti domain bisnis (`core/domain`, `core/application`) bersifat 100% independen.
2. **Content-Addressed Storage (CAS) & FastCDC**: Setiap objek dan chunk file diidentifikasi berdasarkan hash SHA-256 kriptografis, memungkinkan deduplikasi global tingkat blok secara otomatis.
3. **Zero-Knowledge Security**: Data dienkripsi secara lokal di memori sebelum ditulis ke disk atau dikirim ke cloud storage menggunakan **AES-256-GCM**, **age (X25519)**, dan **SQLCipher Argon2id KDF**.
4. **Pure Production Code & Test Isolation**: Pemisahan tegas 100% antara kode produksi `src/` dan unit/integration test suites di direktori `tests/`.

---
*Untuk panduan instalasi dan penggunaan cepat, silakan merujuk ke [Root README](../README.md).*
