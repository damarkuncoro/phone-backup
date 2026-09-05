import { useState } from 'react';
import { Database, HardDrive, Cloud, Cpu, FolderOpen, ExternalLink, CheckCircle, AlertTriangle, RefreshCw, Server } from 'lucide-react';
import { cn } from "@/shared/lib/utils";
import { safeInvoke } from '@/shared/lib/ipc';
import { SettingsCompressionSection } from './SettingsCompressionSection';

interface SettingsStorageTabProps {
  currentBackendType: string;
  onSelectBackend: (backend: any) => void;
  s3Bucket: string;
  setS3Bucket: (v: string) => void;
  s3Region: string;
  setS3Region: (v: string) => void;
  s3Endpoint: string;
  setS3Endpoint: (v: string) => void;
  s3AccessKey: string;
  setS3AccessKey: (v: string) => void;
  s3SecretKey: string;
  setS3SecretKey: (v: string) => void;
  onOpenFolder: (folderType: 'restore' | 'downloads') => void;
}

export function SettingsStorageTab({
  currentBackendType,
  onSelectBackend,
  s3Bucket,
  setS3Bucket,
  s3Region,
  setS3Region,
  s3Endpoint,
  setS3Endpoint,
  s3AccessKey,
  setS3AccessKey,
  s3SecretKey,
  setS3SecretKey,
  onOpenFolder,
}: SettingsStorageTabProps) {
  const [testingCloud, setTestingCloud] = useState(false);
  const [cloudResult, setCloudResult] = useState<{ success: boolean; message: string } | null>(null);

  const handleTestConnection = async (provider: string) => {
    setTestingCloud(true);
    setCloudResult(null);
    try {
      const res = await safeInvoke<{ success: boolean; message: string }>('test_cloud_connection', {
        provider,
        bucket: s3Bucket || 'backup',
        endpoint: s3Endpoint || null,
        region: s3Region || null,
        access_key: s3AccessKey || null,
        secret_key: s3SecretKey || null,
      });
      setCloudResult(res);
    } catch (e: any) {
      setCloudResult({ success: false, message: e?.toString() || 'Connection failed' });
    } finally {
      setTestingCloud(false);
    }
  };

  return (
    <div className="space-y-6 animate-in fade-in duration-200">
      <div className="bg-white p-6 md:p-8 rounded-[32px] border border-slate-100 shadow-sm space-y-6">
        <div>
          <h3 className="text-base font-black text-slate-900 tracking-tight flex items-center gap-2">
            <Database className="w-5 h-5 text-indigo-600" /> Storage Engine Backend
          </h3>
          <p className="text-xs text-slate-400 font-medium mt-0.5">
            Pilih lokasi di mana seluruh snapshot cadangan dan chunk terdeduplikasi akan disimpan.
          </p>
        </div>

        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          <StorageCard
            selected={currentBackendType === 'Local'}
            onClick={() => onSelectBackend({ Local: null })}
            icon={HardDrive}
            title="Local Disk"
            desc="Penyimpanan lokal di hard disk komputer."
            badge="Default"
          />
          <StorageCard
            selected={currentBackendType === 'S3'}
            onClick={() => onSelectBackend({ S3: { bucket: s3Bucket, region: s3Region, endpoint: s3Endpoint, access_key: s3AccessKey, secret_key: s3SecretKey } })}
            icon={Cloud}
            title="AWS S3 / MinIO"
            desc="Cadangkan langsung ke S3 / Cloudflare R2."
            badge="S3"
          />
          <StorageCard
            selected={currentBackendType === 'WebDAV'}
            onClick={() => onSelectBackend({ WebDAV: { endpoint: s3Endpoint, username: s3AccessKey, password: s3SecretKey } })}
            icon={Server}
            title="Nextcloud / WebDAV"
            desc="Simpan ke NAS / Nextcloud pribadi."
            badge="WebDAV"
          />
          <StorageCard
            selected={currentBackendType === 'Mock'}
            onClick={() => onSelectBackend({ Mock: null })}
            icon={Cpu}
            title="Mock Storage"
            desc="Penyimpanan in-memory (Testing)."
            badge="Test"
          />
        </div>

        {(currentBackendType === 'S3' || currentBackendType === 'WebDAV') && (
          <div className="p-6 bg-slate-50 border border-slate-200/80 rounded-3xl space-y-4 animate-in slide-in-from-top-2">
            <div className="flex items-center justify-between">
              <h4 className="text-xs font-black uppercase tracking-wider text-slate-700 flex items-center gap-2">
                <Cloud className="w-4 h-4 text-indigo-600" /> Kredensial {currentBackendType}
              </h4>
              <button
                type="button"
                onClick={() => handleTestConnection(currentBackendType.toLowerCase())}
                disabled={testingCloud}
                className="flex items-center gap-1.5 px-3 py-1 bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 text-white rounded-lg text-xs font-bold transition-all shadow-sm"
              >
                <RefreshCw className={cn("w-3 h-3", testingCloud && "animate-spin")} />
                {testingCloud ? "Menguji..." : "Test Connection"}
              </button>
            </div>

            <div className="flex flex-wrap gap-1.5">
              <span className="text-[10px] font-bold text-slate-400 self-center mr-1">Presets:</span>
              {[
                { name: 'AWS S3', reg: 'us-east-1', ep: '' },
                { name: 'Cloudflare R2', reg: 'auto', ep: 'https://<account_id>.r2.cloudflarestorage.com' },
                { name: 'MinIO Local', reg: 'us-east-1', ep: 'http://localhost:9000' },
                { name: 'Backblaze B2', reg: 'us-west-004', ep: 'https://s3.us-west-004.backblazeb2.com' },
              ].map(p => (
                <button
                  key={p.name}
                  type="button"
                  onClick={() => { setS3Region(p.reg); if (p.ep) setS3Endpoint(p.ep); }}
                  className="px-2.5 py-1 bg-white hover:bg-indigo-50 border border-slate-200 text-slate-700 hover:text-indigo-600 rounded-lg text-[10px] font-bold transition"
                >
                  {p.name}
                </button>
              ))}
            </div>

            <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
              {currentBackendType === 'S3' && (
                <>
                  <input type="text" placeholder="Bucket Name" value={s3Bucket} onChange={(e) => setS3Bucket(e.target.value)} className="w-full bg-white border border-slate-200 px-3 py-2 rounded-xl text-xs font-mono outline-none" />
                  <input type="text" placeholder="Region (e.g. us-east-1, auto)" value={s3Region} onChange={(e) => setS3Region(e.target.value)} className="w-full bg-white border border-slate-200 px-3 py-2 rounded-xl text-xs font-mono outline-none" />
                </>
              )}
              <input type="text" placeholder={currentBackendType === 'S3' ? "Endpoint URL (leave empty for AWS S3)" : "WebDAV URL"} value={s3Endpoint} onChange={(e) => setS3Endpoint(e.target.value)} className="sm:col-span-2 w-full bg-white border border-slate-200 px-3 py-2 rounded-xl text-xs font-mono outline-none" />
              <input type="text" placeholder={currentBackendType === 'S3' ? "Access Key ID" : "Username"} value={s3AccessKey} onChange={(e) => setS3AccessKey(e.target.value)} className="w-full bg-white border border-slate-200 px-3 py-2 rounded-xl text-xs font-mono outline-none" />
              <input type="password" placeholder={currentBackendType === 'S3' ? "Secret Access Key" : "Password / App Token"} value={s3SecretKey} onChange={(e) => setS3SecretKey(e.target.value)} className="w-full bg-white border border-slate-200 px-3 py-2 rounded-xl text-xs font-mono outline-none" />
            </div>

            {cloudResult && (
              <div className={cn("flex items-center gap-2.5 p-3 rounded-xl text-xs font-bold", cloudResult.success ? "bg-emerald-50 text-emerald-700 border border-emerald-200" : "bg-rose-50 text-rose-700 border border-rose-200")}>
                {cloudResult.success ? <CheckCircle className="w-4 h-4 flex-shrink-0" /> : <AlertTriangle className="w-4 h-4 flex-shrink-0" />}
                <span>{cloudResult.message}</span>
              </div>
            )}
          </div>
        )}

        <SettingsCompressionSection />

        <div className="border-t border-slate-100 pt-6 space-y-3">
          <h4 className="text-xs font-black uppercase tracking-wider text-slate-700">Akses Cepat Direktori Sistem</h4>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
            {[
              { type: 'restore' as const, title: 'Buka Folder Restore', desc: 'Lokasi file hasil pemulihan snapshot' },
              { type: 'downloads' as const, title: 'Buka Folder Downloads', desc: 'Berkas unduhan dari HP' },
            ].map(f => (
              <button key={f.type} type="button" onClick={() => onOpenFolder(f.type)} className="flex items-center justify-between p-4 bg-slate-50 hover:bg-indigo-50 border border-slate-200/70 hover:border-indigo-200 rounded-2xl transition-all group text-left">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-xl bg-white flex items-center justify-center text-slate-600 group-hover:text-indigo-600 shadow-sm"><FolderOpen className="w-5 h-5" /></div>
                  <div><p className="text-xs font-black text-slate-800">{f.title}</p><p className="text-[10px] text-slate-400">{f.desc}</p></div>
                </div>
                <ExternalLink className="w-4 h-4 text-slate-400 group-hover:text-indigo-600" />
              </button>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

function StorageCard({ selected, onClick, icon: Icon, title, desc, badge }: any) {
  return (
    <div onClick={onClick} className={cn("relative p-5 rounded-2xl border-2 transition-all cursor-pointer flex flex-col justify-between space-y-3", selected ? "bg-indigo-50/50 border-indigo-600 shadow-sm" : "bg-white border-slate-100 hover:border-slate-200")}>
      <div className="flex items-start justify-between">
        <div className={cn("w-10 h-10 rounded-xl flex items-center justify-center", selected ? "bg-indigo-600 text-white shadow-md shadow-indigo-600/20" : "bg-slate-100 text-slate-500")}><Icon className="w-5 h-5" /></div>
        <span className={cn("text-[10px] font-black uppercase px-2 py-0.5 rounded-full", selected ? "bg-indigo-600 text-white" : "bg-slate-100 text-slate-500")}>{badge}</span>
      </div>
      <div><h4 className="text-xs font-black text-slate-900">{title}</h4><p className="text-[11px] text-slate-400 font-medium leading-relaxed mt-1">{desc}</p></div>
    </div>
  );
}
