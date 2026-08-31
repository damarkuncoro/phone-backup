import { useEffect, useRef } from 'react';
import {
  Download, Copy, Trash2, Info, Folder, Edit3
} from 'lucide-react';
import { type FileEntry } from '@/services/deviceService';

interface FileContextMenuProps {
  x: number;
  y: number;
  file: FileEntry;
  onClose: () => void;
  onDownload: (file: FileEntry) => void;
  onCopyPath: (path: string) => void;
  onInspect: (file: FileEntry) => void;
  onOpenFolder?: (path: string) => void;
  onRename?: (file: FileEntry) => void;
  onDelete?: (file: FileEntry) => void;
}

export function FileContextMenu({
  x,
  y,
  file,
  onClose,
  onDownload,
  onCopyPath,
  onInspect,
  onOpenFolder,
  onRename,
  onDelete
}: FileContextMenuProps) {
  const menuRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        onClose();
      }
    };
    window.addEventListener('click', handleClickOutside);
    window.addEventListener('contextmenu', handleClickOutside);
    return () => {
      window.removeEventListener('click', handleClickOutside);
      window.removeEventListener('contextmenu', handleClickOutside);
    };
  }, [onClose]);

  // Adjust coordinates if menu would overflow window edges
  const style = {
    top: Math.min(y, window.innerHeight - 260),
    left: Math.min(x, window.innerWidth - 220)
  };

  return (
    <div
      ref={menuRef}
      style={style}
      className="fixed z-50 w-56 bg-white/95 backdrop-blur-md rounded-2xl border border-slate-200/90 shadow-2xl p-1.5 animate-in fade-in zoom-in-95 duration-150 text-xs font-bold text-slate-700 select-none space-y-0.5"
    >
      <div className="px-3 py-1.5 border-b border-slate-100 mb-1">
        <p className="font-black text-slate-900 truncate text-[11px]">{file.name}</p>
        <p className="text-[9px] text-slate-400 uppercase tracking-tight">{file.is_dir ? 'Direktori' : 'Berkas'}</p>
      </div>

      {file.is_dir && onOpenFolder && (
        <button
          type="button"
          onClick={() => { onOpenFolder(file.path); onClose(); }}
          className="w-full flex items-center gap-2.5 px-3 py-2 rounded-xl hover:bg-indigo-50 hover:text-indigo-600 transition-all text-left"
        >
          <Folder className="w-4 h-4 text-indigo-600" /> Buka Folder
        </button>
      )}

      <button
        type="button"
        onClick={() => { onDownload(file); onClose(); }}
        className="w-full flex items-center gap-2.5 px-3 py-2 rounded-xl hover:bg-indigo-50 hover:text-indigo-600 transition-all text-left"
      >
        <Download className="w-4 h-4 text-slate-400" /> Unduh ke PC
      </button>

      <button
        type="button"
        onClick={() => { onCopyPath(file.path); onClose(); }}
        className="w-full flex items-center gap-2.5 px-3 py-2 rounded-xl hover:bg-indigo-50 hover:text-indigo-600 transition-all text-left"
      >
        <Copy className="w-4 h-4 text-slate-400" /> Salin Path Lengkap
      </button>

      <button
        type="button"
        onClick={() => { onInspect(file); onClose(); }}
        className="w-full flex items-center gap-2.5 px-3 py-2 rounded-xl hover:bg-indigo-50 hover:text-indigo-600 transition-all text-left"
      >
        <Info className="w-4 h-4 text-slate-400" /> Detail & Checksum
      </button>

      {onRename && (
        <button
          type="button"
          onClick={() => { onRename(file); onClose(); }}
          className="w-full flex items-center gap-2.5 px-3 py-2 rounded-xl hover:bg-indigo-50 hover:text-indigo-600 transition-all text-left"
        >
          <Edit3 className="w-4 h-4 text-slate-400" /> Ganti Nama
        </button>
      )}

      {onDelete && (
        <div className="pt-1 border-t border-slate-100 mt-1">
          <button
            type="button"
            onClick={() => { onDelete(file); onClose(); }}
            className="w-full flex items-center gap-2.5 px-3 py-2 rounded-xl hover:bg-red-50 text-red-600 transition-all text-left"
          >
            <Trash2 className="w-4 h-4" /> Hapus
          </button>
        </div>
      )}
    </div>
  );
}
