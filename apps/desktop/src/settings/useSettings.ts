import { useState, useCallback } from 'react';
import { invokeGetSettings, invokeUpdateSettings, type AppSettings } from '../lib/invoke';


export function useSettings() {
  const [settings, setSettings] = useState<AppSettings>({
    backup_dir: null,
    notes_dir: null,
  });
  const [loading, setLoading] = useState(false);

  const load = useCallback(async (): Promise<AppSettings> => {
    setLoading(true);
    try {
      const result = await invokeGetSettings();
      setSettings(result);
      return result;
    } finally {
      setLoading(false);
    }
  }, []);

  const save = useCallback(
    async (updated: Partial<AppSettings>): Promise<AppSettings> => {
      const merged = { ...settings, ...updated };
      setLoading(true);
      try {
        await invokeUpdateSettings(merged);
        setSettings(merged);
        return merged;
      } finally {
        setLoading(false);
      }
    },
    [settings],
  );

  return { settings, loading, load, save };
}
