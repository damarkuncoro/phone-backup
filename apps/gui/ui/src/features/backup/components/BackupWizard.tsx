import {
    Tablet, CheckCircle2, ShieldCheck, ArrowRight, Loader2, Database, XCircle,
    Search
} from 'lucide-react';
import { useDevices } from '@/features/devices/hooks/useDevices';
import { getDeviceId } from '@/services/deviceService';
import { cn } from "../../../shared/lib/utils";
import { formatBytes } from '@/shared/lib/formatters';
import { FileTree } from '@/shared/components/FileTree';
import { useBackupWizard } from '../hooks/useBackupWizard';
import { MessageSquare, Users, Smartphone, Image as ImageIcon } from 'lucide-react';

interface DataOption {
  id: string;
  label: string;
  icon: any;
  description: string;
  detail?: string;
}

const dataOptions: DataOption[] = [
  { id: 'contacts', label: 'Kontak', icon: Users, description: 'Nama, nomor telepon, dan email.', detail: 'E2E Encrypted' },
  { id: 'sms', label: 'Pesan', icon: MessageSquare, description: 'Riwayat SMS, MMS, dan lampiran.', detail: 'Secure Snapshot' },
  { id: 'photos', label: 'Galeri & Media', icon: ImageIcon, description: 'Foto (DCIM) dan video kamera.', detail: 'High Volume' },
  { id: 'apps', label: 'Daftar Aplikasi', icon: Smartphone, description: 'Daftar aplikasi terinstal (.apk tidak ikut).', detail: 'Metadata Only' },
  { id: 'files', label: 'Dokumen', icon: Database, description: 'Folder Download dan dokumen lokal.', detail: 'File Explorer' },
];

export function BackupWizard() {
  const { devices, loading: devicesLoading } = useDevices();
  const {
    step, setStep,
    selectedDevice, setSelectedDevice,
    selectedData, setSelectedData, toggleData,
    scannedFiles, isCalculating, reviewSearch, setReviewSearch,
    selectedPaths, handleTogglePath, handleNextToConfigure, handleStartBackup,
    progressMsg, progressPercent, error,
    selectedFiles, totalBytes
  } = useBackupWizard();

  return (
    <div className="max-w-4xl mx-auto p-8 animate-in fade-in slide-in-from-bottom-4 duration-500 h-full flex flex-col">
      <div className="mb-8 text-center shrink-0">
        <h1 className="text-3xl font-black text-slate-900 tracking-tight mb-2">Backup Wizard</h1>
        <div className="flex items-center justify-center gap-4 mt-6">
            <StepIndicator step={1} active={step === 'select-device'} completed={!!selectedDevice || step !== 'select-device'} label="Device" />
            <div className="w-12 h-0.5 bg-slate-200" />
            <StepIndicator step={2} active={step === 'select-data'} completed={step === 'configure' || step === 'progress'} label="Data" />
            <div className="w-12 h-0.5 bg-slate-200" />
            <StepIndicator step={3} active={step === 'configure'} completed={step === 'progress'} label="Review" />
        </div>
      </div>

      <div className="bg-white rounded-[40px] border border-slate-100 shadow-2xl shadow-indigo-100/50 flex-1 flex flex-col overflow-hidden min-h-0">
        {step === 'select-device' && (
          <div className="p-10 space-y-8 animate-in fade-in duration-300">
            <h2 className="text-2xl font-black text-slate-900">Pilih Perangkat Sumber</h2>
            {devicesLoading ? (
              <div className="flex flex-col items-center justify-center py-20 gap-4">
                <Loader2 className="w-10 h-10 text-indigo-500 animate-spin" />
                <p className="text-slate-400 font-bold uppercase tracking-widest text-xs">Scanning ADB...</p>
              </div>
            ) : (
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {devices.map(device => {
                    const isSelected = selectedDevice && getDeviceId(device) === getDeviceId(selectedDevice);
                    return (
                        <div
                          key={getDeviceId(device)}
                          onClick={() => setSelectedDevice(device)}
                          className={cn(
                            "p-6 rounded-3xl border-2 transition-all cursor-pointer flex items-center gap-5",
                            isSelected ? "border-indigo-500 bg-indigo-50/30 shadow-lg shadow-indigo-100" : "border-slate-100 hover:border-indigo-200"
                          )}
                        >
                            <div className={cn("w-12 h-12 rounded-2xl flex items-center justify-center shadow-inner", isSelected ? "bg-indigo-600 text-white" : "bg-slate-100 text-slate-400")}>
                                <Tablet className="w-6 h-6" />
                            </div>
                            <div className="flex-1">
                                <p className="font-black text-slate-900">{device.model}</p>
                                <p className="text-[10px] font-bold text-slate-400 uppercase tracking-widest">{device.manufacturer} • Android {device.os_version}</p>
                            </div>
                            {isSelected && <CheckCircle2 className="w-6 h-6 text-indigo-600" />}
                        </div>
                    );
                })}
              </div>
            )}
            <div className="pt-8 mt-auto flex justify-end">
                <button
                  disabled={!selectedDevice}
                  onClick={() => setStep('select-data')}
                  className="px-8 py-4 bg-indigo-600 text-white rounded-2xl font-black shadow-xl shadow-indigo-200 hover:bg-indigo-700 disabled:opacity-50 transition-all flex items-center gap-3"
                >
                    Lanjutkan <ArrowRight className="w-5 h-5" />
                </button>
            </div>
          </div>
        )}

        {step === 'select-data' && (
          <div className="p-10 space-y-8 animate-in fade-in duration-300">
             <div className="flex justify-between items-end">
                <h2 className="text-2xl font-black text-slate-900">Apa yang ingin Anda cadangkan?</h2>
                <button onClick={() => setSelectedData(dataOptions.map(o => o.id))} className="text-[10px] font-black text-indigo-600 uppercase tracking-widest hover:underline px-2">Pilih Semua</button>
             </div>
             <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {dataOptions.map(opt => (
                    <div key={opt.id} onClick={() => toggleData(opt.id)} className={cn("p-6 rounded-[32px] border-2 transition-all cursor-pointer flex items-start gap-5 relative overflow-hidden", selectedData.includes(opt.id) ? "border-indigo-500 bg-indigo-50/30 shadow-lg" : "border-slate-100 hover:border-indigo-200 bg-white")}>
                        <div className={cn("w-12 h-12 rounded-2xl flex items-center justify-center shrink-0 shadow-inner", selectedData.includes(opt.id) ? "bg-indigo-600 text-white" : "bg-slate-50 text-slate-400")}><opt.icon className="w-6 h-6" /></div>
                        <div className="flex-1 min-w-0">
                            <div className="flex items-center gap-2 mb-1">
                                <p className="font-black text-slate-900">{opt.label}</p>
                                <span className={cn("text-[8px] font-black px-1.5 py-0.5 rounded uppercase tracking-tighter", selectedData.includes(opt.id) ? "bg-indigo-100 text-indigo-700" : "bg-slate-100 text-slate-400")}>{opt.detail}</span>
                            </div>
                            <p className="text-xs text-slate-500 leading-relaxed">{opt.description}</p>
                        </div>
                    </div>
                ))}
             </div>
             <div className="pt-8 mt-auto flex justify-between">
                <button onClick={() => setStep('select-device')} className="px-8 py-4 font-black text-slate-400 hover:text-slate-600 transition-all uppercase text-[10px]">Kembali</button>
                <button disabled={selectedData.length === 0} onClick={handleNextToConfigure} className="px-8 py-4 bg-indigo-600 text-white rounded-2xl font-black shadow-xl shadow-indigo-200 hover:bg-indigo-700 disabled:opacity-50 transition-all flex items-center gap-3">Review Rencana <ArrowRight className="w-5 h-5" /></button>
            </div>
          </div>
        )}

        {step === 'configure' && (
            <div className="flex-1 flex flex-col min-h-0 animate-in fade-in duration-300">
                <div className="p-8 border-b border-slate-100 shrink-0">
                    <div className="flex justify-between items-center mb-6">
                        <h2 className="text-2xl font-black text-slate-900 tracking-tight">Eksplorasi Rencana Backup</h2>
                        <div className="flex items-center gap-4">
                            <div className="text-right">
                                <p className="text-[10px] font-black text-slate-400 uppercase">Estimasi Total</p>
                                <p className="text-xl font-black text-indigo-600">{formatBytes(totalBytes)}</p>
                            </div>
                            <div className="w-px h-8 bg-slate-100" />
                            <div className="text-right">
                                <p className="text-[10px] font-black text-slate-400 uppercase">Total File</p>
                                <p className="text-xl font-black text-slate-900">{selectedFiles.length}</p>
                            </div>
                        </div>
                    </div>

                    <div className="relative">
                        <Search className="absolute left-4 top-3 w-4 h-4 text-slate-400" />
                        <input
                            type="text"
                            placeholder="Cari file dalam rencana backup..."
                            value={reviewSearch}
                            onChange={(e) => setReviewSearch(e.target.value)}
                            className="w-full bg-slate-50 border border-slate-100 pl-11 pr-4 py-2.5 rounded-2xl text-sm outline-none focus:ring-2 focus:ring-indigo-500/20 transition-all"
                        />
                    </div>
                </div>

                <div className="flex-1 overflow-y-auto bg-slate-50/50 custom-scrollbar p-6">
                    {isCalculating ? (
                        <div className="h-full flex flex-col items-center justify-center gap-4 text-slate-400">
                            <Loader2 className="w-8 h-8 animate-spin text-indigo-600" />
                            <p className="text-[10px] font-black uppercase tracking-widest">Menganalisis Filesystem HP...</p>
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

                <div className="p-8 border-t border-slate-100 bg-white shrink-0 flex justify-between items-center">
                    <button onClick={() => setStep('select-data')} className="font-black text-slate-400 hover:text-slate-600 transition-all uppercase text-[10px]">Kembali</button>
                    <div className="flex items-center gap-4">
                        <div className="hidden md:flex items-center gap-2 text-emerald-600 bg-emerald-50 px-4 py-2 rounded-xl border border-emerald-100">
                            <ShieldCheck className="w-4 h-4" />
                            <span className="text-[10px] font-black uppercase">AES-256 Ready</span>
                        </div>
                        <button
                            onClick={() => handleStartBackup(dataOptions.length)}
                            className="px-10 py-4 bg-slate-900 text-white rounded-2xl font-black shadow-xl shadow-slate-200 hover:bg-slate-800 transition-all flex items-center gap-3"
                        >
                            Konfirmasi & Mulai Backup <ArrowRight className="w-5 h-5" />
                        </button>
                    </div>
                </div>
            </div>
        )}

        {step === 'progress' && (
            <div className="p-10 flex-1 flex flex-col items-center justify-center text-center space-y-8 animate-in zoom-in duration-500">
                {error ? (
                    <>
                        <div className="w-20 h-20 bg-red-100 text-red-600 rounded-full flex items-center justify-center"><XCircle className="w-12 h-12" /></div>
                        <div><h2 className="text-3xl font-black text-slate-900 mb-2">Backup Gagal</h2><p className="text-red-500 font-medium">{error}</p></div>
                        <button onClick={() => setStep('configure')} className="px-8 py-3 bg-slate-900 text-white rounded-2xl font-black shadow-xl">Coba Lagi</button>
                    </>
                ) : (
                    <>
                        <div className="relative">
                            <div className={cn("w-40 h-40 rounded-full border-8 border-slate-100 transition-all duration-300", progressPercent < 100 ? "border-t-indigo-600 animate-spin" : "border-emerald-500")} />
                            <div className="absolute inset-0 flex items-center justify-center">{progressPercent < 100 ? <Database className="w-12 h-12 text-indigo-600" /> : <CheckCircle2 className="w-12 h-12 text-emerald-500" />}</div>
                        </div>
                        <div>
                            <h2 className="text-3xl font-black text-slate-900 mb-2">{progressPercent < 100 ? "Sedang Mencadangkan..." : "Backup Selesai!"}</h2>
                            <p className="text-slate-500 font-medium">{progressPercent < 100 ? "Mohon jangan cabut perangkat Anda." : "Data Anda kini tersimpan aman dan terenkripsi."}</p>
                        </div>
                        {progressPercent < 100 && (
                            <div className="w-full max-w-sm space-y-4">
                                <div className="h-4 w-full bg-slate-100 rounded-full overflow-hidden"><div className="h-full bg-indigo-600 transition-all duration-500" style={{ width: `${Math.max(progressPercent, 1)}%` }} /></div>
                                <div className="flex justify-between text-xs font-black text-slate-400 uppercase tracking-widest"><span className="truncate max-w-[200px]">{progressMsg}</span><span>{progressPercent}%</span></div>
                                {progressPercent > 95 && <p className="text-[10px] text-amber-500 font-bold animate-pulse">Hampir selesai. Mohon tunggu, sedang memproses data Android...</p>}
                            </div>
                        )}
                        {progressPercent === 100 && <button onClick={() => window.location.reload()} className="px-8 py-3 bg-indigo-600 text-white rounded-2xl font-black shadow-xl shadow-indigo-200">Kembali ke Dashboard</button>}
                    </>
                )}
            </div>
        )}
      </div>
    </div>
  );
}

function StepIndicator({ step, active, completed, label }: { step: number, active: boolean, completed: boolean, label: string }) {
    return (
        <div className="flex flex-col items-center gap-2">
            <div className={cn(
                "w-10 h-10 rounded-full flex items-center justify-center font-black text-sm transition-all",
                active ? "bg-indigo-600 text-white shadow-lg shadow-indigo-200 scale-110" :
                completed ? "bg-emerald-50 text-white" : "bg-slate-100 text-slate-400"
            )}>
                {completed && !active ? <CheckCircle2 className="w-6 h-6" /> : step}
            </div>
            <span className={cn("text-[10px] font-black uppercase tracking-widest", active ? "text-indigo-600" : "text-slate-400")}>{label}</span>
        </div>
    );
}
