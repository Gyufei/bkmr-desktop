import { useState, useCallback } from 'react';
import { invokeGetSystemInfo, type SystemInfo } from '../lib/invoke';


export function useSystemInfo() {
  const [info, setInfo] = useState<SystemInfo | null>(null);
  const [loading, setLoading] = useState(false);

  const load = useCallback(async (): Promise<void> => {
    setLoading(true);
    try {
      const result = await invokeGetSystemInfo();
      setInfo(result);
    } finally {
      setLoading(false);
    }
  }, []);

  return { info, loading, load };
}
