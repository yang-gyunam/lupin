import { useState, useEffect } from "react";
import "./App.css";
import AutomationPanel from "./components/AutomationPanel";
// import ProcessDisguise from "./components/ProcessDisguise";  // TODO: Implement actual process disguise functionality
import AppLauncher from "./components/AppLauncher";
import StatusBar from "./components/StatusBar";
import { useTranslation } from "./hooks/useTranslation";
import { useTrayEvents } from "./hooks/useTrayEvents";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const { t } = useTranslation();
  const [isAutomationActive, setIsAutomationActive] = useState(false);
  // const [currentDisguise, setCurrentDisguise] = useState<string>("lupin");  // TODO: Uncomment when Process Disguise is implemented

  // Check if this is first launch (for future: could check localStorage)
  useEffect(() => {
    const checkFirstLaunch = async () => {
      // For first-time users, we might want to show a setup guide
      // But for now, stay hidden in tray as intended
      // Only show window if user explicitly opens it from tray
      
      // Optional: Show window only on very first launch ever
      const isFirstLaunch = localStorage.getItem('lupin-launched') === null;
      if (isFirstLaunch) {
        localStorage.setItem('lupin-launched', 'true');
        // Show window for initial setup
        const window = getCurrentWindow();
        await window.show();
        // Show in Dock on first launch
        await invoke('show_in_dock');
      }
      // Otherwise, stay in tray
    };
    checkFirstLaunch();
  }, []);

  // Listen for tray events
  useTrayEvents(
    async () => {
      // Start automation from tray using saved config
      try {
        // Load saved config from localStorage
        const savedConfig = localStorage.getItem('lupin-automation-config');
        let config = {
          enabled: true,
          min_interval_ms: 30000,
          max_interval_ms: 840000,
          mouse_movement_range: 300,
          enable_clicks: false,
          enable_keyboard: false,
          keyboard_text: "Working on the project...",
          active_apps: []
        };
        
        if (savedConfig) {
          try {
            const parsed = JSON.parse(savedConfig);
            config = { ...parsed, enabled: true };
          } catch (e) {
            console.error('Failed to parse saved config:', e);
          }
        }
        
        await invoke("start_automation", { config });
        setIsAutomationActive(true);
      } catch (error) {
        console.error("Failed to start automation from tray:", error);
      }
    },
    async () => {
      // Stop automation from tray
      try {
        await invoke("stop_automation");
        setIsAutomationActive(false);
      } catch (error) {
        console.error("Failed to stop automation from tray:", error);
      }
    }
  );

  return (
    <div className="app-container">
      <header className="app-header">
        <div className="logo-section">
          <h1 className="app-title">🎩 {t('app.title')}</h1>
          <span className="app-subtitle">{t('app.subtitle')}</span>
        </div>
        <StatusBar 
          isActive={isAutomationActive}
        />
      </header>

      <main className="app-main">
        <div className="control-grid">
          <div className="left-panel">
            <AutomationPanel 
              isActive={isAutomationActive}
              onToggle={setIsAutomationActive}
            />
          </div>
          
          <div className="right-panel">
            <AppLauncher />
          </div>
          
          {/* TODO: Implement actual process disguise functionality
          <ProcessDisguise 
            currentDisguise={currentDisguise}
            onDisguiseChange={setCurrentDisguise}
          />
          */}
        </div>
      </main>

      <footer className="app-footer">
        <div className="footer-info">
          <span className="stealth-indicator">
            {isAutomationActive ? `🟢 ${t('status.active')}` : `⚫ ${t('status.inactive')}`}
          </span>
          <span className="footer-text">
            {t('footer.reminder')}
          </span>
        </div>
      </footer>
    </div>
  );
}

export default App;