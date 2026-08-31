import { useState, useEffect } from 'react';
import { GitCompare, Plus, Minus, RefreshCw, FileText, Loader2, ArrowLeft, Copy, Check } from 'lucide-react';
import { backupService, type FileDiff } from '@/services/backupService';
import { cn } from "../../../shared/lib/utils";

interface DiffViewerProps {
  oldId: string;
  newId: string;
  onBack: () => void;
}

type DiffType = 'added' | 'modified' | 'deleted' | 'unchanged';

export function DiffViewer({ oldId, newId, onBack }: DiffViewerProps) {
  const [diff, setDiff] = useState<FileDiff | null>(null);
  const [loading, setLoading] = useState(true);
  const [activeTab, setActiveTab] = useState<DiffType>('added');
  const [copiedPath, setCopiedPath] = useState<string | null>(null);

  useEffect(() => {
    loadDiff();
  }, [oldId, newId]);

  const loadDiff = async () => {
    setLoading(true);
    try {
      const result = await backupService.getFileDiff(oldId, newId);
      setDiff(result);
    } catch (err) {
      console.error("Failed to load diff", err);
    } finally {
      setLoading(false);
    }
  };

  const handleCopy = (path: string) => {
    navigator.clipboard.writeText(path);
    setCopiedPath(path);
    setTimeout(() => setCopiedPath(null), 2000);
  };

  if (loading) {
    return (
      <div className="h-full flex flex-col items-center justify-center gap-4 text-slate-400 py-24">
        <Loader2 className="w-10 h-10 animate-spin text-indigo-500" />
        <p className="text-xs font-black uppercase tracking-widest">Membandingkan Perbedaan Snapshot...</p>
      </div>
    );
  }

  const items = diff ? diff[activeTab] : [];

  return (
    <div className="p-6 md:p-8 space-y-6 max-w-7xl mx-auto animate-in fade-in duration-300">
      
      {/* Top Banner Overview */}
      <div className="bg-white rounded-[32px] border border-slate-100 p-6 md:p-8 shadow-sm flex flex-col lg:flex-row lg:items-center justify-between gap-6">
        <div className="flex items-center gap-5 min-w-0">
          <button
            type="button"
            onClick={onBack}
            className="p-3 hover:bg-slate-100 rounded-2xl border border-slate-200 text-slate-500 hover:text-slate-900 transition-all active:scale-95 shrink-0"
          >
            <ArrowLeft className="w-5 h-5" />
          </button>
          <div className="w-14 h-14 bg-indigo-600 rounded-2xl flex items-center justify-center text-white shadow-lg shadow-indigo-200 shrink-0">
            <GitCompare className="w-7 h-7" />
          </div>
          <div className="min-w-0">
            <h1 className="text-2xl md:text-3xl font-black text-slate-900 tracking-tight truncate">
              Perbandingan Snapshot
            </h1>
            <p className="text-xs font-mono text-slate-400 mt-1 truncate">
              {oldId.substring(0, 8)}... &rarr; {newId.substring(0, 8)}...
            </p>
          </div>
        </div>

        {/* Diff Tabs */}
        <div className="flex bg-slate-100 p-1.5 rounded-2xl border border-slate-200/60 overflow-x-auto no-scrollbar shrink-0">
          <DiffTab
            active={activeTab === 'added'}
            count={diff?.added.length || 0}
            label="Ditambahkan"
            color="text-emerald-600"
            icon={Plus}
            onClick={() => setActiveTab('added')}
          />
          <DiffTab
            active={activeTab === 'modified'}
            count={diff?.modified.length || 0}
            label="Diubah"
            color="text-amber-600"
            icon={RefreshCw}
            onClick={() => setActiveTab('modified')}
          />
          <DiffTab
            active={activeTab === 'deleted'}
            count={diff?.deleted.length || 0}
            label="Dihapus"
            color="text-red-600"
            icon={Minus}
            onClick={() => setActiveTab('deleted')}
          />
          <DiffTab
            active={activeTab === 'unchanged'}
            count={diff?.unchanged.length || 0}
            label="Sama"
            color="text-slate-400"
            icon={FileText}
            onClick={() => setActiveTab('unchanged')}
          />
        </div>
      </div>

      {/* Diff Items List */}
      <div className="space-y-3">
        {items.map((path, i) => (
          <div
            key={i}
            className="p-4 bg-white rounded-2xl border border-slate-100 hover:border-indigo-100 shadow-sm transition-all flex items-center justify-between group"
          >
            <div className="flex items-center gap-3.5 min-w-0">
              <div className={cn(
                "w-9 h-9 rounded-xl flex items-center justify-center shrink-0 shadow-sm",
                activeTab === 'added' ? "bg-emerald-50 text-emerald-600" :
                activeTab === 'modified' ? "bg-amber-50 text-amber-600" :
                activeTab === 'deleted' ? "bg-red-50 text-red-600" : "bg-slate-100 text-slate-500"
              )}>
                {activeTab === 'added' && <Plus className="w-4 h-4" />}
                {activeTab === 'modified' && <RefreshCw className="w-4 h-4" />}
                {activeTab === 'deleted' && <Minus className="w-4 h-4" />}
                {activeTab === 'unchanged' && <FileText className="w-4 h-4" />}
              </div>
              <p className="text-xs font-mono font-bold text-slate-700 truncate select-all">{path}</p>
            </div>

            <button
              type="button"
              onClick={() => handleCopy(path)}
              className="px-3.5 py-1.5 bg-slate-50 hover:bg-indigo-50 border border-slate-200/70 hover:border-indigo-200 text-slate-600 hover:text-indigo-600 rounded-xl text-[11px] font-bold transition-all flex items-center gap-1.5 opacity-0 group-hover:opacity-100 shrink-0"
            >
              {copiedPath === path ? <Check className="w-3.5 h-3.5 text-emerald-600" /> : <Copy className="w-3.5 h-3.5" />}
              <span>{copiedPath === path ? 'Tersalin' : 'Salin Path'}</span>
            </button>
          </div>
        ))}

        {items.length === 0 && (
          <div className="py-20 flex flex-col items-center justify-center bg-white rounded-[32px] border-2 border-dashed border-slate-200 text-slate-300 p-8 space-y-3">
            <GitCompare className="w-12 h-12 opacity-20" />
            <p className="font-black uppercase tracking-widest text-xs">Tidak Ada Berkas {activeTab}</p>
          </div>
        )}
      </div>

    </div>
  );
}

function DiffTab({ active, count, label, color, icon: Icon, onClick }: {
  active: boolean, count: number, label: string, color: string, icon: any, onClick: () => void
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      className={cn(
        "flex items-center gap-2 px-3.5 py-2 rounded-xl text-xs font-black uppercase tracking-wider transition-all select-none",
        active ? "bg-white text-slate-900 shadow-sm" : "text-slate-400 hover:text-slate-700"
      )}
    >
      <Icon className={cn("w-4 h-4", active ? color : "opacity-40")} />
      <span>{label}</span>
      <span className={cn(
        "px-2 py-0.5 rounded-full text-[10px]",
        active ? "bg-slate-100 text-slate-700 font-bold" : "bg-slate-200/50 text-slate-400"
      )}>{count}</span>
    </button>
  );
}
