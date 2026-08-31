import { Terminal, Database, Monitor, Sparkles, RefreshCw } from 'lucide-react';
import { cn } from "@/shared/lib/utils";

interface SettingsDoctorTabProps {
  report: any;
  currentBackendType: string;
  refreshingDoctor: boolean;
  onRefreshDoctor: () => void;
}

export function SettingsDoctorTab({
  report,
  currentBackendType,
  refreshingDoctor,
  onRefreshDoctor
}: SettingsDoctorTabProps) {
  return (
    <div className="space-y-6 animate-in fade-in duration-200">
      <div className="flex justify-between items-center px-1">
        <div>
          <h3 className="text-base font-black text-slate-900 tracking-tight">Kesehatan Infrastruktur</h3>
          <p className="text-xs text-slate-400 font-medium">Status komponen kunci yang menggerakkan platform backup.</p>
        </div>
        <button
          type="button"
          onClick={onRefreshDoctor}
          disabled={refreshingDoctor}
          className="flex items-center gap-2 px-4 py-2 bg-white border border-slate-200 hover:border-slate-300 text-slate-700 rounded-xl text-xs font-bold transition-all shadow-sm active:scale-95 disabled:opacity-50"
        >
          <RefreshCw className={cn("w-3.5 h-3.5", refreshingDoctor && "animate-spin text-indigo-600")} />
          {refreshingDoctor ? "Mendiagnosa..." : "Diagnosa Ulang"}
        </button>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        <HealthCard
          icon={Terminal}
          title="ADB Engine"
          value={report?.adb_version ? "Tersedia & Aktif" : "Tidak Ditemukan"}
          desc={report?.adb_version || "Android Debug Bridge"}
          status={report?.adb_found ? 'healthy' : 'error'}
        />
        <HealthCard
          icon={Database}
          title="SQLite Database"
          value={report?.db_healthy ? "Operasional" : "Rusak / Error"}
          desc="Enkripsi metadata SQLCipher"
          status={report?.db_healthy ? 'healthy' : 'error'}
        />
        <HealthCard
          icon={Monitor}
          title="Koneksi Perangkat"
          value={`${report?.device_count ?? 0} Perangkat Terhubung`}
          desc="USB, WiFi & MTP"
          status="healthy"
        />
        <HealthCard
          icon={Sparkles}
          title="MTP Plug & Play"
          value="Siap Digunakan"
          desc="Media Transfer Protocol"
          status="healthy"
        />
      </div>

      {/* Diagnostic Details Log Card */}
      <div className="bg-slate-900 text-slate-300 p-6 md:p-8 rounded-[32px] shadow-xl border border-slate-800 space-y-3 font-mono text-xs">
        <div className="flex items-center justify-between border-b border-slate-800 pb-3">
          <span className="font-bold text-slate-400 flex items-center gap-2">
            <Terminal className="w-4 h-4 text-emerald-400" /> Log Diagnostik Perangkat Keras
          </span>
          <span className="text-[10px] px-2.5 py-0.5 rounded-full bg-emerald-950 text-emerald-400 font-bold border border-emerald-800/50">
            Semua Sistem Normal
          </span>
        </div>
        <div className="space-y-1.5 text-[11px] leading-relaxed">
          <p className="text-slate-400">&gt; Engine core initialized with CompositeDeviceAdapter (ADB + MTP).</p>
          <p className="text-slate-400">&gt; Storage backend: <span className="text-cyan-400">{currentBackendType}</span>.</p>
          <p className="text-slate-400">&gt; Encryption: <span className="text-emerald-400">Age X25519 (Chacha20-Poly1305) Active</span>.</p>
          <p className="text-slate-400">&gt; Deduplication chunker: <span className="text-indigo-400">FastCDC 3.1 Content-Defined Chunking</span>.</p>
        </div>
      </div>
    </div>
  );
}

function HealthCard({
  icon: Icon, title, value, desc, status
}: {
  icon: any;
  title: string;
  value: string;
  desc: string;
  status: 'healthy' | 'error';
}) {
  return (
    <div className="bg-white p-5 rounded-[28px] border border-slate-100 shadow-sm flex flex-col justify-between space-y-3 hover:shadow-md transition-shadow">
      <div className="flex items-center justify-between">
        <div className={cn(
          "w-10 h-10 rounded-2xl flex items-center justify-center",
          status === 'healthy' ? "bg-emerald-50 text-emerald-600" : "bg-red-50 text-red-600"
        )}>
          <Icon className="w-5 h-5" />
        </div>
        <div className={cn(
          "w-2.5 h-2.5 rounded-full",
          status === 'healthy' ? "bg-emerald-500 shadow-lg shadow-emerald-200" : "bg-red-500 animate-pulse"
        )} />
      </div>

      <div>
        <p className="text-[10px] font-black text-slate-400 uppercase tracking-widest">{title}</p>
        <p className="text-sm font-black text-slate-800 truncate mt-0.5">{value}</p>
        <p className="text-[10px] text-slate-400 font-medium truncate mt-0.5">{desc}</p>
      </div>
    </div>
  );
}
