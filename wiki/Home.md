# Welcome to the phone-backup Wiki 📱

**phone-backup** is a high-performance, secure, and professional Android backup platform written in Rust. It is engineered with **Clean Architecture**, **Hexagonal Architecture (Ports & Adapters)**, and **SOLID Principles** to provide enterprise-grade data protection, block-level deduplication, zero-knowledge encryption, and multi-transport connectivity (USB ADB and Local Wi-Fi).

---

## 🧭 Wiki Navigation

| Section | Description |
| :--- | :--- |
| 🚀 **[Getting Started](Getting-Started.md)** | Prerequisites, ADB installation, building from source, and initial system health check (`doctor`). |
| 💻 **[CLI Reference](CLI-Reference.md)** | Comprehensive documentation and syntax examples for all CLI commands and flags. |
| 🖥 **[Desktop GUI Guide](Desktop-GUI-Guide.md)** | Operating the Tauri Dashboard, Live Device File Manager, Visual Diffing Matrix, and APK Exporter. |
| 🏗 **[Architecture & Design](Architecture-and-Design.md)** | In-depth breakdown of Hexagonal Architecture, Ports & Adapters, and core data pipelines. |
| 🔒 **[Security & Encryption](Security-and-Encryption.md)** | Details on SQLCipher AES-256, Argon2id KDF, age (X25519) asymmetric crypto, and Tauri ACL security. |
| 💾 **[Storage & Deduplication](Storage-and-Deduplication.md)** | Content-Addressed Storage (CAS), FastCDC variable chunking, and OpenDAL Cloud S3/R2 storage. |
| 📱 **[Wireless Companion Agent](Wireless-Companion-Agent.md)** | Wi-Fi local backup protocol, Android Companion APK architecture, and zero-USB operation. |
| 👥 **[Contacts & Data Management](Contacts-and-Data-Management.md)** | Relational contact extraction, FTS5 global search, and universal vCard (RFC 6350) export. |
| ❓ **[Troubleshooting & FAQ](Troubleshooting-and-FAQ.md)** | Resolving common issues (ADB port 5037 conflicts, Xiaomi MIUI security permissions, connection drops). |
| 🧪 **[Developer Guide & Testing](Developer-Guide-and-Testing.md)** | Workspace structure, test isolation policies (`src/` vs `tests/`), and contribution guidelines. |

---

## 🌟 Key Highlights

- **Multi-Transport Support**: Connect via physical USB cable (**ADB**) or high-speed local Wi-Fi (**Companion Agent APK**).
- **FastCDC Sub-file Deduplication**: Chunks large files dynamically to save up to 80% disk space across snapshots.
- **Zero-Knowledge Privacy**: Data is encrypted locally before touching any storage media using **AES-256-GCM** or **age X25519**.
- **Encrypted Relational Metadata**: The entire index database (`backup.db`) is encrypted via **SQLCipher** and secured with **Argon2id**.
- **High-Performance Native GUI**: Desktop dashboard built with Tauri, Tailwind CSS, Chart.js, and Native Web Components.

---
*For quick source code navigation and repository overview, visit the [Main Repository README](../README.md).*
