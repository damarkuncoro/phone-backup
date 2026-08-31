import { History, Search, Trash2, ChevronRight, Calendar, HardDrive, FileText, CheckCircle2, XCircle, Clock, Smartphone, Wifi, WifiOff, GitCompare } from 'lucide-react';
import { type Snapshot, getSnapshotId } from '@/services/backupService';
import { getDeviceId } from '@/services/deviceService';
import { formatDate, formatBytes } from '@/shared/lib/formatters';
import { cn } from "../../../shared/lib/utils";
import { useHistory } from '../hooks/useHistory';

interface HistoryPageProps {
  onBrowse?: (snapshotId: string) => void;
  onCompare?: (oldId: string, newId: string) => void;
}

export function HistoryPage({ onBrowse, onCompare }: HistoryPageProps) {
  const {
    devices,
    liveDeviceIds,
    selectedDeviceId, setSelectedDeviceId,
    loading,
    searchQuery, setSearchQuery,
    comparisonSelection, toggleComparison,
    handleDelete,
    filteredSnapshots,
    snapshots
  } = useHistory();

  const handleCompare = () => {
    if (comparisonSelection.length === 2) {
      const s1 = snapshots.find(s => getSnapshotId(s) === comparisonSelection[0]);
      const s2 = snapshots.find(s => getSnapshotId(s) === comparisonSelection[1]);
      if (s1 && s2) {
        const [oldS, newS] = new Date(s1.started_at) < new Date(s2.started_at) ? [s1, s2] : [s2, s1];
        onCompare?.(getSnapshotId(oldS), getSnapshotId(newS));
      }
    }
  };

  return (
    <div className="p-6 md:p-8 space-y-6 max-w-7xl mx-auto animate-in fade-in duration-300">
      
      {/* Top Banner with Search & Compare Actions */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 bg-white p-6 md:p-8 rounded-[32px] border border-slate-100 shadow-sm">
        <div>
          <h1 className="text-2xl md:text-3xl font-black text-slate-900 tracking-tight">
            Arsip Vault & Riwayat
          </h1>
          <p className="text-xs text-slate-500 font-medium mt-1">
            Telusuri dan bandingkan seluruh snapshot cadangan terenkripsi dari perangkat ponsel Anda.
          </p>
        </div>

        <div className="flex items-center gap-3 shrink-0">
          {comparisonSelection.length === 2 && (
            <button
              type="button"
              onClick={handleCompare}
              className="px-5 py-3 bg-indigo-600 hover:bg-indigo-700 text-white rounded-2xl text-xs font-black uppercase tracking-wider shadow-lg shadow-indigo-200 animate-in zoom-in transition-all flex items-center gap-2 active:scale-95"
            >
              <GitCompare className="w-4 h-4" />
              <span>Bandingkan (2) Snapshot</span>
            </button>
          )}

          <div className="relative w-full sm:w-64">
            <Search className="absolute left-3.5 top-3 w-4 h-4 text-slate-400" />
            <input
              type="text"
              placeholder="Cari snapshot..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full bg-slate-50 border border-slate-200/80 pl-10 pr-4 py-2.5 rounded-2xl text-xs font-medium focus:ring-4 focus:ring-indigo-500/10 focus:border-indigo-300 outline-none transition-all"
            />
          </div>
        </div>
      </div>

      {/* Visual Device Picker Bar */}
      <section className="bg-white p-2 rounded-[32px] border border-slate-100 shadow-sm flex items-center gap-2 overflow-x-auto no-scrollbar">
        {devices.map(d => {
          const isSelected = selectedDeviceId === getDeviceId(d);
          const isOnline = liveDeviceIds.has(getDeviceId(d));

          return (
            <button
              key={getDeviceId(d)}
              type="button"
              onClick={() => setSelectedDeviceId(getDeviceId(d))}
              className={cn(
                "flex items-center gap-3 px-5 py-3.5 rounded-[24px] transition-all min-w-fit shrink-0 select-none",
                isSelected
                  ? "bg-slate-900 text-white shadow-xl scale-[1.02] z-10"
                  : "bg-transparent text-slate-400 hover:bg-slate-50 hover:text-slate-700"
              )}
            >
              <div className={cn(
                "w-9 h-9 rounded-xl flex items-center justify-center transition-colors relative shrink-0",
                isSelected ? "bg-indigo-600 text-white" : "bg-slate-100 text-slate-400"
              )}>
                <Smartphone className="w-4 h-4" />
                <div className={cn(
                  "absolute -top-1 -right-1 w-3.5 h-3.5 rounded-full border-2 border-white flex items-center justify-center",
                  isOnline ? "bg-emerald-500" : "bg-slate-300"
                )}>
                  {isOnline ? <Wifi className="w-2 h-2 text-white" /> : <WifiOff className="w-2 h-2 text-white" />}
                </div>
              </div>
              <div className="text-left min-w-0">
                <p className="text-xs font-black leading-none truncate">{d.model}</p>
                <p className={cn("text-[9px] font-bold uppercase mt-1 tracking-wider", isSelected ? "text-indigo-300" : "text-slate-400")}>
                  {isOnline ? 'Online' : 'Arsip'}
                </p>
              </div>
            </button>
          );
        })}
        {devices.length === 0 && (
          <div className="px-6 py-4 text-xs font-bold text-slate-300 italic uppercase tracking-widest">
            Tidak ada riwayat perangkat ditemukan.
          </div>
        )}
      </section>

      {/* Snapshots List */}
      <section className="space-y-3">
        {loading ? (
          <div className="py-20 flex flex-col items-center justify-center text-slate-400 gap-4">
            <div className="w-12 h-12 border-4 border-slate-100 border-t-indigo-600 rounded-full animate-spin" />
            <p className="text-[10px] font-black uppercase tracking-widest">Memuat Riwayat Snapshot...</p>
          </div>
        ) : filteredSnapshots.length > 0 ? (
          <div className="grid grid-cols-1 gap-3">
            {filteredSnapshots.map(snapshot => (
              <SnapshotRow
                key={getSnapshotId(snapshot)}
                snapshot={snapshot}
                isForComparison={comparisonSelection.includes(getSnapshotId(snapshot))}
                onSelectForComparison={() => toggleComparison(getSnapshotId(snapshot))}
                onBrowse={() => onBrowse?.(getSnapshotId(snapshot))}
                onDelete={() => handleDelete(getSnapshotId(snapshot))}
              />
            ))}
          </div>
        ) : (
          <div className="py-20 flex flex-col items-center justify-center bg-white rounded-[32px] border-2 border-dashed border-slate-200 text-slate-400 p-8 space-y-3">
            <History className="w-14 h-14 opacity-20 text-slate-400" />
            <p className="font-black uppercase tracking-widest text-xs">Tidak Ada Snapshot Ditemukan</p>
            <p className="text-xs text-slate-400 text-center max-w-sm">
              Mulai backup pertama Anda di Studio Pencadangan untuk melihat riwayatnya di sini.
            </p>
          </div>
        )}
      </section>

    </div>
  );
}

function SnapshotRow({ snapshot, isForComparison, onSelectForComparison, onBrowse, onDelete }: {
  snapshot: Snapshot,
  isForComparison: boolean,
  onSelectForComparison: () => void,
  onBrowse: () => void,
  onDelete: () => void
}) {
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
          onClick={(e) => { e.stopPropagation(); onBrowse(); }}
          className="px-4 py-2 bg-slate-50 text-slate-600 rounded-xl text-xs font-black uppercase tracking-wider hover:bg-indigo-600 hover:text-white transition-all flex items-center gap-1.5 active:scale-95"
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
