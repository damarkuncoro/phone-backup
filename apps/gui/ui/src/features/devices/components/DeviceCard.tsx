import { type Device, getDeviceId } from "@/services/deviceService";
import { cn } from "../../../shared/lib/utils";
import { useState, useEffect } from "react";
import { safeListen } from "@/shared/lib/ipc";
import { Battery, Smartphone, Usb, Wifi, HardDrive } from "lucide-react";
import { formatBytes } from "@/shared/lib/formatters";

interface DeviceCardProps {
  device: Device;
  isSelected?: boolean;
  onSelect?: (device: Device) => void;
  onDetails?: (device: Device) => void;
  onQuickBackup?: (device: Device) => void;
}

interface DeviceStatus {
  battery_level: number;
  temperature: number;
}

export function DeviceCard({ device, isSelected, onSelect, onDetails, onQuickBackup }: DeviceCardProps) {
  const [status, setStatus] = useState<DeviceStatus | null>(null);
  const deviceId = getDeviceId(device);

  const storageUsed = formatBytes(Number(device.storage_used_bytes));
  const storageTotal = formatBytes(Number(device.storage_total_bytes));
  const storagePercent = device.storage_total_bytes > 0
    ? Math.round((Number(device.storage_used_bytes) / Number(device.storage_total_bytes)) * 100)
    : 0;

  useEffect(() => {
    return safeListen('device-status-update', (event) => {
        const payload = event.payload as any;
        if (payload.device_id === deviceId) {
            setStatus({
                battery_level: payload.battery_level,
                temperature: payload.temperature
            });
        }
    });
  }, [deviceId]);

  return (
    <div
      onClick={() => onSelect?.(device)}
      className={cn(
        "group relative flex flex-col p-5 rounded-[32px] border transition-all cursor-pointer overflow-hidden",
        isSelected
          ? "bg-white border-indigo-200 shadow-xl shadow-indigo-100/50 ring-2 ring-indigo-500/10"
          : "bg-white border-slate-100 hover:border-indigo-100 hover:shadow-lg shadow-sm"
      )}
    >
      {/* Active Indicator Dot */}
      {isSelected && (
          <div className="absolute top-4 right-4 w-2 h-2 rounded-full bg-indigo-500 animate-pulse" />
      )}

      <div className="flex items-start gap-4 mb-6">
        <div className={cn(
          "w-12 h-12 rounded-2xl flex items-center justify-center shrink-0 transition-transform duration-500 group-hover:scale-110",
          isSelected ? "bg-indigo-600 text-white shadow-lg shadow-indigo-200" : "bg-slate-50 text-slate-400"
        )}>
          <Smartphone className="w-6 h-6" />
        </div>

        <div className="flex-1 min-w-0">
            <h3 className="font-black text-slate-900 text-lg leading-tight truncate" title={device.model}>
                {device.model}
            </h3>
            <div className="flex items-center gap-2 mt-1">
                <span className="text-[10px] font-bold text-slate-400 uppercase tracking-wider truncate shrink-0">
                    {device.manufacturer}
                </span>
                <div className="w-1 h-1 bg-slate-200 rounded-full shrink-0" />
                <span className="text-[10px] font-black text-indigo-500 shrink-0">
                    A{device.os_version}
                </span>
            </div>
        </div>
      </div>

      <div className="grid grid-cols-2 gap-3 mb-6">
          <StatusChip
            icon={device.connection_type === 'Mtp' ? HardDrive : device.connection_type === 'Usb' ? Usb : Wifi}
            label={device.connection_type === 'Mtp' ? 'MTP (USB)' : device.connection_type}
            value={device.connection_type === 'Mtp' ? 'Plug & Play' : 'Active'}
            color={device.connection_type === 'Mtp' ? 'text-cyan-600' : 'text-emerald-600'}
            bgColor={device.connection_type === 'Mtp' ? 'bg-cyan-50' : 'bg-emerald-50'}
          />
          <StatusChip
            icon={Battery}
            label="Battery"
            value={status ? `${status.battery_level}%` : (device.connection_type === 'Mtp' ? 'USB Power' : '--%')}
            color={status && status.battery_level < 20 ? "text-red-600" : "text-slate-600"}
            bgColor={status && status.battery_level < 20 ? "bg-red-50" : "bg-slate-50"}
          />
      </div>

      <div className="mt-auto space-y-3">
        <div className="space-y-1.5">
          <div className="flex justify-between items-end px-1">
            <span className="text-[9px] font-black text-slate-400 uppercase tracking-widest">Storage Efficiency</span>
            <span className="text-[10px] font-bold text-slate-600">{storageUsed} / {storageTotal}</span>
          </div>
          <div className="h-2 w-full bg-slate-100 rounded-full overflow-hidden p-0.5">
            <div
              className={cn(
                "h-full rounded-full transition-all duration-1000 ease-out",
                isSelected ? "bg-indigo-600" : "bg-indigo-400",
                storagePercent > 90 ? "bg-red-500" : ""
              )}
              style={{ width: `${storagePercent}%` }}
            />
          </div>
        </div>

        <div className="flex gap-2 pt-1">
          <button
            onClick={(e) => { e.stopPropagation(); onDetails?.(device); }}
            className="flex-1 py-2.5 bg-slate-50 text-slate-600 rounded-xl text-[10px] font-black uppercase tracking-widest hover:bg-slate-100 transition-all border border-transparent hover:border-slate-200"
          >
            Details
          </button>
          <button
            onClick={(e) => { e.stopPropagation(); onQuickBackup?.(device); }}
            className={cn(
              "flex-[1.5] py-2.5 rounded-xl text-[10px] font-black uppercase tracking-widest transition-all shadow-md",
              isSelected ? "bg-indigo-600 text-white hover:bg-indigo-700 shadow-indigo-200" : "bg-white text-indigo-600 border border-indigo-100 hover:bg-indigo-50"
            )}
          >
            Quick Backup
          </button>
        </div>
      </div>
    </div>
  );
}

function StatusChip({ icon: Icon, label, value, color, bgColor }: {
    icon: any, label: string, value: string, color: string, bgColor: string
}) {
    return (
        <div className={cn("flex items-center gap-2 p-2 rounded-2xl border border-transparent", bgColor)}>
            <div className="w-7 h-7 rounded-lg bg-white/50 flex items-center justify-center shadow-sm shrink-0">
                <Icon className={cn("w-3.5 h-3.5", color)} />
            </div>
            <div className="min-w-0">
                <p className="text-[8px] font-black text-slate-400 uppercase tracking-tighter leading-none mb-0.5 truncate">{label}</p>
                <p className={cn("text-[10px] font-black leading-none truncate", color)}>{value}</p>
            </div>
        </div>
    );
}
