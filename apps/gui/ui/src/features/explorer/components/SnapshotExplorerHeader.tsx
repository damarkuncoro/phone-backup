import {
  Folder, Users, MessageSquare, Smartphone, ArrowLeft, Download,
  Search, Shield, RefreshCcw
} from 'lucide-react';
import { cn } from "@/shared/lib/utils";

export type ExplorerMode = 'files' | 'contacts' | 'sms' | 'apps';

interface SnapshotExplorerHeaderProps {
  snapshotId: string;
  mode: ExplorerMode;
  onSetMode: (mode: ExplorerMode) => void;
  searchQuery: string;
  onSearchChange: (q: string) => void;
  loading: boolean;
  onRefresh: () => void;
  onBack: () => void;
  onRestoreAll: () => void;
}

export function SnapshotExplorerHeader({
  snapshotId,
  mode,
  onSetMode,
  searchQuery,
  onSearchChange,
  loading,
  onRefresh,
  onBack,
  onRestoreAll
}: SnapshotExplorerHeaderProps) {
  return (
    <header className="p-8 border-b border-slate-100 flex flex-col gap-6 shrink-0 bg-white/80 backdrop-blur-sm sticky top-0 z-40">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-6">
          <button
            onClick={onBack}
            className="p-3 hover:bg-slate-50 rounded-2xl border border-slate-100 text-slate-400 hover:text-slate-900 transition-all active:scale-95"
          >
            <ArrowLeft className="w-5 h-5" />
          </button>
          <div>
            <h1 className="text-2xl font-black text-slate-900 tracking-tight flex items-center gap-2">
              Vault Explorer <span className="text-slate-300 font-light">/</span> {mode.toUpperCase()}
            </h1>
            <p className="text-[10px] font-black text-slate-400 uppercase tracking-widest flex items-center gap-1.5">
              <Shield className="w-3 h-3 text-emerald-500" /> ID: {snapshotId.substring(0, 12)}...
            </p>
          </div>
        </div>

        <div className="flex items-center gap-4">
          <div className="flex bg-slate-100 p-1 rounded-2xl border border-slate-200/50">
            <ModeTab active={mode === 'files'} icon={Folder} label="Files" onClick={() => onSetMode('files')} />
            <ModeTab active={mode === 'contacts'} icon={Users} label="Contacts" onClick={() => onSetMode('contacts')} />
            <ModeTab active={mode === 'sms'} icon={MessageSquare} label="Messages" onClick={() => onSetMode('sms')} />
            <ModeTab active={mode === 'apps'} icon={Smartphone} label="Apps" onClick={() => onSetMode('apps')} />
          </div>

          {mode === 'files' && (
            <button
              disabled={loading}
              onClick={onRestoreAll}
              className="px-5 py-3 bg-slate-900 text-white rounded-2xl text-[10px] font-black uppercase tracking-widest hover:bg-slate-800 transition-all flex items-center gap-2 shadow-xl shadow-slate-200 disabled:opacity-50 active:scale-95"
            >
              <Download className="w-3.5 h-3.5" />
              Restore Semua
            </button>
          )}
        </div>
      </div>

      <div className="flex items-center gap-4">
        <div className="flex-1 relative group">
          <Search className="absolute left-4 top-3.5 w-4 h-4 text-slate-300 group-focus-within:text-indigo-500 transition-colors" />
          <input
            type="text"
            placeholder={`Cari di dalam ${mode}...`}
            value={searchQuery}
            onChange={(e) => onSearchChange(e.target.value)}
            className="w-full bg-slate-50 border border-slate-100 pl-11 pr-4 py-3 rounded-2xl text-sm focus:ring-4 focus:ring-indigo-500/5 focus:border-indigo-200 outline-none transition-all"
          />
        </div>
        <button
          onClick={onRefresh}
          className="p-3 text-slate-400 hover:text-indigo-600 transition-all bg-slate-50 rounded-2xl border border-slate-100 active:rotate-180 duration-500"
        >
          <RefreshCcw className={cn("w-5 h-5", loading && "animate-spin")} />
        </button>
      </div>
    </header>
  );
}

function ModeTab({
  active, icon: Icon, label, onClick
}: {
  active: boolean;
  icon: any;
  label: string;
  onClick: () => void;
}) {
  return (
    <button
      onClick={onClick}
      className={cn(
        "flex items-center gap-2 px-4 py-2 rounded-xl text-[10px] font-black uppercase tracking-widest transition-all",
        active ? "bg-white text-indigo-600 shadow-sm" : "text-slate-400 hover:text-slate-600"
      )}
    >
      <Icon className="w-3.5 h-3.5" />
      <span className="hidden lg:block">{label}</span>
    </button>
  );
}
