import { useState, useRef, type ChangeEvent } from 'react';
import { deviceService, type FileEntry } from '@/services/deviceService';

interface UseFileActionsProps {
  selectedDeviceId: string | null;
  currentPath: string;
  refresh: () => void;
  selectedPaths: string[];
  clearSelection: () => void;
}

export function useFileActions({
  selectedDeviceId,
  currentPath,
  refresh,
  selectedPaths,
  clearSelection
}: UseFileActionsProps) {
  const [toastMessage, setToastMessage] = useState<string | null>(null);
  const [renameTarget, setRenameTarget] = useState<FileEntry | null>(null);
  const [confirmModal, setConfirmModal] = useState<{
    isOpen: boolean;
    title: string;
    message: string;
    type?: 'danger' | 'warning' | 'info';
    onConfirm: () => void;
  }>({
    isOpen: false,
    title: '',
    message: '',
    onConfirm: () => {}
  });

  const fileInputRef = useRef<HTMLInputElement>(null);

  const showToast = (msg: string) => {
    setToastMessage(msg);
    setTimeout(() => setToastMessage(null), 3000);
  };

  const handleCopyPath = (path: string, e?: React.MouseEvent) => {
    if (e) e.stopPropagation();
    navigator.clipboard.writeText(path);
    showToast(`Path disalin: ${path}`);
  };

  const handleDownload = async (file: FileEntry) => {
    showToast(`Memulai download: ${file.name}`);
    if (selectedDeviceId) {
      try {
        await deviceService.downloadFile(selectedDeviceId, file.path, "workspace/downloads");
        showToast(`Tersimpan di workspace/downloads: ${file.name}`);
      } catch (err) {
        showToast(`Download gagal: ${err}`);
      }
    }
  };

  const handleDownloadSelected = async () => {
    const paths = Array.from(selectedPaths);
    if (paths.length === 0 || !selectedDeviceId) return;

    showToast(`Mengunduh ${paths.length} berkas ke workspace/downloads...`);
    try {
      for (const p of paths) {
        await deviceService.downloadFile(selectedDeviceId, p, "workspace/downloads");
      }
      showToast(`Berhasil mengunduh ${paths.length} berkas`);
    } catch (err) {
      showToast(`Terjadi kesalahan saat mengunduh: ${err}`);
    }
  };

  const handleDeleteSingle = (file: FileEntry, onDeleted?: () => void) => {
    if (!selectedDeviceId) return;
    setConfirmModal({
      isOpen: true,
      title: "Konfirmasi Hapus",
      message: `Hapus ${file.is_dir ? 'direktori' : 'berkas'} "${file.name}" secara permanen?`,
      type: 'danger',
      onConfirm: async () => {
        try {
          await deviceService.deleteFile(selectedDeviceId, file.path);
          showToast(`Berhasil menghapus: ${file.name}`);
          onDeleted?.();
          refresh();
        } catch (err) {
          showToast(`Gagal menghapus: ${err}`);
        }
      }
    });
  };

  const handleDeleteSelected = () => {
    const paths = Array.from(selectedPaths);
    if (paths.length === 0 || !selectedDeviceId) return;

    setConfirmModal({
      isOpen: true,
      title: "Hapus Item Terpilih",
      message: `Hapus ${paths.length} item terpilih secara permanen?`,
      type: 'danger',
      onConfirm: async () => {
        showToast(`Menghapus ${paths.length} item...`);
        let successCount = 0;
        for (const p of paths) {
          try {
            await deviceService.deleteFile(selectedDeviceId, p);
            successCount++;
          } catch (err) {
            console.error(`Gagal menghapus ${p}:`, err);
          }
        }
        clearSelection();
        showToast(`Berhasil menghapus ${successCount} dari ${paths.length} item`);
        refresh();
      }
    });
  };

  const handleRename = async (newName: string, onRenamed?: (updated: FileEntry) => void) => {
    if (!renameTarget || !selectedDeviceId) return;
    const parts = renameTarget.path.split('/');
    parts[parts.length - 1] = newName;
    const newPath = parts.join('/');

    try {
      await deviceService.renameFile(selectedDeviceId, renameTarget.path, newPath);
      showToast(`Nama diubah menjadi: ${newName}`);
      onRenamed?.({ ...renameTarget, name: newName, path: newPath });
      setRenameTarget(null);
      refresh();
    } catch (err) {
      showToast(`Gagal mengubah nama: ${err}`);
    }
  };

  const handleUploadInput = async (e: ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (!files || files.length === 0 || !selectedDeviceId) return;
    const file = files[0];
    showToast(`Mengunggah "${file.name}" ke ${currentPath}...`);
    const targetRemotePath = `${currentPath.replace(/\/$/, '')}/${file.name}`;
    try {
      const localFilePath = (file as any).path || file.name;
      await deviceService.uploadFile(selectedDeviceId, localFilePath, targetRemotePath);
      showToast(`Berhasil mengunggah: ${file.name}`);
      refresh();
    } catch (err) {
      showToast(`Gagal mengunggah berkas: ${err}`);
    } finally {
      if (fileInputRef.current) fileInputRef.current.value = '';
    }
  };

  return {
    toastMessage,
    showToast,
    confirmModal,
    setConfirmModal,
    renameTarget,
    setRenameTarget,
    fileInputRef,
    handleCopyPath,
    handleDownload,
    handleDownloadSelected,
    handleDeleteSingle,
    handleDeleteSelected,
    handleRename,
    handleUploadInput
  };
}
