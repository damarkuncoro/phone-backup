import {
  CheckCircle2, XCircle, FileText, RefreshCw
} from 'lucide-react';
import { cn } from "@/shared/lib/utils";

interface WizardProgressStepProps {
  error: string | null;
  progressPercent: number;
  progressMsg: string;
  totalItems: number;
  currentItems: number;
  onRetry: () => void;
  onFinish: () => void;
}

export function WizardProgressStep({
  error,
  progressPercent,
  progressMsg,
  totalItems,
  currentItems,
  onRetry,
  onFinish
}: WizardProgressStepProps) {
  return (
    <div className="p-8 md:p-12 flex-1 flex flex-col items-center justify-center text-center space-y-8 animate-in zoom-in-95 duration-300">
      {error ? (
        <div className="space-y-6 max-w-md">
          <div className="w-20 h-20 bg-rose-50 text-rose-600 rounded-3xl mx-auto flex items-center justify-center shadow-lg shadow-rose-100">
            <XCircle className="w-10 h-10" />
          </div>
          <div>
            <h2 className="text-2xl font-black text-slate-900 mb-1">Proses Backup Terhenti</h2>
            <p className="text-xs text-rose-600 font-medium leading-relaxed bg-rose-50/70 p-4 rounded-2xl border border-rose-100">
              {error}
            </p>
          </div>
          <button
            type="button"
            onClick={onRetry}
            className="px-8 py-3.5 bg-slate-900 hover:bg-slate-800 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-xl transition-all"
          >
            Coba Ulangi Backup
          </button>
        </div>
      ) : (
        <div className="space-y-8 max-w-lg w-full">
          {/* Progress Animation Gauge */}
          <div className="relative w-40 h-40 mx-auto">
            <div className={cn(
              "w-40 h-40 rounded-full border-8 border-slate-100 transition-all duration-500",
              progressPercent < 100 ? "border-t-indigo-600 animate-spin" : "border-emerald-500"
            )} />
            <div className="absolute inset-0 flex flex-col items-center justify-center">
              {progressPercent < 100 ? (
                <>
                  <span className="text-3xl font-black text-slate-900 tracking-tighter">{progressPercent}%</span>
                  <span className="text-[10px] font-bold text-slate-400 uppercase tracking-widest mt-0.5">Progress</span>
                </>
              ) : (
                <CheckCircle2 className="w-14 h-14 text-emerald-500 animate-in zoom-in-50" />
              )}
            </div>
          </div>

          {/* Progress Text */}
          <div>
            <h2 className="text-2xl md:text-3xl font-black text-slate-900 mb-1">
              {progressPercent < 100 ? "Sedang Mencadangkan Data..." : "Pencadangan Selesai!"}
            </h2>
            <p className="text-xs text-slate-500 font-medium">
              {progressPercent < 100
                ? "Mohon jangan mencabut kabel atau memutus koneksi perangkat Anda."
                : "Seluruh data yang dipilih kini tersimpan aman, terdeduplikasi, dan disegel dengan enkripsi Age."}
            </p>
          </div>

          {/* Progress Meter Bar */}
          {progressPercent < 100 && (
            <div className="space-y-3 bg-slate-50 p-5 rounded-3xl border border-slate-200/70 text-left">
              <div className="flex justify-between items-center text-xs font-black">
                <span className="text-slate-700 truncate max-w-[260px] flex items-center gap-1.5">
                  <FileText className="w-3.5 h-3.5 text-indigo-600 shrink-0" />
                  <span className="truncate">{progressMsg || "Memproses..."}</span>
                </span>
                <span className="text-indigo-600 font-mono shrink-0">
                  {totalItems > 0 ? `${currentItems} / ${totalItems}` : `${progressPercent}%`}
                </span>
              </div>

              <div className="h-3 w-full bg-slate-200 rounded-full overflow-hidden p-0.5">
                <div
                  className="h-full bg-indigo-600 rounded-full transition-all duration-300 ease-out"
                  style={{ width: `${Math.max(progressPercent, 2)}%` }}
                />
              </div>

              {progressPercent > 92 && (
                <p className="text-[10px] text-amber-600 font-bold animate-pulse flex items-center gap-1.5">
                  <RefreshCw className="w-3 h-3 animate-spin" /> Sedang merampungkan manifest snapshot & enkripsi...
                </p>
              )}
            </div>
          )}

          {/* Completion Action Buttons */}
          {progressPercent === 100 && (
            <div className="pt-2 flex flex-col sm:flex-row gap-3 justify-center">
              <button
                type="button"
                onClick={onFinish}
                className="px-8 py-3.5 bg-indigo-600 hover:bg-indigo-700 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-xl shadow-indigo-200 hover:shadow-indigo-300 transition-all active:scale-95"
              >
                Kembali ke Dashboard
              </button>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
