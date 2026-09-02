#!/usr/bin/env bash
# ==============================================================================
# Phone Backup - Central Configuration & Metadata (scripts/common/config.sh)
# Single Source of Truth for crate dependencies, topological order, and version paths.
# ==============================================================================

# Ensure standard tool paths (Node, Homebrew, Android SDK) are in PATH
export PATH="$PATH:/usr/local/bin:/opt/homebrew/bin:$HOME/Library/Android/sdk/platform-tools"

# Topological order of crates for builds, publishing, and dependency checks.
# Dependencies must be built/published before dependees.
export WORKSPACE_CRATES_TOPOLOGICAL=(
    "libs/core/domain"
    "libs/core/ports"
    "libs/core/application"
    "libs/storage/chunking"
    "libs/storage/compression"
    "libs/storage/security"
    "libs/adapters/filesystem"
    "libs/adapters/mock"
    "libs/adapters/adb"
    "libs/adapters/mtp"
    "libs/adapters/agent"
    "libs/adapters/opendal"
    "libs/infrastructure/database-sqlite"
    "apps/cli"
)

# Version files that must be updated synchronously on release
export VERSION_MANIFESTS=(
    "Cargo.toml"
    "apps/gui/src-tauri/tauri.conf.json"
    "apps/gui/ui/package.json"
)

# Key asset locations
export SOURCE_ICON_PATH="apps/gui/src-tauri/icons/phone-backup.png"
export TAURI_ICONS_DIR="apps/gui/src-tauri/icons"
