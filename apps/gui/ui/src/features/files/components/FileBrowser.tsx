import { useState, useMemo } from 'react';
import { Folder, Upload } from 'lucide-react';
import { getDeviceId, type FileEntry } from '@/services/deviceService';
import { formatBytes } from '@/shared/lib/formatters';
import { useFileBrowser } from '../hooks/useFileBrowser';
import { useFileActions } from '../hooks/useFileActions';
import { BreadcrumbNav } from './BreadcrumbNav';
import { StorageSidebar } from './StorageSidebar';
import { FileInspector } from './FileInspector';
import { FileContextMenu } from './FileContextMenu';
import { FileBrowserHeader } from './FileBrowserHeader';
import { FileSelectionBar } from './FileSelectionBar';
import { FileTableView } from './FileTableView';
import { FileGridView } from './FileGridView';
import { FileBrowserModals } from './FileBrowserModals';

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

  const [showSidebar, setShowSidebar] = useState(true);
  const [inspectedFile, setInspectedFile] = useState<FileEntry | null>(null);
  const [contextMenu, setContextMenu] = useState<{ x: number; y: number; file: FileEntry } | null>(null);

  const actions = useFileActions({
    selectedDeviceId,
    currentPath,
    refresh,
    selectedPaths: selection.selectedPaths,
    clearSelection: () => selection.clear()
  });

  const selectedDevice = useMemo(() => {
    return devices.find(d => getDeviceId(d) === selectedDeviceId) || null;
  }, [devices, selectedDeviceId]);

  const handleContextMenu = (e: React.MouseEvent, file: FileEntry) => {
    e.preventDefault();
    e.stopPropagation();
    setContextMenu({ x: e.clientX, y: e.clientY, file });
  };

  return (
    <div className="h-full flex flex-col bg-white animate-in fade-in duration-500 relative overflow-hidden">
      <input ref={actions.fileInputRef} type="file" className="hidden" onChange={actions.handleUploadInput} />

      <FileBrowserModals
        confirmModal={actions.confirmModal}
        onCloseConfirm={() => actions.setConfirmModal(prev => ({ ...prev, isOpen: false }))}
        renameTarget={actions.renameTarget}
        onCloseRename={() => actions.setRenameTarget(null)}
        onRename={(name) => actions.handleRename(name, (f) => setInspectedFile(f))}
        toastMessage={actions.toastMessage}
      />

      {contextMenu && (
        <FileContextMenu
          x={contextMenu.x}
          y={contextMenu.y}
          file={contextMenu.file}
          onClose={() => setContextMenu(null)}
          onDownload={actions.handleDownload}
          onCopyPath={(p) => actions.handleCopyPath(p)}
          onInspect={(f) => setInspectedFile(f)}
          onOpenFolder={(p) => setCurrentPath(p)}
          onRename={(f) => actions.setRenameTarget(f)}
          onDelete={(f) => actions.handleDeleteSingle(f, () => inspectedFile?.path === f.path && setInspectedFile(null))}
        />
      )}

      <FileSelectionBar
        count={selection.count}
        onClearSelection={() => selection.clear()}
        onDownloadSelected={actions.handleDownloadSelected}
        onDeleteSelected={actions.handleDeleteSelected}
      />

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

      <div className="flex-1 flex overflow-hidden">
        {showSidebar && (
          <StorageSidebar currentPath={currentPath} selectedDevice={selectedDevice} onNavigate={(p) => setCurrentPath(p)} />
        )}

        <div className="flex-1 flex flex-col min-w-0 bg-white">
          <div className="p-4 border-b border-slate-100 flex items-center justify-between gap-3 bg-white">
            <div className="flex-1 min-w-0">
              <BreadcrumbNav currentPath={currentPath} breadcrumbs={breadcrumbs} onNavigate={setCurrentPath} onRefresh={refresh} isLoading={loading} />
            </div>
            <button
              type="button"
              onClick={() => actions.fileInputRef.current?.click()}
              title="Unggah berkas dari komputer ke folder ini di ponsel"
              className="flex items-center gap-2 px-4 py-2 bg-indigo-50 hover:bg-indigo-100 text-indigo-700 border border-indigo-200/60 rounded-xl text-xs font-bold transition-all shadow-sm active:scale-95 shrink-0"
            >
              <Upload className="w-3.5 h-3.5" />
              <span>Unggah ke HP</span>
            </button>
          </div>

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
                  onRename={(f) => actions.setRenameTarget(f)}
                  onInspect={(f) => setInspectedFile(f)}
                  onCopyPath={(p) => actions.handleCopyPath(p)}
                  onDownload={actions.handleDownload}
                  onDelete={(f) => actions.handleDeleteSingle(f, () => inspectedFile?.path === f.path && setInspectedFile(null))}
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

          <footer className="px-6 py-3 border-t border-slate-100 bg-white/90 flex justify-between items-center text-[10px] font-black text-slate-400 uppercase tracking-widest shrink-0">
            <div className="flex gap-4">
              <span>{filteredFiles.filter(f => f.is_dir).length} Direktori</span>
              <span>{filteredFiles.filter(f => !f.is_dir).length} Berkas</span>
              {selection.count > 0 && <span className="text-indigo-600 font-black">({selection.count} Terpilih)</span>}
            </div>
            <div>Total: {formatBytes(filteredFiles.reduce((acc, f) => acc + (f.size_bytes || 0), 0))}</div>
          </footer>
        </div>

        {inspectedFile && (
          <FileInspector
            file={inspectedFile}
            deviceId={selectedDeviceId || ''}
            onClose={() => setInspectedFile(null)}
            onDownload={actions.handleDownload}
            onNavigate={(p) => setCurrentPath(p)}
            onRename={(f) => actions.setRenameTarget(f)}
            onDelete={(f) => actions.handleDeleteSingle(f, () => setInspectedFile(null))}
          />
        )}
      </div>
    </div>
  );
}
