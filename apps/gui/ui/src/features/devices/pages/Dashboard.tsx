import { Tablet, Database, Cpu, Loader2, ShieldCheck, HardDrive, FolderOpen, ArrowRight, Activity } from "lucide-react";
import { useDevices } from "../hooks/useDevices";
import { DeviceCard } from "../components/DeviceCard";
import { useState, useEffect } from "react";
import { getDeviceId, type Device } from "@/services/deviceService";
import { backupService } from "@/services/backupService";
import { formatBytes } from "@/shared/lib/formatters";
import { cn } from "@/shared/lib/utils";

interface DashboardProps {
  onBackupClick?: (device?: Device) => void;
  onDeviceDetails?: (device: Device) => void;
  onNavigate?: (tab: 'dashboard' | 'devices' | 'backup' | 'files' | 'history' | 'explorer' | 'settings') => void;
}

export function Dashboard({ onBackupClick, onDeviceDetails, onNavigate }: DashboardProps) {
  const { devices, loading, error } = useDevices();
  const [selectedDeviceId, setSelectedDeviceId] = useState<string | null>(null);
  const [stats, setStats] = useState<{ total_logical_bytes: number, total_deduped_bytes: number, total_snapshots: number } | null>(null);

  const loadStats = async () => {
    try {
      const data = await backupService.getStorageStats();
      setStats(data);
    } catch {
      console.warn("Stats load failed");
    }
  };

  useEffect(() => {
    loadStats();
  }, []);

  const selectedDevice = devices.find(d => getDeviceId(d) === selectedDeviceId) || devices[0];
  const dedupeRatio = stats && stats.total_deduped_bytes > 0
    ? (stats.total_logical_bytes / stats.total_deduped_bytes).toFixed(1)
    : "1.0";

  return (
    <div className="p-6 md:p-8 space-y-8 animate-in fade-in duration-300 max-w-7xl mx-auto relative">
      
      {/* Top Banner / Hero Overview (Clean without duplicate buttons) */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 bg-gradient-to-r from-slate-900 via-slate-900 to-indigo-950 text-white p-6 md:p-8 rounded-[32px] shadow-xl relative overflow-hidden">
        <div className="relative z-10 min-w-0">
          <span className="text-[10px] font-black uppercase tracking-widest text-indigo-400 bg-indigo-950/80 px-3 py-1 rounded-full border border-indigo-800/50">
            Pusat Kontrol Sistem
          </span>
          <h1 className="text-2xl md:text-3xl font-black tracking-tight mt-2 truncate">
            Dashboard
          </h1>
          <p className="text-xs text-slate-300 font-medium mt-1 truncate">
            Pantau status koneksi ponsel Android, riwayat deduplikasi, dan lakukan pencadangan data.
          </p>
        </div>

        {/* Live Status Pill in Banner */}
        <div className="relative z-10 flex items-center gap-3 shrink-0">
          <div className="flex items-center gap-2 px-4 py-2 bg-white/10 backdrop-blur-md rounded-2xl border border-white/10 text-xs font-bold text-slate-200">
            <Activity className="w-4 h-4 text-emerald-400" />
            <span>{devices.length} Perangkat Aktif</span>
          </div>
        </div>

        {/* Decorative Background Glow */}
        <div className="absolute -right-10 -bottom-10 w-64 h-64 bg-indigo-600/20 rounded-full blur-3xl pointer-events-none" />
      </div>

      {/* Stat Cards with Navigation */}
      <section className="grid grid-cols-1 md:grid-cols-3 gap-5">
        <StatCard
          title="Active Devices"
          value={devices.length.toString()}
          icon={Tablet}
          color="bg-indigo-600"
          subtitle="Buka File Manager"
          onClick={() => onNavigate?.('files')}
        />
        <StatCard
          title="Total Backups"
          value={stats?.total_snapshots.toString() || "0"}
          icon={Database}
          color="bg-emerald-600"
          subtitle="Buka Arsip Vault"
          onClick={() => onNavigate?.('history')}
        />
        <StatCard
          title="Deduplication"
          value={`${dedupeRatio}x`}
          icon={Cpu}
          color="bg-amber-500"
          subtitle="Efisiensi Ruang Disk"
        />
      </section>

      {/* Primary Device Hero Card */}
      {selectedDevice && (
        <section className="bg-white rounded-[32px] border border-slate-100 p-6 md:p-8 shadow-sm flex flex-col lg:flex-row gap-6 lg:gap-10 items-center">
          <div className="w-36 h-36 md:w-44 md:h-44 bg-slate-50 rounded-3xl flex items-center justify-center border-2 border-dashed border-slate-200 shrink-0">
            <Tablet className="w-16 h-16 md:w-20 md:h-20 text-slate-300" />
          </div>

          <div className="flex-1 space-y-5 min-w-0 w-full">
            <div className="min-w-0">
              <div className="flex items-center gap-2">
                <span className={cn(
                  "text-[9px] font-black px-2.5 py-0.5 rounded-full uppercase tracking-wider",
                  selectedDevice.connection_type === 'Mtp'
                    ? "bg-cyan-100 text-cyan-800"
                    : selectedDevice.connection_type === 'Wifi'
                    ? "bg-purple-100 text-purple-800"
                    : "bg-emerald-100 text-emerald-800"
                )}>
                  {selectedDevice.connection_type === 'Mtp' ? 'MTP USB' : selectedDevice.connection_type === 'Wifi' ? 'Wireless ADB' : 'USB ADB'}
                </span>
                <span className="text-xs font-bold text-slate-400 uppercase tracking-wider">
                  Android {selectedDevice.os_version}
                </span>
              </div>
              <h3 className="text-2xl md:text-3xl font-black text-slate-900 truncate mt-1" title={selectedDevice.model}>
                {selectedDevice.model}
              </h3>
              <p className="text-slate-400 font-mono text-xs truncate mt-0.5">
                Serial: {selectedDevice.serial}
              </p>
            </div>

            <div className="space-y-1.5">
              <div className="flex justify-between text-xs font-bold">
                <span className="text-slate-500">Kapasitas Penyimpanan Terpakai</span>
                <span className="text-slate-900 font-mono">
                  {formatBytes(Number(selectedDevice.storage_used_bytes))} / {formatBytes(Number(selectedDevice.storage_total_bytes))}
                </span>
              </div>
              <div className="h-2.5 w-full bg-slate-100 rounded-full overflow-hidden p-0.5">
                <div
                  className="h-full bg-indigo-600 rounded-full transition-all duration-1000"
                  style={{ width: `${Math.round((Number(selectedDevice.storage_used_bytes) / Math.max(1, Number(selectedDevice.storage_total_bytes))) * 100)}%` }}
                />
              </div>
            </div>

            <div className="flex flex-wrap gap-3 pt-1">
              {/* Distinct Action 1: Browse Files in File Manager */}
              <button
                type="button"
                onClick={() => onNavigate?.('files')}
                className="px-5 py-3 bg-white border border-slate-200 hover:border-slate-300 text-slate-700 rounded-2xl font-black text-xs uppercase tracking-wider transition-all flex items-center justify-center gap-2 flex-1 sm:flex-none active:scale-95 shadow-sm"
              >
                <FolderOpen className="w-4 h-4 text-indigo-600" />
                <span>Jelajahi Berkas</span>
              </button>

              {/* Distinct Action 2: Primary CTA Backup Now */}
              <button
                type="button"
                onClick={() => onBackupClick?.(selectedDevice)}
                className="px-6 py-3 bg-indigo-600 hover:bg-indigo-700 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-lg shadow-indigo-200 hover:shadow-indigo-300 transition-all flex items-center justify-center gap-2 flex-1 sm:flex-none active:scale-95"
              >
                <ShieldCheck className="w-4 h-4" />
                <span>Backup Now</span>
                <ArrowRight className="w-4 h-4" />
              </button>
            </div>
          </div>
        </section>
      )}

      {/* Connected Devices Grid */}
      <section className="space-y-4">
        <div className="flex items-center justify-between">
          <h2 className="text-xl font-black text-slate-900 tracking-tight">Perangkat Tersambung</h2>
          {loading && (
            <div className="flex items-center gap-2 text-xs font-bold text-indigo-500 animate-pulse">
              <Loader2 className="w-3.5 h-3.5 animate-spin" />
              <span>Memperbarui...</span>
            </div>
          )}
        </div>

        {error && (
          <div className="p-4 bg-red-50 border border-red-100 rounded-2xl text-red-600 text-xs font-bold">
            Gagal memuat perangkat: {error}
          </div>
        )}

        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-5">
          {devices.map(device => {
            const id = getDeviceId(device);
            return (
              <DeviceCard
                key={id}
                device={device}
                isSelected={getDeviceId(selectedDevice) === id}
                onSelect={(d) => setSelectedDeviceId(getDeviceId(d))}
                onDetails={onDeviceDetails}
                onQuickBackup={(d) => onBackupClick?.(d)}
              />
            );
          })}

          {devices.length === 0 && !loading && (
            <div className="col-span-full py-16 flex flex-col items-center justify-center bg-white rounded-[32px] border-2 border-dashed border-slate-200 text-slate-400 p-8 space-y-3">
              <HardDrive className="w-12 h-12 opacity-20 text-slate-400" />
              <p className="font-black uppercase tracking-widest text-xs">Belum Ada Perangkat Terhubung</p>
              <p className="text-xs text-slate-400 text-center max-w-sm">
                Colokkan ponsel Anda menggunakan kabel USB atau gunakan tombol Tambah Perangkat di header.
              </p>
            </div>
          )}
        </div>
      </section>

    </div>
  );
}

function StatCard({ title, value, icon: Icon, color, subtitle, onClick }: {
  title: string;
  value: string;
  icon: any;
  color: string;
  subtitle?: string;
  onClick?: () => void;
}) {
  return (
    <div
      onClick={onClick}
      className={cn(
        "bg-white p-6 rounded-[32px] border border-slate-100 shadow-sm flex items-center gap-5 transition-all select-none",
        onClick ? "cursor-pointer hover:shadow-lg hover:border-indigo-100 hover:scale-[1.01] active:scale-95 group" : "hover:shadow-md"
      )}
    >
      <div className={`${color} w-12 h-12 rounded-2xl flex items-center justify-center text-white shadow-lg shrink-0 transition-transform group-hover:scale-105`}>
        <Icon className="w-6 h-6" />
      </div>
      <div className="min-w-0">
        <p className="text-[10px] font-black text-slate-400 uppercase tracking-widest truncate">{title}</p>
        <p className="text-2xl font-black text-slate-900 tracking-tighter truncate mt-0.5">{value}</p>
        {subtitle && (
          <p className="text-[10px] font-bold text-indigo-500 mt-0.5 opacity-0 group-hover:opacity-100 transition-opacity truncate">
            {subtitle} &rarr;
          </p>
        )}
      </div>
    </div>
  );
}
