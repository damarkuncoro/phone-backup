import { useState, useEffect } from 'react';
import {
  ArrowLeft, Smartphone,
  HardDrive, Info, Clock, Download, ExternalLink,
  History, Settings as SettingsIcon, AlertTriangle,
  Activity, CheckCircle2, RefreshCw
} from 'lucide-react';
import { type Device, getDeviceId, deviceService } from '@/services/deviceService';
import { backupService, type Snapshot, getSnapshotId } from '@/services/backupService';
import { cn } from "../../../shared/lib/utils";
import { formatBytes, formatDate } from '@/shared/lib/formatters';

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

  const storagePercent = device.storage_total_bytes > 0
    ? Math.round((Number(device.storage_used_bytes) / Number(device.storage_total_bytes)) * 100)
    : 0;

  return (
    <div className="h-full flex flex-col bg-slate-50 animate-in fade-in slide-in-from-right-4 duration-500 relative">
      
      {/* Toast Notification */}
      {toastMessage && (
        <div className="fixed top-6 right-6 z-50 bg-slate-900 text-white px-5 py-3 rounded-2xl shadow-2xl border border-slate-700 flex items-center gap-3 text-xs font-bold animate-in slide-in-from-top-4 duration-300">
          <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0" />
          <span>{toastMessage}</span>
        </div>
      )}

      {/* Sticky Header */}
      <header className="bg-white border-b border-slate-200 px-8 py-6 flex items-center justify-between sticky top-0 z-20">
        <div className="flex items-center gap-6">
          <button
            type="button"
            onClick={onBack}
            className="p-2.5 hover:bg-slate-100 rounded-xl border border-slate-200 text-slate-500 transition-all active:scale-95"
          >
            <ArrowLeft className="w-5 h-5" />
          </button>
          <div className="flex items-center gap-4">
            <div className="w-12 h-12 bg-indigo-600 rounded-2xl flex items-center justify-center text-white shadow-lg shadow-indigo-200">
              <Smartphone className="w-6 h-6" />
            </div>
            <div>
              <h1 className="text-2xl font-black text-slate-900 tracking-tight">{device.model}</h1>
              <div className="text-xs font-bold text-slate-400 uppercase tracking-widest flex items-center gap-2">
                {device.manufacturer} <div className="w-1 h-1 bg-slate-300 rounded-full" /> Android {device.os_version}
              </div>
            </div>
          </div>
        </div>

        <div className="flex gap-3">
          <button
            type="button"
            onClick={() => onStartBackup(deviceId)}
            className="px-6 py-2.5 bg-indigo-600 text-white rounded-2xl text-xs font-black uppercase tracking-widest shadow-xl shadow-indigo-200 hover:bg-indigo-700 transition-all flex items-center gap-2 active:scale-95"
          >
            <Download className="w-4 h-4" /> Start Backup
          </button>
          <button
            type="button"
            onClick={() => onNavigate?.('settings')}
            title="Pengaturan Sistem"
            className="p-2.5 bg-white border border-slate-200 rounded-2xl text-slate-400 hover:text-slate-600 transition-all active:scale-95"
          >
            <SettingsIcon className="w-5 h-5" />
          </button>
        </div>
      </header>

      <div className="flex-1 overflow-y-auto p-8">
        <div className="max-w-6xl mx-auto grid grid-cols-1 lg:grid-cols-3 gap-8">

          {/* Left Column: Technical Specs & Status */}
          <div className="lg:col-span-1 space-y-8">
            <section className="bg-white p-8 rounded-[40px] border border-slate-100 shadow-sm space-y-6">
              <div className="flex items-center justify-between">
                <h2 className="text-sm font-black text-slate-900 uppercase tracking-widest flex items-center gap-2">
                  <Info className="w-4 h-4 text-indigo-500" /> Device Specs
                </h2>
                <button
                  type="button"
                  onClick={handleRefreshHardware}
                  disabled={isRefreshingSpecs}
                  title="Perbarui Status Hardware"
                  className="p-1.5 hover:bg-slate-100 rounded-lg text-slate-400 hover:text-indigo-600 transition-all"
                >
                  <RefreshCw className={cn("w-3.5 h-3.5", isRefreshingSpecs && "animate-spin text-indigo-600")} />
                </button>
              </div>

              <div className="space-y-4">
                <SpecRow label="Serial Number" value={device.serial} mono />
                <SpecRow label="Connection" value={device.connection_type} />
                <SpecRow label="Battery" value={batteryStatus ? `${batteryStatus[0]}%` : '---'} />
                <SpecRow label="Temperature" value={batteryStatus ? `${batteryStatus[1]}°C` : '---'} />
                <SpecRow label="OS Version" value={`Android ${device.os_version}`} />
              </div>
            </section>

            <section className="bg-white p-8 rounded-[40px] border border-slate-100 shadow-sm space-y-6">
              <h2 className="text-sm font-black text-slate-900 uppercase tracking-widest flex items-center gap-2">
                <HardDrive className="w-4 h-4 text-indigo-500" /> Storage Info
              </h2>
              <div className="space-y-4">
                <div className="flex justify-between items-end mb-2">
                  <p className="text-xs font-bold text-slate-500">Used Capacity</p>
                  <p className="text-sm font-black text-slate-900">{storagePercent}%</p>
                </div>
                <div className="h-4 w-full bg-slate-100 rounded-full overflow-hidden p-1">
                  <div
                    className={cn(
                      "h-full rounded-full transition-all duration-1000",
                      storagePercent > 90 ? "bg-red-500" : "bg-indigo-600"
                    )}
                    style={{ width: `${storagePercent}%` }}
                  />
                </div>
                <div className="grid grid-cols-2 gap-4 pt-2">
                  <div>
                    <p className="text-[10px] font-black text-slate-400 uppercase">Used</p>
                    <p className="text-sm font-bold text-slate-700">{formatBytes(Number(device.storage_used_bytes))}</p>
                  </div>
                  <div>
                    <p className="text-[10px] font-black text-slate-400 uppercase">Total</p>
                    <p className="text-sm font-bold text-slate-700">{formatBytes(Number(device.storage_total_bytes))}</p>
                  </div>
                </div>
              </div>
            </section>

            {/* Health Alert if battery low or temp high */}
            {batteryStatus && (batteryStatus[0] < 20 || batteryStatus[1] > 40) && (
              <div className="bg-amber-50 border border-amber-100 p-6 rounded-[32px] flex gap-4">
                <AlertTriangle className="w-6 h-6 text-amber-600 shrink-0" />
                <div>
                  <p className="text-xs font-black text-amber-800 uppercase tracking-widest">Hardware Warning</p>
                  <p className="text-xs text-amber-700 mt-1">
                    {batteryStatus[0] < 20 ? "Battery is low. Connect to power before backing up. " : ""}
                    {batteryStatus[1] > 40 ? "Device is overheating. Let it cool down." : ""}
                  </p>
                </div>
              </div>
            )}
          </div>

          {/* Right Column: Backup History */}
          <div className="lg:col-span-2 space-y-8">
            <section className="bg-white p-8 rounded-[40px] border border-slate-100 shadow-sm min-h-[500px] flex flex-col">
              <div className="flex items-center justify-between mb-8">
                <h2 className="text-xl font-black text-slate-900 tracking-tight flex items-center gap-3">
                  <History className="w-6 h-6 text-indigo-600" /> Recent Snapshots
                </h2>
                {loadingHistory && <Clock className="w-4 h-4 text-indigo-500 animate-spin" />}
              </div>

              <div className="flex-1 space-y-4">
                {snapshots.slice(0, 5).map(snapshot => (
                  <div key={getSnapshotId(snapshot)} className="group p-5 bg-slate-50 hover:bg-indigo-50/50 border border-slate-100 hover:border-indigo-100 rounded-3xl transition-all flex items-center justify-between">
                    <div className="flex items-center gap-4">
                      <div className="w-10 h-10 bg-white rounded-xl flex items-center justify-center text-slate-400 group-hover:text-indigo-600 shadow-sm">
                        <Clock className="w-5 h-5" />
                      </div>
                      <div>
                        <p className="text-sm font-black text-slate-800">Snapshot_{getSnapshotId(snapshot).substring(0, 8)}</p>
                        <p className="text-[10px] font-bold text-slate-400 uppercase tracking-widest">{formatDate(snapshot.started_at)}</p>
                      </div>
                    </div>
                    <div className="flex items-center gap-4">
                      <div className="text-right">
                        <p className="text-xs font-black text-slate-700">{formatBytes(snapshot.total_bytes)}</p>
                        <p className="text-[9px] font-bold text-slate-400 uppercase">{snapshot.status}</p>
                      </div>
                      <button
                        type="button"
                        onClick={() => onBrowseHistory(getSnapshotId(snapshot))}
                        title="Telusuri Isi Snapshot"
                        className="p-2 bg-white rounded-lg border border-slate-200 text-slate-400 hover:text-indigo-600 hover:border-indigo-100 opacity-0 group-hover:opacity-100 transition-all"
                      >
                        <ExternalLink className="w-4 h-4" />
                      </button>
                    </div>
                  </div>
                ))}

                {!loadingHistory && snapshots.length === 0 && (
                  <div className="flex-1 flex flex-col items-center justify-center text-slate-300 py-20">
                    <History className="w-16 h-16 mb-4 opacity-10" />
                    <p className="text-xs font-black uppercase tracking-widest">No Backups Yet</p>
                  </div>
                )}
              </div>

              <div className="pt-8 mt-auto border-t border-slate-100 flex justify-center">
                <button
                  type="button"
                  onClick={() => onNavigate?.('history')}
                  className="text-xs font-black text-indigo-600 uppercase tracking-widest hover:underline active:scale-95"
                >
                  View All Snapshots &rarr;
                </button>
              </div>
            </section>

            {/* Quick Actions Grid */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              <ActionCard
                title="Live File System"
                desc="Browse and manage files on device storage"
                icon={HardDrive}
                color="text-emerald-600"
                bgColor="bg-emerald-50"
                onClick={() => onNavigate?.('files')}
              />
              <ActionCard
                title="Hardware Health"
                desc="Run diagnostic tests on battery and CPU"
                icon={Activity}
                color="text-indigo-600"
                bgColor="bg-indigo-50"
                onClick={handleRefreshHardware}
              />
            </div>
          </div>

        </div>
      </div>
    </div>
  );
}

function SpecRow({ label, value, mono }: { label: string, value: string, mono?: boolean }) {
  return (
    <div className="flex justify-between items-center py-3 border-b border-slate-50 last:border-0">
      <span className="text-xs font-bold text-slate-400">{label}</span>
      <span className={cn("text-xs font-black text-slate-700", mono && "font-mono")}>{value}</span>
    </div>
  );
}

function ActionCard({ title, desc, icon: Icon, color, bgColor, onClick }: {
  title: string,
  desc: string,
  icon: any,
  color: string,
  bgColor: string,
  onClick?: () => void
}) {
  return (
    <div
      onClick={onClick}
      className="bg-white p-6 rounded-[32px] border border-slate-100 shadow-sm hover:shadow-md transition-all flex items-start gap-5 group cursor-pointer active:scale-95"
    >
      <div className={cn("w-12 h-12 rounded-2xl flex items-center justify-center shrink-0 transition-transform group-hover:scale-110", bgColor)}>
        <Icon className={cn("w-6 h-6", color)} />
      </div>
      <div>
        <p className="font-black text-slate-900 group-hover:text-indigo-600 transition-colors">{title}</p>
        <p className="text-xs text-slate-500 font-medium mt-1 leading-relaxed">{desc}</p>
      </div>
    </div>
  );
}
