#!/bin/bash
set -e

# Path ke gambar sumber
SOURCE_ICON="apps/gui/src-tauri/icons/phone-backup.png"

# Pastikan script dijalankan dari root project
if [ ! -f "$SOURCE_ICON" ]; then
    echo "❌ Error: Gambar sumber tidak ditemukan di $SOURCE_ICON"
    echo "Pastikan Anda menjalankan script ini dari root direktori proyek."
    exit 1
fi

echo "🎨 Menghasilkan icon untuk Tauri..."

# Gunakan cargo tauri icon untuk menghasilkan semua format yang diperlukan
# Ini akan membuat 32x32.png, 128x128.png, 128x128@2x.png, icon.icns, icon.ico, dll.
cargo tauri icon "$SOURCE_ICON" --output "apps/gui/src-tauri/icons"

echo "✅ Selesai! Icon telah dihasilkan di apps/gui/src-tauri/icons/"
ls -lh apps/gui/src-tauri/icons/
