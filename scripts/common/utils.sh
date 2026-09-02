#!/usr/bin/env bash
# ==============================================================================
# Phone Backup - Shared Shell Utilities (scripts/common/utils.sh)
# Follows SRP (Single Responsibility Principle) for UI, logging, and validation.
# ==============================================================================

# ANSI Color Codes
export COLOR_RESET="\033[0m"
export COLOR_BOLD="\033[1m"
export COLOR_DIM="\033[2m"
export COLOR_RED="\033[1;31m"
export COLOR_GREEN="\033[1;32m"
export COLOR_YELLOW="\033[1;33m"
export COLOR_BLUE="\033[1;34m"
export COLOR_MAGENTA="\033[1;35m"
export COLOR_CYAN="\033[1;36m"
export COLOR_WHITE="\033[1;37m"

# Logging Helpers
log_header() {
    local title="$1"
    echo -e "\n${COLOR_BOLD}${COLOR_CYAN}============================================================${COLOR_RESET}"
    echo -e "${COLOR_BOLD}${COLOR_CYAN}  $title${COLOR_RESET}"
    echo -e "${COLOR_BOLD}${COLOR_CYAN}============================================================${COLOR_RESET}\n"
}

log_step() {
    local step_num="$1"
    local title="$2"
    echo -e "${COLOR_BOLD}${COLOR_BLUE}[$step_num]${COLOR_RESET} ${COLOR_WHITE}$title${COLOR_RESET}"
}

log_info() {
    echo -e "    ${COLOR_CYAN}ℹ️${COLOR_RESET}  $*"
}

log_success() {
    echo -e "    ${COLOR_GREEN}✅${COLOR_RESET} $*"
}

log_warn() {
    echo -e "    ${COLOR_YELLOW}⚠️${COLOR_RESET}  $*"
}

log_error() {
    echo -e "    ${COLOR_RED}❌${COLOR_RESET} $*" >&2
}

log_dim() {
    echo -e "    ${COLOR_DIM}$*${COLOR_RESET}"
}

# Validation and Precondition Checks
ensure_workspace_root() {
    if [ ! -f "Cargo.toml" ] || [ ! -d "libs" ] || [ ! -d "apps" ]; then
        log_error "This script must be run from the root of the phone-backup workspace."
        log_info "Current directory: $(pwd)"
        exit 1
    fi
}

command_exists() {
    command -v "$1" >/dev/null 2>&1
}

require_command() {
    local cmd="$1"
    local install_hint="$2"
    if ! command_exists "$cmd"; then
        log_error "Missing required command: '${COLOR_BOLD}$cmd${COLOR_RESET}'"
        if [ -n "$install_hint" ]; then
            log_info "To install: $install_hint"
        fi
        exit 1
    fi
}

get_os() {
    case "$OSTYPE" in
        darwin*)  echo "macos" ;; 
        linux*)   echo "linux" ;;
        msys*|cygwin*|mingw*) echo "windows" ;;
        *)        echo "unknown" ;;
    esac
}
