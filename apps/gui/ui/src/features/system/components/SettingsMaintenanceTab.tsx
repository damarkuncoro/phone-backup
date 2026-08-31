import { HardDrive, RefreshCw, Trash2 } from 'lucide-react';

interface SettingsMaintenanceTabProps {
  onRunMaintenance: (action: 'gc' | 'prune') => void;
}

export function SettingsMaintenanceTab({
  onRunMaintenance
}: SettingsMaintenanceTabProps) {
  return (
    <div className="space-y-6 animate-in fade-in duration-200">
      <div className="bg-white p-6 md:p-8 rounded-[32px] border border-slate-100 shadow-sm space-y-6">
        <div>
          <h3 className="text-base font-black text-slate-900 tracking-tight flex items-center gap-2">
            <HardDrive className="w-5 h-5 text-indigo-600" /> Pemeliharaan & Optimasi Ruang Disk
          </h3>
          <p className="text-xs text-slate-400 font-medium mt-0.5">
            Jalankan pembersihan berkala untuk membebaskan ruang disk dan menjaga konsistensi repositori.
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {/* GC Card */}
          <div className="p-6 bg-slate-50 border border-slate-200/80 rounded-3xl space-y-4 flex flex-col justify-between">
            <div className="space-y-2">
              <div className="w-10 h-10 rounded-2xl bg-indigo-50 text-indigo-600 flex items-center justify-center">
                <RefreshCw className="w-5 h-5" />
              </div>
              <h4 className="text-sm font-black text-slate-800">Garbage Collection (GC)</h4>
              <p className="text-xs text-slate-500 leading-relaxed font-medium">
                Memindai seluruh penyimpanan dan menghapus chunk deduplikasi yang sudah tidak lagi dirujuk oleh snapshot aktif mana pun.
              </p>
            </div>

            <button
              type="button"
              onClick={() => onRunMaintenance('gc')}
              className="w-full py-3 bg-indigo-600 hover:bg-indigo-700 text-white rounded-2xl font-black text-xs uppercase tracking-wider transition-all shadow-md shadow-indigo-200 flex items-center justify-center gap-2 active:scale-95"
            >
              <RefreshCw className="w-4 h-4" /> Jalankan Garbage Collection
            </button>
          </div>

          {/* Prune Failed Card */}
          <div className="p-6 bg-slate-50 border border-slate-200/80 rounded-3xl space-y-4 flex flex-col justify-between">
            <div className="space-y-2">
              <div className="w-10 h-10 rounded-2xl bg-rose-50 text-rose-600 flex items-center justify-center">
                <Trash2 className="w-5 h-5" />
              </div>
              <h4 className="text-sm font-black text-slate-800">Prune Failed Snapshots</h4>
              <p className="text-xs text-slate-500 leading-relaxed font-medium">
                Menghapus rekaman pencadangan yang terhenti atau gagal di tengah jalan agar tidak memenuhi riwayat arsip vault.
              </p>
            </div>

            <button
              type="button"
              onClick={() => onRunMaintenance('prune')}
              className="w-full py-3 bg-rose-600 hover:bg-rose-700 text-white rounded-2xl font-black text-xs uppercase tracking-wider transition-all shadow-md shadow-rose-200 flex items-center justify-center gap-2 active:scale-95"
            >
              <Trash2 className="w-4 h-4" /> Bersihkan Record Gagal
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
