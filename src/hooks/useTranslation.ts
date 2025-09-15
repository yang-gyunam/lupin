import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { translations, Language } from '../locales/translations';

export function useTranslation() {
  const [language, setLanguage] = useState<Language>('en');
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    // Get OS language from Tauri backend
    invoke<string>('get_system_language')
      .then((osLang) => {
        // Check if OS language starts with 'ko' (Korean)
        const lang: Language = osLang.toLowerCase().startsWith('ko') ? 'ko' : 'en';
        setLanguage(lang);
        setIsLoading(false);
      })
      .catch((error) => {
        console.error('Failed to get system language:', error);
        // Default to English on error
        setLanguage('en');
        setIsLoading(false);
      });
  }, []);

  const t = (key: string): string => {
    const keys = key.split('.');
    let value: any = translations[language];
    
    for (const k of keys) {
      if (value && typeof value === 'object' && k in value) {
        value = value[k];
      } else {
        // Fallback to English if key not found
        value = translations.en;
        for (const k2 of keys) {
          if (value && typeof value === 'object' && k2 in value) {
            value = value[k2];
          } else {
            return key; // Return key if translation not found
          }
        }
        break;
      }
    }
    
    return typeof value === 'string' ? value : key;
  };

  return { t, language, isLoading };
}