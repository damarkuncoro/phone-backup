# Proposal Reorganisasi Struktur Proyek

Saat ini, struktur root project memiliki terlalu banyak direktori di level yang sama (`apps`, `core`, `adapters`, `infrastructure`, `docs`), yang membuat navigasi menjadi sulit dan mencampuradukkan antara aplikasi executable dengan library internal.

Berikut adalah saran reorganisasi untuk membuat struktur yang lebih bersih dan standar (Monorepo Pattern).

---

## 1. Struktur Baru yang Disarankan

```text
phone-backup/
├── apps/                # Executable Applications (User-facing)
│   ├── cli/             # Command Line Interface
│   └── gui/             # Desktop GUI (Tauri)
│
├── libs/                # Internal Shared Libraries (Internal logic)
│   ├── core/            # Domain, Application, & Ports (Business Logic)
│   │   ├── domain/
│   │   ├── application/
│   │   └── ports/
│   ├── adapters/        # Implementation of Ports (IO, DB, ADB, etc.)
│   │   ├── adb/
│   │   ├── filesystem/
│   │   ├── opendal/
│   │   └── ...
│   └── infrastructure/  # Cross-cutting concerns (DB actual, Security etc.)
│       └── database-sqlite/
│
├── docs/                # Project Documentation & References
│   └── references/
│
├── scripts/             # Development & Build Scripts
├── README.md
├── Cargo.toml           # Workspace Config
└── ...
```

---

## 2. Manfaat Reorganisasi

1.  **Pemisahan Concern**: Jelas mana yang merupakan **Entry Point** (`apps`) dan mana yang merupakan **Internal Logic** (`libs`).
2.  **Kerapihan Root**: Root hanya akan memiliki ~3 direktori utama (`apps`, `libs`, `docs`) alih-alih 6-7 direktori yang bercampur.
3.  **Skalabilitas**: Jika di masa depan Anda menambah platform baru (misal `apps/mobile`), tempatnya sudah jelas. Jika menambah adapter baru (misal `libs/adapters/s3`), tempatnya juga jelas.
4.  **Standar Monorepo**: Mengikuti pola umum yang digunakan di proyek Rust besar (seperti Polars atau Meilisearch).

---

## 3. Langkah Migrasi (Cargo Workspace)

Jika Anda setuju, kita perlu melakukan hal berikut:

1.  **Pindahkan Direktori**:
    *   `mv core libs/core`
    *   `mv adapters libs/adapters`
    *   `mv infrastructure libs/infrastructure`
2.  **Update `Cargo.toml` Root**:
    Ubah bagian `members` dan `[workspace.dependencies]` untuk mencerminkan path baru (misal: `path = "libs/core/domain"`).
3.  **Update Imports**:
    Update path relatif di dalam `Cargo.toml` masing-masing package jika ada referensi path relatif (namun biasanya workspace menggunakan dependency name, jadi minimal perubahannya).

---

**Saran saya**: Mulailah dengan membuat direktori `libs/` dan memindahkan modul internal ke sana. Ini akan secara instan membuat struktur proyek terasa lebih profesional dan mudah dikelola.
