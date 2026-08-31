import { X, AlertCircle, Info } from 'lucide-react';
import { cn } from "../lib/utils";

interface ConfirmModalProps {
  isOpen: boolean;
  onClose: () => void;
  onConfirm: () => void;
  title: string;
  message: string;
  confirmText?: string;
  cancelText?: string;
  type?: 'danger' | 'info' | 'warning';
}

export function ConfirmModal({
  isOpen,
  onClose,
  onConfirm,
  title,
  message,
  confirmText = "Lanjutkan",
  cancelText = "Batal",
  type = 'info'
}: ConfirmModalProps) {
  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-[200] flex items-center justify-center p-4 animate-in fade-in duration-200">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-slate-950/60 backdrop-blur-sm"
        onClick={onClose}
      />

      {/* Modal Card */}
      <div className="relative bg-white w-full max-w-md rounded-[32px] shadow-2xl shadow-slate-950/20 overflow-hidden animate-in zoom-in-95 slide-in-from-bottom-4 duration-300">
        <div className="p-8">
            <div className="flex justify-between items-start mb-6">
                <div className={cn(
                    "w-12 h-12 rounded-2xl flex items-center justify-center shadow-inner",
                    type === 'danger' ? "bg-red-50 text-red-500" :
                    type === 'warning' ? "bg-amber-50 text-amber-500" :
                    "bg-indigo-50 text-indigo-600"
                )}>
                    {type === 'danger' ? <X className="w-6 h-6" /> :
                     type === 'warning' ? <AlertCircle className="w-6 h-6" /> :
                     <Info className="w-6 h-6" />}
                </div>
                <button
                    onClick={onClose}
                    className="p-2 hover:bg-slate-50 rounded-xl text-slate-400 transition-colors"
                >
                    <X className="w-5 h-5" />
                </button>
            </div>

            <h3 className="text-xl font-black text-slate-900 tracking-tight mb-2">{title}</h3>
            <p className="text-slate-500 font-medium leading-relaxed">{message}</p>
        </div>

        <div className="p-6 bg-slate-50/50 flex gap-3">
            <button
                onClick={onClose}
                className="flex-1 py-4 bg-white border border-slate-200 text-slate-600 rounded-2xl font-black text-xs uppercase tracking-widest hover:bg-slate-100 transition-all"
            >
                {cancelText}
            </button>
            <button
                onClick={() => {
                    onConfirm();
                    onClose();
                }}
                className={cn(
                    "flex-1 py-4 text-white rounded-2xl font-black text-xs uppercase tracking-widest transition-all shadow-xl active:scale-95",
                    type === 'danger' ? "bg-red-600 hover:bg-red-700 shadow-red-200" :
                    type === 'warning' ? "bg-amber-500 hover:bg-amber-600 shadow-amber-200" :
                    "bg-indigo-600 hover:bg-indigo-700 shadow-indigo-200"
                )}
            >
                {confirmText}
            </button>
        </div>
      </div>
    </div>
  );
}
