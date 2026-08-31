import { useState } from 'react';
import {
  Settings as SettingsIcon, Shield, Database, Activity,
  RefreshCw, Trash2, Key, Save, AlertCircle, CheckCircle2,
  HardDrive, Monitor, Terminal, FolderOpen, Cloud, Lock,
  Copy, Check, Info, Cpu, Sparkles, ExternalLink
} from 'lucide-react';
import { cn } from "@/shared/lib/utils";
import { useSettings } from '../hooks/useSettings';
import { systemService } from '@/services/systemService';

type SettingsTab = 'doctor' | 'storage' | 'security' | 'maintenance' | 'about';

export function SettingsPage() {
  const {
    report,
    settings, setSettings,
    keys,
    loading,
    saving,
    msg,
    handleSave,
    runMaintenance
  } = useSettings();

  const [activeTab, setActiveTab] = useState<SettingsTab>('doctor');
  const [copiedKey, setCopiedKey] = useState(false);
  const [refreshingDoctor, setRefreshingDoctor] = useState(false);

  // S3 Form State if cloud backend is selected
  const [s3Bucket, setS3Bucket] = useState('');
  const [s3Endpoint, setS3Endpoint] = useState('');
  const [s3Region, setS3Region] = useState('us-east-1');
  const [s3AccessKey, setS3AccessKey] = useState('');
  const [s3SecretKey, setS3SecretKey] = useState('');

  if (loading) {
    return (
      <div className="h-full flex flex-col items-center justify-center gap-4 text-slate-400">
        <RefreshCw className="w-8 h-8 animate-spin text-indigo-500" />
        <p className="text-xs font-black uppercase tracking-widest">Mendiagnosa Infrastruktur Sistem...</p>
      </div>
    );
  }

  const currentBackendType = typeof settings?.storage_backend === 'string'
    ? settings.storage_backend
    : Object.keys(settings?.storage_backend || { Local: null })[0];

  const handleCopyPublicKey = () => {
    if (keys && keys[1]) {
      navigator.clipboard.writeText(keys[1]);
      setCopiedKey(true);
      setTimeout(() => setCopiedKey(false), 2500);
    }
  };

  const handleOpenFolder = async (folderType: 'restore' | 'downloads') => {
    try {
      if (folderType === 'restore') {
        await systemService.openRestoreFolder();
      } else {
        await systemService.openDownloadsFolder();
      }
    } catch {
      console.warn("Gagal membuka folder di sistem operasi");
    }
  };

  const handleRefreshDoctor = async () => {
    setRefreshingDoctor(true);
    try {
      await systemService.getDoctorReport();
    } catch {
      console.warn("Refresh doctor failed");
    } finally {
      setTimeout(() => setRefreshingDoctor(false), 600);
    }
  };

  return (
    <div className="p-6 md:p-8 space-y-6 max-w-6xl mx-auto animate-in fade-in duration-300">
      
      {/* Header */}
      <header className="flex flex-col md:flex-row md:items-center justify-between gap-4 bg-white p-6 rounded-[32px] border border-slate-100 shadow-sm">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-2xl bg-indigo-600 text-white flex items-center justify-center shadow-lg shadow-indigo-200 shrink-0">
            <SettingsIcon className="w-6 h-6" />
          </div>
          <div>
            <h1 className="text-2xl md:text-3xl font-black text-slate-900 tracking-tight">
              Pengaturan Sistem
            </h1>
            <p className="text-xs text-slate-400 font-medium mt-0.5">
              Kelola parameter infrastruktur, keamanan enkripsi, penyimpanan, dan pemeliharaan platform.
            </p>
          </div>
        </div>

        {/* Global Toast Alert */}
        {msg && (
          <div className={cn(
            "px-4 py-2.5 rounded-2xl text-xs font-bold flex items-center gap-2.5 animate-in slide-in-from-top-2 border shrink-0",
            msg.type === 'success' ? "bg-emerald-50 text-emerald-800 border-emerald-200" : "bg-red-50 text-red-800 border-red-200"
          )}>
            {msg.type === 'success' ? <CheckCircle2 className="w-4 h-4 text-emerald-600" /> : <AlertCircle className="w-4 h-4 text-red-600" />}
            <span>{msg.text}</span>
          </div>
        )}
      </header>

      {/* Navigation Tabs */}
      <nav className="flex flex-wrap gap-2 p-1.5 bg-slate-100/70 rounded-2xl border border-slate-200/60 select-none">
        <TabButton
          active={activeTab === 'doctor'}
          onClick={() => setActiveTab('doctor')}
          icon={Activity}
          label="System Doctor"
          badge={report?.adb_found && report?.db_healthy ? "Sehat" : "Peringatan"}
          badgeColor={report?.adb_found && report?.db_healthy ? "bg-emerald-100 text-emerald-700" : "bg-amber-100 text-amber-700"}
        />
        <TabButton
          active={activeTab === 'storage'}
          onClick={() => setActiveTab('storage')}
          icon={Database}
          label="Penyimpanan"
          badge={currentBackendType}
          badgeColor="bg-indigo-100 text-indigo-700"
        />
        <TabButton
          active={activeTab === 'security'}
          onClick={() => setActiveTab('security')}
          icon={Shield}
          label="Keamanan & Kunci"
        />
        <TabButton
          active={activeTab === 'maintenance'}
          onClick={() => setActiveTab('maintenance')}
          icon={HardDrive}
          label="Pemeliharaan"
        />
        <TabButton
          active={activeTab === 'about'}
          onClick={() => setActiveTab('about')}
          icon={Info}
          label="Tentang"
        />
      </nav>

      {/* Tab Panels */}
      <div className="space-y-6">

        {/* 1. SYSTEM DOCTOR TAB */}
        {activeTab === 'doctor' && (
          <div className="space-y-6 animate-in fade-in duration-200">
            <div className="flex justify-between items-center px-1">
              <div>
                <h3 className="text-base font-black text-slate-900 tracking-tight">Kesehatan Infrastruktur</h3>
                <p className="text-xs text-slate-400 font-medium">Status komponen kunci yang menggerakkan platform backup.</p>
              </div>
              <button
                type="button"
                onClick={handleRefreshDoctor}
                disabled={refreshingDoctor}
                className="flex items-center gap-2 px-4 py-2 bg-white border border-slate-200 hover:border-slate-300 text-slate-700 rounded-xl text-xs font-bold transition-all shadow-sm active:scale-95 disabled:opacity-50"
              >
                <RefreshCw className={cn("w-3.5 h-3.5", refreshingDoctor && "animate-spin text-indigo-600")} />
                {refreshingDoctor ? "Mendiagnosa..." : "Diagnosa Ulang"}
              </button>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
              <HealthCard
                icon={Terminal}
                title="ADB Engine"
                value={report?.adb_version ? "Tersedia & Aktif" : "Tidak Ditemukan"}
                desc={report?.adb_version || "Android Debug Bridge"}
                status={report?.adb_found ? 'healthy' : 'error'}
              />
              <HealthCard
                icon={Database}
                title="SQLite Database"
                value={report?.db_healthy ? "Operasional" : "Rusak / Error"}
                desc="Enkripsi metadata SQLCipher"
                status={report?.db_healthy ? 'healthy' : 'error'}
              />
              <HealthCard
                icon={Monitor}
                title="Koneksi Perangkat"
                value={`${report?.device_count ?? 0} Perangkat Terhubung`}
                desc="USB, WiFi & MTP"
                status="healthy"
              />
              <HealthCard
                icon={Sparkles}
                title="MTP Plug & Play"
                value="Siap Digunakan"
                desc="Media Transfer Protocol"
                status="healthy"
              />
            </div>

            {/* Diagnostic Details Log Card */}
            <div className="bg-slate-900 text-slate-300 p-6 rounded-[32px] shadow-xl border border-slate-800 space-y-3 font-mono text-xs">
              <div className="flex items-center justify-between border-b border-slate-800 pb-3">
                <span className="font-bold text-slate-400 flex items-center gap-2">
                  <Terminal className="w-4 h-4 text-emerald-400" /> Log Diagnostik Perangkat Keras
                </span>
                <span className="text-[10px] px-2.5 py-0.5 rounded-full bg-emerald-950 text-emerald-400 font-bold border border-emerald-800/50">
                  Semua Sistem Normal
                </span>
              </div>
              <div className="space-y-1 text-[11px] leading-relaxed">
                <p className="text-slate-400">&gt; Engine core initialized with CompositeDeviceAdapter (ADB + MTP).</p>
                <p className="text-slate-400">&gt; Storage backend: <span className="text-cyan-400">{currentBackendType}</span>.</p>
                <p className="text-slate-400">&gt; Encryption: <span className="text-emerald-400">Age X25519 (Chacha20-Poly1305) Active</span>.</p>
                <p className="text-slate-400">&gt; Deduplication chunker: <span className="text-indigo-400">FastCDC 3.1 Content-Defined Chunking</span>.</p>
              </div>
            </div>
          </div>
        )}

        {/* 2. STORAGE TAB */}
        {activeTab === 'storage' && (
          <div className="space-y-6 animate-in fade-in duration-200">
            <div className="bg-white p-6 md:p-8 rounded-[36px] border border-slate-100 shadow-sm space-y-6">
              <div>
                <h3 className="text-base font-black text-slate-900 tracking-tight flex items-center gap-2">
                  <Database className="w-5 h-5 text-indigo-600" /> Storage Engine Backend
                </h3>
                <p className="text-xs text-slate-400 font-medium mt-0.5">
                  Pilih lokasi di mana seluruh snapshot cadangan dan chunk terdeduplikasi akan disimpan.
                </p>
              </div>

              {/* Backend Type Cards */}
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <StorageCard
                  selected={currentBackendType === 'Local'}
                  onClick={() => setSettings(s => s ? { ...s, storage_backend: { Local: null } } : null)}
                  icon={HardDrive}
                  title="Local Disk"
                  desc="Penyimpanan lokal di hard disk komputer (workspace/backups). Sangat cepat dan aman."
                  badge="Default"
                />

                <StorageCard
                  selected={currentBackendType === 'S3'}
                  onClick={() => setSettings(s => s ? {
                    ...s,
                    storage_backend: {
                      S3: {
                        bucket: s3Bucket,
                        region: s3Region,
                        endpoint: s3Endpoint,
                        access_key: s3AccessKey,
                        secret_key: s3SecretKey
                      }
                    }
                  } : null)}
                  icon={Cloud}
                  title="Cloud Storage (S3)"
                  desc="Cadangkan langsung ke Amazon S3, MinIO lokal, atau Cloudflare R2 via OpenDAL."
                  badge="Cloud"
                />

                <StorageCard
                  selected={currentBackendType === 'Mock'}
                  onClick={() => setSettings(s => s ? { ...s, storage_backend: { Mock: null } } : null)}
                  icon={Cpu}
                  title="Mock Storage"
                  desc="Penyimpanan memori sementara tanpa menulis ke disk. Khusus untuk pengujian performa."
                  badge="Testing"
                />
              </div>

              {/* S3 Configuration Form if S3 is active */}
              {currentBackendType === 'S3' && (
                <div className="p-6 bg-slate-50 border border-slate-200/80 rounded-3xl space-y-4 animate-in slide-in-from-top-2">
                  <h4 className="text-xs font-black uppercase tracking-wider text-slate-700 flex items-center gap-2">
                    <Cloud className="w-4 h-4 text-indigo-600" /> Kredensial & Endpoint S3 / MinIO
                  </h4>
                  <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    <div className="space-y-1">
                      <label className="text-[10px] font-black uppercase tracking-widest text-slate-400">Bucket Name</label>
                      <input
                        type="text"
                        placeholder="my-phone-backups"
                        value={s3Bucket}
                        onChange={(e) => setS3Bucket(e.target.value)}
                        className="w-full bg-white border border-slate-200 px-3 py-2 rounded-xl text-xs font-mono outline-none"
                      />
                    </div>
                    <div className="space-y-1">
                      <label className="text-[10px] font-black uppercase tracking-widest text-slate-400">Region</label>
                      <input
                        type="text"
                        placeholder="us-east-1"
                        value={s3Region}
                        onChange={(e) => setS3Region(e.target.value)}
                        className="w-full bg-white border border-slate-200 px-3 py-2 rounded-xl text-xs font-mono outline-none"
                      />
                    </div>
                    <div className="sm:col-span-2 space-y-1">
                      <label className="text-[10px] font-black uppercase tracking-widest text-slate-400">Custom Endpoint (Opsional untuk MinIO / R2)</label>
                      <input
                        type="text"
                        placeholder="https://s3.amazonaws.com atau http://localhost:9000"
                        value={s3Endpoint}
                        onChange={(e) => setS3Endpoint(e.target.value)}
                        className="w-full bg-white border border-slate-200 px-3 py-2 rounded-xl text-xs font-mono outline-none"
                      />
                    </div>
                    <div className="space-y-1">
                      <label className="text-[10px] font-black uppercase tracking-widest text-slate-400">Access Key ID</label>
                      <input
                        type="password"
                        placeholder="AKIAIOSFODNN7EXAMPLE"
                        value={s3AccessKey}
                        onChange={(e) => setS3AccessKey(e.target.value)}
                        className="w-full bg-white border border-slate-200 px-3 py-2 rounded-xl text-xs font-mono outline-none"
                      />
                    </div>
                    <div className="space-y-1">
                      <label className="text-[10px] font-black uppercase tracking-widest text-slate-400">Secret Access Key</label>
                      <input
                        type="password"
                        placeholder="wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
                        value={s3SecretKey}
                        onChange={(e) => setS3SecretKey(e.target.value)}
                        className="w-full bg-white border border-slate-200 px-3 py-2 rounded-xl text-xs font-mono outline-none"
                      />
                    </div>
                  </div>
                </div>
              )}

              {/* Quick Folder Launchers */}
              <div className="border-t border-slate-100 pt-6 space-y-3">
                <h4 className="text-xs font-black uppercase tracking-wider text-slate-700">
                  Akses Cepat Direktori Sistem
                </h4>
                <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
                  <button
                    type="button"
                    onClick={() => handleOpenFolder('restore')}
                    className="flex items-center justify-between p-4 bg-slate-50 hover:bg-indigo-50 border border-slate-200/70 hover:border-indigo-200 rounded-2xl transition-all group text-left"
                  >
                    <div className="flex items-center gap-3">
                      <div className="w-10 h-10 rounded-xl bg-white flex items-center justify-center text-slate-600 group-hover:text-indigo-600 shadow-sm">
                        <FolderOpen className="w-5 h-5" />
                      </div>
                      <div>
                        <p className="text-xs font-black text-slate-800">Buka Folder Restore</p>
                        <p className="text-[10px] text-slate-400">Lokasi file hasil pemulihan snapshot</p>
                      </div>
                    </div>
                    <ExternalLink className="w-4 h-4 text-slate-400 group-hover:text-indigo-600" />
                  </button>

                  <button
                    type="button"
                    onClick={() => handleOpenFolder('downloads')}
                    className="flex items-center justify-between p-4 bg-slate-50 hover:bg-indigo-50 border border-slate-200/70 hover:border-indigo-200 rounded-2xl transition-all group text-left"
                  >
                    <div className="flex items-center gap-3">
                      <div className="w-10 h-10 rounded-xl bg-white flex items-center justify-center text-slate-600 group-hover:text-indigo-600 shadow-sm">
                        <FolderOpen className="w-5 h-5" />
                      </div>
                      <div>
                        <p className="text-xs font-black text-slate-800">Buka Folder Unduhan</p>
                        <p className="text-[10px] text-slate-400">Lokasi berkas unduhan tunggal / batch</p>
                      </div>
                    </div>
                    <ExternalLink className="w-4 h-4 text-slate-400 group-hover:text-indigo-600" />
                  </button>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* 3. SECURITY TAB */}
        {activeTab === 'security' && (
          <div className="space-y-6 animate-in fade-in duration-200">
            <div className="bg-white p-6 md:p-8 rounded-[36px] border border-slate-100 shadow-sm space-y-6">
              <div className="flex justify-between items-center">
                <div>
                  <h3 className="text-base font-black text-slate-900 tracking-tight flex items-center gap-2">
                    <Shield className="w-5 h-5 text-emerald-600" /> Keamanan & Kunci Kriptografi
                  </h3>
                  <p className="text-xs text-slate-400 font-medium mt-0.5">
                    Standar enkripsi modern Age (X25519) dengan perlindungan Chacha20-Poly1305.
                  </p>
                </div>
                <div className="flex items-center gap-2 px-3 py-1.5 bg-emerald-50 text-emerald-700 border border-emerald-200 rounded-xl text-[10px] font-black uppercase tracking-wider">
                  <Lock className="w-3.5 h-3.5" /> Enkripsi Otomatis Aktif
                </div>
              </div>

              <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                {/* Public Key Display */}
                <div className="p-6 bg-slate-50 border border-slate-200/80 rounded-3xl space-y-4">
                  <div className="flex justify-between items-center">
                    <span className="text-[10px] font-black uppercase tracking-widest text-slate-500">
                      Active Public Key (Untuk Enkripsi)
                    </span>
                    <button
                      type="button"
                      onClick={handleCopyPublicKey}
                      className="flex items-center gap-1.5 px-3 py-1.5 bg-white border border-slate-200 text-slate-700 hover:text-indigo-600 rounded-lg text-xs font-bold transition-all shadow-sm active:scale-95"
                    >
                      {copiedKey ? <Check className="w-3.5 h-3.5 text-emerald-600" /> : <Copy className="w-3.5 h-3.5" />}
                      {copiedKey ? "Tersalin!" : "Salin Kunci"}
                    </button>
                  </div>

                  <div className="bg-white p-4 rounded-2xl border border-slate-200 shadow-inner">
                    <code className="text-xs font-mono text-indigo-950 break-all leading-relaxed select-all">
                      {keys ? keys[1] : 'Memuat Public Key...'}
                    </code>
                  </div>
                  <p className="text-[10px] text-slate-500 font-medium leading-relaxed">
                    Kunci publik ini aman untuk dibagikan. Kunci ini digunakan oleh aplikasi untuk menyegel data cadangan Anda sebelum disimpan ke penyimpanan.
                  </p>
                </div>

                {/* Keypair Rotation Card */}
                <div className="p-6 bg-amber-50/70 border border-amber-200/80 rounded-3xl space-y-4 flex flex-col justify-between">
                  <div className="space-y-2">
                    <h4 className="text-xs font-black uppercase tracking-wider text-amber-900 flex items-center gap-2">
                      <Key className="w-4 h-4 text-amber-600" /> Rotasi Kunci & Kunci Privat
                    </h4>
                    <p className="text-xs text-amber-900/80 leading-relaxed font-medium">
                      Kunci privat Anda disimpan di brankas aman lokal. Jika Anda ingin mengganti kunci enkripsi untuk backup berikutnya, Anda dapat membuat pasangan kunci baru.
                    </p>
                  </div>

                  <div className="pt-2">
                    <button
                      type="button"
                      onClick={() => {
                        systemService.generateKeys().then(() => {
                          alert("Pasangan kunci baru berhasil digenerate!");
                          window.location.reload();
                        });
                      }}
                      className="w-full py-3 bg-slate-900 hover:bg-slate-800 text-white rounded-2xl font-black text-xs uppercase tracking-wider transition-all shadow-lg flex items-center justify-center gap-2 active:scale-95"
                    >
                      <Key className="w-4 h-4" /> Generate Pasangan Kunci Baru
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* 4. MAINTENANCE TAB */}
        {activeTab === 'maintenance' && (
          <div className="space-y-6 animate-in fade-in duration-200">
            <div className="bg-white p-6 md:p-8 rounded-[36px] border border-slate-100 shadow-sm space-y-6">
              <div>
                <h3 className="text-base font-black text-slate-900 tracking-tight flex items-center gap-2">
                  <HardDrive className="w-5 h-5 text-indigo-600" /> Pemeliharaan & Optimasi Ruang Disk
                </h3>
                <p className="text-xs text-slate-400 font-medium mt-0.5">
                  Jalankan pembersihan berkala untuk membebaskan ruang disk dan menjaga konsistensi repositori.
                </p>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                {/* GC Card */}
                <div className="p-6 bg-slate-50 border border-slate-200/80 rounded-3xl space-y-4 flex flex-col justify-between">
                  <div className="space-y-2">
                    <div className="w-10 h-10 rounded-2xl bg-indigo-50 text-indigo-600 flex items-center justify-center">
                      <RefreshCw className="w-5 h-5" />
                    </div>
                    <h4 className="text-sm font-black text-slate-800">Garbage Collection (GC)</h4>
                    <p className="text-xs text-slate-500 leading-relaxed font-medium">
                      Memindai seluruh penyimpanan dan menghapus chunk deduplikasi yang sudah tidak lagi dirujuk oleh snapshot aktif mana pun.
                    </p>
                  </div>

                  <button
                    type="button"
                    onClick={() => runMaintenance('gc')}
                    className="w-full py-3 bg-indigo-600 hover:bg-indigo-700 text-white rounded-2xl font-black text-xs uppercase tracking-wider transition-all shadow-md shadow-indigo-200 flex items-center justify-center gap-2 active:scale-95"
                  >
                    <RefreshCw className="w-4 h-4" /> Jalankan Garbage Collection
                  </button>
                </div>

                {/* Prune Failed Card */}
                <div className="p-6 bg-slate-50 border border-slate-200/80 rounded-3xl space-y-4 flex flex-col justify-between">
                  <div className="space-y-2">
                    <div className="w-10 h-10 rounded-2xl bg-rose-50 text-rose-600 flex items-center justify-center">
                      <Trash2 className="w-5 h-5" />
                    </div>
                    <h4 className="text-sm font-black text-slate-800">Prune Failed Snapshots</h4>
                    <p className="text-xs text-slate-500 leading-relaxed font-medium">
                      Menghapus rekaman pencadangan yang terhenti atau gagal di tengah jalan agar tidak memenuhi riwayat arsip vault.
                    </p>
                  </div>

                  <button
                    type="button"
                    onClick={() => runMaintenance('prune')}
                    className="w-full py-3 bg-rose-600 hover:bg-rose-700 text-white rounded-2xl font-black text-xs uppercase tracking-wider transition-all shadow-md shadow-rose-200 flex items-center justify-center gap-2 active:scale-95"
                  >
                    <Trash2 className="w-4 h-4" /> Bersihkan Record Gagal
                  </button>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* 5. ABOUT TAB */}
        {activeTab === 'about' && (
          <div className="space-y-6 animate-in fade-in duration-200">
            <div className="bg-white p-6 md:p-8 rounded-[36px] border border-slate-100 shadow-sm space-y-6">
              <div className="flex items-center gap-4">
                <div className="w-16 h-16 rounded-3xl bg-gradient-to-br from-indigo-600 to-indigo-800 text-white flex items-center justify-center shadow-xl shadow-indigo-200 font-black text-2xl">
                  PB
                </div>
                <div>
                  <h3 className="text-xl font-black text-slate-900 tracking-tight">
                    Phone Backup Platform
                  </h3>
                  <p className="text-xs text-slate-400 font-mono">Versi 0.3.2 (Production Release)</p>
                </div>
              </div>

              <div className="p-5 bg-slate-50 border border-slate-200/80 rounded-3xl text-xs text-slate-600 leading-relaxed space-y-2">
                <p>
                  Aplikasi pencadangan Android canggih dengan teknologi <b>Content-Defined Chunking (FastCDC)</b>, <b>Deduplikasi Global</b>, <b>Enkripsi Kriptografi Age</b>, dan dukungan protokol ganda <b>ADB + MTP</b>.
                </p>
              </div>

              <div className="grid grid-cols-2 sm:grid-cols-4 gap-3 text-center">
                <TechPill label="Rust Core" value="2021 Edition" />
                <TechPill label="UI Framework" value="Tauri v2 + React 19" />
                <TechPill label="Database" value="SQLite (SQLCipher)" />
                <TechPill label="Cloud Storage" value="Apache OpenDAL" />
              </div>
            </div>
          </div>
        )}

      </div>

      {/* Sticky Save Action Bar */}
      <div className="sticky bottom-4 z-20 flex justify-end bg-white/90 backdrop-blur-md p-4 rounded-3xl border border-slate-200/80 shadow-2xl">
        <button
          type="button"
          disabled={saving}
          onClick={handleSave}
          className="px-8 py-3.5 bg-indigo-600 hover:bg-indigo-700 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-lg shadow-indigo-200 hover:shadow-indigo-300 disabled:opacity-50 transition-all flex items-center gap-2 active:scale-95"
        >
          {saving ? <RefreshCw className="w-4 h-4 animate-spin" /> : <Save className="w-4 h-4" />}
          {saving ? "Menyimpan..." : "Simpan Perubahan Pengaturan"}
        </button>
      </div>

    </div>
  );
}

function TabButton({
  active, onClick, icon: Icon, label, badge, badgeColor
}: {
  active: boolean;
  onClick: () => void;
  icon: any;
  label: string;
  badge?: string;
  badgeColor?: string;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      className={cn(
        "flex items-center gap-2 px-4 py-2.5 rounded-xl font-black text-xs transition-all active:scale-95",
        active
          ? "bg-white text-slate-900 shadow-sm ring-1 ring-slate-200/60"
          : "text-slate-500 hover:text-slate-800 hover:bg-white/50"
      )}
    >
      <Icon className={cn("w-4 h-4", active ? "text-indigo-600" : "text-slate-400")} />
      <span>{label}</span>
      {badge && (
        <span className={cn("text-[9px] font-black px-2 py-0.5 rounded-md", badgeColor || "bg-slate-100 text-slate-600")}>
          {badge}
        </span>
      )}
    </button>
  );
}

function HealthCard({
  icon: Icon, title, value, desc, status
}: {
  icon: any;
  title: string;
  value: string;
  desc: string;
  status: 'healthy' | 'error';
}) {
  return (
    <div className="bg-white p-5 rounded-[28px] border border-slate-100 shadow-sm flex flex-col justify-between space-y-3 hover:shadow-md transition-shadow">
      <div className="flex items-center justify-between">
        <div className={cn(
          "w-10 h-10 rounded-2xl flex items-center justify-center",
          status === 'healthy' ? "bg-emerald-50 text-emerald-600" : "bg-red-50 text-red-600"
        )}>
          <Icon className="w-5 h-5" />
        </div>
        <div className={cn(
          "w-2.5 h-2.5 rounded-full",
          status === 'healthy' ? "bg-emerald-500 shadow-lg shadow-emerald-200" : "bg-red-500 animate-pulse"
        )} />
      </div>

      <div>
        <p className="text-[10px] font-black text-slate-400 uppercase tracking-widest">{title}</p>
        <p className="text-sm font-black text-slate-800 truncate mt-0.5">{value}</p>
        <p className="text-[10px] text-slate-400 font-medium truncate mt-0.5">{desc}</p>
      </div>
    </div>
  );
}

function StorageCard({
  selected, onClick, icon: Icon, title, desc, badge
}: {
  selected: boolean;
  onClick: () => void;
  icon: any;
  title: string;
  desc: string;
  badge: string;
}) {
  return (
    <div
      onClick={onClick}
      className={cn(
        "p-5 rounded-3xl border transition-all cursor-pointer flex flex-col justify-between space-y-3",
        selected
          ? "bg-indigo-50/50 border-indigo-300 ring-2 ring-indigo-500/10 shadow-md shadow-indigo-100/50"
          : "bg-slate-50 border-slate-200/70 hover:border-slate-300"
      )}
    >
      <div className="flex items-center justify-between">
        <div className={cn(
          "w-10 h-10 rounded-2xl flex items-center justify-center",
          selected ? "bg-indigo-600 text-white shadow-md shadow-indigo-200" : "bg-white text-slate-600 border border-slate-200"
        )}>
          <Icon className="w-5 h-5" />
        </div>
        <span className={cn(
          "text-[9px] font-black px-2.5 py-0.5 rounded-full uppercase tracking-wider",
          selected ? "bg-indigo-600 text-white" : "bg-slate-200 text-slate-600"
        )}>
          {badge}
        </span>
      </div>

      <div>
        <h4 className="text-xs font-black text-slate-900 uppercase tracking-wider">{title}</h4>
        <p className="text-[11px] text-slate-500 font-medium mt-1 leading-relaxed">{desc}</p>
      </div>
    </div>
  );
}

function TechPill({ label, value }: { label: string; value: string }) {
  return (
    <div className="p-3 bg-slate-50 rounded-2xl border border-slate-200/70">
      <p className="text-[9px] font-black text-slate-400 uppercase tracking-widest">{label}</p>
      <p className="text-xs font-black text-slate-800 mt-0.5">{value}</p>
    </div>
  );
}
