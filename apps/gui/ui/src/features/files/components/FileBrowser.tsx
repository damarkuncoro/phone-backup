import { useState, useMemo } from 'react';
import {
  Folder, File, ChevronRight, Search,
  Download, Trash2, Image as ImageIcon, Video, Music, FileText,
  CheckSquare, Square, X, LayoutGrid, LayoutList,
  ArrowUpDown, ArrowUp, ArrowDown, Copy, Info,
  Smartphone, HardDrive, PanelLeftClose, PanelLeft,
  CheckCircle2
} from 'lucide-react';
import { getDeviceId, type FileEntry, deviceService } from '@/services/deviceService';
import { cn } from "../../../shared/lib/utils";
import { formatBytes, formatDate } from '@/shared/lib/formatters';
import { isImage, isVideo, isAudio, isDocument, isApk, type SortField, type FileCategory } from '../lib/fileUtils';
import { useFileBrowser } from '../hooks/useFileBrowser';
import { BreadcrumbNav } from './BreadcrumbNav';
import { StorageSidebar } from './StorageSidebar';
import { FileInspector } from './FileInspector';
import { FileContextMenu } from './FileContextMenu';

export function FileBrowser() {
  const {
    devices,
    selectedDeviceId, setSelectedDeviceId,
    currentPath, setCurrentPath,
    loading,
    searchQuery, setSearchQuery,
    viewMode, setViewMode,
    sortBy, sortDirection, handleSort,
    category, setCategory,
    selection,
    filteredFiles,
    breadcrumbs,
    refresh
  } = useFileBrowser();

  // Panels & Modals State
  const [showSidebar, setShowSidebar] = useState(true);
  const [inspectedFile, setInspectedFile] = useState<FileEntry | null>(null);
  const [contextMenu, setContextMenu] = useState<{ x: number; y: number; file: FileEntry } | null>(null);
  const [toastMessage, setToastMessage] = useState<string | null>(null);

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
        showToast(`Selesai diunduh: ${file.name}`);
      } catch (err) {
        console.error("Download failed", err);
      }
    }
  };

  const handleContextMenu = (e: React.MouseEvent, file: FileEntry) => {
    e.preventDefault();
    e.stopPropagation();
    setContextMenu({ x: e.clientX, y: e.clientY, file });
  };

  const getSortIcon = (field: SortField) => {
    if (sortBy !== field) return <ArrowUpDown className="w-3.5 h-3.5 text-slate-300 opacity-60 group-hover:opacity-100" />;
    return sortDirection === 'asc'
      ? <ArrowUp className="w-3.5 h-3.5 text-indigo-600" />
      : <ArrowDown className="w-3.5 h-3.5 text-indigo-600" />;
  };

  const renderFileIcon = (fileName: string, isDir: boolean) => {
    if (isDir) {
      return (
        <div className="w-10 h-10 rounded-2xl bg-indigo-50 text-indigo-600 flex items-center justify-center shadow-sm shrink-0">
          <Folder className="w-5 h-5 fill-current opacity-80" />
        </div>
      );
    }
    if (isImage(fileName)) {
      return (
        <div className="w-10 h-10 rounded-2xl bg-amber-50 text-amber-600 flex items-center justify-center shadow-sm shrink-0">
          <ImageIcon className="w-5 h-5" />
        </div>
      );
    }
    if (isVideo(fileName)) {
      return (
        <div className="w-10 h-10 rounded-2xl bg-rose-50 text-rose-600 flex items-center justify-center shadow-sm shrink-0">
          <Video className="w-5 h-5" />
        </div>
      );
    }
    if (isAudio(fileName)) {
      return (
        <div className="w-10 h-10 rounded-2xl bg-purple-50 text-purple-600 flex items-center justify-center shadow-sm shrink-0">
          <Music className="w-5 h-5" />
        </div>
      );
    }
    if (isDocument(fileName)) {
      return (
        <div className="w-10 h-10 rounded-2xl bg-blue-50 text-blue-600 flex items-center justify-center shadow-sm shrink-0">
          <FileText className="w-5 h-5" />
        </div>
      );
    }
    if (isApk(fileName)) {
      return (
        <div className="w-10 h-10 rounded-2xl bg-emerald-50 text-emerald-600 flex items-center justify-center shadow-sm shrink-0">
          <Smartphone className="w-5 h-5" />
        </div>
      );
    }
    return (
      <div className="w-10 h-10 rounded-2xl bg-slate-50 text-slate-400 flex items-center justify-center shadow-sm shrink-0">
        <File className="w-5 h-5" />
      </div>
    );
  };

  const categories: { id: FileCategory; label: string }[] = [
    { id: 'all', label: 'Semua' },
    { id: 'folders', label: 'Folder' },
    { id: 'images', label: 'Foto & Gambar' },
    { id: 'videos', label: 'Video' },
    { id: 'documents', label: 'Dokumen' },
    { id: 'audio', label: 'Audio' },
  ];

  return (
    <div className="h-full flex flex-col bg-white animate-in fade-in duration-500 relative overflow-hidden">

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
        />
      )}

      {/* Dynamic Action Bar for Selection */}
      {selection.count > 0 && (
        <div className="absolute top-0 left-0 right-0 z-40 bg-indigo-600 text-white p-4 flex items-center justify-between animate-in slide-in-from-top-4 duration-300 shadow-2xl">
          <div className="flex items-center gap-6">
            <button onClick={() => selection.clear()} className="p-2 hover:bg-white/10 rounded-xl transition-all">
              <X className="w-5 h-5" />
            </button>
            <div>
              <p className="text-[10px] uppercase font-black tracking-widest text-indigo-200">Mode Pilihan</p>
              <p className="font-black text-sm">{selection.count} Item Terpilih</p>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={() => showToast(`Mengunduh ${selection.count} item terpilih...`)}
              className="flex items-center gap-2 px-5 py-2.5 bg-white text-indigo-600 hover:bg-indigo-50 rounded-xl text-xs font-black uppercase tracking-wider transition-all shadow-md"
            >
              <Download className="w-4 h-4" /> Download ({selection.count})
            </button>
            <button className="flex items-center gap-2 px-5 py-2.5 bg-red-500 hover:bg-red-600 rounded-xl text-xs font-black uppercase tracking-wider transition-all shadow-md">
              <Trash2 className="w-4 h-4" /> Hapus
            </button>
          </div>
        </div>
      )}

      {/* Main Header */}
      <header className="px-6 py-4 border-b border-slate-200/80 bg-white/90 backdrop-blur-md flex items-center justify-between gap-4 shrink-0">
        <div className="flex items-center gap-3 min-w-0">
          <button
            onClick={() => setShowSidebar(!showSidebar)}
            title={showSidebar ? "Sembunyikan Panel Storage" : "Tampilkan Panel Storage"}
            className={cn(
              "p-2.5 rounded-xl border transition-all text-slate-500 hover:text-indigo-600 hover:bg-slate-50",
              showSidebar ? "bg-slate-100 border-slate-200 text-indigo-600" : "bg-white border-slate-200/80"
            )}
          >
            {showSidebar ? <PanelLeftClose className="w-4 h-4" /> : <PanelLeft className="w-4 h-4" />}
          </button>

          <div className="min-w-0">
            <h1 className="text-xl font-black text-slate-900 tracking-tight flex items-center gap-2 truncate">
              File Manager
            </h1>
            <p className="text-[10px] font-black text-slate-400 uppercase tracking-widest flex items-center gap-1.5 truncate">
              <HardDrive className="w-3 h-3 text-emerald-500" /> ADB Explorer Pro
            </p>
          </div>
        </div>

        <div className="flex items-center gap-3 flex-1 max-w-xl justify-end">
          {/* Search Box */}
          <div className="flex-1 relative max-w-xs sm:max-w-sm">
            <Search className="absolute left-3.5 top-3 w-4 h-4 text-slate-400" />
            <input
              type="text"
              placeholder="Cari file di folder ini..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full bg-slate-50 border border-slate-200/80 pl-10 pr-4 py-2.5 rounded-2xl text-xs focus:ring-4 focus:ring-indigo-500/10 focus:border-indigo-300 outline-none transition-all"
            />
          </div>

          {/* View Mode Toggle */}
          <div className="flex items-center bg-slate-100 p-1 rounded-2xl border border-slate-200/60 shrink-0">
            <button
              onClick={() => setViewMode('list')}
              title="Tampilan Tabel"
              className={cn(
                "p-2 rounded-xl transition-all",
                viewMode === 'list' ? "bg-white text-indigo-600 shadow-sm" : "text-slate-400 hover:text-slate-700"
              )}
            >
              <LayoutList className="w-4 h-4" />
            </button>
            <button
              onClick={() => setViewMode('grid')}
              title="Tampilan Grid Kartu"
              className={cn(
                "p-2 rounded-xl transition-all",
                viewMode === 'grid' ? "bg-white text-indigo-600 shadow-sm" : "text-slate-400 hover:text-slate-700"
              )}
            >
              <LayoutGrid className="w-4 h-4" />
            </button>
          </div>

          {/* Device Selector */}
          <select
            value={selectedDeviceId || ''}
            onChange={(e) => setSelectedDeviceId(e.target.value)}
            className="bg-slate-50 border border-slate-200/80 px-3.5 py-2.5 rounded-2xl text-xs font-black text-slate-700 outline-none hover:bg-white transition-all cursor-pointer shrink-0"
          >
            {devices.map(d => (
              <option key={getDeviceId(d)} value={getDeviceId(d)}>{d.model}</option>
            ))}
          </select>
        </div>
      </header>

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

          {/* Breadcrumbs & Category Bar */}
          <div className="p-4 border-b border-slate-100 flex flex-col xl:flex-row xl:items-center justify-between gap-3 bg-white">
            <div className="flex-1 min-w-0">
              <BreadcrumbNav
                currentPath={currentPath}
                breadcrumbs={breadcrumbs}
                onNavigate={setCurrentPath}
                onRefresh={refresh}
                isLoading={loading}
              />
            </div>

            {/* Category Filter Pills */}
            <div className="flex items-center gap-1 overflow-x-auto no-scrollbar shrink-0 self-start xl:self-auto py-0.5">
              {categories.map(c => (
                <button
                  key={c.id}
                  onClick={() => setCategory(c.id)}
                  className={cn(
                    "px-3 py-1.5 rounded-xl text-[10px] font-black uppercase tracking-wider transition-all border",
                    category === c.id
                      ? "bg-slate-900 text-white border-slate-900 shadow-sm"
                      : "bg-slate-50 text-slate-500 border-slate-200/50 hover:bg-slate-100 hover:text-slate-800"
                  )}
                >
                  {c.label}
                </button>
              ))}
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
                /* Table Mode */
                <table className="w-full text-left border-separate border-spacing-0">
                  <thead className="sticky top-0 bg-white/95 backdrop-blur-md z-10 border-b border-slate-100 shadow-sm">
                    <tr>
                      <th className="px-6 py-3 w-14">
                        <button
                          onClick={() => selection.count === filteredFiles.length ? selection.clear() : selection.selectAll(filteredFiles.map(f => f.path))}
                          className="text-slate-300 hover:text-indigo-600 transition-all"
                        >
                          {selection.count === filteredFiles.length ? <CheckSquare className="w-5 h-5 text-indigo-600" /> : <Square className="w-5 h-5" />}
                        </button>
                      </th>
                      <th
                        onClick={() => handleSort('name')}
                        className="px-4 py-3 text-[10px] font-black text-slate-400 uppercase tracking-widest cursor-pointer hover:text-indigo-600 select-none group"
                      >
                        <div className="flex items-center gap-2">
                          Nama Berkas {getSortIcon('name')}
                        </div>
                      </th>
                      <th
                        onClick={() => handleSort('size')}
                        className="px-6 py-3 text-[10px] font-black text-slate-400 uppercase tracking-widest text-right cursor-pointer hover:text-indigo-600 select-none group"
                      >
                        <div className="flex items-center justify-end gap-2">
                          Ukuran {getSortIcon('size')}
                        </div>
                      </th>
                      <th
                        onClick={() => handleSort('date')}
                        className="px-6 py-3 text-[10px] font-black text-slate-400 uppercase tracking-widest cursor-pointer hover:text-indigo-600 select-none group"
                      >
                        <div className="flex items-center gap-2">
                          Modifikasi {getSortIcon('date')}
                        </div>
                      </th>
                      <th className="px-6 py-3 w-28"></th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-slate-50">
                    {filteredFiles.map((file, i) => {
                      const isSelected = selection.isSelected(file.path);
                      const isInspected = inspectedFile?.path === file.path;

                      return (
                        <tr
                          key={i}
                          onContextMenu={(e) => handleContextMenu(e, file)}
                          className={cn(
                            "group transition-all cursor-pointer select-none",
                            isInspected ? "bg-indigo-50/90" : isSelected ? "bg-indigo-50/60" : "hover:bg-slate-50/80"
                          )}
                          onClick={() => {
                            if (file.is_dir) {
                              setCurrentPath(file.path);
                            } else {
                              setInspectedFile(file);
                            }
                          }}
                          onDoubleClick={() => {
                            if (file.is_dir) {
                              setCurrentPath(file.path);
                            }
                          }}
                        >
                          <td className="px-6 py-3.5" onClick={(e) => e.stopPropagation()}>
                            <button
                              onClick={() => selection.toggle(file.path)}
                              className={cn("transition-all", isSelected ? "text-indigo-600" : "text-slate-200 group-hover:text-slate-400")}
                            >
                              {isSelected ? <CheckSquare className="w-5 h-5" /> : <Square className="w-5 h-5" />}
                            </button>
                          </td>
                          <td className="px-4 py-3.5">
                            <div className="flex items-center gap-3.5">
                              {renderFileIcon(file.name, file.is_dir)}
                              <div className="min-w-0">
                                <p className={cn("text-xs font-bold truncate", isSelected ? "text-indigo-900" : "text-slate-800")}>
                                  {file.name}
                                </p>
                                <p className="text-[10px] text-slate-400 font-medium uppercase tracking-tight mt-0.5">
                                  {file.is_dir ? 'Direktori Folder' : file.name.split('.').pop()?.toUpperCase() + ' File'}
                                </p>
                              </div>
                              {file.is_dir && <ChevronRight className="w-3.5 h-3.5 text-slate-300 opacity-0 group-hover:opacity-100 transition-all ml-auto" />}
                            </div>
                          </td>
                          <td className="px-6 py-3.5 text-right">
                            <span className="text-xs font-mono font-bold text-slate-500">
                              {file.is_dir ? '--' : formatBytes(file.size_bytes)}
                            </span>
                          </td>
                          <td className="px-6 py-3.5">
                            <span className="text-xs font-medium text-slate-400">
                              {formatDate(file.modified_at)}
                            </span>
                          </td>
                          <td className="px-6 py-3.5 text-right" onClick={(e) => e.stopPropagation()}>
                            <div className="opacity-0 group-hover:opacity-100 flex items-center justify-end gap-1 transition-all">
                              <button
                                onClick={() => setInspectedFile(file)}
                                title="Detail Berkas"
                                className="p-1.5 hover:bg-white rounded-xl text-slate-400 hover:text-indigo-600 shadow-sm border border-transparent hover:border-slate-200 transition-all"
                              >
                                <Info className="w-4 h-4" />
                              </button>
                              <button
                                onClick={() => handleCopyPath(file.path)}
                                title="Salin Path"
                                className="p-1.5 hover:bg-white rounded-xl text-slate-400 hover:text-indigo-600 shadow-sm border border-transparent hover:border-slate-200 transition-all"
                              >
                                <Copy className="w-4 h-4" />
                              </button>
                              <button
                                onClick={() => handleDownload(file)}
                                title="Unduh ke PC"
                                className="p-1.5 hover:bg-white rounded-xl text-slate-400 hover:text-indigo-600 shadow-sm border border-transparent hover:border-slate-200 transition-all"
                              >
                                <Download className="w-4 h-4" />
                              </button>
                            </div>
                          </td>
                        </tr>
                      );
                    })}
                  </tbody>
                </table>
              ) : (
                /* Grid Cards Mode */
                <div className="p-6 grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6 gap-4">
                  {filteredFiles.map((file, i) => {
                    const isSelected = selection.isSelected(file.path);
                    const isInspected = inspectedFile?.path === file.path;

                    return (
                      <div
                        key={i}
                        onContextMenu={(e) => handleContextMenu(e, file)}
                        onClick={() => {
                          if (file.is_dir) {
                            setCurrentPath(file.path);
                          } else {
                            setInspectedFile(file);
                          }
                        }}
                        onDoubleClick={() => {
                          if (file.is_dir) setCurrentPath(file.path);
                        }}
                        className={cn(
                          "p-4 rounded-3xl border transition-all flex flex-col items-center text-center cursor-pointer select-none group relative",
                          isInspected
                            ? "bg-indigo-50 border-indigo-300 ring-2 ring-indigo-500/30 shadow-lg"
                            : isSelected
                            ? "bg-indigo-50/80 border-indigo-300 ring-2 ring-indigo-500/20 shadow-md"
                            : "bg-white border-slate-100 hover:border-indigo-100 hover:shadow-lg hover:bg-slate-50/60"
                        )}
                      >
                        <button
                          onClick={(e) => { e.stopPropagation(); selection.toggle(file.path); }}
                          className={cn(
                            "absolute top-3 left-3 transition-opacity",
                            isSelected ? "opacity-100 text-indigo-600" : "opacity-0 group-hover:opacity-100 text-slate-300 hover:text-slate-500"
                          )}
                        >
                          {isSelected ? <CheckSquare className="w-4 h-4" /> : <Square className="w-4 h-4" />}
                        </button>

                        <button
                          onClick={(e) => { e.stopPropagation(); setInspectedFile(file); }}
                          className="absolute top-3 right-3 opacity-0 group-hover:opacity-100 text-slate-300 hover:text-indigo-600 transition-opacity"
                        >
                          <Info className="w-4 h-4" />
                        </button>

                        <div className="my-3 scale-110 group-hover:scale-125 transition-transform duration-300">
                          {renderFileIcon(file.name, file.is_dir)}
                        </div>

                        <p className={cn("text-xs font-bold truncate w-full mt-1", isSelected ? "text-indigo-900" : "text-slate-800")} title={file.name}>
                          {file.name}
                        </p>

                        <p className="text-[10px] font-mono text-slate-400 mt-1">
                          {file.is_dir ? 'Folder' : formatBytes(file.size_bytes)}
                        </p>
                      </div>
                    );
                  })}
                </div>
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
          />
        )}
      </div>
    </div>
  );
}
