import {
  PanelLeftClose, PanelLeft, HardDrive, Search, LayoutList, LayoutGrid
} from 'lucide-react';
import { cn } from "@/shared/lib/utils";
import { getDeviceId, type Device } from '@/services/deviceService';

interface FileBrowserHeaderProps {
  showSidebar: boolean;
  onToggleSidebar: () => void;
  searchQuery: string;
  onSearchChange: (query: string) => void;
  viewMode: 'list' | 'grid';
  onViewModeChange: (mode: 'list' | 'grid') => void;
  devices: Device[];
  selectedDeviceId: string | null;
  onSelectDevice: (deviceId: string) => void;
}

export function FileBrowserHeader({
  showSidebar,
  onToggleSidebar,
  searchQuery,
  onSearchChange,
  viewMode,
  onViewModeChange,
  devices,
  selectedDeviceId,
  onSelectDevice
}: FileBrowserHeaderProps) {
  return (
    <header className="px-6 py-4 border-b border-slate-200/80 bg-white/90 backdrop-blur-md flex items-center justify-between gap-4 shrink-0">
      <div className="flex items-center gap-3 min-w-0">
        <button
          onClick={onToggleSidebar}
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
            onChange={(e) => onSearchChange(e.target.value)}
            className="w-full bg-slate-50 border border-slate-200/80 pl-10 pr-4 py-2.5 rounded-2xl text-xs focus:ring-4 focus:ring-indigo-500/10 focus:border-indigo-300 outline-none transition-all"
          />
        </div>

        {/* View Mode Toggle */}
        <div className="flex items-center bg-slate-100 p-1 rounded-2xl border border-slate-200/60 shrink-0">
          <button
            onClick={() => onViewModeChange('list')}
            title="Tampilan Tabel"
            className={cn(
              "p-2 rounded-xl transition-all",
              viewMode === 'list' ? "bg-white text-indigo-600 shadow-sm" : "text-slate-400 hover:text-slate-700"
            )}
          >
            <LayoutList className="w-4 h-4" />
          </button>
          <button
            onClick={() => onViewModeChange('grid')}
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
          onChange={(e) => onSelectDevice(e.target.value)}
          className="bg-slate-50 border border-slate-200/80 px-3.5 py-2.5 rounded-2xl text-xs font-black text-slate-700 outline-none hover:bg-white transition-all cursor-pointer shrink-0"
        >
          {devices.map(d => (
            <option key={getDeviceId(d)} value={getDeviceId(d)}>{d.model}</option>
          ))}
        </select>
      </div>
    </header>
  );
}
