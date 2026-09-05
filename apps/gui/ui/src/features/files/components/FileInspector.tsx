import { useState } from 'react';
import {
  X, Folder, Download, Copy, Check, Hash,
  Shield, Sparkles, Loader2, Edit3, Trash2
} from 'lucide-react';
import { type FileEntry, deviceService } from '@/services/deviceService';
import { FileInspectorPreview } from './FileInspectorPreview';
import { FileInspectorDetails } from './FileInspectorDetails';

interface FileInspectorProps {
  file: FileEntry | null;
  deviceId: string;
  onClose: () => void;
  onDownload: (file: FileEntry) => void;
  onNavigate?: (path: string) => void;
  onRename?: (file: FileEntry) => void;
  onDelete?: (file: FileEntry) => void;
}

export function FileInspector({
  file,
  deviceId,
  onClose,
  onDownload,
  onNavigate,
  onRename,
  onDelete
}: FileInspectorProps) {
  const [hash, setHash] = useState<string | null>(null);
  const [calculatingHash, setCalculatingHash] = useState(false);
  const [copied, setCopied] = useState(false);
  const [copiedHash, setCopiedHash] = useState(false);

  if (!file) return null;

  const handleCalculateHash = async () => {
    if (file.is_dir) return;
    setCalculatingHash(true);
    try {
      const result = await deviceService.calculateHash(deviceId, file.path);
      setHash(result);
    } catch {
      setHash("Gagal menghitung hash");
    } finally {
      setCalculatingHash(false);
    }
  };

  const handleCopyPath = () => {
    navigator.clipboard.writeText(file.path);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleCopyHash = () => {
    if (!hash) return;
    navigator.clipboard.writeText(hash);
    setCopiedHash(true);
    setTimeout(() => setCopiedHash(false), 2000);
  };

  return (
    <aside className="w-80 border-l border-slate-200/80 bg-white flex flex-col h-full shrink-0 shadow-lg animate-in slide-in-from-right-4 duration-300 z-20">
      <div className="p-5 border-b border-slate-100 flex items-center justify-between">
        <div className="flex items-center gap-2">
          <Sparkles className="w-4 h-4 text-indigo-600" />
          <h3 className="text-xs font-black uppercase tracking-wider text-slate-800">Inspektor Berkas</h3>
        </div>
        <button onClick={onClose} className="p-1.5 rounded-xl hover:bg-slate-100 text-slate-400 hover:text-slate-700 transition-all">
          <X className="w-4 h-4" />
        </button>
      </div>

      <div className="flex-1 overflow-y-auto custom-scrollbar p-6 space-y-6">
        <FileInspectorPreview file={file} />

        <div className="grid grid-cols-2 gap-2">
          {file.is_dir ? (
            <button
              onClick={() => onNavigate?.(file.path)}
              className="col-span-2 py-3 bg-indigo-600 text-white rounded-2xl text-xs font-black uppercase tracking-wider hover:bg-indigo-700 transition-all shadow-md shadow-indigo-100 flex items-center justify-center gap-2"
            >
              <Folder className="w-4 h-4" /> Buka Folder
            </button>
          ) : (
            <button
              onClick={() => onDownload(file)}
              className="col-span-2 py-3 bg-indigo-600 text-white rounded-2xl text-xs font-black uppercase tracking-wider hover:bg-indigo-700 transition-all shadow-md shadow-indigo-100 flex items-center justify-center gap-2"
            >
              <Download className="w-4 h-4" /> Unduh ke PC
            </button>
          )}

          <button
            onClick={handleCopyPath}
            className="py-2.5 bg-slate-50 hover:bg-slate-100 border border-slate-200/70 text-slate-700 rounded-xl text-[11px] font-bold transition-all flex items-center justify-center gap-1.5"
          >
            {copied ? <Check className="w-3.5 h-3.5 text-emerald-600" /> : <Copy className="w-3.5 h-3.5" />}
            {copied ? 'Tersalin' : 'Salin Path'}
          </button>

          {!file.is_dir && (
            <button
              onClick={handleCalculateHash}
              disabled={calculatingHash}
              className="py-2.5 bg-slate-50 hover:bg-slate-100 border border-slate-200/70 text-slate-700 rounded-xl text-[11px] font-bold transition-all flex items-center justify-center gap-1.5 disabled:opacity-50"
            >
              {calculatingHash ? <Loader2 className="w-3.5 h-3.5 animate-spin text-indigo-600" /> : <Hash className="w-3.5 h-3.5" />}
              SHA-256
            </button>
          )}

          {onRename && (
            <button
              onClick={() => onRename(file)}
              className="py-2.5 bg-slate-50 hover:bg-slate-100 border border-slate-200/70 text-slate-700 rounded-xl text-[11px] font-bold transition-all flex items-center justify-center gap-1.5"
            >
              <Edit3 className="w-3.5 h-3.5 text-slate-500" /> Ganti Nama
            </button>
          )}

          {onDelete && (
            <button
              onClick={() => onDelete(file)}
              className="py-2.5 bg-red-50 hover:bg-red-100 border border-red-200/70 text-red-600 rounded-xl text-[11px] font-bold transition-all flex items-center justify-center gap-1.5"
            >
              <Trash2 className="w-3.5 h-3.5 text-red-500" /> Hapus
            </button>
          )}
        </div>

        {hash && (
          <div className="p-4 bg-indigo-50/70 rounded-2xl border border-indigo-100 space-y-2">
            <div className="flex items-center justify-between text-[10px] font-black uppercase tracking-wider text-indigo-700">
              <span className="flex items-center gap-1"><Shield className="w-3 h-3 text-indigo-600" /> SHA-256 Hash</span>
              <button onClick={handleCopyHash} className="text-indigo-600 hover:text-indigo-900 font-bold flex items-center gap-1">
                {copiedHash ? <Check className="w-3 h-3 text-emerald-600" /> : <Copy className="w-3.5 h-3.5" />}
                {copiedHash ? 'Tersalin' : 'Salin'}
              </button>
            </div>
            <p className="font-mono text-[10px] text-slate-700 break-all bg-white p-2.5 rounded-xl border border-indigo-100/60 leading-relaxed select-all">
              {hash}
            </p>
          </div>
        )}

        <FileInspectorDetails file={file} />
      </div>
    </aside>
  );
}
