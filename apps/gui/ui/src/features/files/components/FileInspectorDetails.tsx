import { HardDrive, Calendar, Shield, Folder } from 'lucide-react';
import type { FileEntry } from '@/services/deviceService';
import { formatBytes, formatDate } from '@/shared/lib/formatters';

export function FileInspectorDetails({ file }: { file: FileEntry }) {
  return (
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
  );
}
