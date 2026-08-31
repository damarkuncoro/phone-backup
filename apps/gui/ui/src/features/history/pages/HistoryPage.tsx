import { History, Search, Trash2, ChevronRight, Calendar, HardDrive, FileText, CheckCircle2, XCircle, Clock, Smartphone, Wifi, WifiOff } from 'lucide-react';
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
    <div className="p-8 space-y-8 animate-in fade-in duration-500">
      <header className="flex flex-col md:flex-row md:items-end justify-between gap-8">
        <div className="min-w-0">
          <div className="flex items-center gap-3 mb-2">
            <div className="w-12 h-12 bg-indigo-600 rounded-2xl flex items-center justify-center text-white shadow-xl shadow-indigo-200">
                <History className="w-6 h-6" />
            </div>
            <h1 className="text-3xl font-black text-slate-900 tracking-tight">Arsip Vault</h1>
          </div>
          <p className="text-slate-500 font-medium truncate">Telusuri seluruh snapshot data terenkripsi dari perangkat Anda.</p>
        </div>

        <div className="flex items-center gap-4 shrink-0">
            {comparisonSelection.length === 2 && (
                <button
                    onClick={handleCompare}
                    className="px-6 py-3 bg-indigo-600 text-white rounded-2xl text-[10px] font-black uppercase tracking-widest shadow-xl shadow-indigo-200 animate-in zoom-in ring-4 ring-indigo-50"
                >
                    Bandingkan (2) Snapshot
                </button>
            )}

            <div className="relative">
                <Search className="absolute left-4 top-3 w-4 h-4 text-slate-400" />
                <input
                    type="text"
                    placeholder="Cari snapshot..."
                    value={searchQuery}
                    onChange={(e) => setSearchQuery(e.target.value)}
                    className="bg-white border border-slate-200 pl-11 pr-4 py-3 rounded-2xl text-sm focus:ring-4 focus:ring-indigo-500/10 outline-none transition-all w-64 shadow-sm"
                />
            </div>
        </div>
      </header>

      {/* New Visual Device Picker with Offline Support */}
      <section className="bg-white p-2 rounded-[32px] border border-slate-100 shadow-sm flex items-center gap-2 overflow-x-auto no-scrollbar">
          {devices.map(d => {
              const isSelected = selectedDeviceId === getDeviceId(d);
              // Check if device is from ADB (Live) or just from Database (Arsip)
              const isOnline = d.connection_type && d.connection_type !== 'Unknown';

              return (
                  <button
                    key={getDeviceId(d)}
                    onClick={() => setSelectedDeviceId(getDeviceId(d))}
                    className={cn(
                        "flex items-center gap-3 px-6 py-4 rounded-[24px] transition-all min-w-fit shrink-0",
                        isSelected ? "bg-slate-900 text-white shadow-2xl scale-105 z-10" : "bg-transparent text-slate-400 hover:bg-slate-50"
                    )}
                  >
                      <div className={cn(
                          "w-10 h-10 rounded-xl flex items-center justify-center transition-colors relative",
                          isSelected ? "bg-indigo-600 text-white" : "bg-slate-100 text-slate-400"
                      )}>
                          <Smartphone className="w-5 h-5" />
                          <div className={cn(
                              "absolute -top-1 -right-1 w-4 h-4 rounded-full border-2 border-white flex items-center justify-center",
                              isOnline ? "bg-emerald-500" : "bg-slate-300"
                          )}>
                             {isOnline ? <Wifi className="w-2.5 h-2.5 text-white" /> : <WifiOff className="w-2.5 h-2.5 text-white" />}
                          </div>
                      </div>
                      <div className="text-left">
                          <p className="text-xs font-black leading-none">{d.model}</p>
                          <p className={cn("text-[9px] font-bold uppercase mt-1", isSelected ? "text-indigo-300" : "text-slate-300")}>
                            {isOnline ? 'Online' : 'Arsip Vault'}
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

      <section className="space-y-4">
          {loading ? (
              <div className="py-20 flex flex-col items-center justify-center text-slate-400 gap-4">
                  <div className="w-12 h-12 border-4 border-slate-100 border-t-indigo-600 rounded-full animate-spin" />
                  <p className="text-[10px] font-black uppercase tracking-widest">Memuat Riwayat...</p>
              </div>
          ) : filteredSnapshots.length > 0 ? (
              <div className="grid grid-cols-1 gap-4">
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
              <div className="py-20 flex flex-col items-center justify-center bg-slate-50 rounded-[40px] border-2 border-dashed border-slate-200 text-slate-400">
                  <History className="w-16 h-16 mb-4 opacity-20" />
                  <p className="font-black uppercase tracking-widest text-xs">Tidak ada snapshot</p>
                  <p className="text-xs mt-1">Mulai backup pertama Anda untuk melihatnya di sini.</p>
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
                "group bg-white p-6 rounded-[32px] border transition-all flex items-center gap-8 cursor-pointer",
                isForComparison ? "border-indigo-500 ring-2 ring-indigo-500/10 shadow-lg" : "border-slate-100 shadow-sm hover:shadow-md hover:border-indigo-100"
            )}
        >
            <div className={cn(
                "w-14 h-14 rounded-2xl flex items-center justify-center shrink-0",
                snapshot.status === 'Completed' ? "bg-emerald-50 text-emerald-600" :
                snapshot.status === 'Failed' ? "bg-red-50 text-red-600" : "bg-amber-50 text-amber-600"
            )}>
                {snapshot.status === 'Completed' ? <CheckCircle2 className="w-7 h-7" /> :
                 snapshot.status === 'Failed' ? <XCircle className="w-7 h-7" /> : <Clock className="w-7 h-7" />}
            </div>

            <div className="flex-1 min-w-0" onClick={(e) => { e.stopPropagation(); onBrowse(); }}>
                <div className="flex items-center gap-3 mb-1">
                    <h3 className="font-black text-slate-800 truncate">Snapshot_{id.substring(0, 8)}</h3>
                    <StatusBadge status={snapshot.status} />
                </div>
                <div className="flex items-center gap-4 text-[11px] font-bold text-slate-400 uppercase tracking-wider">
                    <span className="flex items-center gap-1.5"><Calendar className="w-3 h-3" /> {formatDate(snapshot.started_at)}</span>
                    <span className="flex items-center gap-1.5"><HardDrive className="w-3 h-3" /> {formatBytes(snapshot.total_bytes)}</span>
                    <span className="flex items-center gap-1.5"><FileText className="w-3 h-3" /> {snapshot.total_files} Files</span>
                </div>
            </div>

            <div className="flex items-center gap-4 opacity-0 group-hover:opacity-100 transition-all">
                <button
                    onClick={(e) => { e.stopPropagation(); onBrowse(); }}
                    className="px-5 py-2.5 bg-slate-50 text-slate-600 rounded-xl text-[10px] font-black uppercase tracking-widest hover:bg-indigo-600 hover:text-white transition-all flex items-center gap-2"
                >
                    Telusuri <ChevronRight className="w-4 h-4" />
                </button>
                <button
                    onClick={(e) => { e.stopPropagation(); onDelete(); }}
                    className="p-2.5 bg-white border border-slate-100 text-slate-400 rounded-xl hover:text-red-500 hover:border-red-100 transition-all"
                >
                    <Trash2 className="w-4 h-4" />
                </button>
            </div>
        </div>
    );
}

function StatusBadge({ status }: { status: Snapshot['status'] }) {
    const styles = {
        Completed: "bg-emerald-100 text-emerald-700",
        Failed: "bg-red-100 text-red-700",
        Running: "bg-indigo-100 text-indigo-700 animate-pulse",
        Pending: "bg-slate-100 text-slate-700",
        Interrupted: "bg-amber-100 text-amber-700",
    };

    return (
        <span className={cn("px-2 py-0.5 rounded-full text-[9px] font-black uppercase tracking-widest", styles[status])}>
            {status}
        </span>
    );
}
