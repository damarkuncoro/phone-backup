import { useState } from "react";
import {
  Usb, Wifi, X, RefreshCw, CheckCircle2,
  AlertCircle, Smartphone, HelpCircle, Loader2, ArrowRight, HardDrive, Sparkles
} from "lucide-react";
import { cn } from "@/shared/lib/utils";
import { deviceService } from "@/services/deviceService";

interface AddDeviceModalProps {
  isOpen: boolean;
  onClose: () => void;
  onDeviceConnected?: () => void;
}

type BrandGuide = "all" | "samsung" | "xiaomi" | "oppo_vivo" | "pixel";
type TabType = "mtp" | "usb" | "wireless";

export function AddDeviceModal({ isOpen, onClose, onDeviceConnected }: AddDeviceModalProps) {
  const [activeTab, setActiveTab] = useState<TabType>("mtp");
  const [selectedBrand, setSelectedBrand] = useState<BrandGuide>("all");
  const [ipAddress, setIpAddress] = useState("");
  const [port, setPort] = useState("5555");
  const [connecting, setConnecting] = useState(false);
  const [scanning, setScanning] = useState(false);
  const [statusMessage, setStatusMessage] = useState<{ type: "success" | "error"; text: string } | null>(null);

  if (!isOpen) return null;

  const handleScanDevices = async (modeName: string) => {
    setScanning(true);
    setStatusMessage(null);
    try {
      const devs = await deviceService.getAll();
      if (devs && devs.length > 0) {
        setStatusMessage({
          type: "success",
          text: `Ditemukan ${devs.length} perangkat terhubung (${modeName})!`
        });
        onDeviceConnected?.();
      } else {
        setStatusMessage({
          type: "error",
          text: `Belum ada perangkat terdeteksi. Pastikan kabel terpasang dan opsi ${modeName === 'MTP' ? '"Transfer File / MTP"' : 'USB Debugging'} telah dipilih di ponsel.`
        });
      }
    } catch {
      setStatusMessage({
        type: "error",
        text: "Gagal memindai perangkat. Pastikan server backup berjalan."
      });
    } finally {
      setScanning(false);
    }
  };

  const handleConnectWireless = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!ipAddress.trim()) {
      setStatusMessage({ type: "error", text: "Silakan masukkan alamat IP ponsel." });
      return;
    }

    setConnecting(true);
    setStatusMessage(null);
    try {
      await deviceService.connectWireless(ipAddress.trim(), parseInt(port || "5555", 10));
      setStatusMessage({
        type: "success",
        text: `Berhasil terhubung ke ${ipAddress}:${port} secara nirkabel!`
      });
      onDeviceConnected?.();
    } catch (err: any) {
      setStatusMessage({
        type: "error",
        text: err?.message || "Gagal menghubungkan. Pastikan ponsel berada di jaringan WiFi yang sama dan Wireless Debugging aktif."
      });
    } finally {
      setConnecting(false);
    }
  };

  return (
    <div className="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-slate-900/80 backdrop-blur-sm animate-in fade-in duration-200">
      <div className="bg-white rounded-[36px] shadow-2xl border border-slate-100 max-w-2xl w-full overflow-hidden flex flex-col max-h-[90vh] animate-in zoom-in-95 duration-200">
        
        {/* Header */}
        <div className="p-6 bg-slate-50 border-b border-slate-100 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-2xl bg-indigo-600 text-white flex items-center justify-center shadow-lg shadow-indigo-200">
              <Smartphone className="w-5 h-5" />
            </div>
            <div>
              <h3 className="text-lg font-black text-slate-900 tracking-tight">
                Tambah Perangkat Baru
              </h3>
              <p className="text-xs text-slate-400 font-medium">
                Pilih metode koneksi yang sesuai dengan kebutuhan Anda
              </p>
            </div>
          </div>

          <button
            data-testid="close-add-device-modal"
            onClick={onClose}
            className="p-2 hover:bg-slate-200/60 rounded-xl text-slate-400 hover:text-slate-700 transition-all"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Tab Selection */}
        <div className="grid grid-cols-3 border-b border-slate-100 bg-slate-50/50 p-2 gap-1.5">
          <button
            onClick={() => { setActiveTab("mtp"); setStatusMessage(null); }}
            className={cn(
              "py-3 px-2 rounded-2xl font-black text-[11px] uppercase tracking-wider transition-all flex items-center justify-center gap-1.5 text-center",
              activeTab === "mtp"
                ? "bg-white text-cyan-700 shadow-sm ring-1 ring-cyan-200"
                : "text-slate-400 hover:text-slate-700"
            )}
          >
            <HardDrive className="w-3.5 h-3.5 text-cyan-600" />
            <span className="truncate">Kabel Biasa (MTP)</span>
          </button>

          <button
            onClick={() => { setActiveTab("usb"); setStatusMessage(null); }}
            className={cn(
              "py-3 px-2 rounded-2xl font-black text-[11px] uppercase tracking-wider transition-all flex items-center justify-center gap-1.5 text-center",
              activeTab === "usb"
                ? "bg-white text-indigo-600 shadow-sm ring-1 ring-indigo-200"
                : "text-slate-400 hover:text-slate-700"
            )}
          >
            <Usb className="w-3.5 h-3.5 text-indigo-600" />
            <span className="truncate">USB Debugging</span>
          </button>

          <button
            onClick={() => { setActiveTab("wireless"); setStatusMessage(null); }}
            className={cn(
              "py-3 px-2 rounded-2xl font-black text-[11px] uppercase tracking-wider transition-all flex items-center justify-center gap-1.5 text-center",
              activeTab === "wireless"
                ? "bg-white text-indigo-600 shadow-sm ring-1 ring-indigo-200"
                : "text-slate-400 hover:text-slate-700"
            )}
          >
            <Wifi className="w-3.5 h-3.5 text-indigo-600" />
            <span className="truncate">Wireless ADB</span>
          </button>
        </div>

        {/* Status Message Alert */}
        {statusMessage && (
          <div className={cn(
            "mx-6 mt-4 p-4 rounded-2xl border flex items-start gap-3 text-xs font-bold animate-in slide-in-from-top-2",
            statusMessage.type === "success"
              ? "bg-emerald-50 border-emerald-200 text-emerald-800"
              : "bg-rose-50 border-rose-200 text-rose-800"
          )}>
            {statusMessage.type === "success" ? (
              <CheckCircle2 className="w-5 h-5 text-emerald-600 shrink-0 mt-0.5" />
            ) : (
              <AlertCircle className="w-5 h-5 text-rose-600 shrink-0 mt-0.5" />
            )}
            <p className="flex-1 leading-relaxed">{statusMessage.text}</p>
          </div>
        )}

        {/* Modal Body Content */}
        <div className="flex-1 overflow-y-auto custom-scrollbar p-6 space-y-6">
          {activeTab === "mtp" && (
            /* MTP Tab Content */
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
                    <span>
                      Sambungkan ponsel ke komputer menggunakan <b>kabel USB biasa</b>.
                    </span>
                  </li>
                  <li className="flex items-start gap-2.5">
                    <span className="w-5 h-5 rounded-full bg-cyan-600 text-white font-black text-[10px] flex items-center justify-center shrink-0 mt-0.5">2</span>
                    <span>
                      Buka kunci layar ponsel &gt; Ketuk notifikasi sambungan USB &gt; Pilih opsi <b>"Transfer File"</b> atau <b>"MTP / Transfer Media"</b> (jangan pilih "Hanya Isi Daya").
                    </span>
                  </li>
                </ol>
              </div>

              <button
                type="button"
                onClick={() => handleScanDevices("MTP")}
                disabled={scanning}
                className="w-full py-3.5 bg-cyan-700 hover:bg-cyan-800 text-white rounded-2xl font-black text-xs uppercase tracking-wider transition-all shadow-lg shadow-cyan-200/50 flex items-center justify-center gap-2 active:scale-95 disabled:opacity-50"
              >
                <RefreshCw className={cn("w-4 h-4", scanning && "animate-spin")} />
                {scanning ? "Memindai Sambungan MTP..." : "Pindai Ulang Sambungan MTP"}
              </button>
            </div>
          )}

          {activeTab === "usb" && (
            /* USB Debugging Tab Content */
            <div className="space-y-5">
              {/* Brand Guide Selector */}
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

              {/* Step By Step Guide */}
              <div className="space-y-3 bg-slate-50 p-5 rounded-3xl border border-slate-200/80">
                <h4 className="text-xs font-black uppercase tracking-wider text-slate-700 flex items-center gap-2">
                  <HelpCircle className="w-4 h-4 text-indigo-600" />
                  Langkah-Langkah Mengaktifkan USB Debugging:
                </h4>

                <ol className="space-y-3 text-xs text-slate-600 leading-relaxed font-medium">
                  <li className="flex items-start gap-2.5">
                    <span className="w-5 h-5 rounded-full bg-indigo-600 text-white font-black text-[10px] flex items-center justify-center shrink-0 mt-0.5">1</span>
                    <span>
                      Buka <b>Pengaturan (Settings)</b> &gt; <b>Tentang Ponsel (About Phone)</b>.
                    </span>
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
                    <span>
                      Sambungkan ponsel ke komputer dengan kabel USB. Di layar ponsel, centang <i>"Selalu izinkan dari komputer ini"</i> dan pilih <b>Izinkan (Allow)</b>.
                    </span>
                  </li>
                </ol>
              </div>

              {/* Action Button */}
              <button
                type="button"
                onClick={() => handleScanDevices("ADB")}
                disabled={scanning}
                className="w-full py-3.5 bg-slate-900 hover:bg-slate-800 text-white rounded-2xl font-black text-xs uppercase tracking-wider transition-all shadow-lg flex items-center justify-center gap-2 active:scale-95 disabled:opacity-50"
              >
                <RefreshCw className={cn("w-4 h-4", scanning && "animate-spin")} />
                {scanning ? "Memindai Sambungan ADB..." : "Pindai Ulang Sambungan ADB"}
              </button>
            </div>
          )}

          {activeTab === "wireless" && (
            /* Wireless ADB Tab Content */
            <form onSubmit={handleConnectWireless} className="space-y-5">
              <div className="p-4 bg-indigo-50/70 border border-indigo-100 rounded-2xl text-xs text-indigo-900 leading-relaxed font-medium space-y-1">
                <p className="font-bold flex items-center gap-1.5">
                  <Wifi className="w-4 h-4 text-indigo-600" /> Syarat Koneksi Wireless ADB:
                </p>
                <p>1. Ponsel dan komputer harus berada dalam <b>jaringan WiFi yang sama</b>.</p>
                <p>2. Aktifkan <b>Wireless Debugging</b> di menu <i>Opsi Pengembang</i> ponsel Anda.</p>
              </div>

              <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
                <div className="sm:col-span-2 space-y-1.5">
                  <label className="text-[10px] font-black uppercase tracking-widest text-slate-400">
                    Alamat IP Ponsel
                  </label>
                  <input
                    type="text"
                    required
                    placeholder="192.168.1.100"
                    value={ipAddress}
                    onChange={(e) => setIpAddress(e.target.value)}
                    className="w-full bg-slate-50 border border-slate-200/80 px-4 py-3 rounded-2xl text-xs font-mono outline-none focus:ring-4 focus:ring-indigo-500/10 focus:border-indigo-300 transition-all"
                  />
                </div>

                <div className="space-y-1.5">
                  <label className="text-[10px] font-black uppercase tracking-widest text-slate-400">
                    Port ADB
                  </label>
                  <input
                    type="number"
                    required
                    placeholder="5555"
                    value={port}
                    onChange={(e) => setPort(e.target.value)}
                    className="w-full bg-slate-50 border border-slate-200/80 px-4 py-3 rounded-2xl text-xs font-mono outline-none focus:ring-4 focus:ring-indigo-500/10 focus:border-indigo-300 transition-all"
                  />
                </div>
              </div>

              <button
                type="submit"
                disabled={connecting}
                className="w-full py-3.5 bg-indigo-600 hover:bg-indigo-700 text-white rounded-2xl font-black text-xs uppercase tracking-wider transition-all shadow-lg shadow-indigo-200 flex items-center justify-center gap-2 active:scale-95 disabled:opacity-50"
              >
                {connecting ? <Loader2 className="w-4 h-4 animate-spin" /> : <ArrowRight className="w-4 h-4" />}
                {connecting ? "Menghubungkan ke Perangkat..." : "Sambungkan Nirkabel"}
              </button>
            </form>
          )}
        </div>

        {/* Footer */}
        <div className="p-4 bg-slate-50 border-t border-slate-100 flex justify-end">
          <button
            type="button"
            onClick={onClose}
            className="px-6 py-2.5 bg-white border border-slate-200 text-slate-700 rounded-xl text-xs font-black uppercase tracking-wider hover:bg-slate-100 transition-all"
          >
            Tutup
          </button>
        </div>

      </div>
    </div>
  );
}
