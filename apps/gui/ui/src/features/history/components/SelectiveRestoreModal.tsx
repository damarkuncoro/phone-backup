import { useState } from 'react';
import { X, CheckSquare, Square, FolderDown, CheckCircle2, AlertCircle, Phone, MessageSquare, PhoneCall, Image, Package } from 'lucide-react';
import { backupService, type Snapshot, getSnapshotId } from '@/services/backupService';
import { systemService } from '@/services/systemService';
import { formatBytes } from '@/shared/lib/formatters';

interface SelectiveRestoreModalProps {
  snapshot: Snapshot | null;
  isOpen: boolean;
  onClose: () => void;
}

interface CategoryOption {
  id: string;
  name: string;
  icon: any;
  description: string;
  defaultPaths: string[];
}

const CATEGORIES: CategoryOption[] = [
  { id: 'contacts', name: 'Buku Telepon (Kontak)', icon: Phone, description: 'Export kontak format vCard (.vcf)', defaultPaths: ['contacts.vcf', 'contacts.csv'] },
  { id: 'sms', name: 'Pesan Singkat (SMS)', icon: MessageSquare, description: 'Riwayat percakapan SMS XML & JSON', defaultPaths: ['sms_backup.xml', 'sms.json'] },
  { id: 'calls', name: 'Riwayat Panggilan (Calls)', icon: PhoneCall, description: 'Statistik & log riwayat panggilan', defaultPaths: ['calls.json', 'call_stats.json'] },
  { id: 'media', name: 'Foto, Video & Media (DCIM)', icon: Image, description: 'Folder kamera DCIM & Pictures', defaultPaths: ['DCIM', 'Pictures', 'Movies', 'Music'] },
  { id: 'apps', name: 'Paket Aplikasi (APKs)', icon: Package, description: 'Base APKs dan bundle aplikasi terpasang', defaultPaths: ['apps', 'apks'] },
];

export function SelectiveRestoreModal({ snapshot, isOpen, onClose }: SelectiveRestoreModalProps) {
  const [selectedIds, setSelectedIds] = useState<string[]>(['contacts', 'sms', 'calls', 'media']);
  const [isRestoring, setIsRestoring] = useState(false);
  const [restoreSuccess, setRestoreSuccess] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  if (!isOpen || !snapshot) return null;

  const snapshotId = getSnapshotId(snapshot);

  const toggleCategory = (id: string) => {
    setSelectedIds(prev => prev.includes(id) ? prev.filter(item => item !== id) : [...prev, id]);
  };

  const handleStartRestore = async () => {
    if (selectedIds.length === 0) return;
    setIsRestoring(true);
    setErrorMessage(null);
    try {
      const activePaths = CATEGORIES
        .filter(c => selectedIds.includes(c.id))
        .flatMap(c => c.defaultPaths);

      await backupService.restoreSnapshot(snapshotId, "workspace/restored_data", activePaths);
      setRestoreSuccess(true);
    } catch (e: any) {
      setErrorMessage(String(e));
    } finally {
      setIsRestoring(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-950/70 backdrop-blur-sm animate-in fade-in duration-200">
      <div className="bg-white rounded-[32px] max-w-xl w-full p-6 md:p-8 shadow-2xl border border-slate-100 flex flex-col space-y-5">
        
        {/* Header */}
        <div className="flex items-center justify-between pb-3 border-b border-slate-100">
          <div>
            <h2 className="text-lg font-black text-slate-900 tracking-tight">Pemulihan Selektif (Granular Restore)</h2>
            <p className="text-xs text-slate-400 mt-0.5">Snapshot: <span className="font-mono text-slate-600 font-bold">{snapshotId}</span> ({formatBytes(snapshot.total_bytes)})</p>
          </div>
          <button onClick={onClose} className="w-8 h-8 rounded-full bg-slate-100 hover:bg-slate-200 flex items-center justify-center text-slate-500">
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* Categories List */}
        <div className="space-y-2.5">
          <p className="text-xs font-bold text-slate-700">Pilih Kategori Data yang Ingin Dipulihkan:</p>
          {CATEGORIES.map(cat => {
            const isChecked = selectedIds.includes(cat.id);
            const Icon = cat.icon;
            return (
              <button
                key={cat.id}
                type="button"
                onClick={() => toggleCategory(cat.id)}
                className={`w-full p-3.5 rounded-2xl border text-left flex items-center justify-between transition-all ${
                  isChecked ? 'bg-indigo-50/70 border-indigo-200 text-indigo-900' : 'bg-slate-50/50 border-slate-100 text-slate-500'
                }`}
              >
                <div className="flex items-center gap-3">
                  <div className={`w-9 h-9 rounded-xl flex items-center justify-center ${isChecked ? 'bg-indigo-600 text-white' : 'bg-slate-200 text-slate-400'}`}>
                    <Icon className="w-4 h-4" />
                  </div>
                  <div>
                    <p className="text-xs font-black">{cat.name}</p>
                    <p className="text-[11px] opacity-75">{cat.description}</p>
                  </div>
                </div>
                {isChecked ? <CheckSquare className="w-5 h-5 text-indigo-600 shrink-0" /> : <Square className="w-5 h-5 text-slate-300 shrink-0" />}
              </button>
            );
          })}
        </div>

        {errorMessage && (
          <div className="p-3.5 bg-red-50 border border-red-200 rounded-xl flex items-center gap-2 text-red-700 text-xs font-bold">
            <AlertCircle className="w-4 h-4 shrink-0" />
            <span>{errorMessage}</span>
          </div>
        )}

        {restoreSuccess && (
          <div className="p-4 bg-emerald-50 border border-emerald-200 rounded-2xl space-y-2">
            <div className="flex items-center gap-2 text-emerald-800 text-xs font-black">
              <CheckCircle2 className="w-4 h-4 text-emerald-600" />
              <span>Pemulihan Selesai ke folder: workspace/restored_data</span>
            </div>
            <button
              onClick={() => systemService.openRestoreFolder()}
              className="px-4 py-1.5 bg-emerald-600 hover:bg-emerald-500 text-white rounded-xl text-xs font-black"
            >
              Buka Folder Hasil Restore
            </button>
          </div>
        )}

        {/* Footer Actions */}
        <div className="flex items-center justify-end gap-3 pt-3 border-t border-slate-100">
          <button onClick={onClose} className="px-5 py-2.5 rounded-xl bg-slate-100 text-slate-600 text-xs font-bold">
            Batal
          </button>
          <button
            onClick={handleStartRestore}
            disabled={isRestoring || selectedIds.length === 0}
            className="px-6 py-2.5 bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 text-white font-black rounded-xl text-xs shadow-lg shadow-indigo-600/20 flex items-center gap-2"
          >
            <FolderDown className="w-4 h-4" />
            {isRestoring ? 'Memulihkan Data...' : `Restore (${selectedIds.length}) Kategori`}
          </button>
        </div>
      </div>
    </div>
  );
}
