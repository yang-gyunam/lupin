import { useState, useEffect } from "react";
import { useTranslation } from "../hooks/useTranslation";

interface StatusBarProps {
  isActive: boolean;
  disguise?: string;  // Made optional since Process Disguise is not implemented yet
}

export default function StatusBar({ isActive }: StatusBarProps) {
  const { t } = useTranslation();
  const [uptime, setUptime] = useState(0);

  useEffect(() => {
    if (isActive) {
      const interval = setInterval(() => {
        setUptime(prev => prev + 1);
      }, 1000);
      return () => clearInterval(interval);
    } else {
      setUptime(0);
    }
  }, [isActive]);

  const formatUptime = (seconds: number) => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  return (
    <div className="status-bar">
      <div className="status-item">
        <span className="status-label">Status:</span>
        <span className={`status-value ${isActive ? 'active' : 'inactive'}`}>
          {isActive ? t('status.active').toUpperCase() : t('status.inactive').toUpperCase()}
        </span>
      </div>
      
      {/* TODO: Show disguise status when Process Disguise is implemented
      <div className="status-item">
        <span className="status-label">{t('status.disguise')}:</span>
        <span className="status-value">{disguise || 'lupin'}</span>
      </div>
      */}
      
      {isActive && (
        <div className="status-item">
          <span className="status-label">Uptime:</span>
          <span className="status-value">{formatUptime(uptime)}</span>
        </div>
      )}
      
      <div className="status-item">
        <span className="status-label">Mode:</span>
        <span className="status-value stealth">{t('status.stealth').toUpperCase()}</span>
      </div>
    </div>
  );
}