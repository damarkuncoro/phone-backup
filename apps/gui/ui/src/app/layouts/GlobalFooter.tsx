import { Database, HardDrive, CheckCircle2, Activity, ShieldCheck } from 'lucide-react';

interface GlobalFooterProps {
  activeTaskMsg?: string | null;
  activeTaskProgress?: number | null;
  storageBackend?: string;
}

export function GlobalFooter({
  activeTaskMsg,
  activeTaskProgress,
  storageBackend = 'Local Disk'
}: GlobalFooterProps) {
  const isBusy = activeTaskMsg && activeTaskProgress !== null && activeTaskProgress !== undefined && activeTaskProgress < 100;

  return (
    <footer className="h-7 bg-slate-900 text-slate-400 border-t border-slate-800 px-4 flex items-center justify-between text-[10px] font-bold select-none shrink-0 z-30 font-mono">
      
      {/* Left: Infrastructure & Health Status */}
      <div className="flex items-center gap-3">
        <div className="flex items-center gap-1.5 text-emerald-400">
          <div className="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse" />
          <Database className="w-3 h-3 text-emerald-400" />
          <span className="font-sans">SQLite SQLCipher</span>
        </div>

        <div className="w-px h-3 bg-slate-800" />

        <div className="flex items-center gap-1 text-slate-400 font-sans">
          <Activity className="w-3 h-3 text-indigo-400" />
          <span>ADB & MTP Engine</span>
        </div>
      </div>

      {/* Center: Live Task / Telemetry */}
      <div className="hidden sm:flex items-center gap-2 max-w-md truncate">
        {isBusy ? (
          <div className="flex items-center gap-2 text-indigo-300 animate-pulse">
            <Activity className="w-3 h-3 animate-spin text-indigo-400 shrink-0" />
            <span className="truncate">{activeTaskMsg}</span>
            <span className="text-white font-mono bg-indigo-900/60 px-1.5 py-0.5 rounded text-[9px]">
              {activeTaskProgress}%
            </span>
          </div>
        ) : (
          <div className="flex items-center gap-1.5 text-slate-500 font-sans">
            <ShieldCheck className="w-3 h-3 text-slate-400" />
            <span>FastCDC Content-Defined Chunker • E2E Encrypted</span>
          </div>
        )}
      </div>

      {/* Right: Storage Engine & Version */}
      <div className="flex items-center gap-3">
        <div className="flex items-center gap-1.5 text-slate-400 font-sans">
          <HardDrive className="w-3 h-3 text-slate-500" />
          <span>{storageBackend}</span>
        </div>

        <div className="w-px h-3 bg-slate-800" />

        <div className="flex items-center gap-1 text-slate-400 font-sans">
          <CheckCircle2 className="w-3 h-3 text-indigo-400" />
          <span>v0.3.2 PRO</span>
        </div>
      </div>

    </footer>
  );
}
