import { CheckCircle2 } from 'lucide-react';
import { ConfirmModal } from '@/shared/components/ConfirmModal';
import { RenameModal } from '@/shared/components/RenameModal';
import type { FileEntry } from '@/services/deviceService';

interface FileBrowserModalsProps {
  confirmModal: {
    isOpen: boolean;
    title: string;
    message: string;
    type?: 'danger' | 'warning' | 'info';
    onConfirm: () => void;
  };
  onCloseConfirm: () => void;
  renameTarget: FileEntry | null;
  onCloseRename: () => void;
  onRename: (newName: string) => void;
  toastMessage: string | null;
}

export function FileBrowserModals({
  confirmModal,
  onCloseConfirm,
  renameTarget,
  onCloseRename,
  onRename,
  toastMessage
}: FileBrowserModalsProps) {
  return (
    <>
      <ConfirmModal
        isOpen={confirmModal.isOpen}
        title={confirmModal.title}
        message={confirmModal.message}
        type={confirmModal.type}
        onClose={onCloseConfirm}
        onConfirm={confirmModal.onConfirm}
        confirmText="Ya, Lanjutkan"
        cancelText="Batal"
      />

      <RenameModal
        isOpen={renameTarget !== null}
        onClose={onCloseRename}
        currentName={renameTarget?.name || ''}
        isDir={renameTarget?.is_dir || false}
        onRename={onRename}
      />

      {toastMessage && (
        <div className="fixed bottom-6 right-6 z-50 bg-slate-900/95 text-white px-5 py-3 rounded-2xl shadow-2xl backdrop-blur-md flex items-center gap-3 border border-white/10 animate-in slide-in-from-bottom-3 duration-200">
          <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0" />
          <span className="text-xs font-bold">{toastMessage}</span>
        </div>
      )}
    </>
  );
}
