import { Info, HardDrive, AlertTriangle, RefreshCw } from 'lucide-react';
import type { Device } from '@/services/deviceService';
import { formatBytes } from '@/shared/lib/formatters';
import { cn } from '@/shared/lib/utils';

interface DeviceSpecsPanelProps {
  device: Device;
  batteryStatus: [number, number] | null;
  isRefreshingSpecs: boolean;
  onRefreshHardware: () => void;
}

export function DeviceSpecsPanel({
  device,
  batteryStatus,
  isRefreshingSpecs,
  onRefreshHardware
}: DeviceSpecsPanelProps) {
  const storagePercent = device.storage_total_bytes > 0
    ? Math.round((Number(device.storage_used_bytes) / Number(device.storage_total_bytes)) * 100)
    : 0;

  return (
    <div className="lg:col-span-1 space-y-6">
      <section className="bg-white p-6 md:p-8 rounded-[32px] border border-slate-100 shadow-sm space-y-6">
        <div className="flex items-center justify-between">
          <h2 className="text-sm font-black text-slate-900 uppercase tracking-widest flex items-center gap-2">
            <Info className="w-4 h-4 text-indigo-600" /> Spesifikasi Hardware
          </h2>
          <button
            type="button"
            onClick={onRefreshHardware}
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
  );
}

function SpecRow({ label, value, mono }: { label: string; value: string; mono?: boolean }) {
  return (
    <div className="flex justify-between items-center py-2.5 border-b border-slate-50 last:border-0">
      <span className="text-xs font-bold text-slate-400">{label}</span>
      <span className={cn("text-xs font-black text-slate-700", mono && "font-mono")}>{value}</span>
    </div>
  );
}
