#!/bin/bash

# Fix bundle_dmg.sh to ignore volume icon parameter

BUNDLE_SCRIPT="src-tauri/target/release/bundle/dmg/bundle_dmg.sh"

if [ -f "$BUNDLE_SCRIPT" ]; then
    echo "Fixing bundle_dmg.sh to ignore --volicon parameter..."
    
    # Create a backup
    cp "$BUNDLE_SCRIPT" "$BUNDLE_SCRIPT.bak"
    
    # Replace the --volicon handling to do nothing
    sed -i '' '168,170s/.*/\t\t\t# VOLUME_ICON_FILE disabled\n\t\t\tshift; shift;;/' "$BUNDLE_SCRIPT"
    
    echo "Fix applied successfully"
    
    # Run the fixed script
    echo "Running fixed bundle_dmg.sh..."
    cd src-tauri/target/release/bundle/dmg
    rm -f lupin_0.1.0_aarch64.dmg
    ./bundle_dmg.sh lupin_0.1.0_aarch64.dmg ../macos/lupin.app --volname lupin --window-size 600 400 --app-drop-link 420 170 --icon lupin.app 180 170 --volicon icon.icns
    cd -
    
    echo "DMG created successfully!"
else
    echo "bundle_dmg.sh not found"
fi