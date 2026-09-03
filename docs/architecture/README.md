# 🏗 Software Architecture Document (SAD)

## 1. Introduction
This document describes the high-level architecture of the **phone-backup** platform. It is designed to be a secure, storage-efficient, resilient, and multi-transport backup engine for Android devices.

## 2. Architectural Goals
- **Decoupling**: Business logic must be completely independent of external transports (ADB, Wi-Fi Agent) and storage backends (Local Disk, Cloud S3/R2).
- **Security**: Data must be encrypted locally before leaving the device's volatile memory context (*Client-Side Zero-Knowledge*).
- **Efficiency**: Sub-file chunking ensures identical data segments are stored only once (*Two-Tier FastCDC Deduplication*).
- **Reliability**: Backups must be recoverable and resumable after any hardware or network disconnection (*Fault-Tolerant Streaming Pipeline*).

---

## 3. Monorepo Structure

The project follows a standardized Monorepo pattern to separate application entry points from shared internal logic.

```text
phone-backup/
├── apps/               # Executable Entry Points
│   ├── cli/            # Professional Command Line Interface
│   └── gui/            # Desktop Dashboard (Tauri + Web Components)
├── libs/               # Internal Shared Libraries
│   ├── core/           # Domain, Application, and Ports (Business Logic)
│   │   ├── domain/     # Pure business entities & concepts
│   │   ├── application/# Use cases & service orchestration
│   │   └── ports/      # Interface definitions (Traits)
│   ├── storage/        # Specialized Data Processing
│   │   └── chunking/   # Expert Chunking Engine (FastCDC, Fixed)
│   ├── adapters/       # IO Implementations (ADB, Wi-Fi, Filesystem)
│   └── infrastructure/ # Persistence & Security (SQLite SQLCipher)
├── docs/               # Technical Documentation & Specifications
└── scripts/            # Build, Release, and Dev Automation
```

---

## 4. Technical Specifications

For detailed technical internals, please refer to the following documents:

| Document | Topic |
| :--- | :--- |
| 📘 **[V4.0 Technical Manual](V4_Technical_Manual.md)** | Logical vs Physical separation, UUIDv7 obfuscation, and Expert Chunking. |
| ⚡ **[Data Optimization Engine](data-optimization-engine.md)** | 19 Compression Pillars, Zstd Auto-Dictionary, FastCDC CAS, and Next Horizons. |
| 📒 **[Chunking Strategy Master](../references/chunk/Chunking_Strategy_Master.md)** | Deep dive into CDC algorithms and deduplication ratios. |
| 📱 **[Companion Agent Roadmap](companion-agent-roadmap.md)** | Wi-Fi local backup protocol and mDNS discovery. |

---

## 5. Security & Privacy (V4.0)

We implement a **Zero-Knowledge** capable security model:
1. **Message-Locked Encryption**: Kunci enkripsi diturunkan menggunakan HKDF sehingga mendukung deduplikasi aman.
2. **Obfuscated Storage**: Nama file di storage menggunakan **UUIDv7**, menyembunyikan identitas konten asli dari sistem file.
3. **Encrypted Database Engine**: Katalog SQLite dilindungi oleh **SQLCipher AES-256**.
4. **Asymmetric Cryptography (age X25519)**: Dukungan penguncian backup dengan kunci publik.

---

## 6. Pipeline Pemrosesan v4.0

```mermaid
graph TD
    A[Start Backup] --> B[Scanner & Classifier]
    B --> C[Streaming Reader]
    C --> D[Expert Chunker]
    D --> E[Logical Dedup (BLAKE3)]
    E -- HIT --> F[Reuse Reference]
    E -- MISS --> G[Zstd Compression]
    G --> H[Convergent Encryption]
    H --> I[Physical ID (UUIDv7)]
    I --> J[Store Encrypted Blob]
    F --> K[Commit Snapshot]
    J --> K
```

---
*phone-backup — Engineered with Rust, Clean Architecture, and Military-Grade Security.*
