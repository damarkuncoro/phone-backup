#!/usr/bin/env bash
# ==============================================================================
# Phone Backup - Release Automation (scripts/release/release.sh)
# Bumps version, validates workspace, runs tests, and creates git release tag.
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common/utils.sh"
source "$SCRIPT_DIR/../common/config.sh"

ensure_workspace_root
require_command "cargo" "Install Rust from https://rustup.rs"
require_command "git" "Install git"

NEW_VERSION="${1:-}"

if [ -z "$NEW_VERSION" ]; then
    echo -e "${COLOR_RED}Error: Version number required.${COLOR_RESET}"
    echo -e "Usage:   $0 <version>"
    echo -e "Example: $0 0.4.2"
    exit 1
fi

# Clean version format (strip leading 'v' if provided)
NEW_VERSION="${NEW_VERSION#v}"

log_header "🚀 Release Preparation for v$NEW_VERSION"

# Extract current version from root Cargo.toml
CURRENT_VERSION=$(grep -m 1 '^version = ' Cargo.toml | tr -d ' ' | cut -d '"' -f 2 || echo "")
log_info "Current Workspace Version: ${COLOR_BOLD}${CURRENT_VERSION:-unknown}${COLOR_RESET}"
log_info "Target Release Version:    ${COLOR_BOLD}${NEW_VERSION}${COLOR_RESET}"

log_step "1/5" "Updating version manifests..."

# 1. Update Cargo.toml (workspace.package and internal dependency versions)
if [ -f "Cargo.toml" ]; then
    if [ "$(get_os)" == "macos" ]; then
        sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
        if [ -n "$CURRENT_VERSION" ]; then
            sed -i '' "s/version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/g" Cargo.toml
        fi
    else
        sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
        if [ -n "$CURRENT_VERSION" ]; then
            sed -i "s/version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/g" Cargo.toml
        fi
    fi
    log_success "Updated Cargo.toml"
fi

# 2. Update Tauri manifest
if [ -f "apps/gui/src-tauri/tauri.conf.json" ]; then
    if [ "$(get_os)" == "macos" ]; then
        sed -i '' "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" apps/gui/src-tauri/tauri.conf.json
    else
        sed -i "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" apps/gui/src-tauri/tauri.conf.json
    fi
    log_success "Updated apps/gui/src-tauri/tauri.conf.json"
fi

# 3. Update Frontend package.json
if [ -f "apps/gui/ui/package.json" ]; then
    if [ "$(get_os)" == "macos" ]; then
        sed -i '' "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" apps/gui/ui/package.json
    else
        sed -i "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" apps/gui/ui/package.json
    fi
    log_success "Updated apps/gui/ui/package.json"
fi

log_step "2/5" "Running workspace test suite..."
cargo test --workspace

log_step "3/5" "Building release binaries..."
cargo build --release --workspace

log_step "4/5" "Creating Git Commit and Release Tag..."
git add Cargo.toml apps/gui/src-tauri/tauri.conf.json apps/gui/ui/package.json
git commit -m "chore: release v$NEW_VERSION" || log_warn "No file changes to commit."
git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"
log_success "Created git tag v$NEW_VERSION"

log_step "5/5" "Release ready!"
echo ""
echo -e "${COLOR_BOLD}${COLOR_GREEN}✨ Release v$NEW_VERSION prepared successfully!${COLOR_RESET}"
echo -e "Next steps to publish:"
echo -e "  1. ${COLOR_CYAN}git push origin main --tags${COLOR_RESET}"
echo -e "  2. ${COLOR_CYAN}./scripts/release/publish.sh${COLOR_RESET}"
