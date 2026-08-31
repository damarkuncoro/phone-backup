import { Smartphone, CheckCircle2, ArrowRight, Loader2, HardDrive, Usb, Wifi } from 'lucide-react';
import { getDeviceId, type Device } from '@/services/deviceService';
import { cn } from "@/shared/lib/utils";
import { formatBytes } from '@/shared/lib/formatters';
import { UI_TOKENS } from '@/shared/theme/tokens';

interface WizardDeviceStepProps {
  devices: Device[];
  devicesLoading: boolean;
  selectedDevice: Device | null;
  onSelectDevice: (device: Device) => void;
  onNext: () => void;
}

export function WizardDeviceStep({
  devices,
  devicesLoading,
  selectedDevice,
  onSelectDevice,
  onNext
}: WizardDeviceStepProps) {
  return (
    <div className="p-6 md:p-8 space-y-6 animate-in fade-in duration-200 flex-1 flex flex-col justify-between">
      <div className="space-y-6">
        <div className="flex justify-between items-end">
          <div>
            <h2 className="text-xl font-black text-slate-900 tracking-tight">
              Pilih Perangkat Sumber
            </h2>
            <p className="text-xs text-slate-400 font-medium mt-0.5">
              Pilih ponsel atau tablet Android yang ingin dicadangkan datanya.
            </p>
          </div>
        </div>

        {devicesLoading ? (
          <div className="flex flex-col items-center justify-center py-20 gap-3">
            <Loader2 className="w-10 h-10 text-indigo-600 animate-spin" />
            <p className="text-slate-400 font-bold uppercase tracking-widest text-xs">Memindai Sambungan Perangkat...</p>
          </div>
        ) : devices.length === 0 ? (
          <div className={UI_TOKENS.emptyState}>
            <Smartphone className="w-12 h-12 text-slate-300 mx-auto" />
            <h3 className="text-sm font-black text-slate-700">Tidak Ada Perangkat Terhubung</h3>
            <p className="text-xs text-slate-400 max-w-md mx-auto text-center">
              Colokkan ponsel Anda menggunakan kabel USB (pilih mode Transfer File) atau aktifkan USB Debugging.
            </p>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {devices.map(device => {
              const devId = getDeviceId(device);
              const isSelected = selectedDevice && getDeviceId(selectedDevice) === devId;
              const isMtp = device.connection_type === 'Mtp';

              return (
                <div
                  key={devId}
                  onClick={() => onSelectDevice(device)}
                  className={cn(
                    "p-6 rounded-[28px] border-2 transition-all cursor-pointer flex items-start gap-4 relative overflow-hidden group select-none",
                    isSelected
                      ? "border-indigo-500 bg-indigo-50/40 shadow-md ring-2 ring-indigo-500/10"
                      : "border-slate-100 hover:border-indigo-200 hover:shadow-md bg-white"
                  )}
                >
                  <div className={cn(
                    "w-12 h-12 rounded-2xl flex items-center justify-center shrink-0 transition-transform group-hover:scale-105",
                    isSelected ? "bg-indigo-600 text-white shadow-lg shadow-indigo-200" : "bg-slate-100 text-slate-500"
                  )}>
                    <Smartphone className="w-6 h-6" />
                  </div>

                  <div className="flex-1 min-w-0">
                    <div className="flex items-center justify-between gap-2">
                      <h3 className="font-black text-slate-900 text-base truncate" title={device.model}>
                        {device.model}
                      </h3>
                      {isSelected && (
                        <CheckCircle2 className="w-5 h-5 text-indigo-600 shrink-0" />
                      )}
                    </div>

                    <div className="flex items-center gap-2 mt-1">
                      <span className="text-[10px] font-bold text-slate-400 uppercase tracking-wider truncate">
                        {device.manufacturer}
                      </span>
                      <div className="w-1 h-1 bg-slate-200 rounded-full shrink-0" />
                      <span className="text-[10px] font-black text-indigo-500 shrink-0">
                        Android {device.os_version}
                      </span>
                    </div>

                    <div className="mt-3 flex items-center gap-2">
                      <span className={cn(
                        "text-[9px] font-black px-2.5 py-0.5 rounded-md flex items-center gap-1",
                        isMtp ? "bg-cyan-50 text-cyan-700 border border-cyan-200" :
                        device.connection_type === 'Wifi' ? "bg-purple-50 text-purple-700 border border-purple-200" :
                        "bg-emerald-50 text-emerald-700 border border-emerald-200"
                      )}>
                        {isMtp ? <HardDrive className="w-3 h-3 text-cyan-600" /> : device.connection_type === 'Wifi' ? <Wifi className="w-3 h-3 text-purple-600" /> : <Usb className="w-3 h-3 text-emerald-600" />}
                        {isMtp ? "MTP (Kabel Biasa)" : device.connection_type === 'Wifi' ? "Wireless ADB" : "USB ADB"}
                      </span>

                      {device.storage_total_bytes > 0 && (
                        <span className="text-[10px] font-bold text-slate-400 font-mono">
                          {formatBytes(Number(device.storage_used_bytes))} / {formatBytes(Number(device.storage_total_bytes))}
                        </span>
                      )}
                    </div>
                  </div>
                </div>
              );
            })}
          </div>
        )}
      </div>

      {/* Step 1 Footer */}
      <div className="pt-6 border-t border-slate-100 flex justify-end">
        <button
          type="button"
          disabled={!selectedDevice}
          onClick={onNext}
          className="px-8 py-3.5 bg-indigo-600 hover:bg-indigo-700 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-lg shadow-indigo-200 hover:shadow-indigo-300 disabled:opacity-50 transition-all flex items-center gap-2.5 active:scale-95"
        >
          <span>Lanjutkan</span>
          <ArrowRight className="w-4 h-4" />
        </button>
      </div>
    </div>
  );
}
