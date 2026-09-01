#!/bin/bash

echo "🔍 MTP Discovery Diagnostic Tool"
echo "--------------------------------"

echo "1. Checking /Volumes directory content:"
ls -F /Volumes

echo ""
echo "2. Checking for Android-like structures in each volume:"
for volume in /Volumes/*; do
    if [ -d "$volume" ]; then
        echo "Checking volume: $volume"

        # Check for common Android folders
        echo "  - Looking for DCIM: $([ -d "$volume/DCIM" ] && echo "FOUND ✅" || echo "not found")"
        echo "  - Looking for Internal storage: $([ -d "$volume/Internal storage" ] && echo "FOUND ✅" || echo "not found")"
        echo "  - Looking for Internal Storage: $([ -d "$volume/Internal Storage" ] && echo "FOUND ✅" || echo "not found")"
        echo "  - Looking for sdcard: $([ -d "$volume/sdcard" ] && echo "FOUND ✅" || echo "not found")"

        # Look one level deeper
        echo "  - Searching one level deeper for 'storage' or 'internal' (case insensitive):"
        find "$volume" -maxdepth 2 -iname "*internal*" -o -iname "*storage*" -o -iname "*sdcard*" 2>/dev/null | head -n 5 | sed 's/^/    - /'
    fi
done

echo ""
echo "3. Rust Logic Emulation (Heuristics):"
for volume in /Volumes/*; do
    if [ -d "$volume" ]; then
        name=$(basename "$volume")
        lower_name=$(echo "$name" | tr '[:upper:]' '[:lower:]')
        is_match=false

        if [[ "$lower_name" == *"android"* ]] || [[ "$lower_name" == *"phone"* ]] || [[ "$lower_name" == *"mtp"* ]] || [[ "$lower_name" == *"pixel"* ]] || [[ "$lower_name" == *"samsung"* ]] || [[ "$lower_name" == *"storage"* ]]; then
            is_match=true
        fi

        if [ -d "$volume/DCIM" ] || [ -d "$volume/Internal storage" ] || [ -d "$volume/Internal Storage" ] || [ -d "$volume/sdcard" ]; then
            is_match=true
        fi

        if [ "$is_match" = true ]; then
            echo "✅ '$name' WOULD BE DETECTED as an Android device."
        else
            echo "❌ '$name' would be ignored by current logic."
        fi
    fi
done

echo ""
echo "💡 TIP: If your phone is connected but not listed above, it means macOS hasn't mounted it as a filesystem volume."
echo "   Consider using a tool like 'OpenMTP' or 'macFUSE' to mount it, or use ADB mode (USB Debugging) instead."
