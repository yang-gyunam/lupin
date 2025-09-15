#!/bin/bash

# Build a completely clean DMG with proper icon size

set -e

DMG_NAME="lupin_0.1.0_aarch64.dmg"
APP_PATH="src-tauri/target/release/bundle/macos/lupin.app"
DMG_PATH="src-tauri/target/release/bundle/dmg/$DMG_NAME"
VOLUME_NAME="lupin"
STAGING_DIR="/tmp/dmg-staging-$$"

echo "Building clean DMG..."

# Clean up any existing files
rm -f "$DMG_PATH"
rm -rf "$STAGING_DIR"

# Create staging directory
echo "Creating staging directory..."
mkdir -p "$STAGING_DIR"

# Copy app to staging
echo "Copying app..."
cp -R "$APP_PATH" "$STAGING_DIR/"

# Create Applications symlink
echo "Creating Applications symlink..."
ln -s /Applications "$STAGING_DIR/Applications"

# Clean any system files that might exist
find "$STAGING_DIR" -name '.DS_Store' -delete 2>/dev/null || true
find "$STAGING_DIR" -name '.fseventsd' -type d -exec rm -rf {} + 2>/dev/null || true
find "$STAGING_DIR" -name '._*' -delete 2>/dev/null || true

# Create a temporary DMG for setting up the view
echo "Creating temporary DMG for setup..."
TEMP_DMG="/tmp/temp-dmg-$$.dmg"
hdiutil create -size 100m -fs HFS+ -volname "$VOLUME_NAME" "$TEMP_DMG"

# Mount the temp DMG
echo "Mounting temp DMG..."
MOUNT_POINT=$(hdiutil attach -readwrite -nobrowse "$TEMP_DMG" | tail -1 | awk '{print $3}')

# Copy contents to mounted DMG
echo "Copying contents to DMG..."
cp -R "$STAGING_DIR"/* "$MOUNT_POINT/"

# Remove any auto-created system files
rm -rf "$MOUNT_POINT/.fseventsd" 2>/dev/null || true
rm -rf "$MOUNT_POINT/.Spotlight-V100" 2>/dev/null || true
rm -rf "$MOUNT_POINT/.Trashes" 2>/dev/null || true

# Set up the view with AppleScript
echo "Setting up DMG appearance..."
osascript <<EOD
tell application "Finder"
    -- Open the disk
    tell disk "$VOLUME_NAME"
        open
        
        -- Configure window
        set current view of container window to icon view
        set toolbar visible of container window to false
        set statusbar visible of container window to false
        set the bounds of container window to {400, 100, 1000, 500}
        
        -- Configure icon view
        set viewOptions to the icon view options of container window
        set arrangement of viewOptions to not arranged
        set icon size of viewOptions to 128
        
        -- Position items
        set position of item "lupin.app" of container window to {180, 170}
        set position of item "Applications" of container window to {420, 170}
        
        -- Force update
        update without registering applications
        delay 3
        
        -- Close to save settings
        close
    end tell
end tell
EOD

# Wait for .DS_Store to be written
echo "Waiting for settings to save..."
sleep 3

# Unmount
echo "Unmounting temp DMG..."
hdiutil detach "$MOUNT_POINT"

# Convert to compressed format
echo "Creating final DMG..."
hdiutil convert "$TEMP_DMG" -format UDZO -o "$DMG_PATH"

# Clean up
rm -f "$TEMP_DMG"
rm -rf "$STAGING_DIR"

echo "✅ Clean DMG created at: $DMG_PATH"