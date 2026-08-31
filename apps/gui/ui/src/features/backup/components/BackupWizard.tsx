import { useState, useEffect } from 'react';
import {
  Smartphone, CheckCircle2, ShieldCheck, ArrowRight, Loader2, Database, XCircle,
  Search, MessageSquare, Users, Image as ImageIcon, PhoneCall, HardDrive,
  Usb, Wifi, Lock, Sparkles, FolderCheck, Check, ArrowLeft, RefreshCw, FileText,
  Zap, FolderSearch, Activity
} from 'lucide-react';
import { useDevices } from '@/features/devices/hooks/useDevices';
import { getDeviceId, type Device } from '@/services/deviceService';
import { cn } from "@/shared/lib/utils";
import { formatBytes } from '@/shared/lib/formatters';
import { FileTree } from '@/shared/components/FileTree';
import { useBackupWizard } from '../hooks/useBackupWizard';

interface DataOption {
  id: string;
  label: string;
  icon: any;
  description: string;
  detail: string;
  requiresAdb?: boolean;
}

const dataOptions: DataOption[] = [
  {
    id: 'full_storage',
    label: 'Seluruh Memori Internal',
    icon: HardDrive,
    description: 'Semua folder & file di memori ponsel (Termasuk WhatsApp, Musik, Rekaman, & Folder Kustom).',
    detail: 'Rekomendasi Total',
    requiresAdb: false
  },
  {
    id: 'photos',
    label: 'Galeri & Media',
    icon: ImageIcon,
    description: 'Foto kamera (DCIM), Gambar (Pictures), dan Video rekaman.',
    detail: 'Volume Tinggi',
    requiresAdb: false
  },
  {
    id: 'chat_media',
    label: 'Media WhatsApp & Chat',
    icon: MessageSquare,
    description: 'Foto, video, voice note, dan dokumen dari percakapan WhatsApp & Telegram.',
    detail: 'Media Sosial',
    requiresAdb: false
  },
  {
    id: 'files',
    label: 'Dokumen & Unduhan',
    icon: FolderCheck,
    description: 'Folder Download, Dokumen, PDF, Arsip Zip, dan file umum.',
    detail: 'File Explorer',
    requiresAdb: false
  },
  {
    id: 'audio',
    label: 'Musik & Rekaman Suara',
    icon: Sparkles,
    description: 'Folder Music, Recordings, VoiceRecorder, Ringtones, dan Podcast.',
    detail: 'Audio & Suara',
    requiresAdb: false
  },
  {
    id: 'contacts',
    label: 'Kontak & Telepon',
    icon: Users,
    description: 'Nama, nomor telepon, email, dan vCard kontak tersimpan.',
    detail: 'E2E Encrypted',
    requiresAdb: true
  },
  {
    id: 'sms',
    label: 'Pesan SMS',
    icon: MessageSquare,
    description: 'Riwayat percakapan SMS masuk & keluar, dan pesan teks.',
    detail: 'Secure Vault',
    requiresAdb: true
  },
  {
    id: 'call_logs',
    label: 'Riwayat Panggilan',
    icon: PhoneCall,
    description: 'Catatan panggilan masuk, keluar, dan panggilan tak terjawab.',
    detail: 'Log Aktivitas',
    requiresAdb: true
  },
  {
    id: 'apps',
    label: 'Daftar Aplikasi',
    icon: Smartphone,
    description: 'Daftar paket aplikasi Android terinstal dan versi APK.',
    detail: 'Metadata Inventory',
    requiresAdb: true
  },
];

interface BackupWizardProps {
  initialDevice?: Device | null;
  onFinish?: () => void;
}

export function BackupWizard({ initialDevice, onFinish }: BackupWizardProps) {
  const { devices, loading: devicesLoading } = useDevices();
  const {
    step, setStep,
    selectedDevice, setSelectedDevice,
    selectedData, setSelectedData, toggleData,
    scannedFiles, isCalculating, reviewSearch, setReviewSearch,
    selectedPaths, handleTogglePath, handleNextToConfigure, handleStartBackup, handleExpressBackup,
    analysisState,
    progressMsg, progressPercent, totalItems, currentItems, error,
    selectedFiles, totalBytes
  } = useBackupWizard();

  const [encryptionEnabled] = useState(true);

  // Auto-select initial device if passed from Dashboard
  useEffect(() => {
    if (initialDevice && !selectedDevice) {
      setSelectedDevice(initialDevice);
    }
  }, [initialDevice, selectedDevice, setSelectedDevice]);

  const isMtpDevice = selectedDevice?.connection_type === 'Mtp';

  return (
    <div className="max-w-5xl mx-auto p-4 md:p-8 animate-in fade-in slide-in-from-bottom-2 duration-300 h-full flex flex-col">
      
      {/* Wizard Header & Stepper */}
      <div className="mb-6 text-center shrink-0">
        <div className="flex items-center justify-center gap-2 mb-2">
          <div className="w-8 h-8 rounded-xl bg-indigo-600 text-white flex items-center justify-center shadow-md shadow-indigo-200">
            <Database className="w-4 h-4" />
          </div>
          <h1 className="text-2xl md:text-3xl font-black text-slate-900 tracking-tight">
            Backup Wizard
          </h1>
        </div>
        <p className="text-xs text-slate-400 font-medium">
          Panduan langkah demi langkah untuk mencadangkan data ponsel Anda secara aman dan terenkripsi.
        </p>

        {/* Interactive Steps Breadcrumb */}
        <div className="flex items-center justify-center gap-2 sm:gap-4 mt-6 max-w-2xl mx-auto">
          <StepBadge
            step={1}
            title="Perangkat"
            active={step === 'select-device'}
            completed={!!selectedDevice && step !== 'select-device'}
            onClick={() => setStep('select-device')}
          />
          <div className="flex-1 max-w-[48px] h-0.5 bg-slate-200" />
          <StepBadge
            step={2}
            title="Modul Data"
            active={step === 'select-data'}
            completed={step === 'configure' || step === 'progress'}
            onClick={() => selectedDevice && setStep('select-data')}
          />
          <div className="flex-1 max-w-[48px] h-0.5 bg-slate-200" />
          <StepBadge
            step={3}
            title="Pratinjau"
            active={step === 'configure'}
            completed={step === 'progress'}
            onClick={() => selectedDevice && selectedData.length > 0 && setStep('configure')}
          />
          <div className="flex-1 max-w-[48px] h-0.5 bg-slate-200" />
          <StepBadge
            step={4}
            title="Proses"
            active={step === 'progress'}
            completed={progressPercent === 100}
          />
        </div>
      </div>

      {/* Wizard Card Container */}
      <div className="bg-white rounded-[32px] border border-slate-100 shadow-2xl shadow-indigo-100/40 flex-1 flex flex-col overflow-hidden min-h-0">
        
        {/* ================= STEP 1: SELECT DEVICE ================= */}
        {step === 'select-device' && (
          <div className="p-6 md:p-10 space-y-6 animate-in fade-in duration-200 flex-1 flex flex-col justify-between">
            <div className="space-y-6">
              <div className="flex justify-between items-end">
                <div>
                  <h2 className="text-xl md:text-2xl font-black text-slate-900 tracking-tight">
                    Pilih Perangkat Sumber
                  </h2>
                  <p className="text-xs text-slate-400 font-medium mt-0.5">
                    Pilih ponsel atau tablet Android yang ingin dicadangkan datanya.
                  </p>
                </div>
              </div>

              {devicesLoading ? (
                <div className="flex flex-col items-center justify-center py-20 gap-3">
                  <Loader2 className="w-10 h-10 text-indigo-600 animate-spin" />
                  <p className="text-slate-400 font-bold uppercase tracking-widest text-xs">Memindai Sambungan Perangkat...</p>
                </div>
              ) : devices.length === 0 ? (
                <div className="py-16 text-center border-2 border-dashed border-slate-200 rounded-3xl p-8 space-y-3">
                  <Smartphone className="w-12 h-12 text-slate-300 mx-auto" />
                  <h3 className="text-sm font-black text-slate-700">Tidak Ada Perangkat Terhubung</h3>
                  <p className="text-xs text-slate-400 max-w-md mx-auto">
                    Colokkan ponsel Anda menggunakan kabel USB (pilih mode Transfer File) atau aktifkan USB Debugging.
                  </p>
                </div>
              ) : (
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  {devices.map(device => {
                    const devId = getDeviceId(device);
                    const isSelected = selectedDevice && getDeviceId(selectedDevice) === devId;
                    const isMtp = device.connection_type === 'Mtp';

                    return (
                      <div
                        key={devId}
                        onClick={() => setSelectedDevice(device)}
                        className={cn(
                          "p-6 rounded-[28px] border-2 transition-all cursor-pointer flex items-start gap-4 relative overflow-hidden group select-none",
                          isSelected
                            ? "border-indigo-500 bg-indigo-50/40 shadow-xl shadow-indigo-100/50 ring-2 ring-indigo-500/10"
                            : "border-slate-100 hover:border-indigo-200 hover:shadow-md bg-white"
                        )}
                      >
                        <div className={cn(
                          "w-12 h-12 rounded-2xl flex items-center justify-center shrink-0 transition-transform group-hover:scale-105",
                          isSelected ? "bg-indigo-600 text-white shadow-lg shadow-indigo-200" : "bg-slate-100 text-slate-500"
                        )}>
                          <Smartphone className="w-6 h-6" />
                        </div>

                        <div className="flex-1 min-w-0">
                          <div className="flex items-center justify-between gap-2">
                            <h3 className="font-black text-slate-900 text-base truncate" title={device.model}>
                              {device.model}
                            </h3>
                            {isSelected && (
                              <CheckCircle2 className="w-5 h-5 text-indigo-600 shrink-0" />
                            )}
                          </div>

                          <div className="flex items-center gap-2 mt-1">
                            <span className="text-[10px] font-bold text-slate-400 uppercase tracking-wider truncate">
                              {device.manufacturer}
                            </span>
                            <div className="w-1 h-1 bg-slate-200 rounded-full shrink-0" />
                            <span className="text-[10px] font-black text-indigo-500 shrink-0">
                              Android {device.os_version}
                            </span>
                          </div>

                          <div className="mt-3 flex items-center gap-2">
                            <span className={cn(
                              "text-[9px] font-black px-2.5 py-0.5 rounded-md flex items-center gap-1",
                              isMtp ? "bg-cyan-50 text-cyan-700 border border-cyan-200" :
                              device.connection_type === 'Wifi' ? "bg-purple-50 text-purple-700 border border-purple-200" :
                              "bg-emerald-50 text-emerald-700 border border-emerald-200"
                            )}>
                              {isMtp ? <HardDrive className="w-3 h-3 text-cyan-600" /> : device.connection_type === 'Wifi' ? <Wifi className="w-3 h-3 text-purple-600" /> : <Usb className="w-3 h-3 text-emerald-600" />}
                              {isMtp ? "MTP (Kabel Biasa)" : device.connection_type === 'Wifi' ? "Wireless ADB" : "USB ADB"}
                            </span>

                            {device.storage_total_bytes > 0 && (
                              <span className="text-[10px] font-bold text-slate-400">
                                {formatBytes(Number(device.storage_used_bytes))} / {formatBytes(Number(device.storage_total_bytes))}
                              </span>
                            )}
                          </div>
                        </div>
                      </div>
                    );
                  })}
                </div>
              )}
            </div>

            {/* Step 1 Footer */}
            <div className="pt-6 border-t border-slate-100 flex justify-end">
              <button
                disabled={!selectedDevice}
                onClick={() => setStep('select-data')}
                className="px-8 py-3.5 bg-indigo-600 hover:bg-indigo-700 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-lg shadow-indigo-200 hover:shadow-indigo-300 disabled:opacity-50 transition-all flex items-center gap-2.5 active:scale-95"
              >
                <span>Lanjutkan</span>
                <ArrowRight className="w-4 h-4" />
              </button>
            </div>
          </div>
        )}

        {/* ================= STEP 2: SELECT DATA ================= */}
        {step === 'select-data' && (
          <div className="p-6 md:p-10 space-y-6 animate-in fade-in duration-200 flex-1 flex flex-col justify-between">
            <div className="space-y-6">
              <div className="flex flex-col sm:flex-row sm:items-end justify-between gap-3">
                <div>
                  <h2 className="text-xl md:text-2xl font-black text-slate-900 tracking-tight">
                    Apa yang ingin Anda cadangkan?
                  </h2>
                  <p className="text-xs text-slate-400 font-medium mt-0.5">
                    Pilih kategori data yang ingin dimasukkan ke dalam paket backup ini.
                  </p>
                </div>

                <div className="flex gap-2 shrink-0">
                  <button
                    type="button"
                    onClick={() => setSelectedData(dataOptions.filter(o => !isMtpDevice || !o.requiresAdb).map(o => o.id))}
                    className="px-3 py-1.5 bg-indigo-50 text-indigo-700 hover:bg-indigo-100 rounded-xl text-[10px] font-black uppercase tracking-wider transition-colors"
                  >
                    Pilih Semua
                  </button>
                  <button
                    type="button"
                    onClick={() => setSelectedData(['photos', 'files'])}
                    className="px-3 py-1.5 bg-slate-50 text-slate-600 hover:bg-slate-100 rounded-xl text-[10px] font-black uppercase tracking-wider transition-colors"
                  >
                    Hanya Media & Foto
                  </button>
                </div>
              </div>

              {isMtpDevice && (
                <div className="p-4 bg-cyan-50 border border-cyan-200 rounded-2xl text-xs text-cyan-900 leading-relaxed font-medium flex items-center gap-3">
                  <Sparkles className="w-5 h-5 text-cyan-600 shrink-0" />
                  <span>
                    <b>Mode MTP Aktif:</b> Anda dapat mencadangkan seluruh Galeri Foto, Video, dan Dokumen tanpa Developer Mode. Kontak & SMS memerlukan koneksi ADB.
                  </span>
                </div>
              )}

              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {dataOptions.map(opt => {
                  const isSelected = selectedData.includes(opt.id);
                  const isDisabled = isMtpDevice && opt.requiresAdb;

                  return (
                    <div
                      key={opt.id}
                      onClick={() => !isDisabled && toggleData(opt.id)}
                      className={cn(
                        "p-5 rounded-[28px] border-2 transition-all flex flex-col justify-between space-y-3 relative overflow-hidden select-none",
                        isDisabled
                          ? "opacity-40 bg-slate-50 border-slate-200/50 cursor-not-allowed"
                          : isSelected
                          ? "border-indigo-500 bg-indigo-50/40 shadow-md shadow-indigo-100 cursor-pointer"
                          : "border-slate-100 hover:border-indigo-200 bg-white cursor-pointer"
                      )}
                    >
                      <div className="flex items-center justify-between">
                        <div className={cn(
                          "w-11 h-11 rounded-2xl flex items-center justify-center shadow-inner",
                          isSelected ? "bg-indigo-600 text-white" : "bg-slate-50 text-slate-400"
                        )}>
                          <opt.icon className="w-5 h-5" />
                        </div>
                        <span className={cn(
                          "text-[9px] font-black px-2 py-0.5 rounded uppercase tracking-wider",
                          isSelected ? "bg-indigo-100 text-indigo-700" : "bg-slate-100 text-slate-400"
                        )}>
                          {opt.detail}
                        </span>
                      </div>

                      <div>
                        <div className="flex items-center justify-between">
                          <h4 className="font-black text-slate-900 text-sm">{opt.label}</h4>
                          {isSelected && <Check className="w-4 h-4 text-indigo-600 stroke-[3]" />}
                        </div>
                        <p className="text-[11px] text-slate-500 font-medium mt-1 leading-relaxed">{opt.description}</p>
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>

            {/* Step 2 Footer with Express Backup Option */}
            <div className="pt-6 border-t border-slate-100 flex flex-col sm:flex-row justify-between items-center gap-3">
              <button
                type="button"
                onClick={() => setStep('select-device')}
                className="px-6 py-3 font-black text-slate-400 hover:text-slate-700 transition-all uppercase text-[10px] tracking-wider flex items-center gap-2"
              >
                <ArrowLeft className="w-4 h-4" /> Kembali
              </button>

              <div className="flex items-center gap-3 w-full sm:w-auto justify-end">
                <button
                  type="button"
                  disabled={selectedData.length === 0}
                  onClick={handleExpressBackup}
                  title="Langsung mulai proses backup tanpa menunggu analisis pohon file selesai (sangat cepat untuk HP 128GB-512GB)"
                  className="px-6 py-3.5 bg-slate-900 hover:bg-slate-800 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-lg transition-all flex items-center gap-2 active:scale-95 disabled:opacity-50"
                >
                  <Zap className="w-4 h-4 text-amber-400" />
                  <span>Mulai Backup Instan</span>
                </button>

                <button
                  type="button"
                  disabled={selectedData.length === 0}
                  onClick={handleNextToConfigure}
                  className="px-8 py-3.5 bg-indigo-600 hover:bg-indigo-700 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-lg shadow-indigo-200 hover:shadow-indigo-300 disabled:opacity-50 transition-all flex items-center gap-2.5 active:scale-95"
                >
                  <span>Review Rencana</span>
                  <ArrowRight className="w-4 h-4" />
                </button>
              </div>
            </div>
          </div>
        )}

        {/* ================= STEP 3: CONFIGURE & REVIEW ================= */}
        {step === 'configure' && (
          <div className="flex-1 flex flex-col min-h-0 animate-in fade-in duration-200">
            {/* Review Header Stats */}
            <div className="p-6 md:p-8 border-b border-slate-100 shrink-0 bg-white space-y-5">
              <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                <div>
                  <h2 className="text-xl md:text-2xl font-black text-slate-900 tracking-tight">
                    Eksplorasi Rencana Backup
                  </h2>
                  <p className="text-xs text-slate-400 font-medium mt-0.5">
                    Tinjau file yang terdeteksi. Anda dapat mengecualikan folder atau file tertentu.
                  </p>
                </div>

                <div className="flex items-center gap-3 bg-slate-50 p-2.5 rounded-2xl border border-slate-200/70">
                  <div className="px-3 text-right">
                    <p className="text-[9px] font-black text-slate-400 uppercase tracking-widest">Total Ukuran</p>
                    <p className="text-base font-black text-indigo-600">{formatBytes(totalBytes)}</p>
                  </div>
                  <div className="w-px h-8 bg-slate-200" />
                  <div className="px-3 text-right">
                    <p className="text-[9px] font-black text-slate-400 uppercase tracking-widest">Total File</p>
                    <p className="text-base font-black text-slate-900">{selectedFiles.length}</p>
                  </div>
                </div>
              </div>

              <div className="relative">
                <Search className="absolute left-4 top-3.5 w-4 h-4 text-slate-400" />
                <input
                  type="text"
                  placeholder="Cari nama file dalam rencana backup..."
                  value={reviewSearch}
                  onChange={(e) => setReviewSearch(e.target.value)}
                  className="w-full bg-slate-50 border border-slate-200/80 pl-11 pr-4 py-3 rounded-2xl text-xs font-medium outline-none focus:ring-4 focus:ring-indigo-500/10 focus:border-indigo-300 transition-all"
                />
              </div>
            </div>

            {/* Tree View Area or Live Analysis HUD */}
            <div className="flex-1 overflow-y-auto bg-slate-50/50 custom-scrollbar p-6">
              {isCalculating ? (
                /* LIVE ANALYSIS HUD (Transparent Telemetry Display for Big Phones) */
                <div className="max-w-xl mx-auto py-10 space-y-6 animate-in zoom-in-95 duration-200">
                  <div className="bg-white p-6 rounded-[32px] border border-slate-100 shadow-xl space-y-6 text-center">
                    
                    <div className="w-14 h-14 rounded-3xl bg-indigo-50 text-indigo-600 mx-auto flex items-center justify-center shadow-inner">
                      <FolderSearch className="w-7 h-7 animate-pulse" />
                    </div>

                    <div>
                      <h3 className="text-lg font-black text-slate-900">
                        Menganalisis Sistem Berkas Ponsel
                      </h3>
                      <p className="text-xs text-slate-400 font-medium mt-1">
                        Memindai data secara cerdas menggunakan indeks Android MediaStore + Path Crawler.
                      </p>
                    </div>

                    {/* Stage Pipeline Indicator */}
                    <div className="grid grid-cols-3 gap-2 text-left text-[10px] font-black uppercase tracking-wider">
                      <div className={cn(
                        "p-2.5 rounded-xl border flex items-center gap-1.5",
                        analysisState.stage === 'mediastore'
                          ? "bg-indigo-50 border-indigo-300 text-indigo-700 animate-pulse"
                          : "bg-emerald-50 border-emerald-200 text-emerald-700"
                      )}>
                        <Activity className="w-3 h-3 shrink-0" />
                        <span className="truncate">1. MediaStore</span>
                      </div>

                      <div className={cn(
                        "p-2.5 rounded-xl border flex items-center gap-1.5",
                        analysisState.stage === 'crawler'
                          ? "bg-indigo-50 border-indigo-300 text-indigo-700 animate-pulse"
                          : analysisState.stage === 'indexing'
                          ? "bg-emerald-50 border-emerald-200 text-emerald-700"
                          : "bg-slate-50 border-slate-200 text-slate-400"
                      )}>
                        <FolderSearch className="w-3 h-3 shrink-0" />
                        <span className="truncate">2. Crawler</span>
                      </div>

                      <div className={cn(
                        "p-2.5 rounded-xl border flex items-center gap-1.5",
                        analysisState.stage === 'indexing'
                          ? "bg-indigo-50 border-indigo-300 text-indigo-700 animate-pulse"
                          : "bg-slate-50 border-slate-200 text-slate-400"
                      )}>
                        <Lock className="w-3 h-3 shrink-0" />
                        <span className="truncate">3. FastCDC</span>
                      </div>
                    </div>

                    {/* Live Counter Box */}
                    <div className="bg-slate-900 text-white p-4 rounded-2xl space-y-2 text-left font-mono">
                      <div className="flex justify-between items-center text-xs">
                        <span className="text-slate-400">Berkas Terhitung:</span>
                        <span className="text-emerald-400 font-bold text-sm">
                          {analysisState.filesCount.toLocaleString()} Berkas
                        </span>
                      </div>
                      <div className="flex justify-between items-center text-xs">
                        <span className="text-slate-400">Total Volume:</span>
                        <span className="text-cyan-400 font-bold">
                          {formatBytes(analysisState.totalBytes)}
                        </span>
                      </div>
                      <div className="border-t border-slate-800 pt-2 text-[10px] text-slate-400 truncate">
                        &gt; {analysisState.currentFolder}
                      </div>
                    </div>

                    {/* Express Skip Button for large storage */}
                    <div className="pt-2">
                      <button
                        type="button"
                        onClick={handleExpressBackup}
                        className="w-full py-3 bg-slate-900 hover:bg-slate-800 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-lg flex items-center justify-center gap-2 active:scale-95"
                      >
                        <Zap className="w-4 h-4 text-amber-400" />
                        <span>Lewati Pratinjau & Langsung Mulai Backup</span>
                      </button>
                    </div>

                  </div>
                </div>
              ) : scannedFiles.length === 0 ? (
                <div className="h-full flex flex-col items-center justify-center py-20 text-slate-400 space-y-2">
                  <FolderCheck className="w-12 h-12 text-slate-300" />
                  <p className="text-xs font-black uppercase tracking-widest">Tidak ada file media yang perlu dipilih manual.</p>
                  <p className="text-[11px] text-slate-400">Data modul (Kontak, SMS, Apps) akan dicadangkan secara otomatis.</p>
                </div>
              ) : (
                <div className="max-w-3xl mx-auto pb-10">
                  <FileTree
                    files={scannedFiles}
                    searchQuery={reviewSearch}
                    selectedPaths={selectedPaths}
                    onToggle={handleTogglePath}
                  />
                </div>
              )}
            </div>

            {/* Step 3 Footer */}
            <div className="p-6 md:p-8 border-t border-slate-100 bg-white shrink-0 flex justify-between items-center">
              <button
                type="button"
                onClick={() => setStep('select-data')}
                className="px-6 py-3 font-black text-slate-400 hover:text-slate-700 transition-all uppercase text-[10px] tracking-wider flex items-center gap-2"
              >
                <ArrowLeft className="w-4 h-4" /> Kembali
              </button>

              <div className="flex items-center gap-4">
                {encryptionEnabled && (
                  <div className="hidden sm:flex items-center gap-2 text-emerald-700 bg-emerald-50 px-4 py-2 rounded-xl border border-emerald-200">
                    <ShieldCheck className="w-4 h-4 text-emerald-600" />
                    <span className="text-[10px] font-black uppercase tracking-wider">Age X25519 Ready</span>
                  </div>
                )}
                <button
                  type="button"
                  disabled={isCalculating}
                  onClick={() => handleStartBackup(dataOptions.length)}
                  className="px-8 py-3.5 bg-slate-900 hover:bg-slate-800 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-xl shadow-slate-200 transition-all flex items-center gap-2.5 active:scale-95 disabled:opacity-50"
                >
                  <Lock className="w-4 h-4 text-indigo-400" />
                  <span>Konfirmasi & Mulai Backup</span>
                  <ArrowRight className="w-4 h-4" />
                </button>
              </div>
            </div>
          </div>
        )}

        {/* ================= STEP 4: LIVE BACKUP STREAM ================= */}
        {step === 'progress' && (
          <div className="p-8 md:p-12 flex-1 flex flex-col items-center justify-center text-center space-y-8 animate-in zoom-in-95 duration-300">
            {error ? (
              <div className="space-y-6 max-w-md">
                <div className="w-20 h-20 bg-rose-50 text-rose-600 rounded-3xl mx-auto flex items-center justify-center shadow-lg shadow-rose-100">
                  <XCircle className="w-10 h-10" />
                </div>
                <div>
                  <h2 className="text-2xl font-black text-slate-900 mb-1">Proses Backup Terhenti</h2>
                  <p className="text-xs text-rose-600 font-medium leading-relaxed bg-rose-50/70 p-4 rounded-2xl border border-rose-100">
                    {error}
                  </p>
                </div>
                <button
                  type="button"
                  onClick={() => setStep('configure')}
                  className="px-8 py-3.5 bg-slate-900 hover:bg-slate-800 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-xl transition-all"
                >
                  Coba Ulangi Backup
                </button>
              </div>
            ) : (
              <div className="space-y-8 max-w-lg w-full">
                {/* Progress Animation Gauge */}
                <div className="relative w-40 h-40 mx-auto">
                  <div className={cn(
                    "w-40 h-40 rounded-full border-8 border-slate-100 transition-all duration-500",
                    progressPercent < 100 ? "border-t-indigo-600 animate-spin" : "border-emerald-500"
                  )} />
                  <div className="absolute inset-0 flex flex-col items-center justify-center">
                    {progressPercent < 100 ? (
                      <>
                        <span className="text-3xl font-black text-slate-900 tracking-tighter">{progressPercent}%</span>
                        <span className="text-[10px] font-bold text-slate-400 uppercase tracking-widest mt-0.5">Progress</span>
                      </>
                    ) : (
                      <CheckCircle2 className="w-14 h-14 text-emerald-500 animate-in zoom-in-50" />
                    )}
                  </div>
                </div>

                {/* Progress Text */}
                <div>
                  <h2 className="text-2xl md:text-3xl font-black text-slate-900 mb-1">
                    {progressPercent < 100 ? "Sedang Mencadangkan Data..." : "Pencadangan Selesai!"}
                  </h2>
                  <p className="text-xs text-slate-500 font-medium">
                    {progressPercent < 100
                      ? "Mohon jangan mencabut kabel atau memutus koneksi perangkat Anda."
                      : "Seluruh data yang dipilih kini tersimpan aman, terdeduplikasi, dan disegel dengan enkripsi Age."}
                  </p>
                </div>

                {/* Progress Meter Bar */}
                {progressPercent < 100 && (
                  <div className="space-y-3 bg-slate-50 p-5 rounded-3xl border border-slate-200/70 text-left">
                    <div className="flex justify-between items-center text-xs font-black">
                      <span className="text-slate-700 truncate max-w-[260px] flex items-center gap-1.5">
                        <FileText className="w-3.5 h-3.5 text-indigo-600 shrink-0" />
                        <span className="truncate">{progressMsg || "Memproses..."}</span>
                      </span>
                      <span className="text-indigo-600 font-mono shrink-0">
                        {totalItems > 0 ? `${currentItems} / ${totalItems}` : `${progressPercent}%`}
                      </span>
                    </div>

                    <div className="h-3 w-full bg-slate-200 rounded-full overflow-hidden p-0.5">
                      <div
                        className="h-full bg-indigo-600 rounded-full transition-all duration-300 ease-out"
                        style={{ width: `${Math.max(progressPercent, 2)}%` }}
                      />
                    </div>

                    {progressPercent > 92 && (
                      <p className="text-[10px] text-amber-600 font-bold animate-pulse flex items-center gap-1.5">
                        <RefreshCw className="w-3 h-3 animate-spin" /> Sedang merampungkan manifest snapshot & enkripsi...
                      </p>
                    )}
                  </div>
                )}

                {/* Completion Action Buttons */}
                {progressPercent === 100 && (
                  <div className="pt-2 flex flex-col sm:flex-row gap-3 justify-center">
                    <button
                      type="button"
                      onClick={() => onFinish ? onFinish() : window.location.reload()}
                      className="px-8 py-3.5 bg-indigo-600 hover:bg-indigo-700 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-xl shadow-indigo-200 hover:shadow-indigo-300 transition-all active:scale-95"
                    >
                      Kembali ke Dashboard
                    </button>
                  </div>
                )}
              </div>
            )}
          </div>
        )}

      </div>
    </div>
  );
}

function StepBadge({
  step, title, active, completed, onClick
}: {
  step: number;
  title: string;
  active: boolean;
  completed: boolean;
  onClick?: () => void;
}) {
  return (
    <div
      onClick={completed || active ? onClick : undefined}
      className={cn(
        "flex items-center gap-2 transition-all select-none",
        (completed || active) && onClick ? "cursor-pointer" : "cursor-default"
      )}
    >
      <div className={cn(
        "w-8 h-8 rounded-full flex items-center justify-center font-black text-xs transition-all",
        active
          ? "bg-indigo-600 text-white shadow-lg shadow-indigo-200 ring-4 ring-indigo-500/10 scale-105"
          : completed
          ? "bg-emerald-500 text-white shadow-sm"
          : "bg-slate-100 text-slate-400"
      )}>
        {completed ? <Check className="w-4 h-4 stroke-[3]" /> : step}
      </div>
      <span className={cn(
        "text-[11px] font-black uppercase tracking-wider hidden sm:inline",
        active ? "text-indigo-600" : completed ? "text-slate-800" : "text-slate-400"
      )}>
        {title}
      </span>
    </div>
  );
}
