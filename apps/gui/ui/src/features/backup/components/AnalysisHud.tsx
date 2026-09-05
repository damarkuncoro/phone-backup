import React from 'react';
import { FolderSearch, Activity, Lock, Zap, Gauge, AlertTriangle, Image, Film, Music, FileText, MessageSquare, Package } from 'lucide-react';
import { cn } from "@/shared/lib/utils";
import { formatBytes } from '@/shared/lib/formatters';
import type { AnalysisState } from '../hooks/useBackupWizard';

interface AnalysisHudProps {
  analysisState: AnalysisState;
  onExpressBackup: () => void;
}

const CATEGORY_ICONS: Record<string, React.ReactNode> = {
  photos: <Image className="w-3 h-3 text-pink-400" />,
  videos: <Film className="w-3 h-3 text-purple-400" />,
  audio: <Music className="w-3 h-3 text-amber-400" />,
  documents: <FileText className="w-3 h-3 text-blue-400" />,
  whatsapp: <MessageSquare className="w-3 h-3 text-emerald-400" />,
  apks: <Package className="w-3 h-3 text-emerald-500" />,
};

export const AnalysisHud: React.FC<AnalysisHudProps> = ({
  analysisState,
  onExpressBackup,
}) => {
  const categories = analysisState.categories || {};
  const hasCategories = Object.keys(categories).length > 0;

  return (
    <div className="max-w-xl mx-auto py-8 space-y-6 animate-in zoom-in-95 duration-200">
      <div className="bg-white p-6 rounded-[32px] border border-slate-100 shadow-xl space-y-5 text-center">
        <div className="w-14 h-14 rounded-3xl bg-indigo-50 text-indigo-600 mx-auto flex items-center justify-center shadow-inner">
          <FolderSearch className="w-7 h-7 animate-pulse" />
        </div>

        <div>
          <h3 className="text-lg font-black text-slate-900">
            Menganalisis Sistem Berkas Ponsel
          </h3>
          <p className="text-xs text-slate-400 font-medium mt-1">
            Scanner V5 Cerdas: Pemindaian multi-thread, filter noise, dan klasifikasi otomatis.
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
              : analysisState.stage === 'indexing' || analysisState.stage === 'ready'
              ? "bg-emerald-50 border-emerald-200 text-emerald-700"
              : "bg-slate-50 border-slate-200 text-slate-400"
          )}>
            <FolderSearch className="w-3 h-3 shrink-0" />
            <span className="truncate">2. Crawler V5</span>
          </div>

          <div className={cn(
            "p-2.5 rounded-xl border flex items-center gap-1.5",
            analysisState.stage === 'indexing'
              ? "bg-indigo-50 border-indigo-300 text-indigo-700 animate-pulse"
              : analysisState.stage === 'ready'
              ? "bg-emerald-50 border-emerald-200 text-emerald-700"
              : "bg-slate-50 border-slate-200 text-slate-400"
          )}>
            <Lock className="w-3 h-3 shrink-0" />
            <span className="truncate">3. FastCDC</span>
          </div>
        </div>

        {/* Live Counter Box */}
        <div className="bg-slate-900 text-white p-4 rounded-2xl space-y-2.5 text-left font-mono">
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

          {analysisState.throughput !== undefined && analysisState.throughput > 0 && (
            <div className="flex justify-between items-center text-[11px] text-amber-300 border-t border-slate-800 pt-1.5">
              <span className="flex items-center gap-1"><Gauge className="w-3 h-3" /> Throughput:</span>
              <span className="font-bold">{Math.round(analysisState.throughput).toLocaleString()} files/sec</span>
            </div>
          )}

          <div className="border-t border-slate-800 pt-2 text-[10px] text-slate-400 truncate">
            &gt; {analysisState.currentFolder}
          </div>
        </div>

        {/* Category Breakdown Badges */}
        {hasCategories && (
          <div className="pt-1 flex flex-wrap gap-1.5 justify-center">
            {Object.entries(categories).map(([catKey, catSummary]) => {
              if (catSummary.file_count === 0) return null;
              return (
                <div key={catKey} className="px-2.5 py-1 rounded-xl bg-slate-50 border border-slate-200 text-[10px] font-bold text-slate-700 flex items-center gap-1.5 shadow-sm">
                  {CATEGORY_ICONS[catKey.toLowerCase()] || <FolderSearch className="w-3 h-3 text-slate-400" />}
                  <span className="capitalize">{catKey}</span>
                  <span className="text-indigo-600 font-mono">({catSummary.file_count})</span>
                </div>
              );
            })}
          </div>
        )}

        {/* Scan Warnings Alert */}
        {analysisState.warnings && analysisState.warnings.length > 0 && (
          <div className="p-3 bg-amber-50 rounded-2xl border border-amber-200 text-left text-xs text-amber-800 flex items-start gap-2">
            <AlertTriangle className="w-4 h-4 text-amber-600 shrink-0 mt-0.5" />
            <div className="space-y-0.5">
              <span className="font-bold">Catatan Peringatan Scan:</span>
              <p className="text-[11px] text-amber-700 line-clamp-2">{analysisState.warnings[0]}</p>
            </div>
          </div>
        )}

        {/* Express Skip Button */}
        <div className="pt-2">
          <button
            type="button"
            onClick={onExpressBackup}
            className="w-full py-3 bg-slate-900 hover:bg-slate-800 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-lg flex items-center justify-center gap-2 active:scale-95 transition"
          >
            <Zap className="w-4 h-4 text-amber-400" />
            <span>Lewati Pratinjau & Langsung Mulai Backup</span>
          </button>
        </div>
      </div>
    </div>
  );
};
