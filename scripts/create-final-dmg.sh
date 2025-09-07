#!/bin/bash

# Final DMG creation with proper icon size preservation

set -e

DMG_NAME="lupin_0.1.0_aarch64.dmg"
APP_PATH="src-tauri/target/release/bundle/macos/lupin.app"
DMG_PATH="src-tauri/target/release/bundle/dmg/$DMG_NAME"
VOLUME_NAME="lupin"
TEMP_DIR="/tmp/lupin-dmg-$$"

echo "Creating final DMG installer..."

# Clean up any existing temp directory
rm -rf "$TEMP_DIR"

# Create a clean temp directory
echo "Creating temporary directory..."
mkdir -p "$TEMP_DIR"

# Copy only the app to temp directory
echo "Copying app..."
cp -R "$APP_PATH" "$TEMP_DIR/"

# Create Applications symlink
echo "Creating Applications symlink..."
ln -s /Applications "$TEMP_DIR/Applications"

# Remove old DMG if exists
if [ -f "$DMG_PATH" ]; then
    echo "Removing old DMG..."
    rm -f "$DMG_PATH"
fi

# Create DMG using hdiutil with specific settings
echo "Creating DMG with proper settings..."
hdiutil create -volname "$VOLUME_NAME" \
    -srcfolder "$TEMP_DIR" \
    -ov \
    -format UDRW \
    -size 100m \
    -fs HFS+ \
    -fsargs "-c c=64,a=16,e=16" \
    "${DMG_PATH}.tmp"

# Mount the DMG for customization
echo "Mounting DMG for customization..."
DEVICE=$(hdiutil attach -readwrite -noverify -noautoopen "${DMG_PATH}.tmp.dmg" | grep '^/dev/' | head -1 | awk '{print $1}')
MOUNT_DIR="/Volumes/$VOLUME_NAME"
echo "Mounted on device: $DEVICE"

# Wait for mount
sleep 1

# Remove .fseventsd if it exists
rm -rf "$MOUNT_DIR/.fseventsd" 2>/dev/null || true

# Apply visual settings with AppleScript
echo "Applying visual settings..."
osascript <<EOD
tell application "Finder"
    tell disk "$VOLUME_NAME"
        open
        set current view of container window to icon view
        set toolbar visible of container window to false
        set statusbar visible of container window to false
        set the bounds of container window to {400, 100, 1000, 500}
        set theViewOptions to the icon view options of container window
        set arrangement of theViewOptions to not arranged
        set icon size of theViewOptions to 128
        delay 1
        set position of item "lupin.app" of container window to {180, 170}
        set position of item "Applications" of container window to {420, 170}
        delay 1
        close
        open
        delay 2
        close
    end tell
end tell
EOD

# Force Finder to write the .DS_Store
echo "Forcing .DS_Store creation..."
sleep 3

# Sync filesystem
sync

# Unmount
echo "Unmounting..."
hdiutil detach "$DEVICE" -force

# Convert to compressed UDZO format
echo "Compressing DMG..."
hdiutil convert "${DMG_PATH}.tmp.dmg" -format UDZO -o "$DMG_PATH"

# Clean up
rm -f "${DMG_PATH}.tmp.dmg"
rm -rf "$TEMP_DIR"

echo "✅ DMG created successfully with proper icon size at: $DMG_PATH"