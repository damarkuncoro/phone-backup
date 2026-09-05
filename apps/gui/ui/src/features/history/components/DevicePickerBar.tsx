import { Smartphone, Wifi, WifiOff } from 'lucide-react';
import { type Device, getDeviceId } from '@/services/deviceService';
import { cn } from '@/shared/lib/utils';

interface DevicePickerBarProps {
  devices: Device[];
  selectedDeviceId: string | null;
  liveDeviceIds: Set<string>;
  onSelectDevice: (deviceId: string) => void;
}

export function DevicePickerBar({ devices, selectedDeviceId, liveDeviceIds, onSelectDevice }: DevicePickerBarProps) {
  return (
    <section className="bg-white p-2 rounded-[32px] border border-slate-100 shadow-sm flex items-center gap-2 overflow-x-auto no-scrollbar">
      {devices.map(d => {
        const isSelected = selectedDeviceId === getDeviceId(d);
        const isOnline = liveDeviceIds.has(getDeviceId(d));

        return (
          <button
            key={getDeviceId(d)}
            type="button"
            onClick={() => onSelectDevice(getDeviceId(d))}
            className={cn(
              "flex items-center gap-3 px-5 py-3.5 rounded-[24px] transition-all min-w-fit shrink-0 select-none",
              isSelected
                ? "bg-slate-900 text-white shadow-xl scale-[1.02] z-10"
                : "bg-transparent text-slate-400 hover:bg-slate-50 hover:text-slate-700"
            )}
          >
            <div className={cn(
              "w-9 h-9 rounded-xl flex items-center justify-center transition-colors relative shrink-0",
              isSelected ? "bg-indigo-600 text-white" : "bg-slate-100 text-slate-400"
            )}>
              <Smartphone className="w-4 h-4" />
              <div className={cn(
                "absolute -top-1 -right-1 w-3.5 h-3.5 rounded-full border-2 border-white flex items-center justify-center",
                isOnline ? "bg-emerald-500" : "bg-slate-300"
              )}>
                {isOnline ? <Wifi className="w-2 h-2 text-white" /> : <WifiOff className="w-2 h-2 text-white" />}
              </div>
            </div>
            <div className="text-left min-w-0">
              <p className="text-xs font-black leading-none truncate">{d.model}</p>
              <p className={cn("text-[9px] font-bold uppercase mt-1 tracking-wider", isSelected ? "text-indigo-300" : "text-slate-400")}>
                {isOnline ? 'Online' : 'Arsip'}
              </p>
            </div>
          </button>
        );
      })}
      {devices.length === 0 && (
        <div className="px-6 py-4 text-xs font-bold text-slate-300 italic uppercase tracking-widest">
          Tidak ada riwayat perangkat ditemukan.
        </div>
      )}
    </section>
  );
}
