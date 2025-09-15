export const translations = {
  en: {
    app: {
      title: "Lupin",
      subtitle: "Anti-Surveillance Automation Tool"
    },
    status: {
      active: "Active",
      inactive: "Inactive",
      disguise: "Disguise",
      stealth: "Stealth Mode"
    },
    automation: {
      title: "⚙️ Automation Control",
      description: "Automatically moves mouse and keyboard to simulate computer usage. Useful for preventing screen savers and avoiding idle status.",
      startButton: "▶️ Start",
      stopButton: "🛑 Stop",
      intervalSettings: "⏱️ Interval Settings",
      intervalDescription: "Set action intervals. Actions occur at random intervals for natural behavior.",
      minInterval: "Min Interval (seconds):",
      maxInterval: "Max Interval (seconds):",
      actions: "🎯 Actions",
      actionsDescription: "Select actions to perform. Mouse movement is enabled by default.",
      mouseRange: "Mouse Movement Range:",
      enableClicks: "Enable Random Clicks",
      enableKeyboard: "Enable Keyboard Activity",
      textToType: "Text to Type:",
      textPlaceholder: "Enter text that will be typed automatically...",
      textHint: "This text will be typed one character at a time with natural delays",
      mouseMovement: "🖱️ Mouse Movement",
      mouseDescription: "Set mouse movement range. Higher values mean larger movements.",
      movementRange: "Movement Range:",
      tip: "💡 Tip: Keep intervals random to avoid detection patterns.",
      currentInterval: "Current:"
    },
    process: {
      title: "🎭 Process Disguise",
      description: "Changes how this application appears in the system process list.",
      currentDisguise: "Current Disguise:",
      noteEditor: "Note Editor",
      noteEditorDesc: "Appears as a simple note-taking application",
      documentViewer: "Document Viewer",
      documentViewerDesc: "Looks like a PDF reader",
      systemMonitor: "System Monitor",
      systemMonitorDesc: "Mimics system resource monitor",
      calculator: "Calculator",
      calculatorDesc: "Disguised as calculator app",
      original: "Lupin (Original)",
      originalDesc: "No disguise applied",
      warning: "⚠️ Note: Process renaming requires administrator privileges on some systems."
    },
    launcher: {
      title: "🚀 Quick App Launcher",
      description: "Launch applications quickly to simulate normal computer usage.",
      notInstalled: "Not Found",
      info: "💡 Click to launch applications. Grayed out apps are not installed on your system."
    },
    footer: {
      reminder: "Remember: With great power comes great responsibility"
    }
  },
  ko: {
    app: {
      title: "Lupin",
      subtitle: "Anti-Surveillance Automation Tool"
    },
    status: {
      active: "활성",
      inactive: "비활성",
      disguise: "위장",
      stealth: "스텔스 모드"
    },
    automation: {
      title: "⚙️ 자동화 제어",
      description: "자동으로 마우스와 키보드를 움직여 컴퓨터 사용 중인 것처럼 보이게 합니다. 화면 보호기 방지, 자리 비움 상태 회피 등에 유용합니다.",
      startButton: "▶️ 시작",
      stopButton: "🛑 정지",
      intervalSettings: "⏱️ 간격 설정",
      intervalDescription: "동작 간격을 설정합니다. 랜덤한 간격으로 자연스럽게 동작합니다.",
      minInterval: "최소 간격 (초):",
      maxInterval: "최대 간격 (초):",
      actions: "🎯 동작",
      actionsDescription: "수행할 동작을 선택합니다. 마우스 움직임은 기본으로 활성화됩니다.",
      mouseRange: "마우스 이동 범위:",
      enableClicks: "랜덤 클릭 활성화",
      enableKeyboard: "키보드 활동 활성화",
      textToType: "입력할 텍스트:",
      textPlaceholder: "자동으로 입력될 텍스트를 입력하세요...",
      textHint: "이 텍스트는 자연스러운 지연과 함께 한 글자씩 입력됩니다",
      mouseMovement: "🖱️ 마우스 움직임",
      mouseDescription: "마우스 움직임의 범위를 설정합니다. 값이 클수록 더 크게 움직입니다.",
      movementRange: "움직임 범위:",
      tip: "💡 팁: 탐지 패턴을 피하기 위해 간격을 랜덤하게 유지하세요.",
      currentInterval: "현재:"
    },
    process: {
      title: "🎭 프로세스 위장",
      description: "시스템 프로세스 목록에서 이 애플리케이션이 표시되는 방식을 변경합니다.",
      currentDisguise: "현재 위장:",
      noteEditor: "메모장",
      noteEditorDesc: "간단한 메모 작성 애플리케이션으로 표시",
      documentViewer: "문서 뷰어",
      documentViewerDesc: "PDF 리더처럼 보임",
      systemMonitor: "시스템 모니터",
      systemMonitorDesc: "시스템 리소스 모니터로 위장",
      calculator: "계산기",
      calculatorDesc: "계산기 앱으로 위장",
      original: "Lupin (원본)",
      originalDesc: "위장 없음",
      warning: "⚠️ 참고: 일부 시스템에서는 프로세스 이름 변경에 관리자 권한이 필요합니다."
    },
    launcher: {
      title: "🚀 빠른 앱 실행기",
      description: "일반적인 컴퓨터 사용을 시뮬레이션하기 위해 애플리케이션을 빠르게 실행합니다.",
      notInstalled: "설치 안 됨",
      info: "💡 클릭하여 애플리케이션을 실행하세요. 회색 앱은 시스템에 설치되지 않았습니다."
    },
    footer: {
      reminder: "기억하세요: 큰 힘에는 큰 책임이 따릅니다"
    }
  }
};

export type Language = keyof typeof translations;
export type TranslationKeys = typeof translations.en;