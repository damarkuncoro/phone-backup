import { useState, useMemo } from 'react';
import { Smartphone, LayoutGrid, LayoutList, Download, Check, Box, ShieldCheck } from 'lucide-react';
import { cn } from '@/shared/lib/utils';
import type { AppData } from './appsUtils';
import { getAppDisplayName, isSystemPackage } from './appsUtils';
import { AppInspectorPane } from './AppInspectorPane';
import { AppListView } from './AppListView';

export * from './appsUtils';

interface AppsExplorerProps {
  apps: AppData[];
  snapshotId: string;
}

export function AppsExplorer({ apps = [], snapshotId }: AppsExplorerProps) {
  const [viewMode, setViewMode] = useState<'grid' | 'table'>('grid');
  const [filterCategory, setFilterCategory] = useState<'all' | 'user' | 'system'>('all');
  const [selectedApp, setSelectedApp] = useState<AppData | null>(apps[0] || null);
  const [searchQuery, setSearchQuery] = useState('');
  const [exportSuccess, setExportSuccess] = useState(false);

  useMemo(() => {
    if (!selectedApp && apps && apps.length > 0) {
      setSelectedApp(apps[0]);
    }
  }, [apps, selectedApp]);

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
              exportSuccess ? "bg-emerald-600 text-white" : "bg-indigo-600 hover:bg-indigo-700 text-white shadow-indigo-100"
            )}
          >
            {exportSuccess ? <Check className="w-3.5 h-3.5" /> : <Download className="w-3.5 h-3.5" />}
            {exportSuccess ? "JSON Diunduh!" : "Ekspor Daftar (.json)"}
          </button>
        </div>
      </div>

      <div className="px-5 py-3 bg-white border-b border-slate-100 flex flex-col sm:flex-row sm:items-center justify-between gap-3">
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

      <div className="flex-1 flex overflow-hidden">
        <div className="flex-1 overflow-y-auto custom-scrollbar p-6">
          <AppListView
            apps={filteredApps}
            selectedApp={selectedApp}
            onSelectApp={setSelectedApp}
            viewMode={viewMode}
          />
        </div>
        <AppInspectorPane selectedApp={selectedApp} />
      </div>
    </div>
  );
}
