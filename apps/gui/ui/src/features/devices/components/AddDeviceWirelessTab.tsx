import { Wifi, Loader2, ArrowRight } from "lucide-react";

interface AddDeviceWirelessTabProps {
  ipAddress: string;
  setIpAddress: (ip: string) => void;
  port: string;
  setPort: (port: string) => void;
  connecting: boolean;
  onSubmit: (e: React.FormEvent) => void;
}

export function AddDeviceWirelessTab({
  ipAddress,
  setIpAddress,
  port,
  setPort,
  connecting,
  onSubmit,
}: AddDeviceWirelessTabProps) {
  return (
    <form onSubmit={onSubmit} className="space-y-5">
      <div className="p-4 bg-indigo-50/70 border border-indigo-100 rounded-2xl text-xs text-indigo-900 leading-relaxed font-medium space-y-1">
        <p className="font-bold flex items-center gap-1.5">
          <Wifi className="w-4 h-4 text-indigo-600" /> Syarat Koneksi Wireless ADB:
        </p>
        <p>1. Ponsel dan komputer harus berada dalam <b>jaringan WiFi yang sama</b>.</p>
        <p>2. Aktifkan <b>Wireless Debugging</b> di menu <i>Opsi Pengembang</i> ponsel Anda.</p>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
        <div className="sm:col-span-2 space-y-1.5">
          <label className="text-[10px] font-black uppercase tracking-widest text-slate-400">
            Alamat IP Ponsel
          </label>
          <input
            type="text"
            required
            placeholder="192.168.1.100"
            value={ipAddress}
            onChange={(e) => setIpAddress(e.target.value)}
            className="w-full bg-slate-50 border border-slate-200/80 px-4 py-3 rounded-2xl text-xs font-mono outline-none focus:ring-4 focus:ring-indigo-500/10 focus:border-indigo-300 transition-all"
          />
        </div>

        <div className="space-y-1.5">
          <label className="text-[10px] font-black uppercase tracking-widest text-slate-400">
            Port ADB
          </label>
          <input
            type="number"
            required
            placeholder="5555"
            value={port}
            onChange={(e) => setPort(e.target.value)}
            className="w-full bg-slate-50 border border-slate-200/80 px-4 py-3 rounded-2xl text-xs font-mono outline-none focus:ring-4 focus:ring-indigo-500/10 focus:border-indigo-300 transition-all"
          />
        </div>
      </div>

      <button
        type="submit"
        disabled={connecting}
        className="w-full py-3.5 bg-indigo-600 hover:bg-indigo-700 text-white rounded-2xl font-black text-xs uppercase tracking-wider transition-all shadow-lg shadow-indigo-200 flex items-center justify-center gap-2 active:scale-95 disabled:opacity-50"
      >
        {connecting ? <Loader2 className="w-4 h-4 animate-spin" /> : <ArrowRight className="w-4 h-4" />}
        {connecting ? "Menghubungkan ke Perangkat..." : "Sambungkan Nirkabel"}
      </button>
    </form>
  );
}
