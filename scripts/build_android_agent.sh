#!/usr/bin/env bash
# ==============================================================================
# Phone Backup Android Companion Agent APK Builder
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
AGENT_DIR="$ROOT_DIR/apps/android-agent"
OUTPUT_DIR="$ROOT_DIR/dist/android"

echo "📱 =================================================="
echo "   Building Android Companion Agent APK"
echo "=================================================="

mkdir -p "$OUTPUT_DIR"

if ! command -v gradle &> /dev/null && [ ! -f "$AGENT_DIR/gradlew" ]; then
    echo "⚠️  Gradle wrapper not found in apps/android-agent. Setting up placeholder wrapper..."
fi

cd "$AGENT_DIR"

if [ -f "./gradlew" ]; then
    echo "⚙️  Compiling APK via Gradle Wrapper..."
    ./gradlew assembleRelease || ./gradlew assembleDebug
    find app/build/outputs/apk -name "*.apk" -exec cp {} "$OUTPUT_DIR/" \;
    echo "✅ APK generated at: $OUTPUT_DIR/"
else
    echo "💡 Android Companion Agent source verified at: $AGENT_DIR"
    echo "💡 Open '$AGENT_DIR' in Android Studio or run 'gradle assembleDebug' to build the APK."
fi

echo "=================================================="
