import { useState, useEffect } from 'react';
import { RefreshCw, Smartphone, ShieldCheck, QrCode, Copy, Check, Link } from 'lucide-react';
import { safeInvoke } from '../../../shared/lib/ipc';
import { UI_TOKENS } from '../../../shared/theme/tokens';

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
  const [copiedKey, setCopiedKey] = useState<string | null>(null);

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

  const copyToClipboard = (text: string, key: string) => {
    navigator.clipboard.writeText(text);
    setCopiedKey(key);
    setTimeout(() => setCopiedKey(null), 2000);
  };

  return (
    <div className={UI_TOKENS.layout.pageContainer}>
      {/* Hero Header Banner */}
      <div className={UI_TOKENS.card.heroBannerDark}>
        <div className="relative z-10 min-w-0">
          <span className="text-[10px] font-black uppercase tracking-widest text-sky-400 bg-sky-950/80 px-3 py-1 rounded-full border border-sky-800/50">
            Wireless Agent Gateway
          </span>
          <h1 className="text-2xl md:text-3xl font-black tracking-tight mt-2 truncate">
            Wireless Agent Pairing
          </h1>
          <p className="text-xs text-slate-300 font-medium mt-1 truncate">
            Hubungkan smartphone Android secara nirkabel via Wi-Fi lokal tanpa kabel USB.
          </p>
        </div>

        <button
          onClick={fetchPairingInfo}
          disabled={loading}
          className="relative z-10 flex items-center gap-2 px-5 py-2.5 bg-white/10 hover:bg-white/20 active:scale-95 text-white rounded-2xl text-xs font-black uppercase tracking-wider border border-white/10 transition-all backdrop-blur-md"
        >
          <RefreshCw className={`w-3.5 h-3.5 ${loading ? 'animate-spin text-sky-400' : ''}`} />
          <span>Refresh Token</span>
        </button>

        <div className="absolute -right-10 -bottom-10 w-64 h-64 bg-sky-600/20 rounded-full blur-3xl pointer-events-none" />
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Left Card: QR & Connection Code */}
        <div className="bg-slate-900/60 border border-white/10 rounded-[32px] p-7 flex flex-col items-center text-center space-y-5 shadow-xl backdrop-blur-xl">
          <div className="p-5 bg-white rounded-[24px] shadow-inner flex items-center justify-center">
            <div className="w-48 h-48 bg-slate-950 flex flex-col items-center justify-center rounded-2xl text-sky-400 p-2 border-2 border-sky-500/30">
              <QrCode className="w-20 h-20 mb-2 animate-pulse text-sky-400" />
              <span className="text-[10px] font-mono text-slate-300 break-all px-2 leading-tight">
                {pairingInfo ? `PORT: ${pairingInfo.port}` : 'CONNECTING...'}
              </span>
              <span className="text-[9px] font-mono text-emerald-400 mt-1 font-bold">
                TOKEN: {pairingInfo?.pairing_token?.substring(0, 8)}...
              </span>
            </div>
          </div>

          <div className="w-full space-y-3 text-left pt-2">
            <div className="bg-slate-950/80 p-4 rounded-2xl border border-white/5 flex items-center justify-between">
              <div>
                <div className="text-[10px] text-slate-400 font-black uppercase tracking-widest">Local Server Address</div>
                <div className="font-mono text-sky-300 text-xs font-bold mt-0.5">
                  http://{pairingInfo?.ip_address || '127.0.0.1'}:{pairingInfo?.port || 3030}
                </div>
              </div>
              <button
                onClick={() => copyToClipboard(`http://${pairingInfo?.ip_address}:${pairingInfo?.port}`, 'address')}
                className="p-2 bg-slate-800 hover:bg-slate-700 rounded-xl text-slate-300 transition active:scale-95"
                title="Copy Address"
              >
                {copiedKey === 'address' ? <Check className="w-4 h-4 text-emerald-400" /> : <Copy className="w-4 h-4" />}
              </button>
            </div>

            <div className="bg-slate-950/80 p-4 rounded-2xl border border-white/5 flex items-center justify-between">
              <div className="min-w-0 flex-1 pr-2">
                <div className="text-[10px] text-slate-400 font-black uppercase tracking-widest">Pairing URI Payload</div>
                <div className="font-mono text-emerald-400 text-xs truncate mt-0.5 font-bold">
                  {pairingInfo?.qr_payload || 'GENERATING...'}
                </div>
              </div>
              <button
                onClick={() => pairingInfo && copyToClipboard(pairingInfo.qr_payload, 'payload')}
                className="p-2 bg-slate-800 hover:bg-slate-700 rounded-xl text-slate-300 transition shrink-0 active:scale-95"
                title="Copy URI"
              >
                {copiedKey === 'payload' ? <Check className="w-4 h-4 text-emerald-400" /> : <Link className="w-4 h-4" />}
              </button>
            </div>
          </div>
        </div>

        {/* Right Card: Instructions & Status */}
        <div className="space-y-6 flex flex-col justify-between">
          <div className="bg-slate-900/60 border border-white/10 rounded-[32px] p-7 space-y-4 backdrop-blur-xl">
            <h3 className="font-black text-sm uppercase tracking-wide text-slate-200 flex items-center gap-2">
              <Smartphone className="w-4 h-4 text-sky-400" />
              Langkah-langkah Pairing Nirkabel:
            </h3>
            <ol className="space-y-3.5 text-xs text-slate-300 list-decimal list-inside leading-relaxed font-medium">
              <li>
                Pastikan ponsel dan komputer terhubung pada <strong className="text-white">jaringan Wi-Fi yang sama</strong>.
              </li>
              <li>
                Buka aplikasi <strong className="text-white">Phone Backup Companion</strong> di smartphone Android.
              </li>
              <li>
                Pilih menu <strong className="text-white">Scan QR / Masukkan URI</strong> dan arahkan kamera ke kode di samping.
              </li>
              <li>
                Setelah terhubung, Anda dapat langsung memulai backup nirkabel berkecepatan tinggi!
              </li>
            </ol>
          </div>

          <div className="bg-emerald-950/40 border border-emerald-500/30 rounded-[28px] p-6 flex items-start gap-3.5 backdrop-blur-md">
            <ShieldCheck className="w-5 h-5 text-emerald-400 shrink-0 mt-0.5" />
            <div>
              <div className="font-black text-emerald-300 text-xs uppercase tracking-wider">Server Socket.IO Aktif</div>
              <p className="text-xs text-emerald-200/80 mt-1">
                {pairingInfo?.server_status || 'Listening on 0.0.0.0:3030'} dengan enkripsi session token aman.
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
