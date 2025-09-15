import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useTranslation } from "../hooks/useTranslation";

interface AppConfig {
  name: string;
  path: string;
  args: string[];
  icon?: string;
}

export default function AppLauncher() {
  const { t } = useTranslation();
  const [apps, setApps] = useState<AppConfig[]>([]);
  const [installedApps, setInstalledApps] = useState<Set<string>>(new Set());

  useEffect(() => {
    loadApps();
  }, []);

  const loadApps = async () => {
    try {
      const availableApps = await invoke<AppConfig[]>("get_available_apps");
      setApps(availableApps);
      
      // Check which apps are installed
      const installed = new Set<string>();
      for (const app of availableApps) {
        const isInstalled = await invoke<boolean>("check_app_installed", { app });
        if (isInstalled) {
          installed.add(app.name);
        }
      }
      setInstalledApps(installed);
    } catch (error) {
      console.error("Failed to load apps:", error);
    }
  };

  const launchApp = async (app: AppConfig) => {
    try {
      await invoke("launch_app", { app });
    } catch (error) {
      console.error(`Failed to launch ${app.name}:`, error);
    }
  };

  return (
    <div className="panel launcher-panel">
      <h2>{t('launcher.title')}</h2>
      <p className="section-description">
        {t('launcher.description')}
      </p>
      
      <div className="apps-grid">
        {apps.map((app) => (
          <button
            key={app.name}
            className={`app-button ${installedApps.has(app.name) ? '' : 'disabled'}`}
            onClick={() => installedApps.has(app.name) && launchApp(app)}
            disabled={!installedApps.has(app.name)}
            title={installedApps.has(app.name) ? `Launch ${app.name}` : `${app.name} not installed`}
          >
            <div className="app-icon">
              {app.name === "Visual Studio Code" && "📝"}
              {app.name === "Chrome" && "🌐"}
              {app.name === "Terminal" && "💻"}
              {app.name === "Slack" && "💬"}
              {app.name === "Notion" && "📓"}
              {app.name === "PowerShell" && "⚡"}
              {app.name === "Notepad++" && "📄"}
              {!["Visual Studio Code", "Chrome", "Terminal", "Slack", "Notion", "PowerShell", "Notepad++"].includes(app.name) && "📱"}
            </div>
            <div className="app-name">{app.name}</div>
            {!installedApps.has(app.name) && (
              <div className="not-installed">{t('launcher.notInstalled')}</div>
            )}
          </button>
        ))}
      </div>

      <div className="launcher-info">
        <p className="info-text">
          {t('launcher.info')}
        </p>
      </div>
    </div>
  );
}