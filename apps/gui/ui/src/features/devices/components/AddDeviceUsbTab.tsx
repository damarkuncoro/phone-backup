import { useState } from "react";
import { RefreshCw, HelpCircle } from "lucide-react";
import { cn } from "@/shared/lib/utils";

export type BrandGuide = "all" | "samsung" | "xiaomi" | "oppo_vivo";

interface AddDeviceUsbTabProps {
  scanning: boolean;
  onScan: () => void;
}

export function AddDeviceUsbTab({ scanning, onScan }: AddDeviceUsbTabProps) {
  const [selectedBrand, setSelectedBrand] = useState<BrandGuide>("all");

  return (
    <div className="space-y-5">
      <div className="space-y-2">
        <label className="text-[10px] font-black uppercase tracking-widest text-slate-400">
          Pilih Merek Ponsel untuk Panduan Spesifik:
        </label>
        <div className="grid grid-cols-2 sm:grid-cols-4 gap-2">
          <button
            type="button"
            onClick={() => setSelectedBrand("all")}
            className={cn(
              "py-2 px-3 rounded-xl text-xs font-bold border transition-all text-center",
              selectedBrand === "all" ? "bg-indigo-50 border-indigo-300 text-indigo-700" : "bg-slate-50 border-slate-200/70 text-slate-600"
            )}
          >
            Standar / Pixel
          </button>
          <button
            type="button"
            onClick={() => setSelectedBrand("samsung")}
            className={cn(
              "py-2 px-3 rounded-xl text-xs font-bold border transition-all text-center",
              selectedBrand === "samsung" ? "bg-indigo-50 border-indigo-300 text-indigo-700" : "bg-slate-50 border-slate-200/70 text-slate-600"
            )}
          >
            Samsung OneUI
          </button>
          <button
            type="button"
            onClick={() => setSelectedBrand("xiaomi")}
            className={cn(
              "py-2 px-3 rounded-xl text-xs font-bold border transition-all text-center",
              selectedBrand === "xiaomi" ? "bg-indigo-50 border-indigo-300 text-indigo-700" : "bg-slate-50 border-slate-200/70 text-slate-600"
            )}
          >
            Xiaomi / HyperOS
          </button>
          <button
            type="button"
            onClick={() => setSelectedBrand("oppo_vivo")}
            className={cn(
              "py-2 px-3 rounded-xl text-xs font-bold border transition-all text-center",
              selectedBrand === "oppo_vivo" ? "bg-indigo-50 border-indigo-300 text-indigo-700" : "bg-slate-50 border-slate-200/70 text-slate-600"
            )}
          >
            Oppo / Vivo / Realme
          </button>
        </div>
      </div>

      <div className="space-y-3 bg-slate-50 p-5 rounded-3xl border border-slate-200/80">
        <h4 className="text-xs font-black uppercase tracking-wider text-slate-700 flex items-center gap-2">
          <HelpCircle className="w-4 h-4 text-indigo-600" />
          Langkah-Langkah Mengaktifkan USB Debugging:
        </h4>

        <ol className="space-y-3 text-xs text-slate-600 leading-relaxed font-medium">
          <li className="flex items-start gap-2.5">
            <span className="w-5 h-5 rounded-full bg-indigo-600 text-white font-black text-[10px] flex items-center justify-center shrink-0 mt-0.5">1</span>
            <span>Buka <b>Pengaturan (Settings)</b> &gt; <b>Tentang Ponsel (About Phone)</b>.</span>
          </li>
          <li className="flex items-start gap-2.5">
            <span className="w-5 h-5 rounded-full bg-indigo-600 text-white font-black text-[10px] flex items-center justify-center shrink-0 mt-0.5">2</span>
            <span>
              {selectedBrand === "xiaomi" ? (
                <>Ketuk <b>Versi OS / MIUI</b> sebanyak <b>7 kali</b> sampai muncul notifikasi <i>"Anda sekarang seorang pengembang"</i>.</>
              ) : selectedBrand === "samsung" ? (
                <>Buka <b>Informasi Perangkat Lunak</b> &gt; Ketuk <b>Nomor Versi (Build Number)</b> sebanyak <b>7 kali</b>.</>
              ) : (
                <>Ketuk <b>Nomor Versi / Nomor Bentukan (Build Number)</b> sebanyak <b>7 kali berturut-turut</b>.</>
              )}
            </span>
          </li>
          <li className="flex items-start gap-2.5">
            <span className="w-5 h-5 rounded-full bg-indigo-600 text-white font-black text-[10px] flex items-center justify-center shrink-0 mt-0.5">3</span>
            <span>
              Kembali ke menu Pengaturan &gt; Buka <b>Opsi Pengembang (Developer Options)</b> &gt; Aktifkan <b>USB Debugging</b>.
              {selectedBrand === "xiaomi" && " (Juga aktifkan 'Install via USB' dan 'USB Debugging (Security settings)')."}
            </span>
          </li>
          <li className="flex items-start gap-2.5">
            <span className="w-5 h-5 rounded-full bg-indigo-600 text-white font-black text-[10px] flex items-center justify-center shrink-0 mt-0.5">4</span>
            <span>Sambungkan ponsel ke komputer dengan kabel USB. Di layar ponsel, centang <i>"Selalu izinkan dari komputer ini"</i> dan pilih <b>Izinkan (Allow)</b>.</span>
          </li>
        </ol>
      </div>

      <button
        type="button"
        onClick={onScan}
        disabled={scanning}
        className="w-full py-3.5 bg-slate-900 hover:bg-slate-800 text-white rounded-2xl font-black text-xs uppercase tracking-wider transition-all shadow-lg flex items-center justify-center gap-2 active:scale-95 disabled:opacity-50"
      >
        <RefreshCw className={cn("w-4 h-4", scanning && "animate-spin")} />
        {scanning ? "Memindai Sambungan ADB..." : "Pindai Ulang Sambungan ADB"}
      </button>
    </div>
  );
}
