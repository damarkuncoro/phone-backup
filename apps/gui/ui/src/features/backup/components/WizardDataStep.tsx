import { Sparkles, ArrowLeft, ArrowRight, Zap } from 'lucide-react';
import { DATA_OPTIONS, type DataOption } from '../lib/wizardDataOptions';
import { WizardDataOptionCard } from './WizardDataOptionCard';

export { DATA_OPTIONS, type DataOption };

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
          {DATA_OPTIONS.map(opt => (
            <WizardDataOptionCard
              key={opt.id}
              option={opt}
              isSelected={selectedData.includes(opt.id)}
              isDisabled={Boolean(isMtpDevice && opt.requiresAdb)}
              onToggle={onToggleData}
            />
          ))}
        </div>
      </div>

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
            title="Langsung mulai proses backup tanpa menunggu analisis pohon file selesai"
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
