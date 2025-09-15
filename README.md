# 🎩 Lupin - Anti-Surveillance Automation Tool

근태 감시 시스템에 대항하는 스마트한 자동화 도구

## 📋 개요

Lupin은 15분 마우스/키보드 감시 시스템을 우회하는 자동화 도구입니다. 
사용자가 잠시 자리를 비우거나 집중해서 문서를 읽는 동안에도 자연스러운 활동 패턴을 생성합니다.

### 주요 기능

- 🖱️ **자동 마우스 움직임**: 설정된 간격으로 미세한 마우스 움직임 생성
- ⌨️ **키보드 활동 시뮬레이션**: 무해한 키 입력으로 활동 상태 유지
- ⏰ **스마트 스케줄링**: 랜덤 간격으로 자연스러운 패턴 생성

## 🚀 설치 방법

### 필수 요구사항

1. **Node.js** (v18 이상)
2. **Rust** (1.70 이상)
3. **Xcode Command Line Tools** (macOS)

### Rust 설치

```bash
# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 설치 후 터미널 재시작 또는
source $HOME/.cargo/env
```

### Lupin 설치 및 실행

```bash
# 저장소 클론
git clone https://github.com/yang-gyunam/lupin.git
cd lupin

# 의존성 설치
npm install

# 개발 모드 실행
npm run dev

# 프로덕션 빌드 (macOS - 깨끗한 DMG 생성)
npm run tauri:build

# 일반 빌드 (다른 플랫폼)
npm run tauri build
```

## 📦 플랫폼별 빌드 가이드

### macOS DMG 빌드

macOS에서 깨끗한 DMG 설치 파일을 생성하는 특별한 빌드 프로세스:

```bash
# 전체 빌드 + 깨끗한 DMG 생성
npm run tauri:build

# DMG만 다시 생성
npm run build-dmg

# 시스템 파일 정리 후 DMG 생성
npm run clean-dmg
```

생성된 파일:
- `src-tauri/target/release/bundle/dmg/lupin_0.1.0_aarch64.dmg` (M1/M2)
- `src-tauri/target/release/bundle/dmg/lupin_0.1.0_x64.dmg` (Intel)

### Windows 빌드

#### Windows에서 직접 빌드

```bash
# Windows 환경에서 실행
npm install
npm run tauri build

# MSI 설치 파일 생성 (기본값)
# 출력: src-tauri/target/release/bundle/msi/lupin_0.1.0_x64-setup.msi
```

## 🔐 macOS 권한 설정 (중요!)

### 첫 실행 시 필수 설정

macOS에서 Lupin이 마우스와 키보드를 제어하려면 접근성 권한이 필요합니다:

1. **System Settings** (시스템 설정) 열기
2. **Privacy & Security** (개인정보 보호 및 보안) 선택
3. **Accessibility** (손쉬운 사용) 클릭
4. 자물쇠 아이콘을 클릭하여 잠금 해제
5. **Lupin** 앱을 목록에 추가하고 체크박스 활성화

> ⚠️ **주의**: 이 권한을 허용하지 않으면 자동화 기능이 작동하지 않습니다.

## 🎮 사용 방법

### 시작 방법

**첫 실행 시:**
- 앱을 처음 실행하면 설정 창이 자동으로 나타납니다
- 설정 완료 후 창을 닫으면 트레이로 최소화됩니다

**이후 실행 시:**
- 앱이 시스템 트레이에서만 실행됩니다 (창 없음)
- 메뉴바(macOS) 또는 시스템 트레이(Windows)에서 아이콘 클릭
- 트레이 아이콘 클릭 → 창 표시/숨기기
- 트레이 아이콘 우클릭 → 메뉴 사용

## 🎮 기능 사용법

### 1. 자동화 설정

1. **Automation Control** 패널에서 활동 간격 설정
   - Min Interval: 최소 대기 시간 (기본 30초)
   - Max Interval: 최대 대기 시간 (기본 14분)

2. **액션 옵션** 선택
   - Enable Random Clicks: 랜덤 클릭 활성화
   - Enable Keyboard Activity: 키보드 활동 활성화

3. **Start Automation** 버튼으로 시작

### 2. 앱 런처

**Quick Launch** 패널에서 업무 앱을 빠르게 실행:
- Visual Studio Code
- Chrome
- Terminal

## ⚠️ 주의사항

1. **백그라운드 실행**: 시스템 트레이에서 조용히 실행됩니다
2. **적절한 간격 설정**: 너무 규칙적인 패턴은 오히려 의심을 받을 수 있습니다

## 🛡️ 보안 기능

- 랜덤 패턴 생성으로 탐지 회피
- 프로세스 이름 위장
- 미세한 마우스 움직임 (5px 범위)
- 자연스러운 활동 간격

## 📚 추가 문서

- [빌드 가이드](docs/BUILD.md) - 상세한 빌드 프로세스 및 스크립트 설명

## 🔧 문제 해결

### Rust 관련 오류
```bash
# Rust가 설치되었는지 확인
rustc --version

# PATH 설정 확인
echo $PATH | grep cargo
```

### 빌드 오류
```bash
# 캐시 정리
npm run clean
npm install
```

### DMG 관련 문제 (macOS)
```bash
# 시스템 파일 정리 후 재빌드
npm run clean-dmg
```

---

**Remember: With great power comes great responsibility** 🕷️