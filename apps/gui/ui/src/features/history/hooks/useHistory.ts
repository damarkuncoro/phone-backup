import { useState, useEffect, useCallback, useMemo } from 'react';
import { backupService, type Snapshot, getSnapshotId } from '@/services/backupService';
import { deviceService, getDeviceId, type Device } from '@/services/deviceService';
import { safeListen } from '@/shared/lib/ipc';

export function useHistory() {
  const [devices, setDevices] = useState<Device[]>([]);
  const [liveDeviceIds, setLiveDeviceIds] = useState<Set<string>>(new Set());
  const [selectedDeviceId, setSelectedDeviceId] = useState<string | null>(null);
  const [snapshots, setSnapshots] = useState<Snapshot[]>([]);
  const [loading, setLoading] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const [comparisonSelection, setComparisonSelection] = useState<string[]>([]);

  const fetchDevices = useCallback(async () => {
    try {
      const [known, live] = await Promise.all([
        deviceService.getAllKnown(),
        deviceService.getAll().catch(() => [])
      ]);
      setDevices(known);
      setLiveDeviceIds(new Set((live || []).map(d => getDeviceId(d))));
      if (known.length > 0 && !selectedDeviceId) {
        setSelectedDeviceId(getDeviceId(known[0]));
      }
    } catch (e) {
      console.error("Failed to fetch known devices", e);
    }
  }, [selectedDeviceId]);

  useEffect(() => {
    fetchDevices();
    return safeListen('device-changed', () => {
      fetchDevices();
    });
  }, [fetchDevices]);

  const loadSnapshots = useCallback(async (deviceId: string) => {
    setLoading(true);
    try {
      const result = await backupService.getSnapshots(deviceId);
      setSnapshots(result.sort((a, b) => new Date(b.started_at).getTime() - new Date(a.started_at).getTime()));
    } catch (err) {
      console.error("Failed to load snapshots", err);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    if (selectedDeviceId) {
      loadSnapshots(selectedDeviceId);
    }
  }, [selectedDeviceId, loadSnapshots]);

  const toggleComparison = useCallback((id: string) => {
    setComparisonSelection(prev => {
        if (prev.includes(id)) return prev.filter(i => i !== id);
        if (prev.length >= 2) return [prev[1], id];
        return [...prev, id];
    });
  }, []);

  const handleDelete = async (id: string) => {
    if (!window.confirm('Are you sure you want to delete this snapshot?')) return;
    try {
        await backupService.deleteSnapshot(id);
        setSnapshots(prev => prev.filter(s => getSnapshotId(s) !== id));
        setComparisonSelection(prev => prev.filter(i => i !== id));
    } catch (err) {
        alert("Failed to delete snapshot");
    }
  };

  const filteredSnapshots = useMemo(() => {
    return snapshots.filter(s =>
      getSnapshotId(s).toLowerCase().includes(searchQuery.toLowerCase()) ||
      s.status.toLowerCase().includes(searchQuery.toLowerCase())
    );
  }, [snapshots, searchQuery]);

  return {
    devices,
    liveDeviceIds,
    selectedDeviceId, setSelectedDeviceId,
    snapshots, loading,
    searchQuery, setSearchQuery,
    comparisonSelection, toggleComparison,
    handleDelete,
    filteredSnapshots
  };
}
