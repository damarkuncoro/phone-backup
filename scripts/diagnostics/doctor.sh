#!/usr/bin/env bash
# ==============================================================================
# Phone Backup - Development Environment Doctor (scripts/diagnostics/doctor.sh)
# Inspects toolchain, system dependencies, USB tools, and runtime environment.
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common/utils.sh"
source "$SCRIPT_DIR/../common/config.sh"

ensure_workspace_root

log_header "🩺 Phone Backup - Environment Doctor"

CHECK_FAILED=0

check_tool() {
    local name="$1"
    local cmd="$2"
    local version_arg="${3:---version}"
    local is_required="${4:-true}"

    if command_exists "$cmd"; then
        local ver
        ver=$($cmd $version_arg 2>&1 | head -n 1)
        echo -e "  ${COLOR_GREEN}✔${COLOR_RESET} ${COLOR_BOLD}$name${COLOR_RESET}: $ver"
    else
        if [ "$is_required" = "true" ]; then
            echo -e "  ${COLOR_RED}✖${COLOR_RESET} ${COLOR_BOLD}$name${COLOR_RESET}: ${COLOR_RED}NOT FOUND (Required)${COLOR_RESET}"
            CHECK_FAILED=1
        else
            echo -e "  ${COLOR_YELLOW}○${COLOR_RESET} ${COLOR_BOLD}$name${COLOR_RESET}: ${COLOR_YELLOW}Not found (Optional)${COLOR_RESET}"
        fi
    fi
}

log_step "1/4" "Checking Core Toolchain & Compilers..."
check_tool "Rust Compiler (rustc)" "rustc" "--version" "true"
check_tool "Cargo Package Manager" "cargo" "--version" "true"
check_tool "Node.js Runtime" "node" "--version" "true"
check_tool "pnpm / npm" "pnpm" "--version" "false"
check_tool "Git Version Control" "git" "--version" "true"

echo ""
log_step "2/4" "Checking Desktop & Mobile Development Tools..."
check_tool "Tauri CLI (cargo-tauri)" "cargo-tauri" "--version" "false"
check_tool "Android Debug Bridge (adb)" "adb" "version" "false"
check_tool "OpenSSL CLI" "openssl" "version" "true"

echo ""
log_step "3/4" "Checking Host System Resources..."
OS_NAME=$(get_os)
log_info "Operating System: ${COLOR_BOLD}$OS_NAME ($(uname -s) $(uname -m))${COLOR_RESET}"

if command_exists "df"; then
    DISK_AVAIL=$(df -h . | awk 'NR==2 {print $4}')
    log_info "Available Workspace Disk Space: ${COLOR_BOLD}$DISK_AVAIL${COLOR_RESET}"
fi

echo ""
log_step "4/4" "Checking Workspace Crate Structure..."
MISSING_CRATES=0
for crate in "${WORKSPACE_CRATES_TOPOLOGICAL[@]}"; do
    if [ ! -f "$crate/Cargo.toml" ]; then
        log_error "Missing workspace crate manifest: $crate/Cargo.toml"
        MISSING_CRATES=$((MISSING_CRATES + 1))
    fi
done

if [ "$MISSING_CRATES" -eq 0 ]; then
    log_success "All ${#WORKSPACE_CRATES_TOPOLOGICAL[@]} workspace crate manifests are present."
else
    CHECK_FAILED=1
fi

echo ""
if [ "$CHECK_FAILED" -eq 0 ]; then
    echo -e "${COLOR_BOLD}${COLOR_GREEN}✅ All critical development tools and dependencies are healthy!${COLOR_RESET}\n"
else
    echo -e "${COLOR_BOLD}${COLOR_YELLOW}⚠️ Some required tools or files are missing. Check logs above.${COLOR_RESET}\n"
fi
