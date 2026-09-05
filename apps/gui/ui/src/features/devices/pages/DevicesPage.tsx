import { useState } from "react";
import { Tablet, RefreshCcw, ShieldCheck, Activity, Zap, Cpu, HelpCircle } from "lucide-react";
import { useDevices } from "../hooks/useDevices";
import { DeviceCard } from "../components/DeviceCard";
import { ConnectionGuideModal } from "../components/ConnectionGuideModal";
import { getDeviceId, type Device } from "@/services/deviceService";
import { cn } from "../../../shared/lib/utils";

interface DevicesPageProps {
  onDeviceDetails?: (device: Device) => void;
}

export function DevicesPage({ onDeviceDetails }: DevicesPageProps) {
  const { devices, loading, error, refreshDevices } = useDevices();
  const [showGuide, setShowGuide] = useState(false);

  return (
    <div className="p-6 md:p-8 space-y-8 max-w-7xl mx-auto animate-in fade-in duration-300">
      
      {/* Top Banner Overview */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 bg-white p-6 md:p-8 rounded-[32px] border border-slate-100 shadow-sm">
        <div>
          <h1 className="text-2xl md:text-3xl font-black text-slate-900 tracking-tight">
            Perangkat Terkoneksi
          </h1>
          <p className="text-xs text-slate-500 font-medium mt-1">
            Kelola dan pantau status koneksi, baterai, dan kapasitas penyimpanan ponsel Android Anda.
          </p>
        </div>

        <div className="flex items-center gap-2">
          <button
            type="button"
            onClick={() => setShowGuide(true)}
            className="px-4 py-3 bg-indigo-50 hover:bg-indigo-100 border border-indigo-100 rounded-2xl text-indigo-700 transition-all shadow-sm flex items-center gap-2 active:scale-95 shrink-0 font-black text-xs uppercase tracking-wider"
          >
            <HelpCircle className="w-4 h-4 text-indigo-600" />
            <span>Panduan Sambungkan HP</span>
          </button>

          <button
            type="button"
            onClick={refreshDevices}
            disabled={loading}
            className="px-5 py-3 bg-slate-50 hover:bg-slate-100 border border-slate-200 rounded-2xl text-slate-700 hover:text-indigo-600 transition-all shadow-sm flex items-center gap-2 active:scale-95 disabled:opacity-50 shrink-0 font-black text-xs uppercase tracking-wider"
          >
            <RefreshCcw className={cn("w-4 h-4", loading && "animate-spin text-indigo-600")} />
            <span>Pindai Ulang</span>
          </button>
        </div>
      </div>

      {error && (
        <div className="p-6 bg-red-50 border border-red-200/80 rounded-[32px] flex items-center gap-4 text-red-700">
          <ShieldCheck className="w-7 h-7 text-red-500 shrink-0" />
          <div>
            <p className="font-black uppercase tracking-widest text-[10px]">Koneksi Gagal</p>
            <p className="text-xs font-bold mt-0.5">{error}</p>
          </div>
        </div>
      )}

      {/* Devices Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
        {devices.map(device => (
          <DeviceCard
            key={getDeviceId(device)}
            device={device}
            onDetails={onDeviceDetails}
          />
        ))}

        {devices.length === 0 && !loading && (
          <div className="col-span-full py-20 flex flex-col items-center justify-center bg-white rounded-[32px] border-2 border-dashed border-slate-200 text-slate-400 p-8 space-y-4">
            <div className="w-16 h-16 bg-slate-50 rounded-3xl flex items-center justify-center shadow-sm">
              <Tablet className="w-8 h-8 opacity-20" />
            </div>
            <div className="text-center space-y-1">
              <p className="font-black uppercase tracking-widest text-xs text-slate-700">Tidak ada perangkat terdeteksi</p>
              <p className="text-xs text-slate-400 max-w-sm">
                Hubungkan ponsel Android dengan kabel USB dan pastikan USB Debugging aktif.
              </p>
            </div>
            <button
              onClick={() => setShowGuide(true)}
              className="px-5 py-2.5 bg-indigo-600 hover:bg-indigo-500 text-white rounded-xl text-xs font-black shadow-lg shadow-indigo-600/20 transition-all flex items-center gap-2"
            >
              <HelpCircle className="w-4 h-4" />
              Buka Petunjuk Sambungkan HP
            </button>
          </div>
        )}
      </div>

      <ConnectionGuideModal isOpen={showGuide} onClose={() => setShowGuide(false)} />

      {/* ADB & MTP Subsystem Status Summary */}
      {devices.length > 0 && (
        <section className="space-y-4 pt-4">
          <h2 className="text-lg font-black text-slate-900 tracking-tight">
            Ringkasan Subsistem Mesin
          </h2>
          <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
            <StatusMetric icon={Activity} label="ADB Server" value="Running" color="text-emerald-600" />
            <StatusMetric icon={Zap} label="Engine Mode" value="Production" color="text-indigo-600" />
            <StatusMetric icon={Cpu} label="Bridge Protocol" value="v1.0.41 ADB" color="text-slate-700" />
            <StatusMetric icon={ShieldCheck} label="Keamanan" value="Age X25519" color="text-emerald-600" />
          </div>
        </section>
      )}
    </div>
  );
}

function StatusMetric({ icon: Icon, label, value, color }: { icon: any, label: string, value: string, color: string }) {
  return (
    <div className="bg-white p-5 rounded-[28px] border border-slate-100 flex items-center gap-4 shadow-sm">
      <div className="w-11 h-11 bg-slate-50 rounded-2xl flex items-center justify-center shrink-0">
        <Icon className="w-5 h-5 text-slate-400" />
      </div>
      <div className="min-w-0">
        <p className="text-[10px] font-black text-slate-400 uppercase tracking-widest truncate">{label}</p>
        <p className={cn("text-sm font-black truncate mt-0.5", color)}>{value}</p>
      </div>
    </div>
  );
}
