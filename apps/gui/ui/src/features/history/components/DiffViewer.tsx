import { useState, useEffect } from 'react';
import { GitCompare, Plus, Minus, RefreshCw, FileText, Loader2, ArrowLeft } from 'lucide-react';
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

  if (loading) {
    return (
      <div className="h-full flex flex-col items-center justify-center gap-4 text-slate-400 bg-white">
        <Loader2 className="w-10 h-10 animate-spin text-indigo-500" />
        <p className="text-[10px] font-black uppercase tracking-widest">Calculating Differences...</p>
      </div>
    );
  }

  const items = diff ? diff[activeTab] : [];

  return (
    <div className="h-full flex flex-col bg-white animate-in slide-in-from-bottom-4 duration-500">
      <header className="p-8 border-b border-slate-100 flex items-center justify-between">
        <div className="flex items-center gap-6">
          <button
            onClick={onBack}
            className="p-3 hover:bg-slate-50 rounded-2xl border border-slate-100 text-slate-400 hover:text-slate-900 transition-all"
          >
            <ArrowLeft className="w-5 h-5" />
          </button>
          <div>
            <div className="flex items-center gap-2 mb-1">
              <GitCompare className="w-5 h-5 text-indigo-600" />
              <h1 className="text-2xl font-black text-slate-900 tracking-tight">Snapshot Comparison</h1>
            </div>
            <p className="text-[10px] font-bold text-slate-400 uppercase tracking-widest">
              Comparing {oldId.substring(0, 8)}... → {newId.substring(0, 8)}...
            </p>
          </div>
        </div>

        <div className="flex bg-slate-100 p-1 rounded-2xl">
          <DiffTab
            active={activeTab === 'added'}
            count={diff?.added.length || 0}
            label="Added"
            color="text-emerald-600"
            icon={Plus}
            onClick={() => setActiveTab('added')}
          />
          <DiffTab
            active={activeTab === 'modified'}
            count={diff?.modified.length || 0}
            label="Modified"
            color="text-amber-600"
            icon={RefreshCw}
            onClick={() => setActiveTab('modified')}
          />
          <DiffTab
            active={activeTab === 'deleted'}
            count={diff?.deleted.length || 0}
            label="Deleted"
            color="text-red-600"
            icon={Minus}
            onClick={() => setActiveTab('deleted')}
          />
          <DiffTab
            active={activeTab === 'unchanged'}
            count={diff?.unchanged.length || 0}
            label="Unchanged"
            color="text-slate-400"
            icon={FileText}
            onClick={() => setActiveTab('unchanged')}
          />
        </div>
      </header>

      <div className="flex-1 overflow-y-auto p-8">
        <div className="max-w-4xl mx-auto space-y-2">
          {items.map((path, i) => (
            <div key={i} className="p-4 bg-slate-50 rounded-2xl border border-slate-100 flex items-center justify-between group hover:border-indigo-100 transition-all">
               <div className="flex items-center gap-4 min-w-0">
                  <div className={cn(
                    "w-8 h-8 rounded-lg flex items-center justify-center shrink-0",
                    activeTab === 'added' ? "bg-emerald-100 text-emerald-600" :
                    activeTab === 'modified' ? "bg-amber-100 text-amber-600" :
                    activeTab === 'deleted' ? "bg-red-100 text-red-600" : "bg-slate-200 text-slate-500"
                  )}>
                    {activeTab === 'added' && <Plus className="w-4 h-4" />}
                    {activeTab === 'modified' && <RefreshCw className="w-4 h-4" />}
                    {activeTab === 'deleted' && <Minus className="w-4 h-4" />}
                    {activeTab === 'unchanged' && <FileText className="w-4 h-4" />}
                  </div>
                  <p className="text-sm font-mono font-bold text-slate-700 truncate">{path}</p>
               </div>
               <button className="px-4 py-1.5 bg-white border border-slate-200 rounded-xl text-[10px] font-black uppercase tracking-widest text-slate-400 hover:text-indigo-600 hover:border-indigo-100 opacity-0 group-hover:opacity-100 transition-all">
                  Inspect
               </button>
            </div>
          ))}

          {items.length === 0 && (
            <div className="py-20 flex flex-col items-center justify-center text-slate-300">
               <GitCompare className="w-16 h-16 mb-4 opacity-10" />
               <p className="font-black uppercase tracking-widest text-xs">No {activeTab} items found</p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

function DiffTab({ active, count, label, color, icon: Icon, onClick }: {
  active: boolean, count: number, label: string, color: string, icon: any, onClick: () => void
}) {
  return (
    <button
      onClick={onClick}
      className={cn(
        "flex items-center gap-2 px-4 py-2 rounded-xl text-xs font-black uppercase tracking-widest transition-all",
        active ? "bg-white shadow-sm" : "text-slate-400 hover:text-slate-600"
      )}
    >
      <Icon className={cn("w-4 h-4", active ? color : "opacity-40")} />
      <span className="hidden lg:block">{label}</span>
      <span className={cn(
        "ml-1 px-1.5 py-0.5 rounded-md text-[9px]",
        active ? "bg-slate-100 text-slate-600" : "bg-slate-200/50 text-slate-400"
      )}>{count}</span>
    </button>
  );
}
