#!/bin/bash

# Clean DMG build script without VolumeIcon

DMG_NAME="lupin_0.1.0_aarch64.dmg"
APP_PATH="src-tauri/target/release/bundle/macos/lupin.app"
DMG_PATH="src-tauri/target/release/bundle/dmg/$DMG_NAME"
VOLUME_NAME="lupin"
TEMP_DMG="${DMG_PATH%.dmg}_temp.dmg"

# Remove old DMG if exists
if [ -f "$DMG_PATH" ]; then
    echo "Removing old DMG..."
    rm -f "$DMG_PATH"
fi

# Remove old temp DMG if exists
if [ -f "$TEMP_DMG" ]; then
    echo "Removing old temp DMG..."
    rm -f "$TEMP_DMG"
fi

# Create temporary DMG
echo "Creating temporary DMG..."
hdiutil create -size 100m -fs HFS+ -volname "$VOLUME_NAME" "$TEMP_DMG"

# Mount the temporary DMG
echo "Mounting temporary DMG..."
MOUNT_OUTPUT=$(hdiutil attach -readwrite -noverify -noautoopen "$TEMP_DMG")
MOUNT_DIR=$(echo "$MOUNT_OUTPUT" | grep "/Volumes" | awk '{print $3}')

# Copy app to DMG
echo "Copying app to DMG..."
cp -R "$APP_PATH" "$MOUNT_DIR/"

# Create Applications symlink
echo "Creating Applications symlink..."
ln -s /Applications "$MOUNT_DIR/Applications"

# Remove .fseventsd if it exists
echo "Removing .fseventsd..."
rm -rf "$MOUNT_DIR/.fseventsd" 2>/dev/null || true

# Hide system files
echo "Hiding system files..."
if [ -d "$MOUNT_DIR/.fseventsd" ]; then
    SetFile -a V "$MOUNT_DIR/.fseventsd" 2>/dev/null || true
fi

# Set icon positions using AppleScript
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

# Unmount the DMG
echo "Unmounting DMG..."
hdiutil detach "$MOUNT_DIR"

# Convert to compressed DMG
echo "Compressing DMG..."
hdiutil convert "$TEMP_DMG" -format UDZO -o "$DMG_PATH"

# Clean up
echo "Cleaning up..."
rm -f "$TEMP_DMG"

echo "DMG created successfully at: $DMG_PATH"