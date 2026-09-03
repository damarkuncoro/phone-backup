import { useState, useEffect } from 'react';
import { Wifi, RefreshCw, Smartphone, ShieldCheck, QrCode } from 'lucide-react';
import { safeInvoke } from '../../../shared/lib/ipc';

interface WirelessPairingInfo {
  ip_address: string;
  port: number;
  pairing_token: string;
  qr_payload: string;
  server_status: string;
}

export function WirelessPairingPage() {
  const [pairingInfo, setPairingInfo] = useState<WirelessPairingInfo | null>(null);
  const [loading, setLoading] = useState(true);

  const fetchPairingInfo = async () => {
    setLoading(true);
    try {
      const info = await safeInvoke<WirelessPairingInfo>('get_wireless_pairing_info');
      setPairingInfo(info);
    } catch (err) {
      console.error('Failed to get pairing info:', err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchPairingInfo();
  }, []);

  return (
    <div className="p-8 max-w-5xl mx-auto space-y-6 text-slate-100">
      <div className="flex items-center justify-between pb-6 border-b border-white/10">
        <div>
          <h1 className="text-2xl font-bold flex items-center gap-2">
            <Wifi className="w-7 h-7 text-sky-400" />
            Wireless Companion Agent Pairing
          </h1>
          <p className="text-sm text-slate-400 mt-1">
            Hubungkan smartphone Android secara nirkabel via Wi-Fi tanpa kabel USB
          </p>
        </div>
        <button
          onClick={fetchPairingInfo}
          disabled={loading}
          className="flex items-center gap-2 px-4 py-2 bg-sky-600 hover:bg-sky-500 rounded-lg text-sm font-medium transition"
        >
          <RefreshCw className={`w-4 h-4 ${loading ? 'animate-spin' : ''}`} />
          Refresh Token
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Left Card: QR & Connection Code */}
        <div className="bg-slate-900/60 border border-white/10 rounded-2xl p-6 flex flex-col items-center text-center space-y-4 shadow-xl backdrop-blur-md">
          <div className="p-4 bg-white rounded-2xl shadow-inner flex items-center justify-center">
            <div className="w-48 h-48 bg-slate-950 flex flex-col items-center justify-center rounded-xl text-sky-400 p-2 border-2 border-sky-500/30">
              <QrCode className="w-24 h-24 mb-2 animate-pulse text-sky-400" />
              <span className="text-[10px] font-mono text-slate-300 break-all px-2 leading-tight">
                {pairingInfo ? `PORT:${pairingInfo.port}` : 'CONNECTING...'}
              </span>
            </div>
          </div>

          <div className="w-full space-y-2 text-left pt-2">
            <div className="bg-slate-950/80 p-3 rounded-lg border border-white/5">
              <div className="text-xs text-slate-400 font-medium">Local Server IP</div>
              <div className="font-mono text-sky-300 text-sm font-semibold">
                {pairingInfo?.ip_address || '127.0.0.1'} : {pairingInfo?.port || 3030}
              </div>
            </div>
            <div className="bg-slate-950/80 p-3 rounded-lg border border-white/5">
              <div className="text-xs text-slate-400 font-medium">One-Time Pairing Token</div>
              <div className="font-mono text-emerald-400 text-xs truncate">
                {pairingInfo?.pairing_token || 'GENERATING...'}
              </div>
            </div>
          </div>
        </div>

        {/* Right Card: Instructions & Status */}
        <div className="space-y-6 flex flex-col justify-between">
          <div className="bg-slate-900/60 border border-white/10 rounded-2xl p-6 space-y-4">
            <h3 className="font-semibold text-base text-slate-200 flex items-center gap-2">
              <Smartphone className="w-5 h-5 text-sky-400" />
              Langkah-langkah Pairing Nirkabel:
            </h3>
            <ol className="space-y-3 text-sm text-slate-300 list-decimal list-inside leading-relaxed">
              <li>
                Pastikan HP dan Komputer terhubung pada <strong>jaringan Wi-Fi yang sama</strong>.
              </li>
              <li>
                Buka aplikasi <strong>Phone Backup Agent</strong> di smartphone Android.
              </li>
              <li>
                Pilih menu <strong>Scan QR Code</strong> dan arahkan kamera ke kode di samping.
              </li>
              <li>
                Setelah terhubung, Anda dapat langsung memulai backup nirkabel berkecepatan tinggi!
              </li>
            </ol>
          </div>

          <div className="bg-emerald-950/40 border border-emerald-500/30 rounded-2xl p-5 flex items-start gap-3">
            <ShieldCheck className="w-6 h-6 text-emerald-400 shrink-0 mt-0.5" />
            <div>
              <div className="font-semibold text-emerald-300 text-sm">Server Socket.IO Aktif</div>
              <p className="text-xs text-emerald-200/80 mt-0.5">
                {pairingInfo?.server_status || 'Listening on 0.0.0.0:3030'} dengan enkripsi session token aman.
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
