import { useState, useEffect, useCallback } from 'react';
import { deviceService, type Device } from '@/services/deviceService';
import { safeListen } from '@/shared/lib/ipc';

export function useDevices() {
  const [devices, setDevices] = useState<Device[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const refreshDevices = useCallback(async () => {
    try {
      const data = await deviceService.getAll();
      setDevices(data);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    refreshDevices();

    return safeListen('device-changed', () => {
        console.log("Device connection changed, refreshing list...");
        refreshDevices();
    });
  }, [refreshDevices]);

  return { devices, loading, error, refreshDevices };
}
