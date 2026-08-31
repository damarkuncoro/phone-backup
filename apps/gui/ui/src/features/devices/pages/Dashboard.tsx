import { Tablet, Database, Cpu, Loader2, HardDrive, Activity } from "lucide-react";
import { useDevices } from "../hooks/useDevices";
import { DeviceCard } from "../components/DeviceCard";
import { StatCard } from "../components/StatCard";
import { PrimaryDeviceHero } from "../components/PrimaryDeviceHero";
import { useState, useEffect } from "react";
import { getDeviceId, type Device } from "@/services/deviceService";
import { backupService } from "@/services/backupService";
import { UI_TOKENS } from "@/shared/theme/tokens";

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
    <div className={UI_TOKENS.layout.pageContainer}>
      
      {/* Top Banner / Hero Overview */}
      <div className={UI_TOKENS.card.heroBannerDark}>
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
        <PrimaryDeviceHero
          device={selectedDevice}
          onBrowseFiles={() => onNavigate?.('files')}
          onBackup={(d) => onBackupClick?.(d)}
        />
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
            <div className={UI_TOKENS.emptyState}>
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
