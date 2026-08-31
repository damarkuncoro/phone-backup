import {
  CheckSquare, Square, ChevronRight, Edit3, Info, Copy, Download, Trash2,
  ArrowUpDown, ArrowUp, ArrowDown
} from 'lucide-react';
import { type FileEntry } from '@/services/deviceService';
import { cn } from "@/shared/lib/utils";
import { formatBytes, formatDate } from '@/shared/lib/formatters';
import type { SortField } from '../lib/fileUtils';
import { FileIcon } from './FileIcon';

interface FileTableViewProps {
  files: FileEntry[];
  selectedPaths: string[];
  inspectedFile: FileEntry | null;
  sortBy: SortField;
  sortDirection: 'asc' | 'desc';
  onSort: (field: SortField) => void;
  onToggleSelect: (path: string) => void;
  onToggleSelectAll: () => void;
  onSelectFile: (file: FileEntry) => void;
  onOpenFolder: (path: string) => void;
  onContextMenu: (e: React.MouseEvent, file: FileEntry) => void;
  onRename: (file: FileEntry) => void;
  onInspect: (file: FileEntry) => void;
  onCopyPath: (path: string) => void;
  onDownload: (file: FileEntry) => void;
  onDelete: (file: FileEntry) => void;
}

export function FileTableView({
  files,
  selectedPaths,
  inspectedFile,
  sortBy,
  sortDirection,
  onSort,
  onToggleSelect,
  onToggleSelectAll,
  onSelectFile,
  onOpenFolder,
  onContextMenu,
  onRename,
  onInspect,
  onCopyPath,
  onDownload,
  onDelete
}: FileTableViewProps) {
  const isAllSelected = files.length > 0 && selectedPaths.length === files.length;

  const getSortIcon = (field: SortField) => {
    if (sortBy !== field) return <ArrowUpDown className="w-3.5 h-3.5 text-slate-300 opacity-60 group-hover:opacity-100" />;
    return sortDirection === 'asc'
      ? <ArrowUp className="w-3.5 h-3.5 text-indigo-600" />
      : <ArrowDown className="w-3.5 h-3.5 text-indigo-600" />;
  };

  return (
    <table className="w-full text-left border-separate border-spacing-0">
      <thead className="sticky top-0 bg-white/95 backdrop-blur-md z-10 border-b border-slate-100 shadow-sm">
        <tr>
          <th className="px-6 py-3 w-14">
            <button
              onClick={onToggleSelectAll}
              className="text-slate-300 hover:text-indigo-600 transition-all"
            >
              {isAllSelected ? <CheckSquare className="w-5 h-5 text-indigo-600" /> : <Square className="w-5 h-5" />}
            </button>
          </th>
          <th
            onClick={() => onSort('name')}
            className="px-4 py-3 text-[10px] font-black text-slate-400 uppercase tracking-widest cursor-pointer hover:text-indigo-600 select-none group"
          >
            <div className="flex items-center gap-2">
              Nama Berkas {getSortIcon('name')}
            </div>
          </th>
          <th
            onClick={() => onSort('size')}
            className="px-6 py-3 text-[10px] font-black text-slate-400 uppercase tracking-widest text-right cursor-pointer hover:text-indigo-600 select-none group"
          >
            <div className="flex items-center justify-end gap-2">
              Ukuran {getSortIcon('size')}
            </div>
          </th>
          <th
            onClick={() => onSort('date')}
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
        {files.map((file, i) => {
          const isSelected = selectedPaths.includes(file.path);
          const isInspected = inspectedFile?.path === file.path;

          return (
            <tr
              key={i}
              onContextMenu={(e) => onContextMenu(e, file)}
              className={cn(
                "group transition-all cursor-pointer select-none",
                isInspected ? "bg-indigo-50/90" : isSelected ? "bg-indigo-50/60" : "hover:bg-slate-50/80"
              )}
              onClick={() => {
                if (file.is_dir) {
                  onOpenFolder(file.path);
                } else {
                  onSelectFile(file);
                }
              }}
              onDoubleClick={() => {
                if (file.is_dir) {
                  onOpenFolder(file.path);
                }
              }}
            >
              <td className="px-6 py-3.5" onClick={(e) => e.stopPropagation()}>
                <button
                  onClick={() => onToggleSelect(file.path)}
                  className={cn("transition-all", isSelected ? "text-indigo-600" : "text-slate-200 group-hover:text-slate-400")}
                >
                  {isSelected ? <CheckSquare className="w-5 h-5" /> : <Square className="w-5 h-5" />}
                </button>
              </td>
              <td className="px-4 py-3.5">
                <div className="flex items-center gap-3.5">
                  <FileIcon fileName={file.name} isDir={file.is_dir} />
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
                    onClick={() => onRename(file)}
                    title="Ganti Nama"
                    className="p-1.5 hover:bg-white rounded-xl text-slate-400 hover:text-indigo-600 shadow-sm border border-transparent hover:border-slate-200 transition-all"
                  >
                    <Edit3 className="w-4 h-4" />
                  </button>
                  <button
                    onClick={() => onInspect(file)}
                    title="Detail Berkas"
                    className="p-1.5 hover:bg-white rounded-xl text-slate-400 hover:text-indigo-600 shadow-sm border border-transparent hover:border-slate-200 transition-all"
                  >
                    <Info className="w-4 h-4" />
                  </button>
                  <button
                    onClick={() => onCopyPath(file.path)}
                    title="Salin Path"
                    className="p-1.5 hover:bg-white rounded-xl text-slate-400 hover:text-indigo-600 shadow-sm border border-transparent hover:border-slate-200 transition-all"
                  >
                    <Copy className="w-4 h-4" />
                  </button>
                  <button
                    onClick={() => onDownload(file)}
                    title="Unduh ke PC"
                    className="p-1.5 hover:bg-white rounded-xl text-slate-400 hover:text-indigo-600 shadow-sm border border-transparent hover:border-slate-200 transition-all"
                  >
                    <Download className="w-4 h-4" />
                  </button>
                  <button
                    onClick={() => onDelete(file)}
                    title="Hapus Berkas"
                    className="p-1.5 hover:bg-red-50 rounded-xl text-slate-400 hover:text-red-600 shadow-sm border border-transparent hover:border-red-200 transition-all"
                  >
                    <Trash2 className="w-4 h-4" />
                  </button>
                </div>
              </td>
            </tr>
          );
        })}
      </tbody>
    </table>
  );
}
