import { Layers } from 'lucide-react';
import { cn } from '@/shared/lib/utils';
import type { AppData } from './appsUtils';
import { getAppColor, getAppDisplayName, isSystemPackage, getInstallerLabel } from './appsUtils';

interface AppListViewProps {
  apps: AppData[];
  selectedApp: AppData | null;
  onSelectApp: (app: AppData) => void;
  viewMode: 'grid' | 'table';
}

export function AppListView({ apps, selectedApp, onSelectApp, viewMode }: AppListViewProps) {
  if (apps.length === 0) {
    return (
      <div className="h-full flex flex-col items-center justify-center py-20 text-slate-300 space-y-3">
        <Layers className="w-12 h-12 opacity-20" />
        <p className="font-black uppercase tracking-widest text-xs">Tidak Ada Aplikasi Ditemukan</p>
        <p className="text-xs text-slate-400">Coba ubah kata kunci pencarian atau filter kategori.</p>
      </div>
    );
  }

  if (viewMode === 'grid') {
    return (
      <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-4">
        {apps.map((app, idx) => {
          const name = getAppDisplayName(app);
          const isSelected = selectedApp?.package_name === app.package_name;
          const isSystem = app.is_system !== undefined ? app.is_system : isSystemPackage(app.package_name);
          const installerInfo = getInstallerLabel(app.installer);

          return (
            <div
              key={idx}
              onClick={() => onSelectApp(app)}
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
                  isSystem ? "bg-slate-100 text-slate-500 border-slate-200" : "bg-emerald-50 text-emerald-700 border-emerald-100"
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
    );
  }

  return (
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
          {apps.map((app, idx) => {
            const name = getAppDisplayName(app);
            const isSelected = selectedApp?.package_name === app.package_name;
            const isSystem = app.is_system !== undefined ? app.is_system : isSystemPackage(app.package_name);
            const installerInfo = getInstallerLabel(app.installer);

            return (
              <tr
                key={idx}
                onClick={() => onSelectApp(app)}
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
                    isSystem ? "bg-slate-100 text-slate-500 border-slate-200" : "bg-emerald-50 text-emerald-700 border-emerald-100"
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
  );
}
