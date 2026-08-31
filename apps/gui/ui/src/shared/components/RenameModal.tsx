import { useState, useEffect } from 'react';
import { X, Edit3, Check } from 'lucide-react';

interface RenameModalProps {
  isOpen: boolean;
  onClose: () => void;
  onRename: (newName: string) => void;
  currentName: string;
  isDir?: boolean;
}

export function RenameModal({
  isOpen,
  onClose,
  onRename,
  currentName,
  isDir = false
}: RenameModalProps) {
  const [name, setName] = useState(currentName);

  useEffect(() => {
    setName(currentName);
  }, [currentName, isOpen]);

  if (!isOpen) return null;

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (name.trim() && name.trim() !== currentName) {
      onRename(name.trim());
      onClose();
    }
  };

  return (
    <div className="fixed inset-0 z-[200] flex items-center justify-center p-4 animate-in fade-in duration-200">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-slate-950/60 backdrop-blur-sm"
        onClick={onClose}
      />

      {/* Modal Card */}
      <div className="relative bg-white w-full max-w-md rounded-[32px] shadow-2xl shadow-slate-950/20 overflow-hidden animate-in zoom-in-95 slide-in-from-bottom-4 duration-300">
        <form onSubmit={handleSubmit}>
          <div className="p-8 space-y-5">
            <div className="flex justify-between items-start">
              <div className="w-12 h-12 rounded-2xl bg-indigo-50 text-indigo-600 flex items-center justify-center shadow-inner">
                <Edit3 className="w-6 h-6" />
              </div>
              <button
                type="button"
                onClick={onClose}
                className="p-2 hover:bg-slate-50 rounded-xl text-slate-400 transition-colors"
              >
                <X className="w-5 h-5" />
              </button>
            </div>

            <div>
              <h3 className="text-xl font-black text-slate-900 tracking-tight">
                Ganti Nama {isDir ? 'Folder' : 'Berkas'}
              </h3>
              <p className="text-xs text-slate-400 font-medium mt-1">
                Masukkan nama baru untuk item ini di penyimpanan ponsel.
              </p>
            </div>

            <div className="space-y-1.5">
              <label className="text-[10px] font-black uppercase tracking-widest text-slate-400">
                Nama Baru
              </label>
              <input
                type="text"
                autoFocus
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="Nama berkas..."
                className="w-full bg-slate-50 border border-slate-200 focus:border-indigo-500 focus:ring-4 focus:ring-indigo-500/10 px-4 py-3 rounded-2xl text-sm font-bold text-slate-800 outline-none transition-all"
              />
            </div>
          </div>

          <div className="p-6 bg-slate-50/50 flex gap-3 border-t border-slate-100">
            <button
              type="button"
              onClick={onClose}
              className="flex-1 py-3.5 bg-white border border-slate-200 text-slate-600 rounded-2xl font-black text-xs uppercase tracking-widest hover:bg-slate-100 transition-all"
            >
              Batal
            </button>
            <button
              type="submit"
              disabled={!name.trim() || name.trim() === currentName}
              className="flex-1 py-3.5 bg-indigo-600 hover:bg-indigo-700 disabled:opacity-50 text-white rounded-2xl font-black text-xs uppercase tracking-widest transition-all shadow-xl shadow-indigo-200 flex items-center justify-center gap-2 active:scale-95"
            >
              <Check className="w-4 h-4" /> Simpan Nama
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
