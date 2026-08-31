import { Tablet, Database, Cpu, Plus, Loader2 } from "lucide-react";
import { useDevices } from "../hooks/useDevices";
import { DeviceCard } from "../components/DeviceCard";
import { useState, useEffect } from "react";
import { getDeviceId, type Device } from "@/services/deviceService";
import { backupService } from "@/services/backupService";
import { formatBytes } from "@/shared/lib/formatters";

interface DashboardProps {
    onBackupClick?: () => void;
    onDeviceDetails?: (device: Device) => void;
}

export function Dashboard({ onBackupClick, onDeviceDetails }: DashboardProps) {
  const { devices, loading, error } = useDevices();
  const [selectedDeviceId, setSelectedDeviceId] = useState<string | null>(null);
  const [stats, setStats] = useState<{ total_logical_bytes: number, total_deduped_bytes: number, total_snapshots: number } | null>(null);

  useEffect(() => {
    async function loadStats() {
        try {
            const data = await backupService.getStorageStats();
            setStats(data);
        } catch (e) {
            console.warn("Stats load failed");
        }
    }
    loadStats();
  }, []);

  const selectedDevice = devices.find(d => getDeviceId(d) === selectedDeviceId) || devices[0];
  const dedupeRatio = stats && stats.total_deduped_bytes > 0
    ? (stats.total_logical_bytes / stats.total_deduped_bytes).toFixed(1)
    : "1.0";

  return (
    <div className="p-8 space-y-10 animate-in fade-in slide-in-from-bottom-4 duration-700">
      <header className="flex justify-between items-end gap-4">
        <div className="min-w-0">
          <h1 className="text-4xl font-black text-slate-900 tracking-tight truncate">Dashboard</h1>
          <p className="text-slate-500 font-medium mt-1 truncate">Manage your connected devices and backups.</p>
        </div>
        <button className="flex items-center gap-2 px-5 py-2.5 bg-slate-900 text-white rounded-2xl font-black text-xs hover:bg-slate-800 transition-all shadow-xl shadow-slate-200 shrink-0">
          <Plus className="w-4 h-4" />
          Add Device
        </button>
      </header>

      <section className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <StatCard title="Active Devices" value={devices.length.toString()} icon={Tablet} color="bg-indigo-600" />
        <StatCard title="Total Backups" value={stats?.total_snapshots.toString() || "0"} icon={Database} color="bg-emerald-600" />
        <StatCard title="Deduplication" value={`${dedupeRatio}x`} icon={Cpu} color="bg-amber-500" />
      </section>

      {selectedDevice && (
          <section className="bg-white rounded-[40px] border border-slate-100 p-8 md:p-10 shadow-sm flex flex-col lg:flex-row gap-8 lg:gap-12 items-center">
              <div className="w-40 h-40 md:w-48 md:h-48 bg-slate-50 rounded-3xl flex items-center justify-center border-2 border-dashed border-slate-200 shrink-0">
                  <Tablet className="w-20 h-20 md:w-24 md:h-24 text-slate-200" />
              </div>

              <div className="flex-1 space-y-6 min-w-0 w-full">
                  <div className="min-w-0">
                      <h3 className="text-3xl md:text-4xl font-black text-slate-900 truncate" title={selectedDevice.model}>
                        {selectedDevice.model}
                      </h3>
                      <p className="text-slate-400 font-mono text-sm truncate">
                        Serial: {selectedDevice.serial} | Android {selectedDevice.os_version}
                      </p>
                  </div>

                  <div className="space-y-2">
                      <div className="flex justify-between text-sm font-bold">
                          <span className="text-slate-500">Storage Capacity</span>
                          <span className="text-slate-900">
                            {formatBytes(Number(selectedDevice.storage_used_bytes))} / {formatBytes(Number(selectedDevice.storage_total_bytes))}
                          </span>
                      </div>
                      <div className="h-3 w-full bg-slate-100 rounded-full overflow-hidden p-0.5">
                          <div
                            className="h-full bg-indigo-600 rounded-full transition-all duration-1000"
                            style={{ width: `${Math.round((Number(selectedDevice.storage_used_bytes) / Number(selectedDevice.storage_total_bytes)) * 100)}%` }}
                          />
                      </div>
                  </div>

                  <div className="flex flex-wrap gap-4">
                      <button className="px-6 py-3 bg-indigo-600 text-white rounded-2xl font-black text-sm shadow-xl shadow-indigo-200 hover:bg-indigo-700 transition-all flex-1 md:flex-none">
                          Scan Device
                      </button>
                      <button
                        onClick={onBackupClick}
                        className="px-6 py-3 bg-white border border-slate-200 text-slate-600 rounded-2xl font-black text-sm hover:bg-slate-50 transition-all flex-1 md:flex-none"
                      >
                          Backup Now
                      </button>
                  </div>
              </div>
          </section>
      )}

      <section className="space-y-6">
        <div className="flex items-center justify-between">
            <h2 className="text-2xl font-black text-slate-900 tracking-tight">Connected Devices</h2>
            {loading && (
                <div className="flex items-center gap-2 text-xs font-bold text-indigo-500 animate-pulse">
                    <Loader2 className="w-3 h-3 animate-spin" />
                    Refreshing...
                </div>
            )}
        </div>

        {error && (
            <div className="p-4 bg-red-50 border border-red-100 rounded-2xl text-red-600 text-sm font-bold">
                Error loading devices: {error}
            </div>
        )}

        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
          {devices.map(device => {
            const id = getDeviceId(device);
            return (
              <DeviceCard
                  key={id}
                  device={device}
                  isSelected={getDeviceId(selectedDevice) === id}
                  onSelect={(d) => setSelectedDeviceId(getDeviceId(d))}
                  onDetails={onDeviceDetails}
              />
            );
          })}

          {devices.length === 0 && !loading && (
            <div className="col-span-full py-20 flex flex-col items-center justify-center bg-slate-50 rounded-3xl border-2 border-dashed border-slate-200 text-slate-400">
                <Tablet className="w-16 h-16 mb-4 opacity-20" />
                <p className="font-black uppercase tracking-widest text-xs">No devices connected</p>
                <p className="text-xs mt-1">Connect your phone via USB or WiFi to start.</p>
            </div>
          )}
        </div>
      </section>
    </div>
  );
}

function StatCard({ title, value, icon: Icon, color }: { title: string, value: string, icon: any, color: string }) {
  return (
    <div className="bg-white p-7 rounded-[32px] border border-slate-100 shadow-sm flex items-center gap-5 hover:shadow-md transition-all">
      <div className={`${color} w-14 h-14 rounded-2xl flex items-center justify-center text-white shadow-lg shrink-0`}>
        <Icon className="w-7 h-7" />
      </div>
      <div className="min-w-0">
        <p className="text-[10px] font-black text-slate-400 uppercase tracking-widest truncate">{title}</p>
        <p className="text-3xl font-black text-slate-900 tracking-tighter truncate">{value}</p>
      </div>
    </div>
  );
}
