import { useState, useEffect, useMemo, useCallback, useRef } from 'react';
import { deviceService, getDeviceId, type Device, type FileEntry, type ScanCategorySummary } from '@/services/deviceService';
import { backupService } from '@/services/backupService';
import { safeListen } from '@/shared/lib/ipc';
import { SAMPLE_ANALYSIS_FOLDERS, resolveDataFilterPaths, filterScannedFiles } from '../lib/wizardHelpers';
import { useBackupReviewData } from './useBackupReviewData';

export type Step = 'select-device' | 'select-data' | 'configure' | 'progress';

export interface AnalysisState {
  stage: 'mediastore' | 'crawler' | 'indexing' | 'ready';
  currentFolder: string;
  filesCount: number;
  totalBytes: number;
  categories?: Record<string, ScanCategorySummary>;
  throughput?: number;
  durationMs?: number;
  warnings?: string[];
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
  const [scannedFiles, setScannedFiles] = useState<FileEntry[]>([]);
  const [isCalculating, setIsCalculating] = useState(false);
  const [reviewSearch, setReviewSearch] = useState('');
  const [selectedPaths, setSelectedPaths] = useState<Set<string>>(new Set());
  const reviewData = useBackupReviewData();

  const [analysisState, setAnalysisState] = useState<AnalysisState>({
    stage: 'mediastore',
    currentFolder: SAMPLE_ANALYSIS_FOLDERS[0],
    filesCount: 0,
    totalBytes: 0,
  });

  const [progressMsg, setProgressMsg] = useState('Initializing...');
  const [totalItems, setTotalItems] = useState(0);
  const [currentItems, setCurrentItems] = useState(0);
  const [isFinished, setIsFinished] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const timerRef = useRef<any>(null);

  useEffect(() => {
    if (step !== 'progress') return;
    return safeListen<ProgressPayload>('progress', (event) => {
      const p = event.payload;
      setProgressMsg(p.message);
      if (p.type === 'start') {
        setTotalItems(p.total || 0);
        setCurrentItems(0);
        setIsFinished(false);
      } else if (p.type === 'inc') {
        setCurrentItems(prev => prev + (p.amount || 0));
      } else if (p.type === 'error') {
        setError(p.message);
      } else if (p.type === 'finish') {
        setIsFinished(true);
        setProgressMsg(p.message || 'Pencadangan Selesai!');
        setTotalItems(t => {
          const finalVal = t > 0 ? t : 1;
          setCurrentItems(finalVal);
          return finalVal;
        });
      }
    });
  }, [step]);

  const toggleData = useCallback((id: string) => {
    setSelectedData(prev => prev.includes(id) ? prev.filter(i => i !== id) : [...prev, id]);
  }, []);

  const handleNextToConfigure = async () => {
    if (!selectedDevice) return;
    const deviceId = getDeviceId(selectedDevice);
    setStep('configure');
    reviewData.fetchLivePreviews(deviceId, selectedData);

    const hasFileCategories = selectedData.some(d => ['full_storage', 'photos', 'chat_media', 'files', 'audio'].includes(d));
    if (!hasFileCategories) {
      setScannedFiles([]);
      setSelectedPaths(new Set());
      setAnalysisState({ stage: 'ready', currentFolder: 'Data Kontak/SMS/Aplikasi Siap', filesCount: 0, totalBytes: 0 });
      setIsCalculating(false);
      return;
    }

    setIsCalculating(true);
    let folderIdx = 0;
    setAnalysisState({ stage: 'mediastore', currentFolder: SAMPLE_ANALYSIS_FOLDERS[0], filesCount: 0, totalBytes: 0 });

    if (timerRef.current) clearInterval(timerRef.current);
    timerRef.current = setInterval(() => {
      folderIdx = (folderIdx + 1) % SAMPLE_ANALYSIS_FOLDERS.length;
      setAnalysisState(prev => ({
        stage: folderIdx < 2 ? 'mediastore' : folderIdx < 4 ? 'crawler' : 'indexing',
        currentFolder: SAMPLE_ANALYSIS_FOLDERS[folderIdx],
        filesCount: prev.filesCount + Math.floor(Math.random() * 450 + 150),
        totalBytes: prev.totalBytes + Math.floor(Math.random() * 250_000_000 + 50_000_000)
      }));
    }, 400);

    try {
      const scanRes = await deviceService.scanDetailed(deviceId);
      const paths = resolveDataFilterPaths(selectedData);
      const filtered = filterScannedFiles(scanRes.files, selectedData, paths);
      setScannedFiles(filtered);
      setSelectedPaths(new Set(filtered.map(f => f.path)));
      setAnalysisState({
        stage: 'ready',
        currentFolder: 'Selesai dianalisis (V5 Engine)',
        filesCount: filtered.length,
        totalBytes: filtered.reduce((acc, f) => acc + (f.size_bytes || 0), 0),
        categories: scanRes.categories,
        throughput: scanRes.metrics?.throughput_files_per_sec,
        durationMs: scanRes.metrics?.duration_ms,
        warnings: scanRes.warnings.map(w => w.message),
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
      if (next.has(path)) { next.delete(path); childrenPaths.forEach(p => next.delete(p)); }
      else { next.add(path); childrenPaths.forEach(p => next.add(p)); }
      return next;
    });
  }, []);

  const handleStartBackup = async (allModulesCount: number) => {
    if (!selectedDevice) return;
    if (timerRef.current) clearInterval(timerRef.current);
    setStep('progress');
    setIsFinished(false);
    setError(null);
    const hasFileCategories = selectedData.some(d => ['full_storage', 'photos', 'chat_media', 'files', 'audio'].includes(d));
    const finalFilePaths = hasFileCategories ? scannedFiles.filter(f => selectedPaths.has(f.path)).map(f => f.path) : ['__NO_FILES__'];
    try {
      const isFull = selectedData.length === allModulesCount && (finalFilePaths.length === scannedFiles.length || scannedFiles.length === 0);
      await backupService.startBackup(getDeviceId(selectedDevice), isFull ? undefined : (finalFilePaths.length > 0 ? finalFilePaths : ['__NO_FILES__']));
      setIsFinished(true);
      setProgressMsg('Pencadangan Selesai!');
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  };

  const handleExpressBackup = async () => {
    if (!selectedDevice) return;
    if (timerRef.current) clearInterval(timerRef.current);
    setStep('progress');
    setIsFinished(false);
    setError(null);
    try {
      await backupService.startBackup(getDeviceId(selectedDevice), undefined);
      setIsFinished(true);
      setProgressMsg('Pencadangan Selesai!');
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  };

  const selectedFiles = useMemo(() => scannedFiles.filter(f => selectedPaths.has(f.path)), [scannedFiles, selectedPaths]);
  const totalBytes = useMemo(() => selectedFiles.reduce((acc, f) => acc + (f.size_bytes || 0), 0), [selectedFiles]);
  const progressPercent = isFinished
    ? 100
    : (totalItems > 0 ? Math.min(99, Math.round((currentItems / totalItems) * 100)) : (currentItems > 0 ? 50 : 10));

  return {
    step, setStep, selectedDevice, setSelectedDevice, selectedData, setSelectedData, toggleData,
    scannedFiles, isCalculating, reviewSearch, setReviewSearch, selectedPaths, handleTogglePath,
    handleNextToConfigure, handleStartBackup, handleExpressBackup, analysisState, progressMsg,
    progressPercent, totalItems, currentItems, error, selectedFiles, totalBytes,
    liveContacts: reviewData.contacts,
    selectedContactIds: reviewData.selectedContactIds,
    toggleContactId: reviewData.toggleContactId,
    selectAllContacts: reviewData.selectAllContacts,
    deselectAllContacts: reviewData.deselectAllContacts,
    loadingStructured: reviewData.loadingStructured
  };
}
