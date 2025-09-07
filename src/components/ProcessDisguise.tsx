import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useTranslation } from "../hooks/useTranslation";

interface Disguise {
  name: string;
  description: string;
}

interface ProcessDisguiseProps {
  currentDisguise: string;
  onDisguiseChange: (disguise: string) => void;
}

export default function ProcessDisguise({ currentDisguise, onDisguiseChange }: ProcessDisguiseProps) {
  const { t } = useTranslation();
  const [disguises, setDisguises] = useState<Disguise[]>([]);
  const [isChanging, setIsChanging] = useState(false);

  useEffect(() => {
    loadDisguises();
  }, []);

  const loadDisguises = async () => {
    try {
      const availableDisguises = await invoke<Disguise[]>("get_disguises");
      setDisguises(availableDisguises);
    } catch (error) {
      console.error("Failed to load disguises:", error);
    }
  };

  const applyDisguise = async (disguiseName: string) => {
    setIsChanging(true);
    try {
      await invoke("set_process_name", { name: disguiseName });
      onDisguiseChange(disguiseName);
    } catch (error) {
      console.error("Failed to apply disguise:", error);
    } finally {
      setIsChanging(false);
    }
  };

  return (
    <div className="panel disguise-panel">
      <h2>{t('process.title')}</h2>
      <p className="section-description">
        {t('process.description')}
      </p>
      
      <div className="current-disguise">
        <span className="label">{t('process.currentDisguise')}</span>
        <span className="disguise-name">{currentDisguise}</span>
      </div>

      <div className="disguise-list">
        <h3>🎪 Available Disguises</h3>
        <p className="subsection-description">
          {t('process.description')}
        </p>
        <div className="disguise-grid">
          {disguises.map((disguise) => (
            <button
              key={disguise.name}
              className={`disguise-option ${currentDisguise === disguise.name ? 'active' : ''}`}
              onClick={() => applyDisguise(disguise.name)}
              disabled={isChanging}
              title={disguise.description}
            >
              <div className="disguise-icon">🎪</div>
              <div className="disguise-info">
                <div className="disguise-title">{disguise.name}</div>
                <div className="disguise-desc">{disguise.description}</div>
              </div>
            </button>
          ))}
        </div>
      </div>

      <div className="warning-section">
        <p className="warning-text">
          {t('process.warning')}
        </p>
      </div>
    </div>
  );
}