import { useState } from 'react';
import { History, Search, GitCompare } from 'lucide-react';
import { type Snapshot, getSnapshotId } from '@/services/backupService';
import { useHistory } from '../hooks/useHistory';
import { DevicePickerBar } from '../components/DevicePickerBar';
import { SnapshotCard } from '../components/SnapshotCard';
import { SelectiveRestoreModal } from '../components/SelectiveRestoreModal';

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

  const [restoreSnapshot, setRestoreSnapshot] = useState<Snapshot | null>(null);

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
            Telusuri, pulihkan secara selektif, dan bandingkan snapshot cadangan terenkripsi dari ponsel Anda.
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
      <DevicePickerBar
        devices={devices}
        selectedDeviceId={selectedDeviceId}
        liveDeviceIds={liveDeviceIds}
        onSelectDevice={setSelectedDeviceId}
      />

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
              <SnapshotCard
                key={getSnapshotId(snapshot)}
                snapshot={snapshot}
                isForComparison={comparisonSelection.includes(getSnapshotId(snapshot))}
                onSelectForComparison={() => toggleComparison(getSnapshotId(snapshot))}
                onBrowse={() => onBrowse?.(getSnapshotId(snapshot))}
                onSelectiveRestore={() => setRestoreSnapshot(snapshot)}
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

      {/* Selective Restore Modal */}
      <SelectiveRestoreModal
        snapshot={restoreSnapshot}
        isOpen={!!restoreSnapshot}
        onClose={() => setRestoreSnapshot(null)}
      />
    </div>
  );
}

