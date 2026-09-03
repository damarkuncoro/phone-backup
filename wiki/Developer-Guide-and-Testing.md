# 🧪 Developer Guide & Testing

Panduan ini ditujukan bagi pengembang yang ingin berkontribusi, menjalankan pengujian, atau memperluas adapter di platform **phone-backup**.

---

## 1. Kebijakan Isolasi Kode & Pengujian (Pure `src/` vs `tests/`)

Workspace **phone-backup** memberlakukan standar rekayasa perangkat lunak ketat:
- **100% Pure Production Code**: Seluruh berkas di dalam folder `src/` seluruh crate workspace tidak boleh memuat modul inline `#[cfg(test)]`.
- **Isolated Integration Test Suites**: Seluruh berkas pengujian (unit, integrasi, simulasi mock, dan verifikasi enkripsi) wajib ditempatkan di direktori `tests/` masing-masing crate.

---

## 2. Menjalankan Seluruh Test Suite

Untuk menjalankan seluruh rangkaian pengujian di seluruh workspace Rust:
```bash
cargo test --workspace
```

### Menjalankan Test untuk Crate Tertentu:
```bash
# Domain tests
cargo test -p phone-backup-domain

# Encrypted Repository tests
cargo test -p phone-backup-adapter-database-sqlite

# Wireless Agent tests
cargo test -p phone-backup-adapter-agent

# ADB Adapter tests
cargo test -p phone-backup-adapter-adb
```

---

## 3. Menambahkan Adapter Baru

Untuk menambahkan adapter perangkat atau backend storage baru:
1. Buat folder baru di bawah `adapters/<nama_adapter>/`.
2. Daftarkan crate baru di root `Cargo.toml`.
3. Implementasikan trait port yang relevan dari `core/ports` (misal: `ports::DevicePort` atau `ports::StoragePort`).
4. Pasang adapter pada Composition Root di `apps/cli/src/main.rs` dan factory terkait.
5. Buat suite pengujian terisolasi di bawah `adapters/<nama_adapter>/tests/`.

---

## 4. Linting & Formatting

Sebelum membuat Pull Request atau commit baru, pastikan format kode sesuai standar:
```bash
# Format kode Rust
cargo fmt --all

# Periksa linter Clippy
cargo clippy --workspace --all-targets -- -D warnings
```

---

## 5. Matriks Pengujian & Analisis Gap

Laporan lengkap hasil pengujian unit, integrasi, dan analisis skenario batas (*edge cases*) yang belum teruji dapat dilihat di:
- 📊 **[Laporan Hasil Uji Coba & Test Matrix](../docs/reports/test-cases-report.md)**

---
*Kembali ke: [Home](Home.md).*
