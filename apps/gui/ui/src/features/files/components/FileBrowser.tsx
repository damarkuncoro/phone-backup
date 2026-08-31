import { useState, useMemo, useRef } from 'react';
import { Folder, CheckCircle2, Upload } from 'lucide-react';
import { getDeviceId, type FileEntry, deviceService } from '@/services/deviceService';
import { formatBytes } from '@/shared/lib/formatters';
import { useFileBrowser } from '../hooks/useFileBrowser';
import { BreadcrumbNav } from './BreadcrumbNav';
import { StorageSidebar } from './StorageSidebar';
import { FileInspector } from './FileInspector';
import { FileContextMenu } from './FileContextMenu';
import { FileBrowserHeader } from './FileBrowserHeader';
import { FileSelectionBar } from './FileSelectionBar';
import { FileTableView } from './FileTableView';
import { FileGridView } from './FileGridView';
import { ConfirmModal } from '@/shared/components/ConfirmModal';
import { RenameModal } from '@/shared/components/RenameModal';

export function FileBrowser() {
  const {
    devices,
    selectedDeviceId, setSelectedDeviceId,
    currentPath, setCurrentPath,
    loading,
    searchQuery, setSearchQuery,
    viewMode, setViewMode,
    sortBy, sortDirection, handleSort,
    selection,
    filteredFiles,
    breadcrumbs,
    refresh
  } = useFileBrowser();

  // Panels, Modals, & Toasts State
  const [showSidebar, setShowSidebar] = useState(true);
  const [inspectedFile, setInspectedFile] = useState<FileEntry | null>(null);
  const [contextMenu, setContextMenu] = useState<{ x: number; y: number; file: FileEntry } | null>(null);
  const [toastMessage, setToastMessage] = useState<string | null>(null);

  // Dialog State
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

  const [renameTarget, setRenameTarget] = useState<FileEntry | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const selectedDevice = useMemo(() => {
    return devices.find(d => getDeviceId(d) === selectedDeviceId) || null;
  }, [devices, selectedDeviceId]);

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
        console.error("Download failed", err);
        showToast(`Download gagal: ${err}`);
      }
    }
  };

  const handleDownloadSelected = async () => {
    const paths = Array.from(selection.selectedPaths);
    if (paths.length === 0 || !selectedDeviceId) return;

    showToast(`Mengunduh ${paths.length} berkas ke workspace/downloads...`);
    try {
      for (const p of paths) {
        await deviceService.downloadFile(selectedDeviceId, p, "workspace/downloads");
      }
      showToast(`Berhasil mengunduh ${paths.length} berkas ke workspace/downloads`);
    } catch (err) {
      showToast(`Terjadi kesalahan saat mengunduh: ${err}`);
    }
  };

  // Single File/Folder Delete
  const handleDeleteSingle = (file: FileEntry) => {
    if (!selectedDeviceId) return;
    setConfirmModal({
      isOpen: true,
      title: "Konfirmasi Hapus",
      message: `Hapus ${file.is_dir ? 'direktori' : 'berkas'} "${file.name}" dari penyimpanan ponsel secara permanen?`,
      type: 'danger',
      onConfirm: async () => {
        try {
          await deviceService.deleteFile(selectedDeviceId, file.path);
          showToast(`Berhasil menghapus: ${file.name}`);
          if (inspectedFile?.path === file.path) {
            setInspectedFile(null);
          }
          refresh();
        } catch (err) {
          console.error("Hapus gagal", err);
          showToast(`Gagal menghapus: ${err}`);
        }
      }
    });
  };

  // Bulk Selected Delete
  const handleDeleteSelected = () => {
    const paths = Array.from(selection.selectedPaths);
    if (paths.length === 0 || !selectedDeviceId) return;

    setConfirmModal({
      isOpen: true,
      title: "Hapus Item Terpilih",
      message: `Hapus ${paths.length} item terpilih dari penyimpanan ponsel secara permanen? Tindakan ini tidak dapat dibatalkan.`,
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
        selection.clear();
        showToast(`Berhasil menghapus ${successCount} dari ${paths.length} item`);
        refresh();
      }
    });
  };

  // Rename File/Folder
  const handleRename = async (newName: string) => {
    if (!renameTarget || !selectedDeviceId) return;
    
    const parts = renameTarget.path.split('/');
    parts[parts.length - 1] = newName;
    const newPath = parts.join('/');

    try {
      await deviceService.renameFile(selectedDeviceId, renameTarget.path, newPath);
      showToast(`Nama diubah menjadi: ${newName}`);
      if (inspectedFile?.path === renameTarget.path) {
        setInspectedFile({ ...renameTarget, name: newName, path: newPath });
      }
      setRenameTarget(null);
      refresh();
    } catch (err) {
      console.error("Rename failed", err);
      showToast(`Gagal mengubah nama: ${err}`);
    }
  };

  // Upload File Trigger
  const handleUploadInput = async (e: React.ChangeEvent<HTMLInputElement>) => {
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
      console.warn("Upload via path failed, retrying:", err);
      showToast(`Gagal mengunggah berkas: ${err}`);
    } finally {
      if (fileInputRef.current) fileInputRef.current.value = '';
    }
  };

  const handleContextMenu = (e: React.MouseEvent, file: FileEntry) => {
    e.preventDefault();
    e.stopPropagation();
    setContextMenu({ x: e.clientX, y: e.clientY, file });
  };

  return (
    <div className="h-full flex flex-col bg-white animate-in fade-in duration-500 relative overflow-hidden">

      {/* Hidden File Input for Upload */}
      <input
        ref={fileInputRef}
        type="file"
        className="hidden"
        onChange={handleUploadInput}
      />

      {/* Confirm Action Dialog */}
      <ConfirmModal
        isOpen={confirmModal.isOpen}
        title={confirmModal.title}
        message={confirmModal.message}
        type={confirmModal.type}
        onClose={() => setConfirmModal(prev => ({ ...prev, isOpen: false }))}
        onConfirm={confirmModal.onConfirm}
        confirmText="Ya, Lanjutkan"
        cancelText="Batal"
      />

      {/* Rename Dialog */}
      <RenameModal
        isOpen={renameTarget !== null}
        onClose={() => setRenameTarget(null)}
        currentName={renameTarget?.name || ''}
        isDir={renameTarget?.is_dir || false}
        onRename={handleRename}
      />

      {/* Floating Toast Notification */}
      {toastMessage && (
        <div className="fixed bottom-6 right-6 z-50 bg-slate-900/95 text-white px-5 py-3 rounded-2xl shadow-2xl backdrop-blur-md flex items-center gap-3 border border-white/10 animate-in slide-in-from-bottom-3 duration-200">
          <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0" />
          <span className="text-xs font-bold">{toastMessage}</span>
        </div>
      )}

      {/* Right-Click Context Menu */}
      {contextMenu && (
        <FileContextMenu
          x={contextMenu.x}
          y={contextMenu.y}
          file={contextMenu.file}
          onClose={() => setContextMenu(null)}
          onDownload={handleDownload}
          onCopyPath={(p) => handleCopyPath(p)}
          onInspect={(f) => setInspectedFile(f)}
          onOpenFolder={(p) => setCurrentPath(p)}
          onRename={(f) => setRenameTarget(f)}
          onDelete={(f) => handleDeleteSingle(f)}
        />
      )}

      {/* Dynamic Action Bar for Selection */}
      <FileSelectionBar
        count={selection.count}
        onClearSelection={() => selection.clear()}
        onDownloadSelected={handleDownloadSelected}
        onDeleteSelected={handleDeleteSelected}
      />

      {/* Main Header */}
      <FileBrowserHeader
        showSidebar={showSidebar}
        onToggleSidebar={() => setShowSidebar(!showSidebar)}
        searchQuery={searchQuery}
        onSearchChange={setSearchQuery}
        viewMode={viewMode}
        onViewModeChange={setViewMode}
        devices={devices}
        selectedDeviceId={selectedDeviceId}
        onSelectDevice={setSelectedDeviceId}
      />

      {/* Main Dual-Pane Body */}
      <div className="flex-1 flex overflow-hidden">

        {/* Left Storage & Shortcuts Sidebar */}
        {showSidebar && (
          <StorageSidebar
            currentPath={currentPath}
            selectedDevice={selectedDevice}
            onNavigate={(p) => setCurrentPath(p)}
          />
        )}

        {/* Middle Area: Breadcrumbs + Files Table/Grid */}
        <div className="flex-1 flex flex-col min-w-0 bg-white">

          {/* Breadcrumbs & Toolbar Bar */}
          <div className="p-4 border-b border-slate-100 flex items-center justify-between gap-3 bg-white">
            <div className="flex-1 min-w-0">
              <BreadcrumbNav
                currentPath={currentPath}
                breadcrumbs={breadcrumbs}
                onNavigate={setCurrentPath}
                onRefresh={refresh}
                isLoading={loading}
              />
            </div>

            {/* Quick Upload Action */}
            <div className="flex items-center gap-2 shrink-0">
              <button
                type="button"
                onClick={() => fileInputRef.current?.click()}
                title="Unggah berkas dari komputer ke folder ini di ponsel"
                className="flex items-center gap-2 px-4 py-2 bg-indigo-50 hover:bg-indigo-100 text-indigo-700 border border-indigo-200/60 rounded-xl text-xs font-bold transition-all shadow-sm active:scale-95"
              >
                <Upload className="w-3.5 h-3.5" />
                <span>Unggah ke HP</span>
              </button>
            </div>
          </div>

          {/* Files Container */}
          <div className="flex-1 overflow-y-auto custom-scrollbar">
            {loading ? (
              <div className="h-full flex flex-col items-center justify-center gap-4 text-slate-400 py-24">
                <div className="w-12 h-12 border-4 border-slate-100 border-t-indigo-600 rounded-full animate-spin" />
                <p className="text-[10px] font-black uppercase tracking-[0.2em]">Menjelajahi Memori HP...</p>
              </div>
            ) : filteredFiles.length > 0 ? (
              viewMode === 'list' ? (
                <FileTableView
                  files={filteredFiles}
                  selectedPaths={selection.selectedPaths}
                  inspectedFile={inspectedFile}
                  sortBy={sortBy}
                  sortDirection={sortDirection}
                  onSort={handleSort}
                  onToggleSelect={(p) => selection.toggle(p)}
                  onToggleSelectAll={() => selection.count === filteredFiles.length ? selection.clear() : selection.selectAll(filteredFiles.map(f => f.path))}
                  onSelectFile={(f) => setInspectedFile(f)}
                  onOpenFolder={(p) => setCurrentPath(p)}
                  onContextMenu={handleContextMenu}
                  onRename={(f) => setRenameTarget(f)}
                  onInspect={(f) => setInspectedFile(f)}
                  onCopyPath={(p) => handleCopyPath(p)}
                  onDownload={handleDownload}
                  onDelete={handleDeleteSingle}
                />
              ) : (
                <FileGridView
                  files={filteredFiles}
                  selectedPaths={selection.selectedPaths}
                  inspectedFile={inspectedFile}
                  onToggleSelect={(p) => selection.toggle(p)}
                  onSelectFile={(f) => setInspectedFile(f)}
                  onOpenFolder={(p) => setCurrentPath(p)}
                  onContextMenu={handleContextMenu}
                  onInspect={(f) => setInspectedFile(f)}
                />
              )
            ) : (
              <div className="h-full flex flex-col items-center justify-center bg-slate-50/50 rounded-[32px] m-6 border-2 border-dashed border-slate-100 text-slate-300 py-24">
                <div className="w-16 h-16 bg-white rounded-3xl flex items-center justify-center shadow-sm mb-4">
                  <Folder className="w-8 h-8 opacity-20" />
                </div>
                <p className="font-black uppercase tracking-[0.2em] text-[10px]">Folder Kosong</p>
                <p className="text-xs mt-1 text-slate-400">Tidak ada file yang sesuai dengan filter di direktori ini.</p>
              </div>
            )}
          </div>

          {/* Footer Status Bar */}
          <footer className="px-6 py-3 border-t border-slate-100 bg-white/90 flex justify-between items-center text-[10px] font-black text-slate-400 uppercase tracking-widest shrink-0">
            <div className="flex gap-4">
              <span>{filteredFiles.filter(f => f.is_dir).length} Direktori</span>
              <span>{filteredFiles.filter(f => !f.is_dir).length} Berkas</span>
              {selection.count > 0 && (
                <span className="text-indigo-600 font-black">({selection.count} Terpilih)</span>
              )}
            </div>
            <div>
              Total: {formatBytes(filteredFiles.reduce((acc, f) => acc + (f.size_bytes || 0), 0))}
            </div>
          </footer>
        </div>

        {/* Right File Inspector Panel */}
        {inspectedFile && (
          <FileInspector
            file={inspectedFile}
            deviceId={selectedDeviceId || ''}
            onClose={() => setInspectedFile(null)}
            onDownload={handleDownload}
            onNavigate={(p) => setCurrentPath(p)}
            onRename={(f) => setRenameTarget(f)}
            onDelete={(f) => handleDeleteSingle(f)}
          />
        )}
      </div>
    </div>
  );
}
