import { Tablet, RefreshCcw, ShieldCheck, Activity, Zap, Cpu } from "lucide-react";
import { useDevices } from "../hooks/useDevices";
import { DeviceCard } from "../components/DeviceCard";
import { getDeviceId, type Device } from "@/services/deviceService";
import { cn } from "../../../shared/lib/utils";

interface DevicesPageProps {
  onDeviceDetails?: (device: Device) => void;
}

export function DevicesPage({ onDeviceDetails }: DevicesPageProps) {
  const { devices, loading, error, refreshDevices } = useDevices();

  return (
    <div className="p-8 space-y-8 animate-in fade-in duration-500">
      <header className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-black text-slate-900 tracking-tight">Perangkat Terkoneksi</h1>
          <p className="text-slate-500 font-medium">Kelola dan pantau status hardware perangkat Android Anda.</p>
        </div>
        <button
          onClick={refreshDevices}
          disabled={loading}
          className="p-3 bg-white border border-slate-200 rounded-2xl text-slate-600 hover:text-indigo-600 hover:border-indigo-100 transition-all shadow-sm flex items-center gap-2 group"
        >
          <RefreshCcw className={cn("w-5 h-5", loading && "animate-spin")} />
          <span className="text-xs font-black uppercase tracking-widest px-1">Pindai Ulang</span>
        </button>
      </header>

      {error && (
          <div className="p-6 bg-red-50 border border-red-100 rounded-[32px] flex items-center gap-4 text-red-600">
              <ShieldCheck className="w-8 h-8 opacity-50" />
              <div>
                  <p className="font-black uppercase tracking-widest text-xs">Koneksi Gagal</p>
                  <p className="text-sm font-bold">{error}</p>
              </div>
          </div>
      )}

      <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
        {devices.map(device => (
          <DeviceCard
            key={getDeviceId(device)}
            device={device}
            onDetails={onDeviceDetails}
          />
        ))}

        {devices.length === 0 && !loading && (
          <div className="col-span-full py-32 flex flex-col items-center justify-center bg-slate-50 rounded-[40px] border-2 border-dashed border-slate-200 text-slate-400">
              <div className="w-20 h-20 bg-white rounded-3xl flex items-center justify-center shadow-sm mb-6">
                <Tablet className="w-10 h-10 opacity-20" />
              </div>
              <p className="font-black uppercase tracking-widest text-xs">Tidak ada perangkat</p>
              <p className="text-xs mt-1 text-slate-400">Hubungkan HP melalui USB untuk memulai.</p>
          </div>
        )}
      </div>

      {devices.length > 0 && (
          <section className="mt-12 space-y-6">
              <h2 className="text-xl font-black text-slate-800">Ringkasan Sistem ADB</h2>
              <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
                  <StatusMetric icon={Activity} label="ADB Server" value="Running" color="text-emerald-500" />
                  <StatusMetric icon={Zap} label="Mode" value="Production" color="text-indigo-500" />
                  <StatusMetric icon={Cpu} label="Bridge Version" value="1.0.41" color="text-slate-500" />
                  <StatusMetric icon={ShieldCheck} label="Security" value="Encrypted" color="text-emerald-500" />
              </div>
          </section>
      )}
    </div>
  );
}

function StatusMetric({ icon: Icon, label, value, color }: { icon: any, label: string, value: string, color: string }) {
    return (
        <div className="bg-white p-5 rounded-3xl border border-slate-100 flex items-center gap-4 shadow-sm">
            <div className="w-10 h-10 bg-slate-50 rounded-xl flex items-center justify-center">
                <Icon className="w-5 h-5 text-slate-400" />
            </div>
            <div>
                <p className="text-[10px] font-black text-slate-400 uppercase tracking-widest">{label}</p>
                <p className={cn("text-sm font-black", color)}>{value}</p>
            </div>
        </div>
    );
}
