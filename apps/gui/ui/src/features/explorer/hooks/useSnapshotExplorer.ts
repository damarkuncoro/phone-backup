import { useState, useEffect, useMemo, useCallback, useRef } from 'react';
import { backupService } from '@/services/backupService';
import { type FileEntry } from '@/services/deviceService';
import { safeListen } from '@/shared/lib/ipc';
import { formatETA } from '@/shared/lib/formatters';

export type ExplorerMode = 'files' | 'contacts' | 'sms' | 'apps';

interface ProgressPayload {
    type: 'start' | 'inc' | 'finish' | 'error' | 'log';
    total?: number;
    amount?: number;
    message: string;
}

export function useSnapshotExplorer(snapshotId: string) {
  // 1. All Refs at the top
  const restoringRef = useRef(false);

  // 2. All basic UI state
  const [mode, setMode] = useState<ExplorerMode>('files');
  const [rawData, setRawData] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedPaths, setSelectedPaths] = useState<Set<string>>(new Set());

  // 3. Restore Progress State
  const [restoring, setRestoring] = useState(false);
  const [progressMsg, setProgressMsg] = useState('');
  const [totalItems, setTotalItems] = useState(0);
  const [currentItems, setCurrentItems] = useState(0);
  const [startTime, setStartTime] = useState<number | null>(null);
  const [eta, setEta] = useState<string | null>(null);

  // Sync ref with state
  useEffect(() => {
    restoringRef.current = restoring;
  }, [restoring]);

  const loadData = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      if (mode === 'files') {
        const result = await backupService.getSnapshotFiles(snapshotId);
        setRawData(Array.isArray(result) ? result : []);
      } else {
        const result = await backupService.getStructuredData(snapshotId, mode);
        setRawData(Array.isArray(result) ? result : []);
      }
    } catch (err) {
      if (typeof err !== 'string' || !err.includes('not found')) {
        console.error(`Failed to load ${mode}`, err);
      }
      setRawData([]);
      setError(typeof err === 'string' ? err : 'Data modul ini tidak tersedia untuk snapshot ini.');
    } finally {
      setLoading(false);
    }
  }, [mode, snapshotId]);

  useEffect(() => {
    loadData();
  }, [loadData]);

  // Persistent listener
  useEffect(() => {
    return safeListen<ProgressPayload>('progress', (event) => {
        const payload = event.payload;

        if (payload.type === 'error') {
            setError(payload.message);
            setRestoring(false);
            return;
        }

        // Only react if we are currently in a restore operation or it's a general log
        if (!restoringRef.current && payload.type !== 'log') return;

        setProgressMsg(payload.message);

        if (payload.type === 'start') {
            setTotalItems(payload.total || 0);
            setCurrentItems(0);
            setStartTime(Date.now());
            setEta("Menghitung...");
        } else if (payload.type === 'inc') {
            setCurrentItems(prev => prev + (payload.amount || 0));
        } else if (payload.type === 'finish') {
            setEta(null);
            setTotalItems(t => { setCurrentItems(t); return t; });
        }
    });
  }, []);

  useEffect(() => {
    if (startTime && currentItems > 0 && totalItems > 0) {
        const elapsed = Date.now() - startTime;
        const remaining = totalItems - currentItems;
        const msPerItem = elapsed / currentItems;
        const msRemaining = remaining * msPerItem;
        setEta(formatETA(msRemaining));
    }
  }, [currentItems, totalItems, startTime]);

  const handleTogglePath = useCallback((path: string, _isFolder: boolean, childrenPaths: string[]) => {
    setSelectedPaths(prev => {
        const next = new Set(prev);
        const isCurrentlySelected = next.has(path);
        if (isCurrentlySelected) {
            next.delete(path);
            childrenPaths.forEach(p => next.delete(p));
        } else {
            next.add(path);
            childrenPaths.forEach(p => next.add(p));
        }
        return next;
    });
  }, []);

  const startRestore = useCallback(async (paths?: string[]) => {
    setRestoring(true);
    setProgressMsg("Menyiapkan pemulihan...");
    setTotalItems(0);
    setCurrentItems(0);
    setStartTime(null);
    setEta(null);

    try {
        await backupService.restoreSnapshot(snapshotId, "workspace/restored_data", paths);
        if (paths) setSelectedPaths(new Set());
    } catch (err) {
        console.error("Restore failed", err);
        setRestoring(false);
        alert("Restore Gagal: " + err);
    }
  }, [snapshotId]);

  const progressPercent = totalItems > 0 ? Math.round((currentItems / totalItems) * 100) : 0;

  return {
    mode, setMode,
    rawData, loading, error, loadData,
    searchQuery, setSearchQuery,
    selectedPaths, setSelectedPaths, handleTogglePath,
    restoring, setRestoring, progressMsg, progressPercent, eta,
    startRestore
  };
}
