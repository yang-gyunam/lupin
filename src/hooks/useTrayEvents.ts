import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';

export const useTrayEvents = (
  onStartAutomation: () => void,
  onStopAutomation: () => void
) => {
  useEffect(() => {
    // Listen for tray menu events
    const unsubscribeStart = listen('tray-start-automation', () => {
      console.log('Starting automation from tray');
      onStartAutomation();
    });

    const unsubscribeStop = listen('tray-stop-automation', () => {
      console.log('Stopping automation from tray');
      onStopAutomation();
    });

    // Cleanup listeners on unmount
    return () => {
      unsubscribeStart.then(fn => fn());
      unsubscribeStop.then(fn => fn());
    };
  }, [onStartAutomation, onStopAutomation]);
};