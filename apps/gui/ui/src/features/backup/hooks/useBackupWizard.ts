import { useState, useEffect, useMemo, useCallback, useRef } from 'react';
import { deviceService, getDeviceId, type Device, type FileEntry } from '@/services/deviceService';
import { backupService } from '@/services/backupService';
import { safeListen } from '@/shared/lib/ipc';

export type Step = 'select-device' | 'select-data' | 'configure' | 'progress';

export interface AnalysisState {
  stage: 'mediastore' | 'crawler' | 'indexing' | 'ready';
  currentFolder: string;
  filesCount: number;
  totalBytes: number;
}

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

  // Selection Analysis State
  const [scannedFiles, setScannedFiles] = useState<FileEntry[]>([]);
  const [isCalculating, setIsCalculating] = useState(false);
  const [reviewSearch, setReviewSearch] = useState('');
  const [selectedPaths, setSelectedPaths] = useState<Set<string>>(new Set());

  // Live Analysis HUD Metrics
  const [analysisState, setAnalysisState] = useState<AnalysisState>({
    stage: 'mediastore',
    currentFolder: '/storage/emulated/0/DCIM/Camera',
    filesCount: 0,
    totalBytes: 0,
  });

  // Progress State
  const [progressMsg, setProgressMsg] = useState('Initializing...');
  const [totalItems, setTotalItems] = useState(0);
  const [currentItems, setCurrentItems] = useState(0);
  const [error, setError] = useState<string | null>(null);

  const timerRef = useRef<any>(null);

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

    // Start live analysis telemetry simulation for visual feedback
    const sampleFolders = [
      '/storage/emulated/0/DCIM/Camera',
      '/storage/emulated/0/Pictures/Screenshots',
      '/storage/emulated/0/Download',
      '/storage/emulated/0/Documents',
      '/storage/emulated/0/Movies',
      '/storage/emulated/0/WhatsApp/Media'
    ];

    let folderIdx = 0;
    setAnalysisState({
      stage: 'mediastore',
      currentFolder: sampleFolders[0],
      filesCount: 0,
      totalBytes: 0
    });

    if (timerRef.current) clearInterval(timerRef.current);
    timerRef.current = setInterval(() => {
      folderIdx = (folderIdx + 1) % sampleFolders.length;
      setAnalysisState(prev => ({
        stage: folderIdx < 2 ? 'mediastore' : folderIdx < 4 ? 'crawler' : 'indexing',
        currentFolder: sampleFolders[folderIdx],
        filesCount: prev.filesCount + Math.floor(Math.random() * 450 + 150),
        totalBytes: prev.totalBytes + Math.floor(Math.random() * 250_000_000 + 50_000_000)
      }));
    }, 400);

    try {
      const files = await deviceService.scan(getDeviceId(selectedDevice));

      let paths: string[] = [];
      if (!selectedData.includes('full_storage')) {
        if (selectedData.includes('photos')) paths.push('/storage/emulated/0/DCIM', '/storage/emulated/0/Pictures', '/storage/emulated/0/Movies', '/DCIM', '/Pictures', '/Movies');
        if (selectedData.includes('chat_media')) paths.push('/storage/emulated/0/Android/media/com.whatsapp', '/storage/emulated/0/WhatsApp', '/storage/emulated/0/Telegram', '/WhatsApp', '/Telegram');
        if (selectedData.includes('files')) paths.push('/storage/emulated/0/Download', '/storage/emulated/0/Documents', '/Download', '/Documents');
        if (selectedData.includes('audio')) paths.push('/storage/emulated/0/Music', '/storage/emulated/0/Recordings', '/storage/emulated/0/VoiceRecorder', '/storage/emulated/0/Podcasts', '/Music', '/Recordings');
      }

      // If full_storage is selected or no specific path restriction, include ALL discovered files
      const filtered = (files || []).filter(f => {
        if (!f || !f.path) return false;
        if (selectedData.includes('full_storage') || paths.length === 0) return true;
        return paths.some(p => f.path.startsWith(p));
      });
      setScannedFiles(filtered);

      // Default: Select all scanned files
      setSelectedPaths(new Set(filtered.map(f => f.path)));

      const totalCalculatedBytes = filtered.reduce((acc, f) => acc + (f.size_bytes || 0), 0);
      setAnalysisState({
        stage: 'ready',
        currentFolder: 'Selesai dianalisis',
        filesCount: filtered.length,
        totalBytes: totalCalculatedBytes
      });
    } catch (e) {
      console.error("Analysis failed", e);
    } finally {
      if (timerRef.current) clearInterval(timerRef.current);
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
    if (timerRef.current) clearInterval(timerRef.current);
    setStep('progress');
    setError(null);

    const finalFilePaths = scannedFiles
      .filter(f => selectedPaths.has(f.path))
      .map(f => f.path);

    try {
      const isFullBackup = selectedData.length === allModulesCount && (finalFilePaths.length === scannedFiles.length || scannedFiles.length === 0);

      await backupService.startBackup(
        getDeviceId(selectedDevice),
        isFullBackup ? undefined : (finalFilePaths.length > 0 ? finalFilePaths : undefined)
      );
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  };

  const handleExpressBackup = async () => {
    if (!selectedDevice) return;
    if (timerRef.current) clearInterval(timerRef.current);
    setStep('progress');
    setError(null);

    try {
      // Direct full/selective streaming without waiting for tree scan
      await backupService.startBackup(getDeviceId(selectedDevice), undefined);
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
    selectedPaths, handleTogglePath, handleNextToConfigure, handleStartBackup, handleExpressBackup,
    analysisState,
    progressMsg, progressPercent, totalItems, currentItems, error,
    selectedFiles, totalBytes
  };
}
