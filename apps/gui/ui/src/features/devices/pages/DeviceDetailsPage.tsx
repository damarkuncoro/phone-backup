import { useState, useEffect } from 'react';
import { ArrowLeft, Smartphone, Download, Settings as SettingsIcon, CheckCircle2 } from 'lucide-react';
import { type Device, getDeviceId, deviceService } from '@/services/deviceService';
import { backupService, type Snapshot } from '@/services/backupService';
import { DeviceSpecsPanel } from '../components/DeviceSpecsPanel';
import { DeviceSnapshotsPanel } from '../components/DeviceSnapshotsPanel';

interface DeviceDetailsPageProps {
  device: Device;
  onBack: () => void;
  onStartBackup: (deviceId: string) => void;
  onBrowseHistory: (snapshotId: string) => void;
  onNavigate?: (view: 'dashboard' | 'devices' | 'backup' | 'files' | 'history' | 'explorer' | 'settings') => void;
}

export function DeviceDetailsPage({
  device,
  onBack,
  onStartBackup,
  onBrowseHistory,
  onNavigate
}: DeviceDetailsPageProps) {
  const [snapshots, setSnapshots] = useState<Snapshot[]>([]);
  const [loadingHistory, setLoadingHistory] = useState(true);
  const [batteryStatus, setBatteryStatus] = useState<[number, number] | null>(null);
  const [toastMessage, setToastMessage] = useState<string | null>(null);
  const [isRefreshingSpecs, setIsRefreshingSpecs] = useState(false);
  const deviceId = getDeviceId(device);

  const showToast = (msg: string) => {
    setToastMessage(msg);
    setTimeout(() => setToastMessage(null), 3000);
  };

  const loadData = async () => {
    setLoadingHistory(true);
    try {
      const [historyData, batteryData] = await Promise.all([
        backupService.getSnapshots(deviceId),
        deviceService.getBattery(deviceId).catch(() => null)
      ]);
      setSnapshots(historyData.sort((a, b) => new Date(b.started_at).getTime() - new Date(a.started_at).getTime()));
      if (batteryData) setBatteryStatus(batteryData);
    } catch (err) {
      console.error("Failed to load device details extra info", err);
    } finally {
      setLoadingHistory(false);
    }
  };

  useEffect(() => {
    loadData();
  }, [deviceId]);

  const handleRefreshHardware = async () => {
    setIsRefreshingSpecs(true);
    try {
      const bData = await deviceService.getBattery(deviceId).catch(() => null);
      if (bData) setBatteryStatus(bData);
      showToast("Status hardware dan baterai diperbarui!");
    } catch {
      showToast("Gagal memperbarui status baterai.");
    } finally {
      setIsRefreshingSpecs(false);
    }
  };

  return (
    <div className="p-6 md:p-8 space-y-8 max-w-7xl mx-auto animate-in fade-in duration-300 relative">
      {toastMessage && (
        <div className="fixed top-20 right-6 z-50 bg-slate-900 text-white px-5 py-3 rounded-2xl shadow-2xl border border-slate-700 flex items-center gap-3 text-xs font-bold animate-in slide-in-from-top-4 duration-300">
          <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0" />
          <span>{toastMessage}</span>
        </div>
      )}

      {/* Top Hero Card */}
      <div className="bg-white rounded-[32px] border border-slate-100 p-6 md:p-8 shadow-sm flex flex-col md:flex-row md:items-center justify-between gap-6">
        <div className="flex items-center gap-5 min-w-0">
          <button
            type="button"
            onClick={onBack}
            className="p-3 hover:bg-slate-100 rounded-2xl border border-slate-200 text-slate-500 hover:text-slate-900 transition-all active:scale-95 shrink-0"
          >
            <ArrowLeft className="w-5 h-5" />
          </button>
          <div className="w-14 h-14 bg-indigo-600 rounded-2xl flex items-center justify-center text-white shadow-lg shadow-indigo-200 shrink-0">
            <Smartphone className="w-7 h-7" />
          </div>
          <div className="min-w-0">
            <div className="flex items-center gap-2 mb-1">
              <span className="text-[9px] font-black px-2.5 py-0.5 rounded-full uppercase tracking-wider bg-indigo-50 text-indigo-700 border border-indigo-100">
                {device.connection_type} Mode
              </span>
              <span className="text-xs font-bold text-slate-400 uppercase tracking-widest truncate">
                {device.manufacturer} • Android {device.os_version}
              </span>
            </div>
            <h1 className="text-2xl md:text-3xl font-black text-slate-900 tracking-tight truncate">
              {device.model}
            </h1>
          </div>
        </div>

        <div className="flex items-center gap-3 shrink-0">
          <button
            type="button"
            onClick={() => onStartBackup(deviceId)}
            className="px-6 py-3 bg-indigo-600 hover:bg-indigo-700 text-white rounded-2xl text-xs font-black uppercase tracking-wider shadow-lg shadow-indigo-200 hover:shadow-indigo-300 transition-all flex items-center gap-2 active:scale-95"
          >
            <Download className="w-4 h-4" />
            <span>Mulai Backup</span>
          </button>
          <button
            type="button"
            onClick={() => onNavigate?.('settings')}
            title="Pengaturan Sistem"
            className="p-3 bg-slate-50 hover:bg-slate-100 border border-slate-200 rounded-2xl text-slate-500 hover:text-slate-700 transition-all active:scale-95 shadow-sm"
          >
            <SettingsIcon className="w-5 h-5" />
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <DeviceSpecsPanel
          device={device}
          batteryStatus={batteryStatus}
          isRefreshingSpecs={isRefreshingSpecs}
          onRefreshHardware={handleRefreshHardware}
        />
        <DeviceSnapshotsPanel
          snapshots={snapshots}
          loadingHistory={loadingHistory}
          onBrowseHistory={onBrowseHistory}
          onNavigate={onNavigate}
          onRefreshHardware={handleRefreshHardware}
        />
      </div>
    </div>
  );
}
