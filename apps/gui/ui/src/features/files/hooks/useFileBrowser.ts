import { useState, useEffect, useMemo, useCallback } from 'react';
import { deviceService, getDeviceId, type FileEntry } from '@/services/deviceService';
import { useDevices } from '@/features/devices/hooks/useDevices';
import { sortFiles, filterByCategory, type SortField, type SortDirection, type FileCategory } from '../lib/fileUtils';
import { useSelection } from './useSelection';

export interface QuickAccessItem {
  id: string;
  name: string;
  path: string;
  icon: string;
}

export const QUICK_ACCESS_ITEMS: QuickAccessItem[] = [
  { id: 'dcim', name: 'Kamera (DCIM)', path: '/storage/emulated/0/DCIM', icon: 'camera' },
  { id: 'pictures', name: 'Foto & Gambar', path: '/storage/emulated/0/Pictures', icon: 'image' },
  { id: 'download', name: 'Unduhan (Download)', path: '/storage/emulated/0/Download', icon: 'download' },
  { id: 'whatsapp', name: 'WhatsApp Media', path: '/storage/emulated/0/Android/media/com.whatsapp/WhatsApp/Media', icon: 'message' },
  { id: 'documents', name: 'Dokumen', path: '/storage/emulated/0/Documents', icon: 'file-text' },
  { id: 'music', name: 'Musik', path: '/storage/emulated/0/Music', icon: 'music' },
];

export function useFileBrowser() {
  const { devices } = useDevices();
  const [selectedDeviceId, setSelectedDeviceId] = useState<string | null>(null);
  const [currentPath, setCurrentPath] = useState<string>('/');
  const [files, setFiles] = useState<FileEntry[]>([]);
  const [loading, setLoading] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const [viewMode, setViewMode] = useState<'list' | 'grid'>('list');
  const [sortBy, setSortBy] = useState<SortField>('name');
  const [sortDirection, setSortDirection] = useState<SortDirection>('asc');
  const [category, setCategory] = useState<FileCategory>('all');

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
  }, [selectedDeviceId, currentPath, loadFiles]);

  const handleSort = useCallback((field: SortField) => {
    if (sortBy === field) {
      setSortDirection(prev => prev === 'asc' ? 'desc' : 'asc');
    } else {
      setSortBy(field);
      setSortDirection('asc');
    }
  }, [sortBy]);

  const filteredFiles = useMemo(() => {
    let list = files.filter(f => (f.name || '').toLowerCase().includes(searchQuery.toLowerCase()));
    list = filterByCategory(list, category);
    return sortFiles(list, sortBy, sortDirection);
  }, [files, searchQuery, category, sortBy, sortDirection]);

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
    viewMode, setViewMode,
    sortBy, setSortBy,
    sortDirection, setSortDirection,
    handleSort,
    category, setCategory,
    selection,
    filteredFiles,
    breadcrumbs,
    quickAccessItems: QUICK_ACCESS_ITEMS
  };
}
