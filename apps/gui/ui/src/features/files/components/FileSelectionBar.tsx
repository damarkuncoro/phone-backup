import { X, Download, Folder, Trash2 } from 'lucide-react';
import { systemService } from '@/services/systemService';

interface FileSelectionBarProps {
  count: number;
  onClearSelection: () => void;
  onDownloadSelected: () => void;
  onDeleteSelected: () => void;
}

export function FileSelectionBar({
  count,
  onClearSelection,
  onDownloadSelected,
  onDeleteSelected
}: FileSelectionBarProps) {
  if (count === 0) return null;

  return (
    <div className="absolute top-0 left-0 right-0 z-40 bg-indigo-600 text-white p-4 flex items-center justify-between animate-in slide-in-from-top-4 duration-300 shadow-2xl">
      <div className="flex items-center gap-6">
        <button onClick={onClearSelection} className="p-2 hover:bg-white/10 rounded-xl transition-all">
          <X className="w-5 h-5" />
        </button>
        <div>
          <p className="text-[10px] uppercase font-black tracking-widest text-indigo-200">Mode Pilihan</p>
          <p className="font-black text-sm">{count} Item Terpilih</p>
        </div>
      </div>
      <div className="flex items-center gap-2">
        <button
          onClick={onDownloadSelected}
          className="flex items-center gap-2 px-5 py-2.5 bg-white text-indigo-600 hover:bg-indigo-50 rounded-xl text-xs font-black uppercase tracking-wider transition-all shadow-md active:scale-95"
        >
          <Download className="w-4 h-4" /> Download ({count})
        </button>
        <button
          onClick={() => systemService.openDownloadsFolder()}
          title="Buka folder hasil unduhan di komputer (Finder / Explorer)"
          className="flex items-center gap-2 px-4 py-2.5 bg-indigo-700/80 hover:bg-indigo-800 text-white rounded-xl text-xs font-bold transition-all shadow-md active:scale-95"
        >
          <Folder className="w-4 h-4" /> Buka Folder PC
        </button>
        <button
          onClick={onDeleteSelected}
          className="flex items-center gap-2 px-5 py-2.5 bg-red-500 hover:bg-red-600 text-white rounded-xl text-xs font-black uppercase tracking-wider transition-all shadow-md active:scale-95"
        >
          <Trash2 className="w-4 h-4" /> Hapus
        </button>
      </div>
    </div>
  );
}
