#!/bin/bash
set -e

# Usage: ./scripts/release.sh 0.3.2

NEW_VERSION=$1

if [ -z "$NEW_VERSION" ]; then
    echo "Usage: $0 <version>"
    exit 1
fi

echo "🚀 Preparing release v$NEW_VERSION..."

# 1. Update versions in root Cargo.toml (workspace.package and internal dependencies)
# Using sed to update version = "..." to version = "NEW_VERSION"
# We target lines that start with version = or reference our internal packages
sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
sed -i '' "s/version = \"0.3.1\"/version = \"$NEW_VERSION\"/g" Cargo.toml

# 2. Update version in apps/gui/src-tauri/tauri.conf.json
sed -i '' "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" apps/gui/src-tauri/tauri.conf.json

# 3. Update version in frontend package.json
sed -i '' "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" apps/gui/ui/package.json

echo "✅ Versions updated to $NEW_VERSION"

# 4. Run tests
echo "🧪 Running tests..."
cargo test --workspace

# 5. Build to ensure everything is correct
echo "🔨 Building release binaries..."
cargo build --release --workspace

# 6. Commit and Tag
echo "📝 Committing and tagging..."
git add .
git commit -m "chore: release v$NEW_VERSION"
git tag -a "v$NEW_VERSION" -m "v$NEW_VERSION"

echo "✨ Done! Now run:"
echo "git push origin main --tags"
echo "cargo publish -p phone-backup-domain"
echo "cargo publish -p phone-backup-ports"
echo "cargo publish -p phone-backup-application"
echo "cargo publish -p phone-backup"
