# Phone Backup Developer Scripts & Automation 🛠️

Centralized, modular automation tooling designed according to **DRY (Don't Repeat Yourself)** and **SOLID (Single Responsibility Principle)** architectural standards.

---

## 📂 Directory Layout

```text
scripts/
├── common/                     # Shared functions and constants
│   ├── utils.sh                # ANSI color codes, logging (log_step, log_info), assertions
│   └── config.sh               # Workspace crates topological dependency order, version manifest list
├── tools/                      # Asset generation & development utilities
│   └── generate_icons.sh       # Multi-resolution Tauri app icon generator
├── release/                    # Release lifecycle and registry publishing
│   ├── release.sh              # Version bumping, workspace tests, release build & git tagging
│   └── publish.sh              # Topological crates.io publisher with index delay & dry-run support
├── diagnostics/                # Hardware & environment health checks
│   ├── doctor.sh               # Environment check (Rust, Cargo, Node.js, Tauri, ADB, OpenSSL)
│   ├── mtp_native.sh           # Pure-Rust low-level USB MTP hardware diagnostic runner
│   └── mtp_volumes.sh          # macOS /Volumes mount point diagnostic
├── run.sh                      # Unified CLI entrypoint and interactive terminal menu
└── README.md                   # This documentation file
```

---

## 🚀 Quick Usage

Use the unified runner `./scripts/run.sh` to execute any tool:

### 1. Environment Doctor
Inspect toolchain versions, mobile development tools, and system health:
```bash
./scripts/run.sh doctor
```

### 2. Native MTP Hardware Diagnostics
Test physical Android phones connected via USB MTP:
```bash
./scripts/run.sh diag:mtp
```

### 3. Generate Desktop Icons
Generate PNG, ICNS, and ICO icons from `apps/gui/src-tauri/icons/phone-backup.png`:
```bash
./scripts/run.sh icons
```

### 4. Release a New Version
Automatically bump versions across `Cargo.toml`, `tauri.conf.json`, `package.json`, run all tests, build release binaries, and create a Git release tag:
```bash
./scripts/run.sh release 0.4.2
```

### 5. Publish to Crates.io
Publish all workspace crates in topological dependency order:
```bash
# Validate publication without uploading
./scripts/run.sh publish --dry-run

# Perform real publication
./scripts/run.sh publish
```

### 6. Interactive Terminal Menu
Run without arguments to launch the interactive selector:
```bash
./scripts/run.sh
```

---

## 🏛 Architecture Principles

1. **SRP (Single Responsibility Principle)**:
   - `common/utils.sh` only handles terminal output, formatting, and assertions.
   - `common/config.sh` holds single-source-of-truth metadata.
   - Each script in `build/`, `release/`, and `diagnostics/` focuses strictly on one task.
2. **DRY (Don't Repeat Yourself)**:
   - Standardized log levels (`log_header`, `log_step`, `log_info`, `log_success`, `log_warn`, `log_error`).
   - Centralized crate list ensures dependencies are always built and published in topological order.
3. **Fail-Fast & Safe**:
   - Every script runs with `set -euo pipefail` to catch errors immediately.
   - Automatic workspace root validation prevents accidental execution from subdirectories.
