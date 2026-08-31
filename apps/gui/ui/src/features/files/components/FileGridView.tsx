import { CheckSquare, Square, Info } from 'lucide-react';
import { type FileEntry } from '@/services/deviceService';
import { cn } from "@/shared/lib/utils";
import { formatBytes } from '@/shared/lib/formatters';
import { FileIcon } from './FileIcon';

interface FileGridViewProps {
  files: FileEntry[];
  selectedPaths: string[];
  inspectedFile: FileEntry | null;
  onToggleSelect: (path: string) => void;
  onSelectFile: (file: FileEntry) => void;
  onOpenFolder: (path: string) => void;
  onContextMenu: (e: React.MouseEvent, file: FileEntry) => void;
  onInspect: (file: FileEntry) => void;
}

export function FileGridView({
  files,
  selectedPaths,
  inspectedFile,
  onToggleSelect,
  onSelectFile,
  onOpenFolder,
  onContextMenu,
  onInspect
}: FileGridViewProps) {
  return (
    <div className="p-6 grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6 gap-4">
      {files.map((file, i) => {
        const isSelected = selectedPaths.includes(file.path);
        const isInspected = inspectedFile?.path === file.path;

        return (
          <div
            key={i}
            onContextMenu={(e) => onContextMenu(e, file)}
            onClick={() => {
              if (file.is_dir) {
                onOpenFolder(file.path);
              } else {
                onSelectFile(file);
              }
            }}
            onDoubleClick={() => {
              if (file.is_dir) onOpenFolder(file.path);
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
              onClick={(e) => { e.stopPropagation(); onToggleSelect(file.path); }}
              className={cn(
                "absolute top-3 left-3 transition-opacity",
                isSelected ? "opacity-100 text-indigo-600" : "opacity-0 group-hover:opacity-100 text-slate-300 hover:text-slate-500"
              )}
            >
              {isSelected ? <CheckSquare className="w-4 h-4" /> : <Square className="w-4 h-4" />}
            </button>

            <button
              onClick={(e) => { e.stopPropagation(); onInspect(file); }}
              className="absolute top-3 right-3 opacity-0 group-hover:opacity-100 text-slate-300 hover:text-indigo-600 transition-opacity"
            >
              <Info className="w-4 h-4" />
            </button>

            <div className="my-3 scale-110 group-hover:scale-125 transition-transform duration-300">
              <FileIcon fileName={file.name} isDir={file.is_dir} />
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
  );
}
