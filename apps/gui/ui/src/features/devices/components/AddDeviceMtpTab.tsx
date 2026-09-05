import { RefreshCw, HelpCircle, Sparkles } from "lucide-react";
import { cn } from "@/shared/lib/utils";

interface AddDeviceMtpTabProps {
  scanning: boolean;
  onScan: () => void;
}

export function AddDeviceMtpTab({ scanning, onScan }: AddDeviceMtpTabProps) {
  return (
    <div className="space-y-5">
      <div className="p-4 bg-cyan-50/80 border border-cyan-200 rounded-3xl text-xs text-cyan-950 leading-relaxed space-y-2">
        <div className="flex items-center gap-2 font-black text-cyan-900 uppercase tracking-wider text-[11px]">
          <Sparkles className="w-4 h-4 text-cyan-600" />
          Rekomendasi Utama untuk Pengguna Awam (Tanpa Developer Mode)
        </div>
        <p className="font-medium text-cyan-900/90">
          Mode ini tidak memerlukan pengaturan rumit di ponsel. Cukup colok kabel USB standar, dan Anda dapat mencadangkan seluruh <b>Foto (DCIM/Pictures)</b>, <b>Video</b>, <b>Dokumen</b>, dan <b>Download</b> dengan cepat.
        </p>
      </div>

      <div className="space-y-3 bg-slate-50 p-5 rounded-3xl border border-slate-200/80">
        <h4 className="text-xs font-black uppercase tracking-wider text-slate-700 flex items-center gap-2">
          <HelpCircle className="w-4 h-4 text-cyan-600" />
          Cara Menghubungkan (Hanya 2 Langkah):
        </h4>

        <ol className="space-y-3.5 text-xs text-slate-600 leading-relaxed font-medium">
          <li className="flex items-start gap-2.5">
            <span className="w-5 h-5 rounded-full bg-cyan-600 text-white font-black text-[10px] flex items-center justify-center shrink-0 mt-0.5">1</span>
            <span>Sambungkan ponsel ke komputer menggunakan <b>kabel USB biasa</b>.</span>
          </li>
          <li className="flex items-start gap-2.5">
            <span className="w-5 h-5 rounded-full bg-cyan-600 text-white font-black text-[10px] flex items-center justify-center shrink-0 mt-0.5">2</span>
            <span>Buka kunci layar ponsel &gt; Ketuk notifikasi sambungan USB &gt; Pilih opsi <b>"Transfer File"</b> atau <b>"MTP / Transfer Media"</b> (jangan pilih "Hanya Isi Daya").</span>
          </li>
        </ol>
      </div>

      <button
        type="button"
        onClick={onScan}
        disabled={scanning}
        className="w-full py-3.5 bg-cyan-700 hover:bg-cyan-800 text-white rounded-2xl font-black text-xs uppercase tracking-wider transition-all shadow-lg shadow-cyan-200/50 flex items-center justify-center gap-2 active:scale-95 disabled:opacity-50"
      >
        <RefreshCw className={cn("w-4 h-4", scanning && "animate-spin")} />
        {scanning ? "Memindai Sambungan MTP..." : "Pindai Ulang Sambungan MTP"}
      </button>
    </div>
  );
}
