#!/bin/bash

# Clean build script for DMG creation
# Removes all macOS system files before creating DMG

echo "Cleaning macOS system files..."

# Clean dist directory
if [ -d "dist" ]; then
    echo "Cleaning dist directory..."
    find dist -name '.DS_Store' -type f -delete 2>/dev/null || true
    find dist -name '.fseventsd' -type d -exec rm -rf {} + 2>/dev/null || true
    find dist -name '.Spotlight-V100' -type d -exec rm -rf {} + 2>/dev/null || true
    find dist -name '.Trashes' -type d -exec rm -rf {} + 2>/dev/null || true
    find dist -name '._*' -type f -delete 2>/dev/null || true
fi

# Clean src-tauri target directory
if [ -d "src-tauri/target" ]; then
    echo "Cleaning src-tauri/target directory..."
    find src-tauri/target -name '.DS_Store' -type f -delete 2>/dev/null || true
    find src-tauri/target -name '.fseventsd' -type d -exec rm -rf {} + 2>/dev/null || true
    find src-tauri/target -name '.Spotlight-V100' -type d -exec rm -rf {} + 2>/dev/null || true
    find src-tauri/target -name '.Trashes' -type d -exec rm -rf {} + 2>/dev/null || true
    find src-tauri/target -name '._*' -type f -delete 2>/dev/null || true
    find src-tauri/target -name '.VolumeIcon.icns' -type f -delete 2>/dev/null || true
fi

# Clean DMG bundle directory specifically
if [ -d "src-tauri/target/release/bundle/dmg" ]; then
    echo "Cleaning DMG bundle directory..."
    find src-tauri/target/release/bundle/dmg -name '.DS_Store' -type f -delete 2>/dev/null || true
    find src-tauri/target/release/bundle/dmg -name '.fseventsd' -type d -exec rm -rf {} + 2>/dev/null || true
    find src-tauri/target/release/bundle/dmg -name '.VolumeIcon.icns' -type f -delete 2>/dev/null || true
fi

# Clean macOS app bundle
if [ -d "src-tauri/target/release/bundle/macos" ]; then
    echo "Cleaning macOS app bundle..."
    find src-tauri/target/release/bundle/macos -name '.DS_Store' -type f -delete 2>/dev/null || true
    find src-tauri/target/release/bundle/macos -name '.fseventsd' -type d -exec rm -rf {} + 2>/dev/null || true
fi

echo "System file cleanup completed!"