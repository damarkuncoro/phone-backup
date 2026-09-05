import { useState, useEffect, useCallback } from 'react';
import { RefreshCcw, QrCode, MessageSquare, Download, CheckCircle2, AlertCircle } from 'lucide-react';
import { safeInvoke } from '../../../shared/lib/ipc';

interface SyncStatus {
  has_synced_data: boolean;
  total_chats: number;
  total_messages: number;
  synced_at?: string;
  has_qr_file: boolean;
}

export function WhatsAppLiveSyncTab() {
  const [status, setStatus] = useState<SyncStatus | null>(null);
  const [htmlContent, setHtmlContent] = useState<string | null>(null);
  const [qrContent, setQrContent] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchStatusAndData = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const syncStatus = await safeInvoke<SyncStatus>('get_whatsapp_sync_status');
      setStatus(syncStatus);

      if (syncStatus.has_synced_data) {
        const html = await safeInvoke<string>('get_synced_whatsapp_html');
        setHtmlContent(html);
      } else if (syncStatus.has_qr_file) {
        const qrHtml = await safeInvoke<string>('get_whatsapp_qr_html');
        setQrContent(qrHtml);
      }
    } catch (e: any) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchStatusAndData();
  }, [fetchStatusAndData]);

  const handleDownload = () => {
    if (!htmlContent) return;
    const blob = new Blob([htmlContent], { type: 'text/html' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'synced_whatsapp_viewer.html';
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="space-y-6">
      {/* Top Banner Status */}
      <div className="bg-slate-900/80 border border-white/10 rounded-2xl p-6 flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
        <div>
          <div className="flex items-center gap-3">
            <h2 className="text-lg font-black text-white">Live Multi-Device WhatsApp Sync</h2>
            {status?.has_synced_data ? (
              <span className="px-2.5 py-0.5 rounded-full text-[11px] font-black bg-emerald-500/20 text-emerald-400 border border-emerald-500/30">
                TERHUBUNG & TERSINKRON
              </span>
            ) : status?.has_qr_file ? (
              <span className="px-2.5 py-0.5 rounded-full text-[11px] font-black bg-amber-500/20 text-amber-400 border border-amber-500/30">
                MENUNGGU SCAN QR
              </span>
            ) : (
              <span className="px-2.5 py-0.5 rounded-full text-[11px] font-black bg-slate-800 text-slate-400">
                SIAP DISINKRONISASIKAN
              </span>
            )}
          </div>
          <p className="text-xs text-slate-400 mt-1">
            {status?.has_synced_data
              ? `Tersinkronisasi ${status.total_chats} percakapan (${status.total_messages} pesan) via WhatsApp Web Protocol.`
              : 'Sinkronisasi riwayat obrolan lengkap secara lokal dengan aman tanpa root.'}
          </p>
        </div>

        <div className="flex items-center gap-2">
          <button
            onClick={fetchStatusAndData}
            disabled={loading}
            className="px-4 py-2 bg-slate-800 hover:bg-slate-700 text-white rounded-xl text-xs font-black flex items-center gap-2 border border-white/10 transition-all"
          >
            <RefreshCcw className={`w-3.5 h-3.5 ${loading ? 'animate-spin' : ''}`} />
            Refresh
          </button>
          {htmlContent && (
            <button
              onClick={handleDownload}
              className="px-4 py-2 bg-emerald-600 hover:bg-emerald-500 text-white rounded-xl text-xs font-black shadow-lg shadow-emerald-600/20 flex items-center gap-2 transition-all"
            >
              <Download className="w-3.5 h-3.5" />
              Download HTML
            </button>
          )}
        </div>
      </div>

      {error && (
        <div className="p-4 bg-red-950/40 border border-red-500/30 rounded-2xl flex items-center gap-3 text-red-400 text-xs">
          <AlertCircle className="w-5 h-5 flex-shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {/* QR Code Frame if pending */}
      {!status?.has_synced_data && qrContent && (
        <div className="bg-slate-900 border border-amber-500/30 rounded-2xl overflow-hidden shadow-2xl">
          <div className="bg-slate-950 px-4 py-2.5 border-b border-white/10 flex items-center justify-between text-xs text-amber-400">
            <span className="font-bold flex items-center gap-2">
              <QrCode className="w-4 h-4" /> Scan QR Code di WhatsApp HP
            </span>
            <span className="text-[10px] bg-amber-500/20 px-2 py-0.5 rounded-full font-black">SCAN DENGAN HP</span>
          </div>
          <iframe
            srcDoc={qrContent}
            title="WhatsApp QR Pairing"
            className="w-full h-[480px] bg-slate-950 border-none"
          />
        </div>
      )}

      {/* Synced HTML Chat Viewer */}
      {htmlContent && (
        <div className="bg-slate-900 border border-white/10 rounded-2xl overflow-hidden shadow-2xl">
          <div className="bg-slate-950 px-4 py-2.5 border-b border-white/10 flex items-center justify-between text-xs text-slate-400">
            <span className="font-mono flex items-center gap-2 text-emerald-400 font-bold">
              <CheckCircle2 className="w-4 h-4" /> WhatsApp Live Archive Viewer
            </span>
            <span className="text-[10px] bg-emerald-500/20 text-emerald-300 px-2 py-0.5 rounded-full font-bold">
              {status?.total_messages} PESAN TERARSIPKAN
            </span>
          </div>
          <iframe
            srcDoc={htmlContent}
            title="Synced WhatsApp Viewer"
            className="w-full h-[580px] bg-slate-950 border-none"
          />
        </div>
      )}

      {!htmlContent && !qrContent && !loading && (
        <div className="p-12 text-center bg-slate-900/40 border border-dashed border-white/10 rounded-2xl space-y-3">
          <MessageSquare className="w-10 h-10 text-slate-600 mx-auto" />
          <p className="text-sm font-bold text-slate-300">Belum Ada Sesi WhatsApp Aktif</p>
          <p className="text-xs text-slate-500 max-w-md mx-auto">
            Jalankan modul sinkronisasi WhatsApp atau klik tombol refresh untuk memuat riwayat obrolan yang tersimpan.
          </p>
        </div>
      )}
    </div>
  );
}
