import {
  Search, ShieldCheck, ArrowLeft, ArrowRight, Lock,
  FolderCheck, FolderSearch, Activity, Zap
} from 'lucide-react';
import { type FileEntry } from '@/services/deviceService';
import { cn } from "@/shared/lib/utils";
import { formatBytes } from '@/shared/lib/formatters';
import { FileTree } from '@/shared/components/FileTree';
import type { AnalysisState } from '../hooks/useBackupWizard';

interface WizardReviewStepProps {
  totalBytes: number;
  selectedFilesCount: number;
  reviewSearch: string;
  onReviewSearchChange: (val: string) => void;
  isCalculating: boolean;
  analysisState: AnalysisState;
  scannedFiles: FileEntry[];
  selectedPaths: Set<string>;
  onTogglePath: (path: string, isFolder: boolean, childrenPaths: string[]) => void;
  encryptionEnabled: boolean;
  onBack: () => void;
  onExpressBackup: () => void;
  onStartBackup: () => void;
}

export function WizardReviewStep({
  totalBytes,
  selectedFilesCount,
  reviewSearch,
  onReviewSearchChange,
  isCalculating,
  analysisState,
  scannedFiles,
  selectedPaths,
  onTogglePath,
  encryptionEnabled,
  onBack,
  onExpressBackup,
  onStartBackup
}: WizardReviewStepProps) {
  return (
    <div className="flex-1 flex flex-col min-h-0 animate-in fade-in duration-200">
      {/* Review Header Stats */}
      <div className="p-6 md:p-8 border-b border-slate-100 shrink-0 bg-white space-y-5">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
          <div>
            <h2 className="text-xl font-black text-slate-900 tracking-tight">
              Eksplorasi Rencana Backup
            </h2>
            <p className="text-xs text-slate-400 font-medium mt-0.5">
              Tinjau file yang terdeteksi. Anda dapat mengecualikan folder atau file tertentu.
            </p>
          </div>

          <div className="flex items-center gap-3 bg-slate-50 p-2.5 rounded-2xl border border-slate-200/70">
            <div className="px-3 text-right">
              <p className="text-[9px] font-black text-slate-400 uppercase tracking-widest">Total Ukuran</p>
              <p className="text-base font-black text-indigo-600">{formatBytes(totalBytes)}</p>
            </div>
            <div className="w-px h-8 bg-slate-200" />
            <div className="px-3 text-right">
              <p className="text-[9px] font-black text-slate-400 uppercase tracking-widest">Total File</p>
              <p className="text-base font-black text-slate-900">{selectedFilesCount}</p>
            </div>
          </div>
        </div>

        <div className="relative">
          <Search className="absolute left-4 top-3.5 w-4 h-4 text-slate-400" />
          <input
            type="text"
            placeholder="Cari nama file dalam rencana backup..."
            value={reviewSearch}
            onChange={(e) => onReviewSearchChange(e.target.value)}
            className="w-full bg-slate-50 border border-slate-200/80 pl-11 pr-4 py-3 rounded-2xl text-xs font-medium outline-none focus:ring-4 focus:ring-indigo-500/10 focus:border-indigo-300 transition-all"
          />
        </div>
      </div>

      {/* Tree View Area or Live Analysis HUD */}
      <div className="flex-1 overflow-y-auto bg-slate-50/50 custom-scrollbar p-6">
        {isCalculating ? (
          /* LIVE ANALYSIS HUD */
          <div className="max-w-xl mx-auto py-10 space-y-6 animate-in zoom-in-95 duration-200">
            <div className="bg-white p-6 rounded-[32px] border border-slate-100 shadow-xl space-y-6 text-center">
              
              <div className="w-14 h-14 rounded-3xl bg-indigo-50 text-indigo-600 mx-auto flex items-center justify-center shadow-inner">
                <FolderSearch className="w-7 h-7 animate-pulse" />
              </div>

              <div>
                <h3 className="text-lg font-black text-slate-900">
                  Menganalisis Sistem Berkas Ponsel
                </h3>
                <p className="text-xs text-slate-400 font-medium mt-1">
                  Memindai data secara cerdas menggunakan indeks Android MediaStore + Path Crawler.
                </p>
              </div>

              {/* Stage Pipeline Indicator */}
              <div className="grid grid-cols-3 gap-2 text-left text-[10px] font-black uppercase tracking-wider">
                <div className={cn(
                  "p-2.5 rounded-xl border flex items-center gap-1.5",
                  analysisState.stage === 'mediastore'
                    ? "bg-indigo-50 border-indigo-300 text-indigo-700 animate-pulse"
                    : "bg-emerald-50 border-emerald-200 text-emerald-700"
                )}>
                  <Activity className="w-3 h-3 shrink-0" />
                  <span className="truncate">1. MediaStore</span>
                </div>

                <div className={cn(
                  "p-2.5 rounded-xl border flex items-center gap-1.5",
                  analysisState.stage === 'crawler'
                    ? "bg-indigo-50 border-indigo-300 text-indigo-700 animate-pulse"
                    : analysisState.stage === 'indexing'
                    ? "bg-emerald-50 border-emerald-200 text-emerald-700"
                    : "bg-slate-50 border-slate-200 text-slate-400"
                )}>
                  <FolderSearch className="w-3 h-3 shrink-0" />
                  <span className="truncate">2. Crawler</span>
                </div>

                <div className={cn(
                  "p-2.5 rounded-xl border flex items-center gap-1.5",
                  analysisState.stage === 'indexing'
                    ? "bg-indigo-50 border-indigo-300 text-indigo-700 animate-pulse"
                    : "bg-slate-50 border-slate-200 text-slate-400"
                )}>
                  <Lock className="w-3 h-3 shrink-0" />
                  <span className="truncate">3. FastCDC</span>
                </div>
              </div>

              {/* Live Counter Box */}
              <div className="bg-slate-900 text-white p-4 rounded-2xl space-y-2 text-left font-mono">
                <div className="flex justify-between items-center text-xs">
                  <span className="text-slate-400">Berkas Terhitung:</span>
                  <span className="text-emerald-400 font-bold text-sm">
                    {analysisState.filesCount.toLocaleString()} Berkas
                  </span>
                </div>
                <div className="flex justify-between items-center text-xs">
                  <span className="text-slate-400">Total Volume:</span>
                  <span className="text-cyan-400 font-bold">
                    {formatBytes(analysisState.totalBytes)}
                  </span>
                </div>
                <div className="border-t border-slate-800 pt-2 text-[10px] text-slate-400 truncate">
                  &gt; {analysisState.currentFolder}
                </div>
              </div>

              {/* Express Skip Button for large storage */}
              <div className="pt-2">
                <button
                  type="button"
                  onClick={onExpressBackup}
                  className="w-full py-3 bg-slate-900 hover:bg-slate-800 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-lg flex items-center justify-center gap-2 active:scale-95"
                >
                  <Zap className="w-4 h-4 text-amber-400" />
                  <span>Lewati Pratinjau & Langsung Mulai Backup</span>
                </button>
              </div>

            </div>
          </div>
        ) : scannedFiles.length === 0 ? (
          <div className="h-full flex flex-col items-center justify-center py-20 text-slate-400 space-y-2">
            <FolderCheck className="w-12 h-12 text-slate-300" />
            <p className="text-xs font-black uppercase tracking-widest">Tidak ada file media yang perlu dipilih manual.</p>
            <p className="text-[11px] text-slate-400">Data modul (Kontak, SMS, Apps) akan dicadangkan secara otomatis.</p>
          </div>
        ) : (
          <div className="max-w-3xl mx-auto pb-10">
            <FileTree
              files={scannedFiles}
              searchQuery={reviewSearch}
              selectedPaths={selectedPaths}
              onToggle={onTogglePath}
            />
          </div>
        )}
      </div>

      {/* Step 3 Footer */}
      <div className="p-6 md:p-8 border-t border-slate-100 bg-white shrink-0 flex justify-between items-center">
        <button
          type="button"
          onClick={onBack}
          className="px-6 py-3 font-black text-slate-400 hover:text-slate-700 transition-all uppercase text-[10px] tracking-wider flex items-center gap-2"
        >
          <ArrowLeft className="w-4 h-4" /> Kembali
        </button>

        <div className="flex items-center gap-4">
          {encryptionEnabled && (
            <div className="hidden sm:flex items-center gap-2 text-emerald-700 bg-emerald-50 px-4 py-2 rounded-xl border border-emerald-200">
              <ShieldCheck className="w-4 h-4 text-emerald-600" />
              <span className="text-[10px] font-black uppercase tracking-wider">Age X25519 Ready</span>
            </div>
          )}
          <button
            type="button"
            disabled={isCalculating}
            onClick={onStartBackup}
            className="px-8 py-3.5 bg-slate-900 hover:bg-slate-800 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-xl shadow-slate-200 transition-all flex items-center gap-2.5 active:scale-95 disabled:opacity-50"
          >
            <Lock className="w-4 h-4 text-indigo-400" />
            <span>Konfirmasi & Mulai Backup</span>
            <ArrowRight className="w-4 h-4" />
          </button>
        </div>
      </div>
    </div>
  );
}
