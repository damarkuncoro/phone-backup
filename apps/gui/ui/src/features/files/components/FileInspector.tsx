import { useState } from 'react';
import {
  X, File, Folder, Download, Copy, Check, Hash,
  Calendar, HardDrive, Shield, Sparkles, Image as ImageIcon,
  Video, Music, FileText, Smartphone, Loader2
} from 'lucide-react';
import { type FileEntry, deviceService } from '@/services/deviceService';
import { formatBytes, formatDate } from '@/shared/lib/formatters';
import { isImage, isVideo, isAudio, isDocument, isApk } from '../lib/fileUtils';

interface FileInspectorProps {
  file: FileEntry | null;
  deviceId: string;
  onClose: () => void;
  onDownload: (file: FileEntry) => void;
  onNavigate?: (path: string) => void;
}

export function FileInspector({
  file,
  deviceId,
  onClose,
  onDownload,
  onNavigate
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
    } catch (err) {
      console.error("Gagal menghitung hash", err);
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

  const renderIcon = () => {
    if (file.is_dir) {
      return (
        <div className="w-16 h-16 rounded-3xl bg-indigo-50 text-indigo-600 flex items-center justify-center shadow-inner">
          <Folder className="w-8 h-8 fill-current opacity-80" />
        </div>
      );
    }
    if (isImage(file.name)) {
      return (
        <div className="w-16 h-16 rounded-3xl bg-amber-50 text-amber-600 flex items-center justify-center shadow-inner">
          <ImageIcon className="w-8 h-8" />
        </div>
      );
    }
    if (isVideo(file.name)) {
      return (
        <div className="w-16 h-16 rounded-3xl bg-rose-50 text-rose-600 flex items-center justify-center shadow-inner">
          <Video className="w-8 h-8" />
        </div>
      );
    }
    if (isAudio(file.name)) {
      return (
        <div className="w-16 h-16 rounded-3xl bg-purple-50 text-purple-600 flex items-center justify-center shadow-inner">
          <Music className="w-8 h-8" />
        </div>
      );
    }
    if (isDocument(file.name)) {
      return (
        <div className="w-16 h-16 rounded-3xl bg-blue-50 text-blue-600 flex items-center justify-center shadow-inner">
          <FileText className="w-8 h-8" />
        </div>
      );
    }
    if (isApk(file.name)) {
      return (
        <div className="w-16 h-16 rounded-3xl bg-emerald-50 text-emerald-600 flex items-center justify-center shadow-inner">
          <Smartphone className="w-8 h-8" />
        </div>
      );
    }
    return (
      <div className="w-16 h-16 rounded-3xl bg-slate-100 text-slate-500 flex items-center justify-center shadow-inner">
        <File className="w-8 h-8" />
      </div>
    );
  };

  return (
    <aside className="w-80 border-l border-slate-200/80 bg-white flex flex-col h-full shrink-0 shadow-lg animate-in slide-in-from-right-4 duration-300 z-20">
      {/* Header */}
      <div className="p-5 border-b border-slate-100 flex items-center justify-between">
        <div className="flex items-center gap-2">
          <Sparkles className="w-4 h-4 text-indigo-600" />
          <h3 className="text-xs font-black uppercase tracking-wider text-slate-800">Inspektor Berkas</h3>
        </div>
        <button
          onClick={onClose}
          className="p-1.5 rounded-xl hover:bg-slate-100 text-slate-400 hover:text-slate-700 transition-all"
        >
          <X className="w-4 h-4" />
        </button>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto custom-scrollbar p-6 space-y-6">
        {/* Visual Preview Badge */}
        <div className="flex flex-col items-center text-center p-5 bg-slate-50 rounded-3xl border border-slate-100">
          <div className="mb-4">{renderIcon()}</div>
          <h4 className="font-black text-slate-900 text-sm break-all leading-snug" title={file.name}>
            {file.name}
          </h4>
          <span className="mt-2 px-3 py-1 bg-white border border-slate-200/60 rounded-full text-[10px] font-black uppercase tracking-wider text-slate-500 shadow-sm">
            {file.is_dir ? 'Direktori Folder' : file.name.split('.').pop()?.toUpperCase() + ' File'}
          </span>
        </div>

        {/* Action Buttons */}
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
        </div>

        {/* SHA-256 Hash Display */}
        {hash && (
          <div className="p-4 bg-indigo-50/70 rounded-2xl border border-indigo-100 space-y-2">
            <div className="flex items-center justify-between text-[10px] font-black uppercase tracking-wider text-indigo-700">
              <span className="flex items-center gap-1"><Shield className="w-3 h-3 text-indigo-600" /> SHA-256 Hash</span>
              <button
                onClick={handleCopyHash}
                className="text-indigo-600 hover:text-indigo-900 font-bold flex items-center gap-1"
              >
                {copiedHash ? <Check className="w-3 h-3 text-emerald-600" /> : <Copy className="w-3 h-3" />}
                {copiedHash ? 'Tersalin' : 'Salin'}
              </button>
            </div>
            <p className="font-mono text-[10px] text-slate-700 break-all bg-white p-2.5 rounded-xl border border-indigo-100/60 leading-relaxed select-all">
              {hash}
            </p>
          </div>
        )}

        {/* Metadata Details List */}
        <div className="space-y-3">
          <h5 className="text-[10px] font-black uppercase tracking-widest text-slate-400">Informasi Berkas</h5>

          <div className="space-y-2 text-xs">
            <div className="flex items-start justify-between py-2 border-b border-slate-100 gap-4">
              <span className="text-slate-400 font-medium flex items-center gap-1.5 shrink-0">
                <HardDrive className="w-3.5 h-3.5" /> Ukuran
              </span>
              <span className="font-mono font-bold text-slate-800 text-right">
                {file.is_dir ? 'Direktori' : `${formatBytes(file.size_bytes)} (${file.size_bytes.toLocaleString()} bytes)`}
              </span>
            </div>

            <div className="flex items-start justify-between py-2 border-b border-slate-100 gap-4">
              <span className="text-slate-400 font-medium flex items-center gap-1.5 shrink-0">
                <Calendar className="w-3.5 h-3.5" /> Modifikasi
              </span>
              <span className="font-medium text-slate-800 text-right">
                {formatDate(file.modified_at)}
              </span>
            </div>

            <div className="flex items-start justify-between py-2 border-b border-slate-100 gap-4">
              <span className="text-slate-400 font-medium flex items-center gap-1.5 shrink-0">
                <Shield className="w-3.5 h-3.5" /> Izin Akses
              </span>
              <span className="font-mono font-bold text-slate-800 text-right">
                {file.permissions || (file.is_dir ? 'drwxr-xr-x' : '-rw-r--r--')}
              </span>
            </div>

            <div className="py-2 space-y-1">
              <span className="text-slate-400 font-medium flex items-center gap-1.5">
                <Folder className="w-3.5 h-3.5" /> Lokasi Penuh
              </span>
              <p className="font-mono text-[10px] text-slate-600 break-all bg-slate-50 p-2.5 rounded-xl border border-slate-100 select-all">
                {file.path}
              </p>
            </div>
          </div>
        </div>
      </div>
    </aside>
  );
}
