import {
  HardDrive, Smartphone, Camera, Image, Download,
  MessageSquare, FileText, Music, Film, FolderGit2
} from 'lucide-react';
import { cn } from '@/shared/lib/utils';
import { type Device } from '@/services/deviceService';
import { formatStorageSize } from '@/shared/lib/formatters';

interface StorageSidebarProps {
  currentPath: string;
  selectedDevice: Device | null;
  onNavigate: (path: string) => void;
}

export function StorageSidebar({
  currentPath,
  selectedDevice,
  onNavigate
}: StorageSidebarProps) {
  const volumes = [
    {
      id: 'internal',
      name: 'Internal Storage',
      path: '/sdcard',
      icon: Smartphone,
      badge: 'Utama'
    },
    {
      id: 'root',
      name: 'Root File System',
      path: '/',
      icon: HardDrive,
      badge: 'Linux'
    }
  ];

  const shortcuts = [
    { id: 'dcim', name: 'Kamera (DCIM)', path: '/storage/emulated/0/DCIM', icon: Camera },
    { id: 'pictures', name: 'Foto & Galeri', path: '/storage/emulated/0/Pictures', icon: Image },
    { id: 'download', name: 'Unduhan (Download)', path: '/storage/emulated/0/Download', icon: Download },
    { id: 'whatsapp', name: 'WhatsApp Media', path: '/storage/emulated/0/Android/media/com.whatsapp/WhatsApp/Media', icon: MessageSquare },
    { id: 'documents', name: 'Dokumen', path: '/storage/emulated/0/Documents', icon: FileText },
    { id: 'music', name: 'Musik & Audio', path: '/storage/emulated/0/Music', icon: Music },
    { id: 'movies', name: 'Video & Film', path: '/storage/emulated/0/Movies', icon: Film },
    { id: 'android', name: 'Data Android', path: '/storage/emulated/0/Android', icon: FolderGit2 },
  ];

  const usedBytes = selectedDevice?.storage_used_bytes || 0;
  const totalBytes = selectedDevice?.storage_total_bytes || 0;
  const usedPercent = totalBytes > 0 ? Math.min(100, Math.round((usedBytes / totalBytes) * 100)) : 0;

  return (
    <aside className="w-64 bg-slate-50/70 border-r border-slate-200/80 flex flex-col h-full shrink-0 select-none">
      <div className="flex-1 overflow-y-auto custom-scrollbar p-4 space-y-6">

        {/* Storage Volumes */}
        <div>
          <h4 className="px-3 text-[10px] font-black uppercase tracking-wider text-slate-400 mb-2">
            Penyimpanan
          </h4>
          <div className="space-y-1">
            {volumes.map(v => {
              const isActive = v.path === '/'
                ? currentPath === '/'
                : currentPath.startsWith(v.path) || currentPath.startsWith('/storage/emulated/0');
              return (
                <button
                  key={v.id}
                  onClick={() => onNavigate(v.path)}
                  className={cn(
                    "w-full flex items-center justify-between px-3 py-2.5 rounded-2xl text-xs font-bold transition-all",
                    isActive
                      ? "bg-indigo-600 text-white shadow-sm shadow-indigo-100"
                      : "text-slate-600 hover:bg-white hover:text-indigo-600"
                  )}
                >
                  <div className="flex items-center gap-2.5 truncate">
                    <v.icon className={cn("w-4 h-4 shrink-0", isActive ? "text-white" : "text-slate-400")} />
                    <span className="truncate">{v.name}</span>
                  </div>
                  <span className={cn(
                    "text-[9px] px-2 py-0.5 rounded-lg font-black uppercase tracking-wider",
                    isActive ? "bg-white/20 text-white" : "bg-slate-200/60 text-slate-500"
                  )}>
                    {v.badge}
                  </span>
                </button>
              );
            })}
          </div>
        </div>

        {/* Shortcuts */}
        <div>
          <h4 className="px-3 text-[10px] font-black uppercase tracking-wider text-slate-400 mb-2">
            Pintasan Cepat
          </h4>
          <div className="space-y-0.5">
            {shortcuts.map(s => {
              const isActive = currentPath.startsWith(s.path);
              return (
                <button
                  key={s.id}
                  onClick={() => onNavigate(s.path)}
                  className={cn(
                    "w-full flex items-center gap-2.5 px-3 py-2 rounded-xl text-xs font-medium transition-all text-left truncate",
                    isActive
                      ? "bg-indigo-50 text-indigo-700 font-bold border border-indigo-100"
                      : "text-slate-600 hover:bg-white hover:text-slate-900"
                  )}
                >
                  <s.icon className={cn("w-4 h-4 shrink-0", isActive ? "text-indigo-600" : "text-slate-400")} />
                  <span className="truncate">{s.name}</span>
                </button>
              );
            })}
          </div>
        </div>

      </div>

      {/* Storage Capacity Gauge Card */}
      {totalBytes > 0 && (
        <div className="p-4 border-t border-slate-200/60 bg-white/80">
          <div className="flex items-center justify-between text-[10px] font-black uppercase tracking-wider text-slate-400 mb-1.5">
            <span>Kapasitas HP</span>
            <span className="text-slate-700">{usedPercent}%</span>
          </div>
          <div className="w-full h-2 bg-slate-100 rounded-full overflow-hidden mb-2">
            <div
              className={cn(
                "h-full rounded-full transition-all duration-500",
                usedPercent > 90 ? "bg-red-500" : usedPercent > 75 ? "bg-amber-500" : "bg-indigo-600"
              )}
              style={{ width: `${usedPercent}%` }}
            />
          </div>
          <div className="flex justify-between text-[10px] text-slate-400 font-mono">
            <span>{formatStorageSize(usedBytes)} terpakai</span>
            <span>{formatStorageSize(totalBytes)}</span>
          </div>
        </div>
      )}
    </aside>
  );
}
