import {
  Settings as SettingsIcon, Shield, Database, Activity,
  RefreshCw, Trash2, Key, Save, AlertCircle, CheckCircle2,
  HardDrive, Monitor, Terminal
} from 'lucide-react';
import { cn } from "../../../shared/lib/utils";
import { useSettings } from '../hooks/useSettings';

export function SettingsPage() {
  const {
    report,
    settings, setSettings,
    keys,
    loading,
    saving,
    msg,
    handleSave,
    runMaintenance
  } = useSettings();

  if (loading) {
    return (
      <div className="h-full flex flex-col items-center justify-center gap-4 text-slate-400">
        <RefreshCw className="w-8 h-8 animate-spin text-indigo-500" />
        <p className="text-[10px] font-black uppercase tracking-widest">Mendiagnosa Sistem...</p>
      </div>
    );
  }

  return (
    <div className="p-8 space-y-8 animate-in fade-in duration-500 max-w-5xl mx-auto">
      <header className="flex justify-between items-end">
        <div>
          <h1 className="text-3xl font-black text-slate-900 tracking-tight flex items-center gap-3">
            <SettingsIcon className="w-8 h-8 text-indigo-600" /> Pengaturan Sistem
          </h1>
          <p className="text-slate-500 font-medium">Konfigurasi parameter infrastruktur dan keamanan.</p>
        </div>

        {msg && (
            <div className={cn(
                "px-4 py-2 rounded-xl text-xs font-bold flex items-center gap-2 animate-in slide-in-from-top-2",
                msg.type === 'success' ? "bg-emerald-50 text-emerald-600" : "bg-red-50 text-red-600"
            )}>
                {msg.type === 'success' ? <CheckCircle2 className="w-4 h-4" /> : <AlertCircle className="w-4 h-4" />}
                {msg.text}
            </div>
        )}
      </header>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-8">

        {/* System Health Section */}
        <section className="bg-white p-8 rounded-[40px] border border-slate-100 shadow-sm space-y-6">
            <h2 className="text-sm font-black text-slate-900 uppercase tracking-widest flex items-center gap-2">
                <Activity className="w-4 h-4 text-indigo-500" /> System Doctor
            </h2>
            <div className="space-y-4">
                <HealthRow
                    label="ADB Server"
                    value={report?.adb_version || 'Not Found'}
                    status={report?.adb_found ? 'healthy' : 'error'}
                    icon={Terminal}
                />
                <HealthRow
                    label="Database Status"
                    value={report?.db_healthy ? 'SQLite Operational' : 'Corrupted'}
                    status={report?.db_healthy ? 'healthy' : 'error'}
                    icon={Database}
                />
                <HealthRow
                    label="Active Links"
                    value={`${report?.device_count} Perangkat`}
                    status="healthy"
                    icon={Monitor}
                />
            </div>
        </section>

        {/* Maintenance Section */}
        <section className="bg-white p-8 rounded-[40px] border border-slate-100 shadow-sm space-y-6">
            <h2 className="text-sm font-black text-slate-900 uppercase tracking-widest flex items-center gap-2">
                <HardDrive className="w-4 h-4 text-indigo-500" /> Pemeliharaan
            </h2>
            <div className="grid grid-cols-1 gap-3">
                <button
                    onClick={() => runMaintenance('gc')}
                    className="flex items-center justify-between p-4 bg-slate-50 hover:bg-indigo-50 rounded-2xl border border-slate-100 hover:border-indigo-100 transition-all group"
                >
                    <div className="flex items-center gap-3">
                        <RefreshCw className="w-5 h-5 text-slate-400 group-hover:text-indigo-600" />
                        <div className="text-left">
                            <p className="text-xs font-black text-slate-700 uppercase">Garbage Collection</p>
                            <p className="text-[10px] text-slate-400 font-medium">Bersihkan objek data yang tidak terpakai</p>
                        </div>
                    </div>
                </button>
                <button
                    onClick={() => runMaintenance('prune')}
                    className="flex items-center justify-between p-4 bg-slate-50 hover:bg-red-50 rounded-2xl border border-slate-100 hover:border-red-100 transition-all group"
                >
                    <div className="flex items-center gap-3">
                        <Trash2 className="w-5 h-5 text-slate-400 group-hover:text-red-500" />
                        <div className="text-left">
                            <p className="text-xs font-black text-slate-700 uppercase">Prune Failed</p>
                            <p className="text-[10px] text-slate-400 font-medium">Hapus record backup yang gagal/tidak lengkap</p>
                        </div>
                    </div>
                </button>
            </div>
        </section>

        {/* Security Section */}
        <section className="bg-white p-8 rounded-[40px] border border-slate-100 shadow-sm space-y-6 md:col-span-2">
            <div className="flex justify-between items-center">
                <h2 className="text-sm font-black text-slate-900 uppercase tracking-widest flex items-center gap-2">
                    <Shield className="w-4 h-4 text-indigo-500" /> Keamanan & Kunci
                </h2>
                <button className="flex items-center gap-2 px-4 py-2 bg-slate-900 text-white rounded-xl font-black text-[10px] uppercase tracking-widest hover:bg-slate-800 transition-all">
                    <Key className="w-3.5 h-3.5" /> Rotate Keys
                </button>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
                <div className="space-y-4">
                    <div className="flex items-center justify-between p-4 bg-slate-50 rounded-2xl border border-slate-100">
                        <div>
                            <p className="text-xs font-black text-slate-700 uppercase">Storage Engine</p>
                            <p className="text-[10px] text-slate-400 font-medium">
                                Menggunakan: {typeof settings?.storage_backend === 'string' ? settings.storage_backend : Object.keys(settings?.storage_backend || {})[0]}
                            </p>
                        </div>
                        <select
                            value={typeof settings?.storage_backend === 'string' ? settings.storage_backend : Object.keys(settings?.storage_backend || {})[0]}
                            onChange={(e) => {
                                const val = e.target.value;
                                setSettings(s => s ? {...s, storage_backend: val === 'Local' ? { Local: null } : { Mock: null }} : null)
                            }}
                            className="bg-white border border-slate-200 px-3 py-1.5 rounded-xl text-xs font-bold outline-none"
                        >
                            <option value="Local">Local Disk</option>
                            <option value="Mock">Mock Storage</option>
                        </select>
                    </div>

                    <div className="flex items-center justify-between p-4 bg-slate-50 rounded-2xl border border-slate-100">
                        <div>
                            <p className="text-xs font-black text-slate-700 uppercase">Enkripsi Snapshot</p>
                            <p className="text-[10px] text-slate-400 font-medium">Otomatis disegel dengan AES-256</p>
                        </div>
                        <div className="px-3 py-1.5 bg-emerald-50 text-emerald-600 rounded-xl text-[10px] font-black uppercase tracking-widest border border-emerald-100">Aktif</div>
                    </div>
                </div>

                <div className="p-6 bg-indigo-50/50 border border-indigo-100 rounded-3xl space-y-4">
                    <p className="text-[10px] font-black text-indigo-500 uppercase tracking-widest">Active Public Key</p>
                    <div className="bg-white p-4 rounded-xl border border-indigo-100 shadow-sm">
                        <code className="text-[10px] font-mono text-indigo-900 break-all leading-relaxed">
                            {keys ? keys[1] : 'Memuat Kunci...'}
                        </code>
                    </div>
                    <p className="text-[9px] text-indigo-400 font-medium leading-relaxed italic">
                        *Kunci ini digunakan untuk menyegel backup Anda. Simpan kunci privat Anda di tempat yang sangat aman.
                    </p>
                </div>
            </div>
        </section>
      </div>

      <div className="flex justify-center pt-8">
          <button
            disabled={saving}
            onClick={handleSave}
            className="px-12 py-4 bg-indigo-600 text-white rounded-[24px] font-black shadow-2xl shadow-indigo-200 hover:bg-indigo-700 disabled:opacity-50 transition-all flex items-center gap-3"
          >
            {saving ? <RefreshCw className="w-5 h-5 animate-spin" /> : <Save className="w-5 h-5" />}
            Simpan Perubahan
          </button>
      </div>
    </div>
  );
}

function HealthRow({ label, value, status, icon: Icon }: { label: string, value: string, status: 'healthy' | 'error', icon: any }) {
    return (
        <div className="flex items-center gap-4 p-4 bg-slate-50 rounded-2xl border border-slate-100">
            <div className={cn(
                "w-10 h-10 rounded-xl flex items-center justify-center",
                status === 'healthy' ? "bg-emerald-50 text-emerald-500" : "bg-red-50 text-red-500"
            )}>
                <Icon className="w-5 h-5" />
            </div>
            <div className="flex-1 min-w-0">
                <p className="text-[10px] font-black text-slate-400 uppercase tracking-widest">{label}</p>
                <p className="text-sm font-black text-slate-700 truncate">{value}</p>
            </div>
            <div className={cn(
                "w-2.5 h-2.5 rounded-full",
                status === 'healthy' ? "bg-emerald-500 shadow-lg shadow-emerald-200" : "bg-red-500 animate-pulse"
            )} />
        </div>
    );
}
