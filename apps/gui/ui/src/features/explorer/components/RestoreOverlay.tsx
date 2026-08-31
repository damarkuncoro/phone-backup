import { CheckCircle2, Clock, Folder } from 'lucide-react';
import { cn } from "@/shared/lib/utils";
import { systemService } from '@/services/systemService';

interface RestoreOverlayProps {
  isOpen: boolean;
  progressPercent: number;
  progressMsg: string;
  eta?: string;
  onClose: () => void;
}

export function RestoreOverlay({
  isOpen,
  progressPercent,
  progressMsg,
  eta,
  onClose
}: RestoreOverlayProps) {
  if (!isOpen) return null;

  return (
    <div className="absolute inset-0 z-[100] bg-slate-900/95 backdrop-blur-md flex flex-col items-center justify-center text-center p-8 animate-in fade-in duration-300">
      <div className="relative mb-12">
        <div className={cn(
          "w-56 h-56 rounded-full border-8 border-white/5 transition-all duration-700",
          progressPercent < 100 ? "border-t-indigo-500 animate-spin" : "border-emerald-500 shadow-[0_0_50px_-12px_rgba(16,185,129,0.5)]"
        )} />
        <div className="absolute inset-0 flex flex-col items-center justify-center">
          {progressPercent < 100 ? (
            <>
              <span className="text-5xl font-black text-white leading-none tracking-tighter">{progressPercent}%</span>
              <span className="text-[10px] font-black text-indigo-400 uppercase tracking-[0.3em] mt-2 ml-1">Restoring</span>
            </>
          ) : (
            <CheckCircle2 className="w-20 h-26 text-emerald-500 animate-in zoom-in duration-500" />
          )}
        </div>
      </div>

      <div className="max-w-md w-full space-y-8">
        <div>
          <h2 className="text-3xl font-black text-white mb-3 tracking-tight">
            {progressPercent < 100 ? "Memulihkan Data" : "Restore Selesai!"}
          </h2>
          <div className="min-h-[40px] flex items-center justify-center">
            <p className="text-slate-400 text-sm font-medium px-6 leading-relaxed">
              {progressPercent < 100 ? progressMsg : (
                <>
                  Data telah dipulihkan ke direktori kerja Anda: <br/>
                  <span className="text-indigo-400 font-mono text-[10px] break-all bg-indigo-500/10 px-3 py-1.5 rounded-lg mt-3 inline-block border border-indigo-500/20">
                    workspace/restored_data
                  </span>
                </>
              )}
            </p>
          </div>
        </div>

        {progressPercent < 100 && (
          <div className="bg-white/5 p-5 rounded-[32px] border border-white/10 flex items-center justify-between shadow-2xl">
            <div className="flex items-center gap-4 text-left">
              <div className="w-10 h-10 bg-indigo-500/20 rounded-2xl flex items-center justify-center text-indigo-400 shadow-inner">
                <Clock className="w-5 h-5" />
              </div>
              <div>
                <p className="text-[9px] font-black text-slate-500 uppercase tracking-widest">Estimasi Sisa Waktu</p>
                <p className="text-sm font-black text-white leading-none mt-1">{eta || 'Menghitung...'}</p>
              </div>
            </div>
            <div className="w-px h-8 bg-white/10 mx-2" />
            <div className="text-right pr-2">
              <p className="text-[9px] font-black text-slate-500 uppercase tracking-widest">Status</p>
              <p className="text-sm font-black text-indigo-400 leading-none mt-1">Running</p>
            </div>
          </div>
        )}

        {progressPercent === 100 && (
          <div className="flex gap-4 animate-in slide-in-from-bottom-4 duration-500">
            <button
              onClick={() => systemService.openRestoreFolder()}
              className="flex-1 py-4 bg-indigo-600 text-white rounded-[24px] font-black text-xs uppercase tracking-widest hover:bg-indigo-700 transition-all shadow-2xl flex items-center justify-center gap-3 border-t border-white/20 active:scale-95"
            >
              <Folder className="w-4 h-4" /> Buka Folder
            </button>
            <button
              onClick={onClose}
              className="px-12 py-4 bg-white/10 text-white rounded-[24px] font-black text-xs uppercase tracking-widest hover:bg-white/20 transition-all border border-white/10 active:scale-95"
            >
              Tutup
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
