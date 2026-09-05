import { useState } from "react";
import { Usb, Wifi, X, CheckCircle2, AlertCircle, Smartphone, HardDrive } from "lucide-react";
import { cn } from "@/shared/lib/utils";
import { deviceService } from "@/services/deviceService";
import { AddDeviceMtpTab } from "./AddDeviceMtpTab";
import { AddDeviceUsbTab } from "./AddDeviceUsbTab";
import { AddDeviceWirelessTab } from "./AddDeviceWirelessTab";

interface AddDeviceModalProps {
  isOpen: boolean;
  onClose: () => void;
  onDeviceConnected?: () => void;
}

type TabType = "mtp" | "usb" | "wireless";

export function AddDeviceModal({ isOpen, onClose, onDeviceConnected }: AddDeviceModalProps) {
  const [activeTab, setActiveTab] = useState<TabType>("mtp");
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
        setStatusMessage({ type: "success", text: `Ditemukan ${devs.length} perangkat terhubung (${modeName})!` });
        onDeviceConnected?.();
      } else {
        setStatusMessage({
          type: "error",
          text: `Belum ada perangkat terdeteksi. Pastikan kabel terpasang dan opsi ${modeName === 'MTP' ? '"Transfer File / MTP"' : 'USB Debugging'} telah dipilih di ponsel.`
        });
      }
    } catch {
      setStatusMessage({ type: "error", text: "Gagal memindai perangkat. Pastikan server backup berjalan." });
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
      setStatusMessage({ type: "success", text: `Berhasil terhubung ke ${ipAddress}:${port} secara nirkabel!` });
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
      <div className="bg-white rounded-[32px] shadow-2xl border border-slate-100 max-w-2xl w-full overflow-hidden flex flex-col max-h-[90vh] animate-in zoom-in-95 duration-200">
        <div className="p-6 bg-slate-50 border-b border-slate-100 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-2xl bg-indigo-600 text-white flex items-center justify-center shadow-lg shadow-indigo-200">
              <Smartphone className="w-5 h-5" />
            </div>
            <div>
              <h3 className="text-lg font-black text-slate-900 tracking-tight">Tambah Perangkat Baru</h3>
              <p className="text-xs text-slate-400 font-medium">Pilih metode koneksi yang sesuai dengan kebutuhan Anda</p>
            </div>
          </div>
          <button data-testid="close-add-device-modal" onClick={onClose} className="p-2 hover:bg-slate-200/60 rounded-xl text-slate-400 hover:text-slate-700 transition-all">
            <X className="w-5 h-5" />
          </button>
        </div>

        <div className="grid grid-cols-3 border-b border-slate-100 bg-slate-50/50 p-2 gap-1.5">
          <button onClick={() => { setActiveTab("mtp"); setStatusMessage(null); }} className={cn("py-3 px-2 rounded-2xl font-black text-[11px] uppercase tracking-wider transition-all flex items-center justify-center gap-1.5 text-center", activeTab === "mtp" ? "bg-white text-cyan-700 shadow-sm ring-1 ring-cyan-200" : "text-slate-400 hover:text-slate-700")}>
            <HardDrive className="w-3.5 h-3.5 text-cyan-600" />
            <span className="truncate">Kabel Biasa (MTP)</span>
          </button>
          <button onClick={() => { setActiveTab("usb"); setStatusMessage(null); }} className={cn("py-3 px-2 rounded-2xl font-black text-[11px] uppercase tracking-wider transition-all flex items-center justify-center gap-1.5 text-center", activeTab === "usb" ? "bg-white text-indigo-600 shadow-sm ring-1 ring-indigo-200" : "text-slate-400 hover:text-slate-700")}>
            <Usb className="w-3.5 h-3.5 text-indigo-600" />
            <span className="truncate">USB Debugging</span>
          </button>
          <button onClick={() => { setActiveTab("wireless"); setStatusMessage(null); }} className={cn("py-3 px-2 rounded-2xl font-black text-[11px] uppercase tracking-wider transition-all flex items-center justify-center gap-1.5 text-center", activeTab === "wireless" ? "bg-white text-indigo-600 shadow-sm ring-1 ring-indigo-200" : "text-slate-400 hover:text-slate-700")}>
            <Wifi className="w-3.5 h-3.5 text-indigo-600" />
            <span className="truncate">Wireless ADB</span>
          </button>
        </div>

        {statusMessage && (
          <div className={cn("mx-6 mt-4 p-4 rounded-2xl border flex items-start gap-3 text-xs font-bold animate-in slide-in-from-top-2", statusMessage.type === "success" ? "bg-emerald-50 border-emerald-200 text-emerald-800" : "bg-rose-50 border-rose-200 text-rose-800")}>
            {statusMessage.type === "success" ? <CheckCircle2 className="w-5 h-5 text-emerald-600 shrink-0 mt-0.5" /> : <AlertCircle className="w-5 h-5 text-rose-600 shrink-0 mt-0.5" />}
            <p className="flex-1 leading-relaxed">{statusMessage.text}</p>
          </div>
        )}

        <div className="flex-1 overflow-y-auto custom-scrollbar p-6 space-y-6">
          {activeTab === "mtp" && <AddDeviceMtpTab scanning={scanning} onScan={() => handleScanDevices("MTP")} />}
          {activeTab === "usb" && <AddDeviceUsbTab scanning={scanning} onScan={() => handleScanDevices("ADB")} />}
          {activeTab === "wireless" && <AddDeviceWirelessTab ipAddress={ipAddress} setIpAddress={setIpAddress} port={port} setPort={setPort} connecting={connecting} onSubmit={handleConnectWireless} />}
        </div>

        <div className="p-4 bg-slate-50 border-t border-slate-100 flex justify-end">
          <button type="button" onClick={onClose} className="px-6 py-2.5 bg-white border border-slate-200 text-slate-700 rounded-xl text-xs font-black uppercase tracking-wider hover:bg-slate-100 transition-all">
            Tutup
          </button>
        </div>
      </div>
    </div>
  );
}
