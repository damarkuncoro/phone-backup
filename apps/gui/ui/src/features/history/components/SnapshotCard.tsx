import { Calendar, HardDrive, FileText, CheckCircle2, XCircle, Clock, ChevronRight, Trash2, FolderDown } from 'lucide-react';
import { type Snapshot, getSnapshotId } from '@/services/backupService';
import { formatDate, formatBytes } from '@/shared/lib/formatters';
import { cn } from '@/shared/lib/utils';

interface SnapshotCardProps {
  snapshot: Snapshot;
  isForComparison: boolean;
  onSelectForComparison: () => void;
  onBrowse: () => void;
  onSelectiveRestore: () => void;
  onDelete: () => void;
}

export function SnapshotCard({
  snapshot,
  isForComparison,
  onSelectForComparison,
  onBrowse,
  onSelectiveRestore,
  onDelete
}: SnapshotCardProps) {
  const id = getSnapshotId(snapshot);

  return (
    <div
      onClick={onSelectForComparison}
      className={cn(
        "group bg-white p-5 rounded-[28px] border transition-all flex items-center gap-6 cursor-pointer select-none",
        isForComparison
          ? "border-indigo-500 ring-2 ring-indigo-500/20 shadow-md bg-indigo-50/20"
          : "border-slate-100 shadow-sm hover:shadow-md hover:border-indigo-100"
      )}
    >
      <div className={cn(
        "w-12 h-12 rounded-2xl flex items-center justify-center shrink-0 shadow-sm",
        snapshot.status === 'Completed' ? "bg-emerald-50 text-emerald-600" :
        snapshot.status === 'Failed' ? "bg-red-50 text-red-600" : "bg-amber-50 text-amber-600"
      )}>
        {snapshot.status === 'Completed' ? <CheckCircle2 className="w-6 h-6" /> :
         snapshot.status === 'Failed' ? <XCircle className="w-6 h-6" /> : <Clock className="w-6 h-6" />}
      </div>

      <div className="flex-1 min-w-0" onClick={(e) => { e.stopPropagation(); onBrowse(); }}>
        <div className="flex items-center gap-2.5 mb-1">
          <h3 className="font-black text-slate-800 text-sm truncate group-hover:text-indigo-600 transition-colors">
            Snapshot_{id.substring(0, 8)}
          </h3>
          <StatusBadge status={snapshot.status} />
        </div>
        <div className="flex items-center gap-4 text-[10px] font-bold text-slate-400 uppercase tracking-wider">
          <span className="flex items-center gap-1.5"><Calendar className="w-3 h-3 text-slate-400" /> {formatDate(snapshot.started_at)}</span>
          <span className="flex items-center gap-1.5"><HardDrive className="w-3 h-3 text-slate-400" /> {formatBytes(snapshot.total_bytes)}</span>
          <span className="flex items-center gap-1.5"><FileText className="w-3 h-3 text-slate-400" /> {snapshot.total_files} Files</span>
        </div>
      </div>

      <div className="flex items-center gap-2">
        <button
          type="button"
          onClick={(e) => { e.stopPropagation(); onSelectiveRestore(); }}
          title="Pemulihan Selektif"
          className="px-3 py-2 bg-indigo-50 text-indigo-700 hover:bg-indigo-600 hover:text-white rounded-xl text-xs font-black transition-all flex items-center gap-1.5 active:scale-95"
        >
          <FolderDown className="w-3.5 h-3.5" />
          <span className="hidden sm:inline">Restore</span>
        </button>
        <button
          type="button"
          onClick={(e) => { e.stopPropagation(); onBrowse(); }}
          className="px-4 py-2 bg-slate-50 text-slate-600 rounded-xl text-xs font-black uppercase tracking-wider hover:bg-slate-900 hover:text-white transition-all flex items-center gap-1.5 active:scale-95"
        >
          <span>Telusuri</span>
          <ChevronRight className="w-3.5 h-3.5" />
        </button>
        <button
          type="button"
          onClick={(e) => { e.stopPropagation(); onDelete(); }}
          title="Hapus Snapshot"
          className="p-2 bg-white border border-slate-200 text-slate-400 rounded-xl hover:text-red-600 hover:border-red-200 transition-all opacity-0 group-hover:opacity-100"
        >
          <Trash2 className="w-4 h-4" />
        </button>
      </div>
    </div>
  );
}

function StatusBadge({ status }: { status: Snapshot['status'] }) {
  const styles = {
    Completed: "bg-emerald-50 text-emerald-700 border-emerald-100",
    Failed: "bg-red-50 text-red-700 border-red-100",
    Running: "bg-indigo-50 text-indigo-700 border-indigo-100 animate-pulse",
    Pending: "bg-slate-50 text-slate-700 border-slate-100",
    Interrupted: "bg-amber-50 text-amber-700 border-amber-100",
  };

  return (
    <span className={cn("px-2 py-0.5 rounded-full text-[9px] font-black uppercase tracking-wider border", styles[status])}>
      {status}
    </span>
  );
}
