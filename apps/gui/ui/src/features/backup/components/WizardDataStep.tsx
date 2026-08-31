import {
  Sparkles, Check, ArrowLeft, ArrowRight, Zap,
  HardDrive, Image as ImageIcon, MessageSquare, FolderCheck, Users, PhoneCall, Smartphone
} from 'lucide-react';
import { cn } from "@/shared/lib/utils";

export interface DataOption {
  id: string;
  label: string;
  icon: any;
  description: string;
  detail: string;
  requiresAdb?: boolean;
}

export const DATA_OPTIONS: DataOption[] = [
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

interface WizardDataStepProps {
  isMtpDevice: boolean;
  selectedData: string[];
  onToggleData: (id: string) => void;
  onSelectAll: () => void;
  onSelectMediaOnly: () => void;
  onBack: () => void;
  onExpressBackup: () => void;
  onNext: () => void;
}

export function WizardDataStep({
  isMtpDevice,
  selectedData,
  onToggleData,
  onSelectAll,
  onSelectMediaOnly,
  onBack,
  onExpressBackup,
  onNext
}: WizardDataStepProps) {
  return (
    <div className="p-6 md:p-8 space-y-6 animate-in fade-in duration-200 flex-1 flex flex-col justify-between">
      <div className="space-y-6">
        <div className="flex flex-col sm:flex-row sm:items-end justify-between gap-3">
          <div>
            <h2 className="text-xl font-black text-slate-900 tracking-tight">
              Apa yang ingin Anda cadangkan?
            </h2>
            <p className="text-xs text-slate-400 font-medium mt-0.5">
              Pilih kategori data yang ingin dimasukkan ke dalam paket backup ini.
            </p>
          </div>

          <div className="flex gap-2 shrink-0">
            <button
              type="button"
              onClick={onSelectAll}
              className="px-3 py-1.5 bg-indigo-50 text-indigo-700 hover:bg-indigo-100 rounded-xl text-[10px] font-black uppercase tracking-wider transition-colors"
            >
              Pilih Semua
            </button>
            <button
              type="button"
              onClick={onSelectMediaOnly}
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
          {DATA_OPTIONS.map(opt => {
            const isSelected = selectedData.includes(opt.id);
            const isDisabled = isMtpDevice && opt.requiresAdb;

            return (
              <div
                key={opt.id}
                onClick={() => !isDisabled && onToggleData(opt.id)}
                className={cn(
                  "p-5 rounded-[28px] border-2 transition-all flex flex-col justify-between space-y-3 relative overflow-hidden select-none",
                  isDisabled
                    ? "opacity-40 bg-slate-50 border-slate-200/50 cursor-not-allowed"
                    : isSelected
                    ? "border-indigo-500 bg-indigo-50/40 shadow-md ring-2 ring-indigo-500/10 cursor-pointer"
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
          onClick={onBack}
          className="px-6 py-3 font-black text-slate-400 hover:text-slate-700 transition-all uppercase text-[10px] tracking-wider flex items-center gap-2"
        >
          <ArrowLeft className="w-4 h-4" /> Kembali
        </button>

        <div className="flex items-center gap-3 w-full sm:w-auto justify-end">
          <button
            type="button"
            disabled={selectedData.length === 0}
            onClick={onExpressBackup}
            title="Langsung mulai proses backup tanpa menunggu analisis pohon file selesai (sangat cepat untuk HP 128GB-512GB)"
            className="px-6 py-3.5 bg-slate-900 hover:bg-slate-800 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-lg transition-all flex items-center gap-2 active:scale-95 disabled:opacity-50"
          >
            <Zap className="w-4 h-4 text-amber-400" />
            <span>Mulai Backup Instan</span>
          </button>

          <button
            type="button"
            disabled={selectedData.length === 0}
            onClick={onNext}
            className="px-8 py-3.5 bg-indigo-600 hover:bg-indigo-700 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-lg shadow-indigo-200 hover:shadow-indigo-300 disabled:opacity-50 transition-all flex items-center gap-2.5 active:scale-95"
          >
            <span>Review Rencana</span>
            <ArrowRight className="w-4 h-4" />
          </button>
        </div>
      </div>
    </div>
  );
}
