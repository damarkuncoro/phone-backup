import { useState, useEffect, useMemo, useCallback } from 'react';
import { deviceService, getDeviceId, type Device, type FileEntry } from '@/services/deviceService';
import { backupService } from '@/services/backupService';
import { safeListen } from '@/shared/lib/ipc';

export type Step = 'select-device' | 'select-data' | 'configure' | 'progress';

interface ProgressPayload {
  type: 'start' | 'inc' | 'finish' | 'error' | 'log';
  total?: number;
  amount?: number;
  message: string;
}

export function useBackupWizard() {
  const [step, setStep] = useState<Step>('select-device');
  const [selectedDevice, setSelectedDevice] = useState<Device | null>(null);
  const [selectedData, setSelectedData] = useState<string[]>(['contacts', 'sms', 'photos', 'apps']);

  // Selection Analysis
  const [scannedFiles, setScannedFiles] = useState<FileEntry[]>([]);
  const [isCalculating, setIsCalculating] = useState(false);
  const [reviewSearch, setReviewSearch] = useState('');
  const [selectedPaths, setSelectedPaths] = useState<Set<string>>(new Set());

  // Progress State
  const [progressMsg, setProgressMsg] = useState('Initializing...');
  const [totalItems, setTotalItems] = useState(0);
  const [currentItems, setCurrentItems] = useState(0);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (step !== 'progress') return;

    return safeListen<ProgressPayload>('progress', (event) => {
        const payload = event.payload;
        setProgressMsg(payload.message);

        if (payload.type === 'start') {
            setTotalItems(payload.total || 0);
            setCurrentItems(0);
        } else if (payload.type === 'inc') {
            setCurrentItems(prev => prev + (payload.amount || 0));
        } else if (payload.type === 'error') {
            setError(payload.message);
        } else if (payload.type === 'finish') {
            setTotalItems(t => {
                setCurrentItems(t);
                return t;
            });
        }
    });
  }, [step]);

  const toggleData = useCallback((id: string) => {
    setSelectedData(prev =>
      prev.includes(id) ? prev.filter(i => i !== id) : [...prev, id]
    );
  }, []);

  const handleNextToConfigure = async () => {
    if (!selectedDevice) return;
    setStep('configure');
    setIsCalculating(true);

    try {
        const files = await deviceService.scan(getDeviceId(selectedDevice));

        let paths: string[] = [];
        if (selectedData.includes('photos')) paths.push('/storage/emulated/0/DCIM', '/storage/emulated/0/Pictures');
        if (selectedData.includes('files')) paths.push('/storage/emulated/0/Download', '/storage/emulated/0/Documents');

        const filtered = (files || []).filter(f => f && f.path && paths.some(p => f.path.startsWith(p)));
        setScannedFiles(filtered);

        // Default: Select all scanned files
        setSelectedPaths(new Set(filtered.map(f => f.path)));
    } catch (e) {
        console.error("Analysis failed", e);
    } finally {
        setIsCalculating(false);
    }
  };

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

  const handleStartBackup = async (allModulesCount: number) => {
    if (!selectedDevice) return;
    setStep('progress');
    setError(null);

    const finalFilePaths = scannedFiles
        .filter(f => selectedPaths.has(f.path))
        .map(f => f.path);

    try {
      const isFullBackup = selectedData.length === allModulesCount && finalFilePaths.length === scannedFiles.length;

      await backupService.startBackup(
        getDeviceId(selectedDevice),
        isFullBackup ? undefined : (finalFilePaths.length > 0 ? finalFilePaths : undefined)
      );
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  };

  const selectedFiles = useMemo(() => scannedFiles.filter(f => selectedPaths.has(f.path)), [scannedFiles, selectedPaths]);
  const totalBytes = useMemo(() => selectedFiles.reduce((acc, f) => acc + (f.size_bytes || 0), 0), [selectedFiles]);
  const progressPercent = totalItems > 0 ? Math.round((currentItems / totalItems) * 100) : 0;

  return {
    step, setStep,
    selectedDevice, setSelectedDevice,
    selectedData, setSelectedData, toggleData,
    scannedFiles, isCalculating, reviewSearch, setReviewSearch,
    selectedPaths, handleTogglePath, handleNextToConfigure, handleStartBackup,
    progressMsg, progressPercent, totalItems, currentItems, error,
    selectedFiles, totalBytes
  };
}
