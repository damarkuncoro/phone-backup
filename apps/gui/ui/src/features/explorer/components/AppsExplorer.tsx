import { useState, useMemo } from 'react';
import {
  Smartphone, LayoutGrid, LayoutList, Download, Copy, Check,
  ExternalLink, ShieldCheck, Box,
  Layers
} from 'lucide-react';
import { cn } from '@/shared/lib/utils';

export interface AppData {
  id?: any;
  app_name?: string;
  name?: string;
  package_name: string;
  version_name?: string;
  version_code?: number;
  installer?: string | null;
  is_system?: boolean;
}

interface AppsExplorerProps {
  apps: AppData[];
  snapshotId: string;
}

const APP_AVATAR_COLORS = [
  'from-indigo-500 to-purple-600 text-white',
  'from-emerald-500 to-teal-600 text-white',
  'from-sky-500 to-blue-600 text-white',
  'from-amber-500 to-orange-600 text-white',
  'from-rose-500 to-pink-600 text-white',
  'from-violet-500 to-purple-700 text-white',
  'from-teal-500 to-cyan-700 text-white',
];

function getAppColor(name: string): string {
  let hash = 0;
  for (let i = 0; i < name.length; i++) {
    hash = name.charCodeAt(i) + ((hash << 5) - hash);
  }
  return APP_AVATAR_COLORS[Math.abs(hash) % APP_AVATAR_COLORS.length];
}

function getAppDisplayName(app: AppData): string {
  if (app.app_name && app.app_name.trim()) return app.app_name.trim();
  if (app.name && app.name.trim()) return app.name.trim();
  
  // Fallback prettify package_name: com.whatsapp -> WhatsApp
  const parts = (app.package_name || '').split('.');
  const lastPart = parts[parts.length - 1] || app.package_name;
  if (!lastPart) return 'Aplikasi';
  return lastPart.charAt(0).toUpperCase() + lastPart.slice(1);
}

function isSystemPackage(pkg: string): boolean {
  if (!pkg) return false;
  const p = pkg.toLowerCase();
  return (
    p.startsWith('android.') ||
    p.startsWith('com.android.') ||
    p.startsWith('com.google.android.ext.') ||
    p.startsWith('com.google.android.overlay') ||
    p.startsWith('com.mediatek.') ||
    p.startsWith('com.qualcomm.') ||
    p.startsWith('com.sec.android.app.launcher') ||
    p.includes('overlay') ||
    p.includes('systemui')
  );
}

function getInstallerLabel(installer?: string | null): { label: string; isPlayStore: boolean } {
  if (!installer) return { label: 'Sideload / APK', isPlayStore: false };
  const inst = installer.toLowerCase();
  if (inst.includes('vending') || inst.includes('google')) {
    return { label: 'Google Play Store', isPlayStore: true };
  }
  if (inst.includes('samsungapps')) {
    return { label: 'Samsung Galaxy Store', isPlayStore: false };
  }
  if (inst.includes('packageinstaller')) {
    return { label: 'Package Installer', isPlayStore: false };
  }
  return { label: installer, isPlayStore: false };
}

export function AppsExplorer({ apps = [], snapshotId }: AppsExplorerProps) {
  const [viewMode, setViewMode] = useState<'grid' | 'table'>('grid');
  const [filterCategory, setFilterCategory] = useState<'all' | 'user' | 'system'>('all');
  const [selectedApp, setSelectedApp] = useState<AppData | null>(apps[0] || null);
  const [copiedField, setCopiedField] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [exportSuccess, setExportSuccess] = useState(false);

  // Sync selected app when apps changes
  useMemo(() => {
    if (!selectedApp && apps && apps.length > 0) {
      setSelectedApp(apps[0]);
    }
  }, [apps, selectedApp]);

  // Filter apps
  const filteredApps = useMemo(() => {
    return apps.filter(a => {
      if (!a) return false;
      const isSystem = a.is_system !== undefined ? a.is_system : isSystemPackage(a.package_name);
      
      if (filterCategory === 'user' && isSystem) return false;
      if (filterCategory === 'system' && !isSystem) return false;

      if (!searchQuery) return true;
      const q = searchQuery.toLowerCase();
      const name = getAppDisplayName(a).toLowerCase();
      const pkg = (a.package_name || '').toLowerCase();
      return name.includes(q) || pkg.includes(q);
    });
  }, [apps, filterCategory, searchQuery]);

  const userAppsCount = useMemo(() => {
    return apps.filter(a => a && !(a.is_system !== undefined ? a.is_system : isSystemPackage(a.package_name))).length;
  }, [apps]);

  const systemAppsCount = apps.length - userAppsCount;

  const handleCopy = (text: string, id: string) => {
    navigator.clipboard.writeText(text);
    setCopiedField(id);
    setTimeout(() => setCopiedField(null), 2000);
  };

  const handleExportJson = () => {
    const exportData = apps.map(a => ({
      name: getAppDisplayName(a),
      package_name: a.package_name,
      version_name: a.version_name,
      version_code: a.version_code,
      installer: a.installer,
      is_system: a.is_system !== undefined ? a.is_system : isSystemPackage(a.package_name)
    }));

    const blob = new Blob([JSON.stringify(exportData, null, 2)], { type: 'application/json;charset=utf-8;' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.setAttribute('download', `Daftar_Aplikasi_Snapshot_${snapshotId.substring(0, 8)}.json`);
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);

    setExportSuccess(true);
    setTimeout(() => setExportSuccess(false), 3000);
  };

  return (
    <div className="flex flex-col h-full bg-slate-50/50 rounded-[32px] border border-slate-200/80 overflow-hidden shadow-sm">
      
      {/* Top Header Bar */}
      <div className="p-5 bg-white border-b border-slate-200/80 flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-2xl bg-indigo-50 text-indigo-600 flex items-center justify-center shadow-inner">
            <Smartphone className="w-5 h-5" />
          </div>
          <div>
            <h2 className="text-sm font-black text-slate-900 tracking-tight flex items-center gap-2">
              Inventaris Aplikasi Vault
              <span className="text-[10px] font-black uppercase px-2.5 py-0.5 bg-indigo-50 text-indigo-700 rounded-full border border-indigo-100">
                {apps.length} Aplikasi
              </span>
            </h2>
            <p className="text-[10px] text-slate-400 font-bold uppercase tracking-widest">
              {userAppsCount} Aplikasi Pengguna • {systemAppsCount} Paket Sistem
            </p>
          </div>
        </div>

        <div className="flex items-center gap-2.5 shrink-0">
          {/* View Mode Toggle */}
          <div className="flex items-center bg-slate-100 p-1 rounded-xl border border-slate-200/70">
            <button
              onClick={() => setViewMode('grid')}
              title="Tampilan Grid Kartu"
              className={cn(
                "p-1.5 rounded-lg transition-all",
                viewMode === 'grid' ? "bg-white text-indigo-600 shadow-sm" : "text-slate-400 hover:text-slate-700"
              )}
            >
              <LayoutGrid className="w-4 h-4" />
            </button>
            <button
              onClick={() => setViewMode('table')}
              title="Tampilan Tabel Rinci"
              className={cn(
                "p-1.5 rounded-lg transition-all",
                viewMode === 'table' ? "bg-white text-indigo-600 shadow-sm" : "text-slate-400 hover:text-slate-700"
              )}
            >
              <LayoutList className="w-4 h-4" />
            </button>
          </div>

          <button
            onClick={handleExportJson}
            disabled={apps.length === 0}
            className={cn(
              "flex items-center gap-2 px-4 py-2 rounded-xl text-xs font-black uppercase tracking-wider transition-all shadow-md active:scale-95 disabled:opacity-50",
              exportSuccess
                ? "bg-emerald-600 text-white"
                : "bg-indigo-600 hover:bg-indigo-700 text-white shadow-indigo-100"
            )}
          >
            {exportSuccess ? <Check className="w-3.5 h-3.5" /> : <Download className="w-3.5 h-3.5" />}
            {exportSuccess ? "JSON Diunduh!" : "Ekspor Daftar (.json)"}
          </button>
        </div>
      </div>

      {/* Filter and Search Bar */}
      <div className="px-5 py-3 bg-white border-b border-slate-100 flex flex-col sm:flex-row sm:items-center justify-between gap-3">
        {/* Category Pills */}
        <div className="flex items-center gap-1.5 overflow-x-auto no-scrollbar">
          <button
            onClick={() => setFilterCategory('all')}
            className={cn(
              "px-3 py-1.5 rounded-xl text-[10px] font-black uppercase tracking-wider transition-all border",
              filterCategory === 'all'
                ? "bg-slate-900 text-white border-slate-900 shadow-sm"
                : "bg-slate-50 text-slate-500 border-slate-200/50 hover:bg-slate-100 hover:text-slate-800"
            )}
          >
            Semua ({apps.length})
          </button>
          <button
            onClick={() => setFilterCategory('user')}
            className={cn(
              "px-3 py-1.5 rounded-xl text-[10px] font-black uppercase tracking-wider transition-all border flex items-center gap-1",
              filterCategory === 'user'
                ? "bg-indigo-600 text-white border-indigo-600 shadow-sm"
                : "bg-slate-50 text-slate-500 border-slate-200/50 hover:bg-slate-100 hover:text-slate-800"
            )}
          >
            <Box className="w-3 h-3" /> Aplikasi Pengguna ({userAppsCount})
          </button>
          <button
            onClick={() => setFilterCategory('system')}
            className={cn(
              "px-3 py-1.5 rounded-xl text-[10px] font-black uppercase tracking-wider transition-all border flex items-center gap-1",
              filterCategory === 'system'
                ? "bg-slate-800 text-white border-slate-800 shadow-sm"
                : "bg-slate-50 text-slate-500 border-slate-200/50 hover:bg-slate-100 hover:text-slate-800"
            )}
          >
            <ShieldCheck className="w-3 h-3" /> Paket Sistem ({systemAppsCount})
          </button>
        </div>

        {/* Local Search */}
        <div className="relative w-full sm:w-64">
          <input
            type="text"
            placeholder="Cari nama atau package..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full bg-slate-50 border border-slate-200/80 px-3.5 py-1.5 rounded-xl text-xs outline-none focus:ring-2 focus:ring-indigo-500/10 focus:border-indigo-300 transition-all"
          />
        </div>
      </div>

      {/* Main Apps Content */}
      <div className="flex-1 flex overflow-hidden">
        
        {/* Apps Grid or Table List */}
        <div className="flex-1 overflow-y-auto custom-scrollbar p-6">
          {filteredApps.length > 0 ? (
            viewMode === 'grid' ? (
              /* Grid Cards Mode */
              <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-4">
                {filteredApps.map((app, idx) => {
                  const name = getAppDisplayName(app);
                  const isSelected = selectedApp?.package_name === app.package_name;
                  const isSystem = app.is_system !== undefined ? app.is_system : isSystemPackage(app.package_name);
                  const installerInfo = getInstallerLabel(app.installer);

                  return (
                    <div
                      key={idx}
                      onClick={() => setSelectedApp(app)}
                      className={cn(
                        "p-5 rounded-3xl border transition-all flex flex-col items-start cursor-pointer select-none group relative",
                        isSelected
                          ? "bg-indigo-50/90 border-indigo-400 ring-2 ring-indigo-500/20 shadow-md"
                          : "bg-white border-slate-200/70 hover:border-indigo-200 hover:shadow-lg hover:bg-slate-50/50"
                      )}
                    >
                      <div className="flex items-center justify-between w-full mb-3">
                        <div className={cn("w-12 h-12 rounded-2xl bg-gradient-to-br flex items-center justify-center font-black text-sm shadow-md group-hover:scale-105 transition-transform", getAppColor(name))}>
                          {name.substring(0, 2).toUpperCase()}
                        </div>

                        <span className={cn(
                          "text-[9px] font-black uppercase tracking-wider px-2 py-0.5 rounded-full border",
                          isSystem
                            ? "bg-slate-100 text-slate-500 border-slate-200"
                            : "bg-emerald-50 text-emerald-700 border-emerald-100"
                        )}>
                          {isSystem ? 'Sistem' : 'User'}
                        </span>
                      </div>

                      <h4 className={cn("text-sm font-black truncate w-full tracking-tight", isSelected ? "text-indigo-950" : "text-slate-900")}>
                        {name}
                      </h4>

                      <p className="text-[10px] font-mono text-slate-400 truncate w-full mt-0.5" title={app.package_name}>
                        {app.package_name}
                      </p>

                      <div className="mt-4 pt-3 border-t border-slate-100 w-full flex items-center justify-between text-[10px] font-bold text-slate-400">
                        <span className="bg-slate-100 px-2 py-0.5 rounded-lg text-slate-600 font-mono">
                          v{app.version_name || (app.version_code ? `code ${app.version_code}` : '1.0')}
                        </span>
                        <span className="truncate max-w-[100px]" title={installerInfo.label}>
                          {installerInfo.isPlayStore ? 'Play Store' : 'APK'}
                        </span>
                      </div>
                    </div>
                  );
                })}
              </div>
            ) : (
              /* Table Mode */
              <div className="bg-white rounded-3xl border border-slate-200/80 overflow-hidden shadow-sm">
                <table className="w-full text-left border-collapse">
                  <thead className="bg-slate-50 border-b border-slate-100 text-[10px] font-black text-slate-400 uppercase tracking-wider">
                    <tr>
                      <th className="px-6 py-3.5">Nama Aplikasi</th>
                      <th className="px-6 py-3.5">Package ID</th>
                      <th className="px-6 py-3.5">Versi</th>
                      <th className="px-6 py-3.5">Kategori</th>
                      <th className="px-6 py-3.5">Sumber Pemasang</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-slate-100 text-xs">
                    {filteredApps.map((app, idx) => {
                      const name = getAppDisplayName(app);
                      const isSelected = selectedApp?.package_name === app.package_name;
                      const isSystem = app.is_system !== undefined ? app.is_system : isSystemPackage(app.package_name);
                      const installerInfo = getInstallerLabel(app.installer);

                      return (
                        <tr
                          key={idx}
                          onClick={() => setSelectedApp(app)}
                          className={cn(
                            "cursor-pointer transition-all",
                            isSelected ? "bg-indigo-50/80 font-bold" : "hover:bg-slate-50/60"
                          )}
                        >
                          <td className="px-6 py-3.5">
                            <div className="flex items-center gap-3">
                              <div className={cn("w-8 h-8 rounded-xl bg-gradient-to-br flex items-center justify-center font-black text-[10px] shadow-sm shrink-0", getAppColor(name))}>
                                {name.substring(0, 2).toUpperCase()}
                              </div>
                              <span className="font-bold text-slate-900 truncate">{name}</span>
                            </div>
                          </td>
                          <td className="px-6 py-3.5 font-mono text-slate-400 text-[11px] truncate max-w-xs">
                            {app.package_name}
                          </td>
                          <td className="px-6 py-3.5 font-mono text-slate-600 text-[11px]">
                            v{app.version_name || app.version_code || '1.0'}
                          </td>
                          <td className="px-6 py-3.5">
                            <span className={cn(
                              "text-[9px] font-black uppercase tracking-wider px-2.5 py-0.5 rounded-full border",
                              isSystem
                                ? "bg-slate-100 text-slate-500 border-slate-200"
                                : "bg-emerald-50 text-emerald-700 border-emerald-100"
                            )}>
                              {isSystem ? 'Sistem' : 'User'}
                            </span>
                          </td>
                          <td className="px-6 py-3.5 text-slate-500 text-[11px]">
                            {installerInfo.label}
                          </td>
                        </tr>
                      );
                    })}
                  </tbody>
                </table>
              </div>
            )
          ) : (
            <div className="h-full flex flex-col items-center justify-center py-20 text-slate-300 space-y-3">
              <Layers className="w-12 h-12 opacity-20" />
              <p className="font-black uppercase tracking-widest text-xs">Tidak Ada Aplikasi Ditemukan</p>
              <p className="text-xs text-slate-400">Coba ubah kata kunci pencarian atau filter kategori.</p>
            </div>
          )}
        </div>

        {/* Right Pane: App Inspector Card */}
        {selectedApp && (
          <div className="w-80 md:w-96 bg-white border-l border-slate-200/80 flex flex-col overflow-y-auto custom-scrollbar p-6 shrink-0 space-y-6">
            <div className="flex flex-col items-center text-center pb-6 border-b border-slate-100 relative">
              <div className={cn("w-20 h-20 rounded-3xl bg-gradient-to-br flex items-center justify-center font-black text-2xl shadow-xl mb-4", getAppColor(getAppDisplayName(selectedApp)))}>
                {getAppDisplayName(selectedApp).substring(0, 2).toUpperCase()}
              </div>

              <h3 className="text-lg font-black text-slate-900 tracking-tight leading-snug">
                {getAppDisplayName(selectedApp)}
              </h3>

              <p className="text-xs font-mono text-slate-400 mt-1 break-all select-all">
                {selectedApp.package_name}
              </p>

              {/* Play Store Link Button */}
              <div className="flex items-center gap-2 mt-4">
                <a
                  href={`https://play.google.com/store/apps/details?id=${selectedApp.package_name}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center gap-2 px-4 py-2 bg-indigo-50 text-indigo-700 hover:bg-indigo-100 rounded-xl text-xs font-black uppercase tracking-wider transition-all"
                >
                  <ExternalLink className="w-3.5 h-3.5" /> Buka di Play Store
                </a>
              </div>
            </div>

            {/* App Details List */}
            <div className="space-y-4">
              <h4 className="text-[10px] font-black uppercase tracking-widest text-slate-400">
                Informasi Paket APK
              </h4>

              <div className="space-y-2.5">
                <div className="p-3.5 bg-slate-50 rounded-2xl border border-slate-100 flex justify-between items-center">
                  <span className="text-[11px] font-bold text-slate-500">Versi Aplikasi</span>
                  <span className="text-xs font-mono font-black text-slate-900">
                    {selectedApp.version_name || 'Tidak diketahui'}
                  </span>
                </div>

                <div className="p-3.5 bg-slate-50 rounded-2xl border border-slate-100 flex justify-between items-center">
                  <span className="text-[11px] font-bold text-slate-500">Version Code</span>
                  <span className="text-xs font-mono font-black text-slate-900">
                    {selectedApp.version_code || '--'}
                  </span>
                </div>

                <div className="p-3.5 bg-slate-50 rounded-2xl border border-slate-100 flex justify-between items-center">
                  <span className="text-[11px] font-bold text-slate-500">Tipe Paket</span>
                  <span className="text-xs font-bold text-indigo-600">
                    {isSystemPackage(selectedApp.package_name) ? 'Paket Sistem' : 'Aplikasi Pengguna'}
                  </span>
                </div>

                <div className="p-3.5 bg-slate-50 rounded-2xl border border-slate-100 flex justify-between items-center">
                  <span className="text-[11px] font-bold text-slate-500">Sumber Pemasang</span>
                  <span className="text-xs font-bold text-slate-800">
                    {getInstallerLabel(selectedApp.installer).label}
                  </span>
                </div>
              </div>
            </div>

            {/* Quick Action Buttons */}
            <div className="pt-4 border-t border-slate-100 space-y-2">
              <button
                onClick={() => handleCopy(selectedApp.package_name, 'copy-pkg')}
                className="w-full flex items-center justify-center gap-2 py-3 bg-slate-100 hover:bg-slate-200 text-slate-700 rounded-2xl text-xs font-black uppercase tracking-wider transition-all"
              >
                {copiedField === 'copy-pkg' ? <Check className="w-4 h-4 text-emerald-600" /> : <Copy className="w-4 h-4" />}
                {copiedField === 'copy-pkg' ? 'Package ID Tersalin!' : 'Salin Package ID'}
              </button>

              <button
                onClick={() => handleCopy(JSON.stringify(selectedApp, null, 2), 'copy-json')}
                className="w-full flex items-center justify-center gap-2 py-3 bg-slate-50 hover:bg-slate-100 text-slate-600 rounded-2xl text-xs font-bold transition-all border border-slate-200/60"
              >
                {copiedField === 'copy-json' ? <Check className="w-4 h-4 text-emerald-600" /> : <Copy className="w-4 h-4" />}
                {copiedField === 'copy-json' ? 'JSON Tersalin!' : 'Salin Metadata JSON'}
              </button>
            </div>

          </div>
        )}

      </div>
    </div>
  );
}
