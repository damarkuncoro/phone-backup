#!/usr/bin/env bash
set -euo pipefail

# ==============================================================================
# phone-backup Automated Production Release & Packaging Script
# ==============================================================================

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="${PROJECT_ROOT}/dist"
VERSION="0.4.1"

echo "=================================================="
echo "🚀 Building phone-backup Production Release v${VERSION}"
echo "=================================================="

# 1. Clean and prepare dist directory
rm -rf "${DIST_DIR}"
mkdir -p "${DIST_DIR}/bin" "${DIST_DIR}/archives"

# 2. Build Release CLI Binary
echo "🔨 Compiling optimized release CLI binary..."
cd "${PROJECT_ROOT}"
CARGO_INCREMENTAL=0 cargo build --release -p phone-backup

cp "${PROJECT_ROOT}/target/release/phone-backup" "${DIST_DIR}/bin/phone-backup"
strip "${DIST_DIR}/bin/phone-backup" 2>/dev/null || true

# 3. Build Desktop GUI React Bundle
echo "📦 Building Desktop GUI web client bundle..."
cd "${PROJECT_ROOT}/apps/gui/ui"
npm run build

# 4. Generate Tarball Distribution
echo "🎁 Packaging standalone distribution archive..."
cd "${DIST_DIR}"
ARCH_NAME="phone-backup-v${VERSION}-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)"
tar -czf "archives/${ARCH_NAME}.tar.gz" -C bin phone-backup

# 5. Generate Cryptographic SHA-256 Checksums
echo "🔐 Generating cryptographic SHA-256 checksums..."
cd "${DIST_DIR}/archives"
shasum -a 256 "${ARCH_NAME}.tar.gz" > "${ARCH_NAME}.tar.gz.sha256"

echo "=================================================="
echo "✅ Production Release Build Completed Successfully!"
echo "📍 Artifact: ${DIST_DIR}/archives/${ARCH_NAME}.tar.gz"
echo "📄 Checksum: $(cat ${DIST_DIR}/archives/${ARCH_NAME}.tar.gz.sha256)"
echo "=================================================="
