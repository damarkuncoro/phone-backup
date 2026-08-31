#!/bin/bash
set -e

echo "📦 Publishing crates to crates.io..."

# Order matters: publish dependencies first
CRATES=(
    "core/domain"
    "core/ports"
    "core/application"
    "adapters/filesystem"
    "adapters/mock"
    "adapters/adb"
    "adapters/agent"
    "adapters/opendal"
    "infrastructure/database-sqlite"
    "apps/cli"
)

for crate in "${CRATES[@]}"; do
    echo "Publishing $crate..."
    cargo publish --manifest-path "$crate/Cargo.toml" --no-verify
    # We use --no-verify because the workspace build might have already been verified
    # and it speeds up the process, but use with caution.
    sleep 5 # Give crates.io some time to index
done

echo "✅ All crates published!"
