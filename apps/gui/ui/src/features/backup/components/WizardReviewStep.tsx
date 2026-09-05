import React, { useState } from 'react';
import { Search, ShieldCheck, ArrowLeft, ArrowRight, Lock, FolderCheck, Users, HardDrive } from 'lucide-react';
import { type FileEntry } from '@/services/deviceService';
import { formatBytes } from '@/shared/lib/formatters';
import { FileTree } from '@/shared/components/FileTree';
import type { ContactData } from '@/features/explorer/components/contactsUtils';
import type { AnalysisState } from '../hooks/useBackupWizard';
import { AnalysisHud } from './AnalysisHud';
import { WizardContactsPreview } from './WizardContactsPreview';

interface WizardReviewStepProps {
  totalBytes: number;
  selectedFilesCount: number;
  reviewSearch: string;
  onReviewSearchChange: (val: string) => void;
  isCalculating: boolean;
  analysisState: AnalysisState;
  scannedFiles: FileEntry[];
  selectedPaths: Set<string>;
  onTogglePath: (path: string, isFolder: boolean, childrenPaths: string[]) => void;
  encryptionEnabled: boolean;
  onBack: () => void;
  onExpressBackup: () => void;
  onStartBackup: () => void;
  selectedData?: string[];
  liveContacts?: ContactData[];
  loadingStructured?: boolean;
  selectedContactIds?: Set<string>;
  onToggleContact?: (id: string) => void;
  onSelectAllContacts?: () => void;
  onDeselectAllContacts?: () => void;
}

export const WizardReviewStep: React.FC<WizardReviewStepProps> = ({
  totalBytes, selectedFilesCount, reviewSearch, onReviewSearchChange, isCalculating,
  analysisState, scannedFiles, selectedPaths, onTogglePath, encryptionEnabled, onBack,
  onExpressBackup, onStartBackup, selectedData = [], liveContacts = [], loadingStructured = false,
  selectedContactIds = new Set(), onToggleContact, onSelectAllContacts, onDeselectAllContacts
}) => {
  const hasFiles = selectedData.some(d => ['full_storage', 'photos', 'chat_media', 'files', 'audio'].includes(d));
  const hasContacts = selectedData.includes('contacts');
  const [activeTab, setActiveTab] = useState<'files' | 'contacts'>(hasFiles ? 'files' : 'contacts');
  const [contactSearch, setContactSearch] = useState('');

  return (
    <div className="flex-1 flex flex-col min-h-0 animate-in fade-in duration-200">
      {/* Review Header Stats */}
      <div className="p-6 md:p-8 border-b border-slate-100 shrink-0 bg-white space-y-4">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
          <div>
            <h2 className="text-xl font-black text-slate-900 tracking-tight">Eksplorasi Rencana Backup</h2>
            <p className="text-xs text-slate-400 font-medium mt-0.5">
              Tinjau daftar data dan berkas yang akan disimpan ke arsip backup.
            </p>
          </div>

          <div className="flex items-center gap-3 bg-slate-50 p-2.5 rounded-2xl border border-slate-200/70">
            <div className="px-3 text-right">
              <p className="text-[9px] font-black text-slate-400 uppercase tracking-widest">Total Ukuran</p>
              <p className="text-base font-black text-indigo-600">{formatBytes(totalBytes)}</p>
            </div>
            <div className="w-px h-8 bg-slate-200" />
            <div className="px-3 text-right">
              <p className="text-[9px] font-black text-slate-400 uppercase tracking-widest">Total File</p>
              <p className="text-base font-black text-slate-900">{selectedFilesCount}</p>
            </div>
          </div>
        </div>

        {/* Tab switchers if both files and contacts exist */}
        {hasContacts && hasFiles && (
          <div className="flex items-center gap-2 pt-1 border-t border-slate-100">
            <button
              type="button"
              onClick={() => setActiveTab('files')}
              className={`px-4 py-2 rounded-xl text-xs font-bold transition-all flex items-center gap-2 ${
                activeTab === 'files' ? 'bg-indigo-600 text-white shadow-md shadow-indigo-200' : 'text-slate-500 hover:bg-slate-100'
              }`}
            >
              <HardDrive className="w-4 h-4" />
              <span>Pohon File ({scannedFiles.length})</span>
            </button>
            <button
              type="button"
              onClick={() => setActiveTab('contacts')}
              className={`px-4 py-2 rounded-xl text-xs font-bold transition-all flex items-center gap-2 ${
                activeTab === 'contacts' ? 'bg-indigo-600 text-white shadow-md shadow-indigo-200' : 'text-slate-500 hover:bg-slate-100'
              }`}
            >
              <Users className="w-4 h-4" />
              <span>Kontak HP ({selectedContactIds.size}/{liveContacts.length})</span>
            </button>
          </div>
        )}

        {activeTab === 'files' && hasFiles && (
          <div className="relative">
            <Search className="absolute left-4 top-3.5 w-4 h-4 text-slate-400" />
            <input
              type="text"
              placeholder="Cari nama file dalam rencana backup..."
              value={reviewSearch}
              onChange={(e) => onReviewSearchChange(e.target.value)}
              className="w-full bg-slate-50 border border-slate-200/80 pl-11 pr-4 py-3 rounded-2xl text-xs font-medium outline-none focus:ring-4 focus:ring-indigo-500/10 focus:border-indigo-300 transition-all"
            />
          </div>
        )}
      </div>

      {/* Main Review Content Area */}
      <div className="flex-1 overflow-y-auto bg-slate-50/50 custom-scrollbar p-6">
        {activeTab === 'contacts' || !hasFiles ? (
          <WizardContactsPreview
            contacts={liveContacts}
            searchQuery={contactSearch}
            onSearchChange={setContactSearch}
            isLoading={loadingStructured}
            selectedContactIds={selectedContactIds}
            onToggleContact={onToggleContact}
            onSelectAll={onSelectAllContacts}
            onDeselectAll={onDeselectAllContacts}
          />
        ) : isCalculating ? (
          <AnalysisHud analysisState={analysisState} onExpressBackup={onExpressBackup} />
        ) : scannedFiles.length === 0 ? (
          <div className="h-full flex flex-col items-center justify-center py-20 text-slate-400 space-y-2">
            <FolderCheck className="w-12 h-12 text-slate-300" />
            <p className="text-xs font-black uppercase tracking-widest">Tidak ada file media yang perlu dipilih manual.</p>
            <p className="text-[11px] text-slate-400">Data modul (Kontak, SMS, Apps) akan dicadangkan secara otomatis.</p>
          </div>
        ) : (
          <div className="max-w-3xl mx-auto pb-10">
            <FileTree files={scannedFiles} searchQuery={reviewSearch} selectedPaths={selectedPaths} onToggle={onTogglePath} />
          </div>
        )}
      </div>

      {/* Step 3 Footer */}
      <div className="p-6 md:p-8 border-t border-slate-100 bg-white shrink-0 flex justify-between items-center">
        <button
          type="button"
          onClick={onBack}
          className="px-6 py-3 font-black text-slate-400 hover:text-slate-700 transition-all uppercase text-[10px] tracking-wider flex items-center gap-2"
        >
          <ArrowLeft className="w-4 h-4" /> Kembali
        </button>

        <div className="flex items-center gap-4">
          {encryptionEnabled && (
            <div className="hidden sm:flex items-center gap-2 text-emerald-700 bg-emerald-50 px-4 py-2 rounded-xl border border-emerald-200">
              <ShieldCheck className="w-4 h-4 text-emerald-600" />
              <span className="text-[10px] font-black uppercase tracking-wider">Age X25519 Ready</span>
            </div>
          )}
          <button
            type="button"
            disabled={isCalculating}
            onClick={onStartBackup}
            className="px-8 py-3.5 bg-slate-900 hover:bg-slate-800 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-xl shadow-slate-200 transition-all flex items-center gap-2.5 active:scale-95 disabled:opacity-50"
          >
            <Lock className="w-4 h-4 text-indigo-400" />
            <span>Konfirmasi & Mulai Backup</span>
            <ArrowRight className="w-4 h-4" />
          </button>
        </div>
      </div>
    </div>
  );
};
