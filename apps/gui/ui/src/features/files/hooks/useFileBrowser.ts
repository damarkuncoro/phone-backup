import { useState, useEffect, useMemo, useCallback } from 'react';
import { deviceService, getDeviceId, type FileEntry } from '@/services/deviceService';
import { useDevices } from '@/features/devices/hooks/useDevices';
import { sortFiles } from '../lib/fileUtils';
import { useSelection } from './useSelection';

export function useFileBrowser() {
  const { devices } = useDevices();
  const [selectedDeviceId, setSelectedDeviceId] = useState<string | null>(null);
  const [currentPath, setCurrentPath] = useState<string>('/');
  const [files, setFiles] = useState<FileEntry[]>([]);
  const [loading, setLoading] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');

  const selection = useSelection();

  useEffect(() => {
    if (devices.length > 0 && !selectedDeviceId) {
        setSelectedDeviceId(getDeviceId(devices[0]));
    }
  }, [devices, selectedDeviceId]);

  const loadFiles = useCallback(async (deviceId: string, path: string) => {
    setLoading(true);
    try {
      const result = await deviceService.browse(deviceId, path);
      setFiles(result);
    } catch (err) {
      console.error("Failed to load files", err);
      setFiles([]);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    if (selectedDeviceId) {
        loadFiles(selectedDeviceId, currentPath);
    }
    selection.clear();
  }, [selectedDeviceId, currentPath, loadFiles]); // Selection clear logic included here

  const filteredFiles = useMemo(() => {
    const list = files.filter(f => (f.name || '').toLowerCase().includes(searchQuery.toLowerCase()));
    return sortFiles(list);
  }, [files, searchQuery]);

  const breadcrumbs = useMemo(() => {
    const parts = currentPath.split('/').filter(Boolean);
    let path = '';
    return [{ name: 'Root', path: '/' }, ...parts.map(p => {
      path += '/' + p;
      return { name: p, path };
    })];
  }, [currentPath]);

  return {
    devices,
    selectedDeviceId, setSelectedDeviceId,
    currentPath, setCurrentPath,
    loading,
    searchQuery, setSearchQuery,
    selection,
    filteredFiles,
    breadcrumbs
  };
}
