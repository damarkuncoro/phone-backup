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
    <div className="p-6 md:p-8 space-y-8 max-w-7xl mx-auto animate-in fade-in duration-300 relative">
      
      {/* Toast Notification */}
      {toastMessage && (
        <div className="fixed top-20 right-6 z-50 bg-slate-900 text-white px-5 py-3 rounded-2xl shadow-2xl border border-slate-700 flex items-center gap-3 text-xs font-bold animate-in slide-in-from-top-4 duration-300">
          <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0" />
          <span>{toastMessage}</span>
        </div>
      )}

      {/* Top Hero Card with Back & Actions */}
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

      {/* Main 2-Column Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">

        {/* Left Column: Specs & Storage */}
        <div className="lg:col-span-1 space-y-6">
          <section className="bg-white p-6 md:p-8 rounded-[32px] border border-slate-100 shadow-sm space-y-6">
            <div className="flex items-center justify-between">
              <h2 className="text-sm font-black text-slate-900 uppercase tracking-widest flex items-center gap-2">
                <Info className="w-4 h-4 text-indigo-600" /> Spesifikasi Hardware
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

            <div className="space-y-3">
              <SpecRow label="Serial Number" value={device.serial} mono />
              <SpecRow label="Tipe Koneksi" value={device.connection_type} />
              <SpecRow label="Daya Baterai" value={batteryStatus ? `${batteryStatus[0]}%` : '---'} />
              <SpecRow label="Suhu Perangkat" value={batteryStatus ? `${batteryStatus[1]}°C` : '---'} />
              <SpecRow label="Versi Android" value={`Android ${device.os_version}`} />
            </div>
          </section>

          <section className="bg-white p-6 md:p-8 rounded-[32px] border border-slate-100 shadow-sm space-y-5">
            <h2 className="text-sm font-black text-slate-900 uppercase tracking-widest flex items-center gap-2">
              <HardDrive className="w-4 h-4 text-indigo-600" /> Kapasitas Memori
            </h2>
            <div className="space-y-3">
              <div className="flex justify-between items-end">
                <p className="text-xs font-bold text-slate-500">Persentase Terpakai</p>
                <p className="text-sm font-black text-slate-900">{storagePercent}%</p>
              </div>
              <div className="h-3 w-full bg-slate-100 rounded-full overflow-hidden p-0.5">
                <div
                  className={cn(
                    "h-full rounded-full transition-all duration-1000",
                    storagePercent > 90 ? "bg-red-500" : "bg-indigo-600"
                  )}
                  style={{ width: `${storagePercent}%` }}
                />
              </div>
              <div className="grid grid-cols-2 gap-4 pt-2">
                <div className="p-3 bg-slate-50 rounded-2xl border border-slate-100">
                  <p className="text-[10px] font-black text-slate-400 uppercase">Terpakai</p>
                  <p className="text-sm font-black text-slate-800 font-mono mt-0.5">{formatBytes(Number(device.storage_used_bytes))}</p>
                </div>
                <div className="p-3 bg-slate-50 rounded-2xl border border-slate-100">
                  <p className="text-[10px] font-black text-slate-400 uppercase">Total</p>
                  <p className="text-sm font-black text-slate-800 font-mono mt-0.5">{formatBytes(Number(device.storage_total_bytes))}</p>
                </div>
              </div>
            </div>
          </section>

          {/* Health Alert if battery low or temp high */}
          {batteryStatus && (batteryStatus[0] < 20 || batteryStatus[1] > 40) && (
            <div className="bg-amber-50 border border-amber-200/70 p-5 rounded-[28px] flex gap-3.5 items-start">
              <AlertTriangle className="w-5 h-5 text-amber-600 shrink-0 mt-0.5" />
              <div>
                <p className="text-xs font-black text-amber-900 uppercase tracking-widest">Peringatan Hardware</p>
                <p className="text-xs text-amber-800/90 mt-1 leading-relaxed">
                  {batteryStatus[0] < 20 ? "Baterai HP rendah (<20%). Hubungkan ke charger sebelum backup. " : ""}
                  {batteryStatus[1] > 40 ? "Suhu perangkat tinggi (>40°C). Biarkan dingin beberapa saat." : ""}
                </p>
              </div>
            </div>
          )}
        </div>

        {/* Right Column: Snapshots & Actions */}
        <div className="lg:col-span-2 space-y-6">
          <section className="bg-white p-6 md:p-8 rounded-[32px] border border-slate-100 shadow-sm min-h-[460px] flex flex-col justify-between space-y-6">
            <div>
              <div className="flex items-center justify-between mb-6">
                <h2 className="text-lg font-black text-slate-900 tracking-tight flex items-center gap-2.5">
                  <History className="w-5 h-5 text-indigo-600" /> Riwayat Snapshot Perangkat
                </h2>
                {loadingHistory && <Clock className="w-4 h-4 text-indigo-500 animate-spin" />}
              </div>

              <div className="space-y-3">
                {snapshots.slice(0, 5).map(snapshot => (
                  <div
                    key={getSnapshotId(snapshot)}
                    onClick={() => onBrowseHistory(getSnapshotId(snapshot))}
                    className="group p-4 bg-slate-50 hover:bg-indigo-50/60 border border-slate-100 hover:border-indigo-100 rounded-2xl transition-all flex items-center justify-between cursor-pointer select-none"
                  >
                    <div className="flex items-center gap-3.5">
                      <div className="w-10 h-10 bg-white rounded-xl flex items-center justify-center text-slate-400 group-hover:text-indigo-600 shadow-sm shrink-0">
                        <Clock className="w-4 h-4" />
                      </div>
                      <div>
                        <p className="text-xs font-bold text-slate-800 group-hover:text-indigo-950">Snapshot_{getSnapshotId(snapshot).substring(0, 8)}</p>
                        <p className="text-[10px] font-medium text-slate-400 uppercase tracking-widest">{formatDate(snapshot.started_at)}</p>
                      </div>
                    </div>
                    <div className="flex items-center gap-4">
                      <div className="text-right">
                        <p className="text-xs font-mono font-bold text-slate-700">{formatBytes(snapshot.total_bytes)}</p>
                        <span className="text-[9px] font-black uppercase px-2 py-0.5 rounded-full bg-emerald-50 text-emerald-700">
                          {snapshot.status}
                        </span>
                      </div>
                      <ExternalLink className="w-4 h-4 text-slate-400 group-hover:text-indigo-600 transition-colors" />
                    </div>
                  </div>
                ))}

                {!loadingHistory && snapshots.length === 0 && (
                  <div className="flex flex-col items-center justify-center text-slate-300 py-16">
                    <History className="w-12 h-12 mb-3 opacity-20" />
                    <p className="text-xs font-black uppercase tracking-widest">Belum Ada Backup</p>
                  </div>
                )}
              </div>
            </div>

            <div className="pt-4 border-t border-slate-100 flex justify-center">
              <button
                type="button"
                onClick={() => onNavigate?.('history')}
                className="text-xs font-black text-indigo-600 uppercase tracking-wider hover:underline active:scale-95"
              >
                Lihat Semua Snapshot di Arsip Vault &rarr;
              </button>
            </div>
          </section>

          {/* Quick Actions Grid */}
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <ActionCard
              title="File Explorer Live"
              desc="Jelajahi berkas dan folder aktif di penyimpanan ponsel"
              icon={HardDrive}
              color="text-emerald-600"
              bgColor="bg-emerald-50"
              onClick={() => onNavigate?.('files')}
            />
            <ActionCard
              title="Diagnostik Hardware"
              desc="Uji kesehatan baterai, koneksi, dan temperatur sensor"
              icon={Activity}
              color="text-indigo-600"
              bgColor="bg-indigo-50"
              onClick={handleRefreshHardware}
            />
          </div>
        </div>

      </div>
    </div>
  );
}

function SpecRow({ label, value, mono }: { label: string, value: string, mono?: boolean }) {
  return (
    <div className="flex justify-between items-center py-2.5 border-b border-slate-50 last:border-0">
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
      className="bg-white p-6 rounded-[32px] border border-slate-100 shadow-sm hover:shadow-md hover:border-indigo-100 transition-all flex items-start gap-4 group cursor-pointer active:scale-95 select-none"
    >
      <div className={cn("w-12 h-12 rounded-2xl flex items-center justify-center shrink-0 transition-transform group-hover:scale-105", bgColor)}>
        <Icon className={cn("w-6 h-6", color)} />
      </div>
      <div>
        <p className="font-black text-slate-900 text-sm group-hover:text-indigo-600 transition-colors">{title}</p>
        <p className="text-xs text-slate-500 font-medium mt-0.5 leading-relaxed">{desc}</p>
      </div>
    </div>
  );
}
