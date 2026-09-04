import { Shield, X, Download, Database } from 'lucide-react';
import { type FileEntry } from '@/services/deviceService';
import { FileTree } from '@/shared/components/FileTree';
import { ConfirmModal } from '@/shared/components/ConfirmModal';
import { useSnapshotExplorer } from '../hooks/useSnapshotExplorer';
import { useState } from 'react';
import { ContactsExplorer, getContactPhones } from '../components/ContactsExplorer';
import { SmsExplorer } from '../components/SmsExplorer';
import { CallsExplorer } from '../components/CallsExplorer';
import { AppsExplorer } from '../components/AppsExplorer';
import { RestoreOverlay } from '../components/RestoreOverlay';
import { SnapshotExplorerHeader, type ExplorerMode } from '../components/SnapshotExplorerHeader';

interface SnapshotExplorerProps {
  snapshotId: string;
  onBack: () => void;
}

export function SnapshotExplorer({ snapshotId, onBack }: SnapshotExplorerProps) {
  const {
    mode, setMode,
    rawData, loading, error, loadData,
    searchQuery, setSearchQuery,
    selectedPaths, setSelectedPaths, handleTogglePath,
    restoring, setRestoring, progressMsg,
    progressPercent, eta,
    startRestore
  } = useSnapshotExplorer(snapshotId);

  const [confirmState, setConfirmState] = useState<{
    isOpen: boolean;
    title: string;
    message: string;
    onConfirm: () => void;
  }>({ isOpen: false, title: '', message: '', onConfirm: () => {} });

  const handleRestoreClick = (path?: string) => setConfirmState({
    isOpen: true,
    title: "Konfirmasi Pemulihan",
    message: path ? `Restore item: ${path}?` : "Restore seluruh isi snapshot ini ke komputer?",
    onConfirm: () => startRestore(path ? [path] : undefined)
  });

  const handleRestoreSelectedClick = () => {
    if (selectedPaths.size === 0) return;
    setConfirmState({
      isOpen: true,
      title: "Restore Item Terpilih",
      message: `Restore ${selectedPaths.size} item terpilih ke komputer?`,
      onConfirm: () => startRestore(Array.from(selectedPaths))
    });
  };

  return (
    <div className="h-full flex flex-col bg-white animate-in slide-in-from-right-4 duration-500 relative overflow-hidden">

      <ConfirmModal
        isOpen={confirmState.isOpen}
        title={confirmState.title}
        message={confirmState.message}
        onClose={() => setConfirmState(prev => ({ ...prev, isOpen: false }))}
        onConfirm={confirmState.onConfirm}
        confirmText="Ya, Restore"
      />

      {/* Progress Overlay for Restore */}
      <RestoreOverlay
        isOpen={restoring}
        progressPercent={progressPercent}
        progressMsg={progressMsg}
        eta={eta || undefined}
        onClose={() => setRestoring(false)}
      />

      {/* Selection Action Bar */}
      {selectedPaths.size > 0 && mode === 'files' && !restoring && (
        <div className="absolute top-0 left-0 right-0 z-[60] bg-indigo-600 text-white p-4 flex items-center justify-between animate-in slide-in-from-top-4 duration-300 shadow-2xl border-b border-white/10">
          <div className="flex items-center gap-6">
            <button
              onClick={() => setSelectedPaths(new Set())}
              className="p-2.5 hover:bg-white/10 rounded-2xl transition-all border border-transparent hover:border-white/10"
            >
              <X className="w-5 h-5" />
            </button>
            <div>
              <p className="font-black uppercase tracking-[0.2em] text-[10px] opacity-70 leading-none mb-1 text-indigo-200">Selection Active</p>
              <p className="font-black text-sm leading-none">{selectedPaths.size} Item Terpilih</p>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={handleRestoreSelectedClick}
              className="flex items-center gap-3 px-6 py-3 bg-white text-indigo-600 hover:bg-indigo-50 rounded-2xl text-xs font-black uppercase tracking-widest transition-all shadow-xl active:scale-95"
            >
              <Download className="w-4 h-4" />
              Restore Terpilih
            </button>
          </div>
        </div>
      )}

      {/* Header Bar */}
      <SnapshotExplorerHeader
        snapshotId={snapshotId}
        mode={mode as ExplorerMode}
        onSetMode={(m) => setMode(m)}
        searchQuery={searchQuery}
        onSearchChange={setSearchQuery}
        loading={loading}
        onRefresh={loadData}
        onBack={onBack}
        onRestoreAll={() => handleRestoreClick()}
      />

      {/* Content Area */}
      <div className="flex-1 overflow-y-auto custom-scrollbar">
        {loading ? (
          <div className="h-full flex flex-col items-center justify-center gap-4 text-slate-400">
            <div className="w-12 h-12 border-4 border-slate-100 border-t-indigo-600 rounded-full animate-spin" />
            <p className="text-[10px] font-black uppercase tracking-widest">Membuka Vault Snapshot...</p>
          </div>
        ) : error ? (
          <div className="h-full flex flex-col items-center justify-center p-8 text-center space-y-4">
            <div className="w-16 h-16 bg-slate-50 rounded-3xl flex items-center justify-center text-slate-300">
              <Shield className="w-8 h-8 opacity-20" />
            </div>
            <div>
              <p className="font-black text-slate-900 uppercase tracking-widest text-[10px]">Data Tidak Tersedia</p>
              <p className="text-sm text-slate-400 mt-2 max-w-xs mx-auto leading-relaxed">{error}</p>
            </div>
          </div>
        ) : (
          <div className="p-8">
            {mode === 'files' && (
              <div className="max-w-4xl mx-auto pb-10">
                <FileTree
                  files={rawData as FileEntry[]}
                  searchQuery={searchQuery}
                  selectedPaths={selectedPaths}
                  onToggle={handleTogglePath}
                />
              </div>
            )}

            {mode === 'contacts' && (
              <div className="max-w-6xl mx-auto h-[620px]">
                <ContactsExplorer
                  contacts={(Array.isArray(rawData) ? rawData : []).filter((c: any) => {
                    if (!c) return false;
                    if (!searchQuery) return true;
                    const q = searchQuery.toLowerCase();
                    return (c.display_name || '').toLowerCase().includes(q) || getContactPhones(c).some(p => p.number.toLowerCase().includes(q));
                  })}
                  snapshotId={snapshotId}
                />
              </div>
            )}

            {mode === 'sms' && (
              <div className="max-w-6xl mx-auto h-[620px]">
                <SmsExplorer
                  messages={(Array.isArray(rawData) ? rawData : []).filter((m: any) => m && ((m.body || '').toLowerCase().includes(searchQuery.toLowerCase()) || (m.address || '').includes(searchQuery)))}
                  snapshotId={snapshotId}
                />
              </div>
            )}

            {mode === 'calls' && (
              <div className="max-w-6xl mx-auto h-[620px]">
                <CallsExplorer
                  calls={(Array.isArray(rawData) ? rawData : []).filter((c: any) => c && ((c.name || '').toLowerCase().includes(searchQuery.toLowerCase()) || (c.number || '').includes(searchQuery)))}
                  snapshotId={snapshotId}
                />
              </div>
            )}

            {mode === 'apps' && (
              <div className="max-w-6xl mx-auto h-[620px]">
                <AppsExplorer
                  apps={(Array.isArray(rawData) ? rawData : []).filter((a: any) => a && ((a.name || a.app_name || '').toLowerCase().includes(searchQuery.toLowerCase()) || (a.package_name || '').toLowerCase().includes(searchQuery.toLowerCase())))}
                  snapshotId={snapshotId}
                />
              </div>
            )}

            {mode === 'files' && rawData.length === 0 && (
              <div className="py-20 flex flex-col items-center justify-center text-slate-300">
                <Database className="w-16 h-16 mb-4 opacity-10" />
                <p className="font-black uppercase tracking-widest text-[10px]">Vault Kosong</p>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
