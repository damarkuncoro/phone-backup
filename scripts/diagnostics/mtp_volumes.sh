#!/usr/bin/env bash
# ==============================================================================
# Phone Backup - MTP Filesystem Volumes Diagnostic (scripts/diagnostics/mtp_volumes.sh)
# Inspects macOS /Volumes filesystem mounts for Android storage structures.
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../common/utils.sh"

log_header "🔍 MTP Volumes Mount Diagnostic"

if [ "$(get_os)" != "macos" ]; then
    log_info "Note: This diagnostic tool is designed specifically for macOS /Volumes directory."
fi

log_step "1/3" "Scanning /Volumes mount points..."
if [ -d "/Volumes" ]; then
    ls -F /Volumes | while read -r vol; do
        log_dim "/Volumes/$vol"
    done
else
    log_warn "/Volumes directory not found on this system."
fi

echo ""
log_step "2/3" "Evaluating Android directory structures..."
FOUND_ANDROID_VOLUME=false

for volume in /Volumes/*; do
    if [ -d "$volume" ]; then
        vol_name=$(basename "$volume")
        echo -e "  📂 ${COLOR_BOLD}$vol_name${COLOR_RESET} ($volume)"

        has_dcim=$([ -d "$volume/DCIM" ] && echo "FOUND ✅" || echo "no")
        has_internal=$([ -d "$volume/Internal storage" ] || [ -d "$volume/Internal Storage" ] && echo "FOUND ✅" || echo "no")
        has_sdcard=$([ -d "$volume/sdcard" ] && echo "FOUND ✅" || echo "no")

        echo "     - DCIM:             $has_dcim"
        echo "     - Internal storage: $has_internal"
        echo "     - sdcard:           $has_sdcard"

        if [ "$has_dcim" == "FOUND ✅" ] || [ "$has_internal" == "FOUND ✅" ] || [ "$has_sdcard" == "FOUND ✅" ]; then
            FOUND_ANDROID_VOLUME=true
        fi
    fi
done

echo ""
log_step "3/3" "Diagnostic Result..."
if [ "$FOUND_ANDROID_VOLUME" = true ]; then
    log_success "At least one mounted volume matches Android filesystem heuristics."
else
    log_info "No Android storage volume detected in /Volumes."
    log_info "Tip: If your phone is connected via USB, use native MTP mode:"
    log_info "     ${COLOR_CYAN}./scripts/run.sh diag:mtp${COLOR_RESET}"
fi
echo ""
