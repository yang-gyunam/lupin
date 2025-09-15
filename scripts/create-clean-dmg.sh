#!/bin/bash

# Clean DMG creation script
# Creates a DMG with only the app and Applications symlink, no system files

set -e

DMG_NAME="lupin_0.1.0_aarch64.dmg"
APP_PATH="src-tauri/target/release/bundle/macos/lupin.app"
DMG_PATH="src-tauri/target/release/bundle/dmg/$DMG_NAME"
VOLUME_NAME="lupin"
TEMP_DIR="/tmp/lupin-dmg-$$"

echo "Creating clean DMG installer..."

# Clean up any existing temp directory
if [ -d "$TEMP_DIR" ]; then
    rm -rf "$TEMP_DIR"
fi

# Create a clean temp directory
echo "Creating temporary directory..."
mkdir -p "$TEMP_DIR"

# Copy only the app to temp directory
echo "Copying app to temporary directory..."
cp -R "$APP_PATH" "$TEMP_DIR/"

# Create Applications symlink
echo "Creating Applications symlink..."
ln -s /Applications "$TEMP_DIR/Applications"

# Remove any system files that might have been created
echo "Removing any system files..."
find "$TEMP_DIR" -name '.DS_Store' -type f -delete 2>/dev/null || true
find "$TEMP_DIR" -name '.fseventsd' -type d -exec rm -rf {} + 2>/dev/null || true
find "$TEMP_DIR" -name '.Spotlight-V100' -type d -exec rm -rf {} + 2>/dev/null || true
find "$TEMP_DIR" -name '.Trashes' -type d -exec rm -rf {} + 2>/dev/null || true
find "$TEMP_DIR" -name '._*' -type f -delete 2>/dev/null || true

# Remove old DMG if exists
if [ -f "$DMG_PATH" ]; then
    echo "Removing old DMG..."
    rm -f "$DMG_PATH"
fi

# Create DMG from clean directory
echo "Creating DMG..."
hdiutil create -volname "$VOLUME_NAME" \
    -srcfolder "$TEMP_DIR" \
    -ov \
    -format UDZO \
    "$DMG_PATH"

# Clean up temp directory
echo "Cleaning up..."
rm -rf "$TEMP_DIR"

# Apply visual settings to DMG
echo "Applying DMG visual settings..."
TEMP_DMG="${DMG_PATH%.dmg}_rw.dmg"

# Convert to read-write DMG for modifications
hdiutil convert "$DMG_PATH" -format UDRW -o "$TEMP_DMG"

# Mount the DMG
MOUNT_OUTPUT=$(hdiutil attach -readwrite -noverify -noautoopen "$TEMP_DMG")
DEVICE=$(echo "$MOUNT_OUTPUT" | grep '^/dev/' | awk '{print $1}')
MOUNT_DIR="/Volumes/$VOLUME_NAME"

# Wait for mount to complete
sleep 1

# Remove only .fseventsd (keep .DS_Store for view settings)
rm -rf "$MOUNT_DIR/.fseventsd" 2>/dev/null || true

# Set icon positions using AppleScript (matching tauri.conf.json settings)
echo "Setting icon positions..."
osascript <<EOD
tell application "Finder"
    tell disk "$VOLUME_NAME"
        open
        set current view of container window to icon view
        set toolbar visible of container window to false
        set statusbar visible of container window to false
        set the bounds of container window to {400, 100, 1000, 500}
        set viewOptions to the icon view options of container window
        set arrangement of viewOptions to not arranged
        set icon size of viewOptions to 128
        set position of item "lupin.app" of container window to {180, 170}
        set position of item "Applications" of container window to {420, 170}
        close
        open
        update without registering applications
        delay 2
    end tell
end tell
EOD

# Create .DS_Store to save view settings
echo "Saving view settings..."
sleep 2

# Sync to ensure all changes are written
sync

# Unmount the DMG
echo "Unmounting DMG..."
hdiutil detach "$DEVICE" -quiet

# Convert back to compressed read-only DMG
echo "Creating final DMG..."
rm -f "$DMG_PATH"
hdiutil convert "$TEMP_DMG" -format UDZO -o "$DMG_PATH"
rm -f "$TEMP_DMG"

echo "✅ DMG created successfully at: $DMG_PATH"
echo "   No system files included!"