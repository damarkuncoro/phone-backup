import { useState } from 'react';
import { ExternalLink, Copy, Check } from 'lucide-react';
import { cn } from '@/shared/lib/utils';
import type { AppData } from './appsUtils';
import { getAppColor, getAppDisplayName, isSystemPackage, getInstallerLabel } from './appsUtils';

interface AppInspectorPaneProps {
  selectedApp: AppData | null;
}

export function AppInspectorPane({ selectedApp }: AppInspectorPaneProps) {
  const [copiedField, setCopiedField] = useState<string | null>(null);

  if (!selectedApp) return null;

  const handleCopy = (text: string, id: string) => {
    navigator.clipboard.writeText(text);
    setCopiedField(id);
    setTimeout(() => setCopiedField(null), 2000);
  };

  const name = getAppDisplayName(selectedApp);
  const installerInfo = getInstallerLabel(selectedApp.installer);

  return (
    <div className="w-80 md:w-96 bg-white border-l border-slate-200/80 flex flex-col overflow-y-auto custom-scrollbar p-6 shrink-0 space-y-6">
      <div className="flex flex-col items-center text-center pb-6 border-b border-slate-100 relative">
        <div className={cn("w-20 h-20 rounded-3xl bg-gradient-to-br flex items-center justify-center font-black text-2xl shadow-xl mb-4", getAppColor(name))}>
          {name.substring(0, 2).toUpperCase()}
        </div>

        <h3 className="text-lg font-black text-slate-900 tracking-tight leading-snug">
          {name}
        </h3>

        <p className="text-xs font-mono text-slate-400 mt-1 break-all select-all">
          {selectedApp.package_name}
        </p>

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
              {installerInfo.label}
            </span>
          </div>
        </div>
      </div>

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
  );
}
