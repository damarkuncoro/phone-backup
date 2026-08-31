import {
    Folder, Users, MessageSquare, Smartphone, ArrowLeft, Download,
    Search, Loader2, Database, Shield, RefreshCcw, X, Clock, CheckCircle2
} from 'lucide-react';
import { systemService } from '@/services/systemService';
import { type FileEntry } from '@/services/deviceService';
import { cn } from "../../../shared/lib/utils";
import { FileTree } from '@/shared/components/FileTree';
import { ConfirmModal } from '@/shared/components/ConfirmModal';
import { useSnapshotExplorer } from '../hooks/useSnapshotExplorer';
import { useState } from 'react';

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
  }>({
      isOpen: false,
      title: '',
      message: '',
      onConfirm: () => {}
  });

  const handleRestoreClick = (path?: string) => {
    setConfirmState({
        isOpen: true,
        title: "Konfirmasi Pemulihan",
        message: path ? `Restore item: ${path}?` : "Restore seluruh isi snapshot ini ke komputer?",
        onConfirm: () => startRestore(path ? [path] : undefined)
    });
  };

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

      {/* Progress Overlay for Restore - Higher Z-index */}
      {restoring && (
          <div className="absolute inset-0 z-[100] bg-slate-900/95 backdrop-blur-md flex flex-col items-center justify-center text-center p-8 animate-in fade-in duration-300">
              <div className="relative mb-12">
                  <div className={cn(
                      "w-56 h-56 rounded-full border-8 border-white/5 transition-all duration-700",
                      progressPercent < 100 ? "border-t-indigo-500 animate-spin" : "border-emerald-500 shadow-[0_0_50px_-12px_rgba(16,185,129,0.5)]"
                  )} />
                  <div className="absolute inset-0 flex flex-col items-center justify-center">
                      {progressPercent < 100 ? (
                          <>
                            <span className="text-5xl font-black text-white leading-none tracking-tighter">{progressPercent}%</span>
                            <span className="text-[10px] font-black text-indigo-400 uppercase tracking-[0.3em] mt-2 ml-1">Restoring</span>
                          </>
                      ) : (
                          <CheckCircle2 className="w-20 h-26 text-emerald-500 animate-in zoom-in duration-500" />
                      )}
                  </div>
              </div>

              <div className="max-w-md w-full space-y-8">
                  <div>
                      <h2 className="text-3xl font-black text-white mb-3 tracking-tight">
                          {progressPercent < 100 ? "Memulihkan Data" : "Restore Selesai!"}
                      </h2>
                      <div className="min-h-[40px] flex items-center justify-center">
                        <p className="text-slate-400 text-sm font-medium px-6 leading-relaxed">
                            {progressPercent < 100 ? progressMsg : (
                                <>
                                    Data telah dipulihkan ke direktori kerja Anda: <br/>
                                    <span className="text-indigo-400 font-mono text-[10px] break-all bg-indigo-500/10 px-3 py-1.5 rounded-lg mt-3 inline-block border border-indigo-500/20">
                                        workspace/restored_data
                                    </span>
                                </>
                            )}
                        </p>
                      </div>
                  </div>

                  {progressPercent < 100 && (
                      <div className="bg-white/5 p-5 rounded-[32px] border border-white/10 flex items-center justify-between shadow-2xl">
                          <div className="flex items-center gap-4 text-left">
                              <div className="w-10 h-10 bg-indigo-500/20 rounded-2xl flex items-center justify-center text-indigo-400 shadow-inner">
                                  <Clock className="w-5 h-5" />
                              </div>
                              <div>
                                  <p className="text-[9px] font-black text-slate-500 uppercase tracking-widest">Estimasi Sisa Waktu</p>
                                  <p className="text-sm font-black text-white leading-none mt-1">{eta || 'Menghitung...'}</p>
                              </div>
                          </div>
                          <div className="w-px h-8 bg-white/10 mx-2" />
                          <div className="text-right pr-2">
                             <p className="text-[9px] font-black text-slate-500 uppercase tracking-widest">Status</p>
                             <p className="text-sm font-black text-indigo-400 leading-none mt-1">Running</p>
                          </div>
                      </div>
                  )}

                  {progressPercent === 100 && (
                      <div className="flex gap-4 animate-in slide-in-from-bottom-4 duration-500">
                        <button
                            onClick={() => systemService.openRestoreFolder()}
                            className="flex-1 py-4 bg-indigo-600 text-white rounded-[24px] font-black text-xs uppercase tracking-widest hover:bg-indigo-700 transition-all shadow-2xl flex items-center justify-center gap-3 border-t border-white/20"
                        >
                            <Folder className="w-4 h-4" /> Buka Folder
                        </button>
                        <button
                            onClick={() => setRestoring(false)}
                            className="px-12 py-4 bg-white/10 text-white rounded-[24px] font-black text-xs uppercase tracking-widest hover:bg-white/20 transition-all border border-white/10"
                        >
                            Tutup
                        </button>
                      </div>
                  )}
              </div>
          </div>
      )}

      {/* Selection Action Bar - Increased Z-index */}
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
                    <p className="text-[10px] font-black text-slate-400 uppercase tracking-widest flex items-center gap-2">
                        <Shield className="w-3 h-3 text-emerald-500" /> ID: {snapshotId.substring(0, 12)}...
                    </p>
                </div>
            </div>

            <div className="flex items-center gap-4">
                <div className="flex bg-slate-100 p-1 rounded-2xl border border-slate-200/50">
                    <ModeTab active={mode === 'files'} icon={Folder} label="Files" onClick={() => setMode('files')} />
                    <ModeTab active={mode === 'contacts'} icon={Users} label="Contacts" onClick={() => setMode('contacts')} />
                    <ModeTab active={mode === 'sms'} icon={MessageSquare} label="Messages" onClick={() => setMode('sms')} />
                    <ModeTab active={mode === 'apps'} icon={Smartphone} label="Apps" onClick={() => setMode('apps')} />
                </div>

                {mode === 'files' && (
                    <button
                        disabled={loading}
                        onClick={() => handleRestoreClick()}
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
                    onChange={(e) => setSearchQuery(e.target.value)}
                    className="w-full bg-slate-50 border border-slate-100 pl-11 pr-4 py-3 rounded-2xl text-sm focus:ring-4 focus:ring-indigo-500/5 focus:border-indigo-200 outline-none transition-all"
                />
            </div>
            <button onClick={loadData} className="p-3 text-slate-400 hover:text-indigo-600 transition-all bg-slate-50 rounded-2xl border border-slate-100 active:rotate-180 duration-500">
                <RefreshCcw className={cn("w-5 h-5", loading && "animate-spin")} />
            </button>
        </div>
      </header>

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

                  {mode === 'contacts' && <ContactList contacts={rawData.filter(c => (c.display_name || '').toLowerCase().includes(searchQuery.toLowerCase()))} />}
                  {mode === 'sms' && <SmsList messages={rawData.filter(m => (m.body || '').toLowerCase().includes(searchQuery.toLowerCase()) || (m.address || '').includes(searchQuery))} />}
                  {mode === 'apps' && <AppGrid apps={rawData.filter(a => (a.name || '').toLowerCase().includes(searchQuery.toLowerCase()))} />}

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

function ModeTab({ active, icon: Icon, label, onClick }: { active: boolean, icon: any, label: string, onClick: () => void }) {
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

function ContactList({ contacts }: { contacts: any[] }) {
    return (
        <div className="max-w-2xl mx-auto space-y-3">
            {contacts.map((c, i) => (
                <div key={i} className="p-5 bg-white border border-slate-100 rounded-[32px] flex items-center gap-5 hover:shadow-lg transition-all group">
                    <div className="w-12 h-12 bg-indigo-100 text-indigo-600 rounded-2xl flex items-center justify-center font-black text-sm shadow-inner group-hover:scale-110 transition-transform">
                        {c.display_name?.[0] || '?'}
                    </div>
                    <div className="flex-1">
                        <p className="font-black text-slate-800 text-lg tracking-tight">{c.display_name}</p>
                        <p className="text-xs font-bold text-slate-400 uppercase tracking-widest">{c.phone_numbers?.[0] || 'No number'}</p>
                    </div>
                </div>
            ))}
        </div>
    );
}

function SmsList({ messages }: { messages: any[] }) {
    return (
        <div className="max-w-3xl mx-auto space-y-4">
            {messages.map((m, i) => (
                <div key={i} className="p-8 bg-slate-50 rounded-[40px] border border-slate-100 space-y-4 hover:bg-white hover:shadow-xl transition-all">
                    <div className="flex justify-between items-center">
                        <span className="text-xs font-black text-indigo-600 bg-indigo-50 px-3 py-1 rounded-full">{m.address}</span>
                        <span className="text-[10px] font-bold text-slate-400 uppercase tracking-widest">{new Date(m.date).toLocaleString()}</span>
                    </div>
                    <p className="text-slate-700 leading-relaxed font-medium">{m.body}</p>
                </div>
            ))}
        </div>
    );
}

function AppGrid({ apps }: { apps: any[] }) {
    return (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {apps.map((a, i) => (
                <div key={i} className="p-6 bg-white border border-slate-100 rounded-[32px] flex items-start gap-5 hover:shadow-xl hover:border-indigo-100 transition-all group">
                    <div className="w-16 h-16 bg-slate-50 rounded-[24px] flex items-center justify-center group-hover:bg-indigo-50 transition-colors">
                        <Smartphone className="w-8 h-8 text-slate-200 group-hover:text-indigo-400 transition-colors" />
                    </div>
                    <div className="flex-1 min-w-0">
                        <p className="font-black text-slate-800 truncate text-lg tracking-tight">{a.name}</p>
                        <p className="text-[10px] font-mono text-slate-400 truncate uppercase tracking-tighter">{a.package_name}</p>
                        <div className="mt-3 flex gap-2">
                            <span className="text-[9px] px-2.5 py-1 bg-slate-100 rounded-lg font-black text-slate-500 uppercase tracking-widest">v{a.version_name || '?' }</span>
                        </div>
                    </div>
                </div>
            ))}
        </div>
    );
}
