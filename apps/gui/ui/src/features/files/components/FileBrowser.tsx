import {
  Folder, File, ChevronRight, Home, ArrowLeft, Search,
  Download, Trash2, Image as ImageIcon,
  CheckSquare, Square, X, MoreVertical
} from 'lucide-react';
import { getDeviceId } from '@/services/deviceService';
import { cn } from "../../../shared/lib/utils";
import { getParentPath } from '../lib/pathUtils';
import { formatBytes, formatDate } from '@/shared/lib/formatters';
import { isImage } from '../lib/fileUtils';
import { useFileBrowser } from '../hooks/useFileBrowser';

export function FileBrowser() {
  const {
    devices,
    selectedDeviceId, setSelectedDeviceId,
    currentPath, setCurrentPath,
    loading,
    searchQuery, setSearchQuery,
    selection,
    filteredFiles,
    breadcrumbs
  } = useFileBrowser();

  return (
    <div className="h-full flex flex-col bg-white animate-in fade-in duration-500 relative">

      {/* Dynamic Action Bar for Selection */}
      {selection.count > 0 && (
          <div className="absolute top-0 left-0 right-0 z-30 bg-indigo-600 text-white p-4 flex items-center justify-between animate-in slide-in-from-top-4 duration-300 shadow-2xl">
              <div className="flex items-center gap-6">
                  <button onClick={() => selection.clear()} className="p-2 hover:bg-white/10 rounded-xl transition-all">
                      <X className="w-5 h-5" />
                  </button>
                  <p className="font-black uppercase tracking-widest text-xs">{selection.count} Item Terpilih</p>
              </div>
              <div className="flex items-center gap-2">
                  <button className="flex items-center gap-2 px-4 py-2 bg-white/10 hover:bg-white/20 rounded-xl text-xs font-black uppercase transition-all">
                      <Download className="w-4 h-4" /> Download
                  </button>
                  <button className="flex items-center gap-2 px-4 py-2 bg-red-500 hover:bg-red-600 rounded-xl text-xs font-black uppercase transition-all">
                      <Trash2 className="w-4 h-4" /> Hapus
                  </button>
              </div>
          </div>
      )}

      <header className="p-8 border-b border-slate-100 flex items-center justify-between gap-8">
        <div className="flex items-center gap-4 min-w-0">
            <div className="w-12 h-12 bg-indigo-50 text-indigo-600 rounded-2xl flex items-center justify-center shrink-0">
                <Folder className="w-6 h-6" />
            </div>
            <div className="min-w-0">
                <h1 className="text-2xl font-black text-slate-900 tracking-tight truncate">File Manager</h1>
                <p className="text-[10px] font-bold text-slate-400 uppercase tracking-[0.2em] truncate shrink-0">Live ADB Access</p>
            </div>
        </div>

        <div className="flex-1 max-w-md relative hidden md:block">
            <Search className="absolute left-4 top-3.5 w-4 h-4 text-slate-300" />
            <input
                type="text"
                placeholder="Cari file di folder ini..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full bg-slate-50 border border-slate-100 pl-11 pr-4 py-3 rounded-2xl text-sm focus:ring-4 focus:ring-indigo-500/10 focus:border-indigo-200 outline-none transition-all"
            />
        </div>

        <div className="flex items-center gap-3 shrink-0">
            <select
                value={selectedDeviceId || ''}
                onChange={(e) => setSelectedDeviceId(e.target.value)}
                className="bg-slate-50 border border-slate-100 px-4 py-3 rounded-2xl text-xs font-black text-slate-600 outline-none hover:bg-white transition-all cursor-pointer"
            >
                {devices.map(d => (
                    <option key={getDeviceId(d)} value={getDeviceId(d)}>{d.model}</option>
                ))}
            </select>
        </div>
      </header>

      {/* Breadcrumbs Component */}
      <nav className="px-8 py-3 bg-slate-50/30 border-b border-slate-100 flex items-center gap-2 overflow-x-auto no-scrollbar">
          <button
            onClick={() => setCurrentPath(getParentPath(currentPath))}
            disabled={currentPath === '/'}
            className="p-2 hover:bg-white rounded-xl text-slate-400 hover:text-indigo-600 disabled:opacity-0 transition-all border border-transparent hover:border-slate-100"
          >
              <ArrowLeft className="w-4 h-4" />
          </button>

          <div className="h-4 w-px bg-slate-200 mx-2" />

          {breadcrumbs.map((bc, i) => (
              <div key={bc.path} className="flex items-center shrink-0">
                  {i > 0 && <ChevronRight className="w-3.5 h-3.5 text-slate-300 mx-1" />}
                  <button
                    onClick={() => setCurrentPath(bc.path)}
                    className={cn(
                        "px-3 py-1.5 rounded-lg text-xs font-bold transition-all",
                        i === breadcrumbs.length - 1 ? "bg-white text-indigo-600 shadow-sm border border-slate-100" : "text-slate-400 hover:text-slate-600"
                    )}
                  >
                      {i === 0 ? <Home className="w-3.5 h-3.5" /> : bc.name}
                  </button>
              </div>
          ))}
      </nav>

      <div className="flex-1 overflow-y-auto custom-scrollbar">
          {loading ? (
              <div className="h-full flex flex-col items-center justify-center gap-4 text-slate-400">
                  <div className="w-12 h-12 border-4 border-slate-100 border-t-indigo-600 rounded-full animate-spin" />
                  <p className="text-[10px] font-black uppercase tracking-[0.2em]">Sinkronisasi Filesystem...</p>
              </div>
          ) : filteredFiles.length > 0 ? (
              <table className="w-full text-left border-separate border-spacing-0">
                  <thead className="sticky top-0 bg-white/90 backdrop-blur-md z-10 border-b border-slate-100">
                      <tr>
                          <th className="px-8 py-4 w-14">
                              <button
                                onClick={() => selection.count === filteredFiles.length ? selection.clear() : selection.selectAll(filteredFiles.map(f => f.path))}
                                className="text-slate-300 hover:text-indigo-500 transition-all"
                              >
                                  {selection.count === filteredFiles.length ? <CheckSquare className="w-5 h-5 text-indigo-600" /> : <Square className="w-5 h-5" />}
                              </button>
                          </th>
                          <th className="px-4 py-4 text-[10px] font-black text-slate-400 uppercase tracking-widest">Nama File</th>
                          <th className="px-8 py-4 text-[10px] font-black text-slate-400 uppercase tracking-widest text-right">Ukuran</th>
                          <th className="px-8 py-4 text-[10px] font-black text-slate-400 uppercase tracking-widest">Modifikasi</th>
                          <th className="px-8 py-4 w-24"></th>
                      </tr>
                  </thead>
                  <tbody className="divide-y divide-slate-50">
                      {filteredFiles.map((file, i) => {
                          const isSelected = selection.isSelected(file.path);
                          return (
                            <tr
                                key={i}
                                className={cn(
                                    "group transition-all cursor-pointer select-none",
                                    isSelected ? "bg-indigo-50/50" : "hover:bg-slate-50"
                                )}
                                onClick={() => selection.toggle(file.path)}
                                onDoubleClick={() => {
                                    if (file.is_dir) {
                                        setCurrentPath(file.path);
                                    }
                                }}
                            >
                                <td className="px-8 py-4" onClick={(e) => e.stopPropagation()}>
                                    <button
                                        onClick={() => selection.toggle(file.path)}
                                        className={cn("transition-all", isSelected ? "text-indigo-600" : "text-slate-200 group-hover:text-slate-400")}
                                    >
                                        {isSelected ? <CheckSquare className="w-5 h-5" /> : <Square className="w-5 h-5" />}
                                    </button>
                                </td>
                                <td className="px-4 py-4" onClick={(e) => {
                                    if (file.is_dir) {
                                        e.stopPropagation();
                                        setCurrentPath(file.path);
                                    }
                                }}>
                                    <div className="flex items-center gap-4">
                                        <div className={cn(
                                            "w-11 h-11 rounded-2xl flex items-center justify-center transition-all shadow-sm",
                                            file.is_dir ? "bg-indigo-50 text-indigo-600" :
                                            isImage(file.name) ? "bg-amber-50 text-amber-600" : "bg-slate-50 text-slate-400"
                                        )}>
                                            {file.is_dir ? <Folder className="w-5 h-5 fill-current opacity-80" /> :
                                            isImage(file.name) ? <ImageIcon className="w-5 h-5" /> : <File className="w-5 h-5" />}
                                        </div>
                                        <div className="min-w-0">
                                            <p className={cn("text-sm font-bold truncate", isSelected ? "text-indigo-900" : "text-slate-700")}>
                                                {file.name}
                                            </p>
                                            <p className="text-[10px] text-slate-400 font-medium uppercase tracking-tighter">
                                                {file.is_dir ? 'Folder' : file.name.split('.').pop() + ' File'}
                                            </p>
                                        </div>
                                        {file.is_dir && <ChevronRight className="w-3.5 h-3.5 text-slate-300 opacity-0 group-hover:opacity-100 transition-all ml-auto" />}
                                    </div>
                                </td>
                                <td className="px-8 py-4 text-right">
                                    <span className="text-xs font-mono font-bold text-slate-400">
                                        {file.is_dir ? '--' : formatBytes(file.size_bytes)}
                                    </span>
                                </td>
                                <td className="px-8 py-4">
                                    <span className="text-xs font-medium text-slate-400">
                                        {formatDate(file.modified_at)}
                                    </span>
                                </td>
                                <td className="px-8 py-4 text-right" onClick={(e) => e.stopPropagation()}>
                                    <div className="opacity-0 group-hover:opacity-100 flex items-center justify-end gap-1 transition-all">
                                        <button className="p-2 hover:bg-white rounded-xl text-slate-400 hover:text-indigo-600 shadow-sm border border-transparent hover:border-slate-100">
                                            <Download className="w-4 h-4" />
                                        </button>
                                        <button className="p-2 hover:bg-white rounded-xl text-slate-400 hover:text-indigo-600 shadow-sm border border-transparent hover:border-slate-100">
                                            <MoreVertical className="w-4 h-4" />
                                        </button>
                                    </div>
                                </td>
                            </tr>
                          );
                      })}
                  </tbody>
              </table>
          ) : (
              <div className="h-full flex flex-col items-center justify-center bg-slate-50/50 rounded-[40px] m-8 border-2 border-dashed border-slate-100 text-slate-300">
                  <div className="w-20 h-20 bg-white rounded-[32px] flex items-center justify-center shadow-sm mb-6">
                    <Folder className="w-10 h-10 opacity-10" />
                  </div>
                  <p className="font-black uppercase tracking-[0.2em] text-[10px]">Folder Kosong</p>
                  <p className="text-xs mt-2 text-slate-400">Tidak ada file yang ditemukan di direktori ini.</p>
              </div>
          )}
      </div>

      {/* Footer Info */}
      <footer className="px-8 py-4 border-t border-slate-100 bg-white flex justify-between items-center text-[10px] font-black text-slate-400 uppercase tracking-widest">
          <div className="flex gap-6">
              <span>{filteredFiles.filter(f => f.is_dir).length} Folders</span>
              <span>{filteredFiles.filter(f => !f.is_dir).length} Files</span>
          </div>
          <div>
              Total: {formatBytes(filteredFiles.reduce((acc, f) => acc + (f.size_bytes || 0), 0))}
          </div>
      </footer>
    </div>
  );
}
