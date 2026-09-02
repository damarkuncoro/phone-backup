#!/usr/bin/env bash
# ==============================================================================
# Phone Backup - Unified Scripts Runner (scripts/run.sh)
# Master dispatcher for all build, release, and diagnostic workflows.
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common/utils.sh"
source "$SCRIPT_DIR/common/config.sh"

ensure_workspace_root

show_help() {
    echo -e "${COLOR_BOLD}${COLOR_CYAN}Phone Backup - Automation & Tooling CLI${COLOR_RESET}"
    echo -e "${COLOR_DIM}Usage: ./scripts/run.sh <command> [args...]${COLOR_RESET}\n"
    echo -e "${COLOR_BOLD}Available Commands:${COLOR_RESET}"
    echo -e "  ${COLOR_GREEN}doctor${COLOR_RESET}           Run environment health checks (Rust, Cargo, Node, ADB)"
    echo -e "  ${COLOR_GREEN}icons${COLOR_RESET}            Generate multi-resolution icons for Tauri app"
    echo -e "  ${COLOR_GREEN}diag:mtp${COLOR_RESET}         Run native pure-Rust USB MTP hardware diagnostic"
    echo -e "  ${COLOR_GREEN}diag:volumes${COLOR_RESET}     Scan macOS /Volumes filesystem mounts for MTP devices"
    echo -e "  ${COLOR_GREEN}release <ver>${COLOR_RESET}    Bump version manifests, run tests, build and git tag"
    echo -e "  ${COLOR_GREEN}publish${COLOR_RESET}          Publish workspace crates to crates.io in topological order"
    echo -e "  ${COLOR_GREEN}publish --dry-run${COLOR_RESET}Validate crate publishing without uploading"
    echo -e "  ${COLOR_GREEN}help${COLOR_RESET}             Show this help message"
    echo ""
}

show_interactive_menu() {
    log_header "⚡ Phone Backup - Interactive Developer Menu"
    echo -e "Please select an action:"
    echo -e "  ${COLOR_BOLD}1)${COLOR_RESET} Run Environment Doctor (${COLOR_GREEN}doctor${COLOR_RESET})"
    echo -e "  ${COLOR_BOLD}2)${COLOR_RESET} Run Native MTP USB Diagnostic (${COLOR_GREEN}diag:mtp${COLOR_RESET})"
    echo -e "  ${COLOR_BOLD}3)${COLOR_RESET} Run /Volumes MTP Mounts Diagnostic (${COLOR_GREEN}diag:volumes${COLOR_RESET})"
    echo -e "  ${COLOR_BOLD}4)${COLOR_RESET} Generate Tauri App Icons (${COLOR_GREEN}icons${COLOR_RESET})"
    echo -e "  ${COLOR_BOLD}5)${COLOR_RESET} Prepare a New Release (${COLOR_GREEN}release${COLOR_RESET})"
    echo -e "  ${COLOR_BOLD}6)${COLOR_RESET} Dry-Run Crates.io Publishing (${COLOR_GREEN}publish --dry-run${COLOR_RESET})"
    echo -e "  ${COLOR_BOLD}q)${COLOR_RESET} Quit"
    echo ""
    read -r -p "Enter choice [1-6, q]: " choice

    case "$choice" in
        1) "$SCRIPT_DIR/diagnostics/doctor.sh" ;;
        2) "$SCRIPT_DIR/diagnostics/mtp_native.sh" ;;
        3) "$SCRIPT_DIR/diagnostics/mtp_volumes.sh" ;;
        4) "$SCRIPT_DIR/tools/generate_icons.sh" ;;
        5)
            read -r -p "Enter new version (e.g. 0.4.2): " ver
            "$SCRIPT_DIR/release/release.sh" "$ver"
            ;;
        6) "$SCRIPT_DIR/release/publish.sh" --dry-run ;;
        q|Q) echo "Bye!"; exit 0 ;;
        *) log_error "Invalid selection."; exit 1 ;;
    esac
}

COMMAND="${1:-}"
shift || true

case "$COMMAND" in
    doctor)
        "$SCRIPT_DIR/diagnostics/doctor.sh" "$@"
        ;;
    icons|generate_icons)
        "$SCRIPT_DIR/tools/generate_icons.sh" "$@"
        ;;
    diag:mtp|mtp)
        "$SCRIPT_DIR/diagnostics/mtp_native.sh" "$@"
        ;;
    diag:volumes|volumes)
        "$SCRIPT_DIR/diagnostics/mtp_volumes.sh" "$@"
        ;;
    release)
        "$SCRIPT_DIR/release/release.sh" "$@"
        ;;
    publish)
        "$SCRIPT_DIR/release/publish.sh" "$@"
        ;;
    help|--help|-h)
        show_help
        ;;
    "")
        show_interactive_menu
        ;;
    *)
        log_error "Unknown command: '$COMMAND'"
        echo ""
        show_help
        exit 1
        ;;
esac
