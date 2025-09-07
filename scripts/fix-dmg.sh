#!/bin/bash

# DMG 파일 경로
DMG_PATH="src-tauri/target/release/bundle/dmg/lupin_0.1.0_aarch64.dmg"

if [ ! -f "$DMG_PATH" ]; then
  echo "DMG file not found: $DMG_PATH"
  exit 1
fi

echo "Fixing DMG volume icon visibility..."

# 임시 디렉토리 생성
TEMP_DIR=$(mktemp -d)
echo "Created temp directory: $TEMP_DIR"

# DMG 마운트
echo "Mounting DMG..."
hdiutil attach "$DMG_PATH" -mountpoint "$TEMP_DIR" -nobrowse -noautoopen

# .VolumeIcon.icns 파일 숨기기
if [ -f "$TEMP_DIR/.VolumeIcon.icns" ]; then
  echo "Hiding .VolumeIcon.icns..."
  SetFile -a V "$TEMP_DIR/.VolumeIcon.icns"
  echo "Successfully hidden .VolumeIcon.icns"
else
  echo "Warning: .VolumeIcon.icns not found"
fi

# DMG 언마운트
echo "Unmounting DMG..."
hdiutil detach "$TEMP_DIR"

# 임시 디렉토리 삭제
rmdir "$TEMP_DIR"

echo "DMG fix completed!"