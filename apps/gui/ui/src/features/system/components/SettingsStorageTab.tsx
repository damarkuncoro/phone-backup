import { Database, HardDrive, Cloud, Cpu, FolderOpen, ExternalLink } from 'lucide-react';
import { cn } from "@/shared/lib/utils";
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

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <StorageCard
            selected={currentBackendType === 'Local'}
            onClick={() => onSelectBackend({ Local: null })}
            icon={HardDrive}
            title="Local Disk"
            desc="Penyimpanan lokal di hard disk komputer (workspace/backups)."
            badge="Default"
          />
          <StorageCard
            selected={currentBackendType === 'S3'}
            onClick={() => onSelectBackend({ S3: { bucket: s3Bucket, region: s3Region, endpoint: s3Endpoint, access_key: s3AccessKey, secret_key: s3SecretKey } })}
            icon={Cloud}
            title="Cloud Storage (S3)"
            desc="Cadangkan langsung ke Amazon S3, MinIO lokal, atau Cloudflare R2."
            badge="Cloud"
          />
          <StorageCard
            selected={currentBackendType === 'Mock'}
            onClick={() => onSelectBackend({ Mock: null })}
            icon={Cpu}
            title="Mock Storage"
            desc="Penyimpanan memori sementara tanpa menulis ke disk (Testing)."
            badge="Testing"
          />
        </div>

        {currentBackendType === 'S3' && (
          <div className="p-6 bg-slate-50 border border-slate-200/80 rounded-3xl space-y-4 animate-in slide-in-from-top-2">
            <h4 className="text-xs font-black uppercase tracking-wider text-slate-700 flex items-center gap-2">
              <Cloud className="w-4 h-4 text-indigo-600" /> Kredensial &amp; Endpoint S3 / MinIO
            </h4>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <input type="text" placeholder="Bucket Name" value={s3Bucket} onChange={(e) => setS3Bucket(e.target.value)} className="w-full bg-white border border-slate-200 px-3 py-2 rounded-xl text-xs font-mono outline-none" />
              <input type="text" placeholder="Region (us-east-1)" value={s3Region} onChange={(e) => setS3Region(e.target.value)} className="w-full bg-white border border-slate-200 px-3 py-2 rounded-xl text-xs font-mono outline-none" />
              <input type="text" placeholder="Custom Endpoint (https://s3...)" value={s3Endpoint} onChange={(e) => setS3Endpoint(e.target.value)} className="sm:col-span-2 w-full bg-white border border-slate-200 px-3 py-2 rounded-xl text-xs font-mono outline-none" />
              <input type="password" placeholder="Access Key ID" value={s3AccessKey} onChange={(e) => setS3AccessKey(e.target.value)} className="w-full bg-white border border-slate-200 px-3 py-2 rounded-xl text-xs font-mono outline-none" />
              <input type="password" placeholder="Secret Access Key" value={s3SecretKey} onChange={(e) => setS3SecretKey(e.target.value)} className="w-full bg-white border border-slate-200 px-3 py-2 rounded-xl text-xs font-mono outline-none" />
            </div>
          </div>
        )}

        <SettingsCompressionSection />

        <div className="border-t border-slate-100 pt-6 space-y-3">
          <h4 className="text-xs font-black uppercase tracking-wider text-slate-700">Akses Cepat Direktori Sistem</h4>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
            <button type="button" onClick={() => onOpenFolder('restore')} className="flex items-center justify-between p-4 bg-slate-50 hover:bg-indigo-50 border border-slate-200/70 hover:border-indigo-200 rounded-2xl transition-all group text-left">
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 rounded-xl bg-white flex items-center justify-center text-slate-600 group-hover:text-indigo-600 shadow-sm"><FolderOpen className="w-5 h-5" /></div>
                <div><p className="text-xs font-black text-slate-800">Buka Folder Restore</p><p className="text-[10px] text-slate-400">Lokasi file hasil pemulihan snapshot</p></div>
              </div>
              <ExternalLink className="w-4 h-4 text-slate-400 group-hover:text-indigo-600" />
            </button>
            <button type="button" onClick={() => onOpenFolder('downloads')} className="flex items-center justify-between p-4 bg-slate-50 hover:bg-indigo-50 border border-slate-200/70 hover:border-indigo-200 rounded-2xl transition-all group text-left">
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 rounded-xl bg-white flex items-center justify-center text-slate-600 group-hover:text-indigo-600 shadow-sm"><FolderOpen className="w-5 h-5" /></div>
                <div><p className="text-xs font-black text-slate-800">Buka Folder Unduhan</p><p className="text-[10px] text-slate-400">Lokasi berkas unduhan tunggal / batch</p></div>
              </div>
              <ExternalLink className="w-4 h-4 text-slate-400 group-hover:text-indigo-600" />
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

function StorageCard({ selected, onClick, icon: Icon, title, desc, badge }: { selected: boolean; onClick: () => void; icon: any; title: string; desc: string; badge: string; }) {
  return (
    <div onClick={onClick} className={cn("p-5 rounded-3xl border transition-all cursor-pointer flex flex-col justify-between space-y-3", selected ? "bg-indigo-50/50 border-indigo-300 ring-2 ring-indigo-500/10 shadow-md shadow-indigo-100/50" : "bg-slate-50 border-slate-200/70 hover:border-slate-300")}>
      <div className="flex items-center justify-between">
        <div className={cn("w-10 h-10 rounded-2xl flex items-center justify-center", selected ? "bg-indigo-600 text-white shadow-md shadow-indigo-200" : "bg-white text-slate-600 border border-slate-200")}><Icon className="w-5 h-5" /></div>
        <span className={cn("text-[9px] font-black px-2.5 py-0.5 rounded-full uppercase tracking-wider", selected ? "bg-indigo-600 text-white" : "bg-slate-200 text-slate-600")}>{badge}</span>
      </div>
      <div>
        <h4 className="text-xs font-black text-slate-900 uppercase tracking-wider">{title}</h4>
        <p className="text-[11px] text-slate-500 font-medium mt-1 leading-relaxed">{desc}</p>
      </div>
    </div>
  );
}
