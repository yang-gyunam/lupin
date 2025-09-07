#!/bin/bash

# bundle_dmg.sh 스크립트 경로
BUNDLE_SCRIPT="src-tauri/target/release/bundle/dmg/bundle_dmg.sh"

if [ -f "$BUNDLE_SCRIPT" ]; then
    echo "Patching bundle_dmg.sh to skip VolumeIcon creation..."
    
    # VolumeIcon 관련 라인들을 주석 처리
    sed -i.bak '464,468s/^/#/' "$BUNDLE_SCRIPT"
    sed -i.bak '541,544s/^/#/' "$BUNDLE_SCRIPT"
    
    echo "Patch applied successfully"
else
    echo "bundle_dmg.sh not found"
fi