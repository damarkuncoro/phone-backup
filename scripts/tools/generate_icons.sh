#!/usr/bin/env bash
# ==============================================================================
# Phone Backup - App Icon Generator (scripts/build/generate_icons.sh)
# Generates all multi-resolution icons for Tauri desktop app.
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common/utils.sh"
source "$SCRIPT_DIR/../common/config.sh"

ensure_workspace_root
require_command "cargo" "Install Rust from https://rustup.rs"

log_header "🎨 Tauri App Icon Generator"

log_step "1/3" "Validating source icon..."
if [ ! -f "$SOURCE_ICON_PATH" ]; then
    log_error "Source icon not found at '$SOURCE_ICON_PATH'."
    exit 1
fi
log_success "Source icon found at $SOURCE_ICON_PATH"

log_step "2/3" "Checking cargo-tauri CLI..."
if ! command_exists "cargo-tauri"; then
    log_warn "cargo-tauri not found globally, falling back to 'npx @tauri-apps/cli'..."
    if command_exists "npx"; then
        npx -y @tauri-apps/cli icon "$SOURCE_ICON_PATH" --output "$TAURI_ICONS_DIR"
    else
        log_error "Neither cargo-tauri nor npx is available. Please run: cargo install tauri-cli"
        exit 1
    fi
else
    cargo tauri icon "$SOURCE_ICON_PATH" --output "$TAURI_ICONS_DIR"
fi

log_step "3/3" "Verifying generated assets..."
if [ -d "$TAURI_ICONS_DIR" ]; then
    log_success "Icons generated successfully in $TAURI_ICONS_DIR:"
    ls -lh "$TAURI_ICONS_DIR" | tail -n +2 | while read -r line; do
        log_dim "$line"
    done
else
    log_error "Failed to locate generated icons directory."
    exit 1
fi

echo ""
log_success "Icon generation completed!"
