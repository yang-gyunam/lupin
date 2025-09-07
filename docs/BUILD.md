# 빌드 가이드

## 📋 빌드 스크립트 설명

### 🧹 clean-build.sh
macOS 시스템 파일을 제거하는 스크립트

**용도:**
- `.DS_Store`, `.fseventsd`, `.Spotlight-V100` 등 시스템 파일 제거
- DMG 빌드 전 디렉토리 정리

**실행 방법:**
```bash
./scripts/clean-build.sh
```

### 📦 build-clean-dmg.sh
깨끗한 DMG 설치 파일을 생성하는 메인 스크립트

**특징:**
- 임시 디렉토리에서 DMG 생성
- 시스템 파일 자동 제거
- 아이콘 크기와 위치 설정
- `.DS_Store` 파일 보존 (뷰 설정 저장용)

**실행 방법:**
```bash
./scripts/build-clean-dmg.sh
```

### 🔧 create-final-dmg.sh
대체 DMG 생성 스크립트 (백업용)

**실행 방법:**
```bash
./scripts/create-final-dmg.sh
```

## 🏗️ 빌드 프로세스

### 1. 전체 빌드 (권장)
```bash
npm run tauri:build
```

이 명령은 다음 작업을 순차적으로 수행합니다:
1. `clean-build.sh` - 시스템 파일 정리
2. `tauri build` - Tauri 앱 빌드
3. `build-clean-dmg.sh` - 깨끗한 DMG 생성

### 2. DMG만 재생성
기존 앱 번들에서 DMG만 다시 생성:
```bash
npm run build-dmg
```

### 3. 정리 후 DMG 생성
시스템 파일 정리 후 DMG 생성:
```bash
npm run clean-dmg
```

## 🎯 DMG 설정

### tauri.conf.json 설정
```json
"macOS": {
  "dmg": {
    "windowSize": {
      "width": 600,
      "height": 400
    },
    "appPosition": {
      "x": 180,
      "y": 170
    },
    "applicationFolderPosition": {
      "x": 420,
      "y": 170
    }
  }
}
```

### 최종 DMG 사양
- **창 크기**: 600x400 픽셀
- **아이콘 크기**: 128x128 픽셀
- **lupin.app 위치**: (180, 170)
- **Applications 링크 위치**: (420, 170)
- **배경색**: 다크 모드 호환

## 🐛 문제 해결

### DMG에 시스템 파일이 보이는 경우
```bash
# 1. 정리 스크립트 실행
./scripts/clean-build.sh

# 2. DMG 재생성
./scripts/build-clean-dmg.sh
```

### 아이콘 크기가 작게 보이는 경우
`.DS_Store` 파일이 제대로 저장되지 않은 경우입니다.
```bash
# build-clean-dmg.sh 사용 (권장)
npm run build-dmg
```

### DMG 마운트 오류
```bash
# 기존 마운트된 볼륨 확인
ls /Volumes/

# 강제 언마운트
hdiutil detach /Volumes/lupin -force

# DMG 재생성
npm run build-dmg
```

## 📝 빌드 결과

빌드 완료 후 생성되는 파일:

### macOS
```
src-tauri/target/release/bundle/
├── macos/
│   └── lupin.app
└── dmg/
    └── lupin_0.1.0_aarch64.dmg  # 최종 설치 파일
```

### Windows
```
src-tauri/target/release/bundle/
└── msi/
    └── lupin_0.1.0_x64_en-US.msi
```

### Linux
```
src-tauri/target/release/bundle/
├── appimage/
│   └── lupin_0.1.0_amd64.AppImage
└── deb/
    └── lupin_0.1.0_amd64.deb
```

## ✅ 체크리스트

DMG 빌드 전 확인사항:
- [ ] Rust 최신 버전 설치
- [ ] Node.js 18+ 설치
- [ ] Tauri CLI 설치 (`cargo install tauri-cli`)
- [ ] Xcode Command Line Tools 설치 (macOS)
- [ ] 이전 빌드 파일 정리

## 🚀 배포

### GitHub Releases
1. 태그 생성:
```bash
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

2. GitHub에서 Release 생성
3. DMG 파일 업로드

### 코드 서명 (선택사항)
```bash
# Developer ID로 서명
codesign --deep --force --verify --verbose --sign "Developer ID Application: Your Name" lupin.app

# DMG 서명
codesign --sign "Developer ID Application: Your Name" lupin_0.1.0_aarch64.dmg
```