import React from 'react';
import { FolderSearch, Activity, Lock, Zap } from 'lucide-react';
import { cn } from "@/shared/lib/utils";
import { formatBytes } from '@/shared/lib/formatters';
import type { AnalysisState } from '../hooks/useBackupWizard';

interface AnalysisHudProps {
  analysisState: AnalysisState;
  onExpressBackup: () => void;
}

export const AnalysisHud: React.FC<AnalysisHudProps> = ({
  analysisState,
  onExpressBackup,
}) => {
  return (
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

        {/* Express Skip Button */}
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
  );
};
