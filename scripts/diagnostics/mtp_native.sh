#!/usr/bin/env bash
# ==============================================================================
# Phone Backup - Native MTP Device Diagnostic Runner (scripts/diagnostics/mtp_native.sh)
# Executes direct low-level USB MTP protocol diagnostics and tests.
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common/utils.sh"

ensure_workspace_root
require_command "cargo" "Install Rust from https://rustup.rs"

log_header "🔌 Native USB MTP Diagnostic Runner"

log_info "Running real device MTP diagnostic suite..."
cargo run --example real_device_test --package phone-backup-adapter-mtp
