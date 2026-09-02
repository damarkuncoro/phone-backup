#!/usr/bin/env bash
# ==============================================================================
# Phone Backup - Topological Crates Publisher (scripts/release/publish.sh)
# Publishes workspace crates to crates.io in strict dependency order.
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common/utils.sh"
source "$SCRIPT_DIR/../common/config.sh"

ensure_workspace_root
require_command "cargo" "Install Rust from https://rustup.rs"

DRY_RUN=false
if [[ "${1:-}" == "--dry-run" ]]; then
    DRY_RUN=true
    log_warn "Running in DRY-RUN mode. No packages will be published."
fi

log_header "📦 Crates.io Topological Publisher"

TOTAL_CRATES=${#WORKSPACE_CRATES_TOPOLOGICAL[@]}
INDEX=0

for crate_path in "${WORKSPACE_CRATES_TOPOLOGICAL[@]}"; do
    INDEX=$((INDEX + 1))
    log_step "$INDEX/$TOTAL_CRATES" "Processing crate: ${COLOR_BOLD}$crate_path${COLOR_RESET}..."

    if [ ! -f "$crate_path/Cargo.toml" ]; then
        log_error "Manifest not found: $crate_path/Cargo.toml"
        exit 1
    fi

    if [ "$DRY_RUN" = true ]; then
        log_info "Dry-run publishing $crate_path..."
        cargo publish --manifest-path "$crate_path/Cargo.toml" --dry-run
        log_success "Verified $crate_path (Dry run)"
    else
        log_info "Publishing $crate_path to crates.io..."
        cargo publish --manifest-path "$crate_path/Cargo.toml" --no-verify
        log_success "Published $crate_path"
        
        # Give crates.io time to register & index the package before dependent crates publish
        if [ "$INDEX" -lt "$TOTAL_CRATES" ]; then
            log_dim "Waiting 8 seconds for crates.io index sync..."
            sleep 8
        fi
    fi
done

echo ""
log_success "🎉 All $TOTAL_CRATES crates processed successfully!"
