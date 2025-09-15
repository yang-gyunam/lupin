import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useTranslation } from "../hooks/useTranslation";

interface AutomationConfig {
  enabled: boolean;
  min_interval_ms: number;
  max_interval_ms: number;
  mouse_movement_range: number;
  enable_clicks: boolean;
  enable_keyboard: boolean;
  keyboard_text: string;
  active_apps: string[];
}

interface AutomationPanelProps {
  isActive: boolean;
  onToggle: (active: boolean) => void;
}

export default function AutomationPanel({ isActive, onToggle }: AutomationPanelProps) {
  const { t } = useTranslation();
  
  // Load saved config from localStorage or use defaults
  const loadSavedConfig = (): AutomationConfig => {
    const saved = localStorage.getItem('lupin-automation-config');
    if (saved) {
      try {
        return JSON.parse(saved);
      } catch (e) {
        console.error('Failed to parse saved config:', e);
      }
    }
    // Default config
    return {
      enabled: false,
      min_interval_ms: 30000,
      max_interval_ms: 840000,
      mouse_movement_range: 300,
      enable_clicks: false,
      enable_keyboard: false,
      keyboard_text: "I'm working on the project documentation. Need to review the latest changes and update the specifications accordingly. The implementation looks good so far.",
      active_apps: []
    };
  };
  
  const [config, setConfig] = useState<AutomationConfig>(loadSavedConfig());
  
  // Save config whenever it changes
  useEffect(() => {
    localStorage.setItem('lupin-automation-config', JSON.stringify(config));
  }, [config]);

  const handleToggle = async () => {
    const newState = !isActive;
    const newConfig = { 
      ...config, 
      enabled: newState,
      // Ensure keyboard_text is sent as Some(String) to Rust
      keyboard_text: config.keyboard_text || "Working on the project..."
    };
    
    try {
      if (newState) {
        console.log("Starting automation with config:", newConfig);
        await invoke("start_automation", { config: newConfig });
      } else {
        await invoke("stop_automation");
      }
      setConfig(newConfig);
      onToggle(newState);
    } catch (error) {
      console.error("Failed to toggle automation:", error);
    }
  };

  const updateInterval = (min: number, max: number) => {
    setConfig(prev => ({
      ...prev,
      min_interval_ms: min * 1000,
      max_interval_ms: max * 1000
    }));
  };

  return (
    <div className="panel automation-panel">
      <h2>{t('automation.title')}</h2>
      <p className="section-description">
        {t('automation.description')}
      </p>
      
      <div className="toggle-section">
        <button 
          className={`toggle-button ${isActive ? 'active' : ''}`}
          onClick={handleToggle}
        >
          {isActive ? t('automation.stopButton') : t('automation.startButton')} Automation
        </button>
      </div>

      <div className="config-section">
        <h3>{t('automation.intervalSettings')}</h3>
        <p className="subsection-description">
          {t('automation.intervalDescription')}
        </p>
        <div className="interval-controls">
          <label>
            {t('automation.minInterval')}
            <input 
              type="number" 
              min="10" 
              max="600"
              value={config.min_interval_ms / 1000}
              onChange={(e) => updateInterval(Number(e.target.value), config.max_interval_ms / 1000)}
              disabled={isActive}
            />
          </label>
          <label>
            {t('automation.maxInterval')}
            <input 
              type="number" 
              min="30" 
              max="900"
              value={config.max_interval_ms / 1000}
              onChange={(e) => updateInterval(config.min_interval_ms / 1000, Number(e.target.value))}
              disabled={isActive}
            />
          </label>
        </div>

        <div className="action-controls">
          <h3>{t('automation.actions')}</h3>
          <p className="subsection-description">
            {t('automation.actionsDescription')}
          </p>
          <label className="checkbox-label">
            <input 
              type="checkbox"
              checked={config.enable_clicks}
              onChange={(e) => setConfig(prev => ({ ...prev, enable_clicks: e.target.checked }))}
              disabled={isActive}
            />
            {t('automation.enableClicks')}
          </label>
          <label className="checkbox-label">
            <input 
              type="checkbox"
              checked={config.enable_keyboard}
              onChange={(e) => setConfig(prev => ({ ...prev, enable_keyboard: e.target.checked }))}
              disabled={isActive}
            />
            {t('automation.enableKeyboard')}
          </label>
          {config.enable_keyboard && (
            <div style={{ marginTop: '12px' }}>
              <label>
                {t('automation.textToType')}
                <textarea
                  value={config.keyboard_text}
                  onChange={(e) => setConfig(prev => ({ ...prev, keyboard_text: e.target.value }))}
                  disabled={isActive}
                  style={{
                    width: '100%',
                    minHeight: '80px',
                    marginTop: '8px',
                    padding: '8px',
                    backgroundColor: 'rgba(0, 0, 0, 0.3)',
                    border: '1px solid rgba(255, 255, 255, 0.1)',
                    borderRadius: '4px',
                    color: '#fff',
                    fontSize: '14px',
                    resize: 'vertical'
                  }}
                  placeholder={t('automation.textPlaceholder')}
                />
              </label>
              <div style={{ fontSize: '12px', opacity: 0.7, marginTop: '4px' }}>
                {t('automation.textHint')}
              </div>
            </div>
          )}
        </div>

        <div className="movement-control">
          <h3>{t('automation.mouseMovement')}</h3>
          <p className="subsection-description">
            {t('automation.mouseDescription')}
          </p>
          <label style={{ display: 'block' }}>
            {t('automation.movementRange')}: {config.mouse_movement_range}px
            <input 
              type="range"
              min="5"
              max="300"
              value={config.mouse_movement_range}
              onChange={(e) => setConfig(prev => ({ ...prev, mouse_movement_range: Number(e.target.value) }))}
              disabled={isActive}
              style={{ width: '100%', marginTop: '8px' }}
            />
            <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: '12px', opacity: 0.7, marginTop: '4px' }}>
              <span>5px</span>
              <span>150px</span>
              <span>300px</span>
            </div>
          </label>
        </div>
      </div>

      <div className="info-section">
        <p className="info-text">
          {t('automation.tip')}
          {t('automation.currentInterval')} {config.min_interval_ms/1000}s - {config.max_interval_ms/1000}s
        </p>
      </div>
    </div>
  );
}