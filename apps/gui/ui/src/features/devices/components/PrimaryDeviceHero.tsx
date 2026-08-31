import { Tablet, FolderOpen, ShieldCheck, ArrowRight } from "lucide-react";
import { type Device } from "@/services/deviceService";
import { formatBytes } from "@/shared/lib/formatters";
import { cn } from "@/shared/lib/utils";

interface PrimaryDeviceHeroProps {
  device: Device;
  onBrowseFiles?: () => void;
  onBackup?: (device: Device) => void;
}

export function PrimaryDeviceHero({
  device,
  onBrowseFiles,
  onBackup
}: PrimaryDeviceHeroProps) {
  const isMtp = device.connection_type === 'Mtp';
  const isWifi = device.connection_type === 'Wifi';

  return (
    <section className="bg-white rounded-[32px] border border-slate-100 p-6 md:p-8 shadow-sm flex flex-col lg:flex-row gap-6 lg:gap-10 items-center">
      <div className="w-36 h-36 md:w-44 md:h-44 bg-slate-50 rounded-3xl flex items-center justify-center border-2 border-dashed border-slate-200 shrink-0">
        <Tablet className="w-16 h-16 md:w-20 md:h-20 text-slate-300" />
      </div>

      <div className="flex-1 space-y-5 min-w-0 w-full">
        <div className="min-w-0">
          <div className="flex items-center gap-2">
            <span className={cn(
              "text-[9px] font-black px-2.5 py-0.5 rounded-full uppercase tracking-wider",
              isMtp
                ? "bg-cyan-100 text-cyan-800"
                : isWifi
                ? "bg-purple-100 text-purple-800"
                : "bg-emerald-100 text-emerald-800"
            )}>
              {isMtp ? 'MTP USB' : isWifi ? 'Wireless ADB' : 'USB ADB'}
            </span>
            <span className="text-xs font-bold text-slate-400 uppercase tracking-wider">
              Android {device.os_version}
            </span>
          </div>
          <h3 className="text-2xl md:text-3xl font-black text-slate-900 truncate mt-1" title={device.model}>
            {device.model}
          </h3>
          <p className="text-slate-400 font-mono text-xs truncate mt-0.5">
            Serial: {device.serial}
          </p>
        </div>

        <div className="space-y-1.5">
          <div className="flex justify-between text-xs font-bold">
            <span className="text-slate-500">Kapasitas Penyimpanan Terpakai</span>
            <span className="text-slate-900 font-mono">
              {formatBytes(Number(device.storage_used_bytes))} / {formatBytes(Number(device.storage_total_bytes))}
            </span>
          </div>
          <div className="h-2.5 w-full bg-slate-100 rounded-full overflow-hidden p-0.5">
            <div
              className="h-full bg-indigo-600 rounded-full transition-all duration-1000"
              style={{ width: `${Math.round((Number(device.storage_used_bytes) / Math.max(1, Number(device.storage_total_bytes))) * 100)}%` }}
            />
          </div>
        </div>

        <div className="flex flex-wrap gap-3 pt-1">
          {/* Distinct Action 1: Browse Files in File Manager */}
          <button
            type="button"
            onClick={onBrowseFiles}
            className="px-5 py-3 bg-white border border-slate-200 hover:border-slate-300 text-slate-700 rounded-2xl font-black text-xs uppercase tracking-wider transition-all flex items-center justify-center gap-2 flex-1 sm:flex-none active:scale-95 shadow-sm"
          >
            <FolderOpen className="w-4 h-4 text-indigo-600" />
            <span>Jelajahi Berkas</span>
          </button>

          {/* Distinct Action 2: Primary CTA Backup Now */}
          <button
            type="button"
            onClick={() => onBackup?.(device)}
            className="px-6 py-3 bg-indigo-600 hover:bg-indigo-700 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-lg shadow-indigo-200 hover:shadow-indigo-300 transition-all flex items-center justify-center gap-2 flex-1 sm:flex-none active:scale-95"
          >
            <ShieldCheck className="w-4 h-4" />
            <span>Backup Now</span>
            <ArrowRight className="w-4 h-4" />
          </button>
        </div>
      </div>
    </section>
  );
}
