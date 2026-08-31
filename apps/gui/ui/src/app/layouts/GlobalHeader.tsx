import {
  LayoutDashboard, Tablet, History, RefreshCcw, Folder, Settings, Search,
  Smartphone, Usb, Wifi, HardDrive, ShieldCheck, RefreshCw, Plus, ChevronDown
} from 'lucide-react';
import { type Device, getDeviceId } from '@/services/deviceService';
import { cn } from '@/shared/lib/utils';

const viewTitles: Record<string, { label: string; icon: any; breadcrumb: string }> = {
  dashboard: { label: 'Dashboard', icon: LayoutDashboard, breadcrumb: 'Beranda' },
  devices: { label: 'Perangkat', icon: Tablet, breadcrumb: 'Manajemen Perangkat' },
  backup: { label: 'Backup Wizard', icon: History, breadcrumb: 'Studio Pencadangan' },
  history: { label: 'Riwayat Backup', icon: RefreshCcw, breadcrumb: 'Arsip & Restore' },
  explorer: { label: 'Vault Explorer', icon: Folder, breadcrumb: 'Eksplorasi Snapshot' },
  diff: { label: 'Diff Viewer', icon: RefreshCcw, breadcrumb: 'Perbandingan Snapshot' },
  files: { label: 'File Browser', icon: Folder, breadcrumb: 'Penjelajah Berkas' },
  settings: { label: 'Pengaturan Sistem', icon: Settings, breadcrumb: 'Infrastruktur & Keamanan' },
  search: { label: 'Pencarian Berkas', icon: Search, breadcrumb: 'Indeks Global' },
  'device-details': { label: 'Detail Perangkat', icon: Smartphone, breadcrumb: 'Spesifikasi & Log' },
};

interface GlobalHeaderProps {
  activeView: string;
  devices: Device[];
  selectedDevice: Device | null;
  onSelectDevice: (device: Device) => void;
  onRefreshDevices: () => void;
  isRefreshingDevices?: boolean;
  onOpenAddDevice?: () => void;
}

export function GlobalHeader({
  activeView,
  devices,
  selectedDevice,
  onSelectDevice,
  onRefreshDevices,
  isRefreshingDevices,
  onOpenAddDevice
}: GlobalHeaderProps) {
  const currentViewMeta = viewTitles[activeView] || {
    label: activeView.toUpperCase(),
    icon: LayoutDashboard,
    breadcrumb: 'Aplikasi'
  };
  const Icon = currentViewMeta.icon;

  return (
    <header className="h-16 bg-white/80 backdrop-blur-md border-b border-slate-200/80 px-6 flex items-center justify-between gap-4 shrink-0 select-none z-30">
      
      {/* Left: View Title & Breadcrumb */}
      <div className="flex items-center gap-3 min-w-0">
        <div className="w-9 h-9 rounded-xl bg-indigo-50 text-indigo-600 flex items-center justify-center shrink-0 shadow-sm border border-indigo-100/50">
          <Icon className="w-4 h-4" />
        </div>
        <div className="min-w-0">
          <div className="flex items-center gap-1.5 text-[10px] font-bold text-slate-400 uppercase tracking-widest truncate">
            <span>Phone Backup</span>
            <span>/</span>
            <span className="text-slate-500">{currentViewMeta.breadcrumb}</span>
          </div>
          <h2 className="text-sm font-black text-slate-900 tracking-tight truncate leading-tight">
            {currentViewMeta.label}
          </h2>
        </div>
      </div>

      {/* Center: Active Device Selector Pill */}
      <div className="hidden md:flex items-center">
        {devices.length === 0 ? (
          <div className="flex items-center gap-2 px-3 py-1.5 bg-slate-100/80 rounded-xl border border-slate-200 text-xs font-bold text-slate-400">
            <Smartphone className="w-3.5 h-3.5" />
            <span>Tidak Ada Perangkat Terhubung</span>
          </div>
        ) : (
          <div className="flex items-center gap-2 bg-slate-100/90 p-1 rounded-2xl border border-slate-200/80 shadow-inner">
            <span className="text-[9px] font-black uppercase tracking-wider text-slate-400 px-2">
              Perangkat Aktif:
            </span>
            <div className="relative group">
              <select
                value={selectedDevice ? getDeviceId(selectedDevice) : ''}
                onChange={(e) => {
                  const found = devices.find(d => getDeviceId(d) === e.target.value);
                  if (found) onSelectDevice(found);
                }}
                className="appearance-none bg-white hover:bg-slate-50 text-slate-800 text-xs font-black px-3 py-1.5 pr-7 rounded-xl border border-slate-200 shadow-sm outline-none cursor-pointer transition-all"
              >
                {devices.map(device => (
                  <option key={getDeviceId(device)} value={getDeviceId(device)}>
                    {device.model} ({device.connection_type === 'Mtp' ? 'MTP' : device.connection_type === 'Wifi' ? 'WiFi' : 'USB'})
                  </option>
                ))}
              </select>
              <ChevronDown className="w-3.5 h-3.5 text-slate-400 absolute right-2 top-2 pointer-events-none" />
            </div>

            {selectedDevice && (
              <span className={cn(
                "text-[9px] font-black px-2 py-0.5 rounded-lg uppercase tracking-wider flex items-center gap-1",
                selectedDevice.connection_type === 'Mtp'
                  ? "bg-cyan-100 text-cyan-800"
                  : selectedDevice.connection_type === 'Wifi'
                  ? "bg-purple-100 text-purple-800"
                  : "bg-emerald-100 text-emerald-800"
              )}>
                {selectedDevice.connection_type === 'Mtp' ? <HardDrive className="w-2.5 h-2.5" /> : selectedDevice.connection_type === 'Wifi' ? <Wifi className="w-2.5 h-2.5" /> : <Usb className="w-2.5 h-2.5" />}
                {selectedDevice.connection_type === 'Mtp' ? 'MTP' : selectedDevice.connection_type === 'Wifi' ? 'WiFi' : 'USB'}
              </span>
            )}
          </div>
        )}
      </div>

      {/* Right: Quick Actions & Status Badges */}
      <div className="flex items-center gap-2.5">
        {/* Refresh Device Scan Button */}
        <button
          type="button"
          onClick={onRefreshDevices}
          disabled={isRefreshingDevices}
          title="Pindai Ulang Koneksi Perangkat"
          className="w-8 h-8 rounded-xl bg-slate-100 hover:bg-slate-200 text-slate-600 flex items-center justify-center transition-all active:scale-95 disabled:opacity-50"
        >
          <RefreshCw className={cn("w-3.5 h-3.5", isRefreshingDevices && "animate-spin text-indigo-600")} />
        </button>

        {/* E2E Age Shield Security Badge */}
        <div
          title="Keamanan Enkripsi Age X25519 Aktif"
          className="hidden sm:flex items-center gap-1.5 px-2.5 py-1 bg-emerald-50 text-emerald-700 border border-emerald-200/80 rounded-xl text-[10px] font-black uppercase tracking-wider"
        >
          <ShieldCheck className="w-3.5 h-3.5 text-emerald-600" />
          <span>Age Encrypted</span>
        </div>

        {/* Add Device Button */}
        {onOpenAddDevice && (
          <button
            type="button"
            onClick={onOpenAddDevice}
            className="flex items-center gap-1.5 px-3 py-1.5 bg-indigo-600 hover:bg-indigo-700 text-white rounded-xl text-xs font-black shadow-sm shadow-indigo-200 hover:shadow-md transition-all active:scale-95"
          >
            <Plus className="w-3.5 h-3.5" />
            <span className="hidden sm:inline">Tambah Perangkat</span>
          </button>
        )}
      </div>

    </header>
  );
}
