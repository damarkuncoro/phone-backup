import { Shield, Lock, Copy, Check, Key } from 'lucide-react';
import { systemService } from '@/services/systemService';

interface SettingsSecurityTabProps {
  keys: [string, string] | null;
  copiedKey: boolean;
  onCopyPublicKey: () => void;
}

export function SettingsSecurityTab({
  keys,
  copiedKey,
  onCopyPublicKey
}: SettingsSecurityTabProps) {
  return (
    <div className="space-y-6 animate-in fade-in duration-200">
      <div className="bg-white p-6 md:p-8 rounded-[32px] border border-slate-100 shadow-sm space-y-6">
        <div className="flex justify-between items-center">
          <div>
            <h3 className="text-base font-black text-slate-900 tracking-tight flex items-center gap-2">
              <Shield className="w-5 h-5 text-emerald-600" /> Keamanan & Kunci Kriptografi
            </h3>
            <p className="text-xs text-slate-400 font-medium mt-0.5">
              Standar enkripsi modern Age (X25519) dengan perlindungan Chacha20-Poly1305.
            </p>
          </div>
          <div className="flex items-center gap-2 px-3 py-1.5 bg-emerald-50 text-emerald-700 border border-emerald-200 rounded-xl text-[10px] font-black uppercase tracking-wider">
            <Lock className="w-3.5 h-3.5" /> Enkripsi Otomatis Aktif
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Public Key Display */}
          <div className="p-6 bg-slate-50 border border-slate-200/80 rounded-3xl space-y-4">
            <div className="flex justify-between items-center">
              <span className="text-[10px] font-black uppercase tracking-widest text-slate-500">
                Active Public Key (Untuk Enkripsi)
              </span>
              <button
                type="button"
                onClick={onCopyPublicKey}
                className="flex items-center gap-1.5 px-3 py-1.5 bg-white border border-slate-200 text-slate-700 hover:text-indigo-600 rounded-lg text-xs font-bold transition-all shadow-sm active:scale-95"
              >
                {copiedKey ? <Check className="w-3.5 h-3.5 text-emerald-600" /> : <Copy className="w-3.5 h-3.5" />}
                {copiedKey ? "Tersalin!" : "Salin Kunci"}
              </button>
            </div>

            <div className="bg-white p-4 rounded-2xl border border-slate-200 shadow-inner">
              <code className="text-xs font-mono text-indigo-950 break-all leading-relaxed select-all">
                {keys ? keys[1] : 'Memuat Public Key...'}
              </code>
            </div>
            <p className="text-[10px] text-slate-500 font-medium leading-relaxed">
              Kunci publik ini aman untuk dibagikan. Kunci ini digunakan oleh aplikasi untuk menyegel data cadangan Anda sebelum disimpan ke penyimpanan.
            </p>
          </div>

          {/* Keypair Rotation Card */}
          <div className="p-6 bg-amber-50/70 border border-amber-200/80 rounded-3xl space-y-4 flex flex-col justify-between">
            <div className="space-y-2">
              <h4 className="text-xs font-black uppercase tracking-wider text-amber-900 flex items-center gap-2">
                <Key className="w-4 h-4 text-amber-600" /> Rotasi Kunci & Kunci Privat
              </h4>
              <p className="text-xs text-amber-900/80 leading-relaxed font-medium">
                Kunci privat Anda disimpan di brankas aman lokal. Jika Anda ingin mengganti kunci enkripsi untuk backup berikutnya, Anda dapat membuat pasangan kunci baru.
              </p>
            </div>

            <div className="pt-2">
              <button
                type="button"
                onClick={() => {
                  systemService.generateKeys().then(() => {
                    alert("Pasangan kunci baru berhasil digenerate!");
                    window.location.reload();
                  });
                }}
                className="w-full py-3 bg-slate-900 hover:bg-slate-800 text-white rounded-2xl font-black text-xs uppercase tracking-wider transition-all shadow-lg flex items-center justify-center gap-2 active:scale-95"
              >
                <Key className="w-4 h-4" /> Generate Pasangan Kunci Baru
              </button>
            </div>
          </div>
        </div>

        {/* Zero-Knowledge Emergency Recovery Kit Card */}
        <div className="p-6 bg-gradient-to-r from-indigo-900 to-slate-900 text-white rounded-3xl flex flex-col md:flex-row items-start md:items-center justify-between gap-6 shadow-xl">
          <div className="space-y-1.5 max-w-xl">
            <span className="text-[10px] font-black uppercase tracking-widest text-indigo-300 bg-indigo-950/80 px-3 py-1 rounded-full border border-indigo-700/50">
              Cold Storage Protection
            </span>
            <h4 className="text-lg font-black tracking-tight text-white mt-1">
              Lembar Pemulihan Kunci Darurat (Emergency Recovery Kit)
            </h4>
            <p className="text-xs text-slate-300 font-medium leading-relaxed">
              Cetak salinan fisik kunci rahasia enkripsi Age X25519 dan petunjuk pemulihan ke kertas untuk disimpan di brankas fisik. Lembar ini adalah jaminan Anda bisa memulihkan data jika komputer rusak.
            </p>
          </div>

          <button
            type="button"
            onClick={() => {
              if (!keys) {
                alert("Memuat kunci enkripsi...");
                return;
              }
              const printWindow = window.open('', '_blank');
              if (printWindow) {
                const dateStr = new Date().toISOString();
                printWindow.document.write(`
                  <!DOCTYPE html>
                  <html>
                  <head>
                    <title>Phone Backup - Emergency Recovery Kit</title>
                    <style>
                      body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; padding: 40px; color: #0f172a; line-height: 1.5; }
                      .badge { display: inline-block; padding: 4px 12px; background: #e0e7ff; color: #4338ca; border-radius: 9999px; font-weight: 800; font-size: 11px; text-transform: uppercase; }
                      h1 { margin: 12px 0 6px 0; font-size: 24px; font-weight: 900; }
                      .box { background: #f8fafc; border: 1.5px solid #cbd5e1; border-radius: 12px; padding: 16px; margin: 20px 0; }
                      .label { font-size: 11px; font-weight: 800; text-transform: uppercase; color: #64748b; margin-bottom: 6px; }
                      .key { font-family: monospace; font-size: 13px; font-weight: 700; word-break: break-all; background: #fff; padding: 10px; border: 1px solid #e2e8f0; border-radius: 6px; }
                      .warning { background: #fff1f2; border: 1.5px solid #fecdd3; border-radius: 12px; padding: 16px; margin: 20px 0; color: #9f1239; }
                    </style>
                  </head>
                  <body>
                    <span class="badge">Cold Storage Recovery Document</span>
                    <h1>Phone Backup Emergency Recovery Kit</h1>
                    <p style="color: #64748b; font-size: 13px;">Dicetak pada: ${dateStr}</p>
                    <div class="warning">
                      <h3 style="margin: 0 0 4px 0;">⚠️ SANGAT RAHASIA - SIMPAN DI BRANKAS</h3>
                      <p style="margin: 0; font-size: 12px;">Kunci Rahasia ini diperlukan untuk mendekripsi seluruh cadangan Anda.</p>
                    </div>
                    <div class="box">
                      <div class="label">Public Key (Identitas Enkripsi)</div>
                      <div class="key">${keys[1]}</div>
                    </div>
                    <div class="box">
                      <div class="label">Secret Key (Kunci Privat Pemulihan)</div>
                      <div class="key">${keys[0]}</div>
                    </div>
                  </body>
                  </html>
                `);
                printWindow.document.close();
                printWindow.focus();
                printWindow.print();
              }
            }}
            className="px-6 py-3.5 bg-white hover:bg-slate-100 text-slate-900 rounded-2xl font-black text-xs uppercase tracking-wider transition-all shadow-lg shrink-0 flex items-center gap-2 active:scale-95"
          >
            <Shield className="w-4 h-4 text-indigo-600" /> Cetak Lembar Pemulihan
          </button>
        </div>
      </div>
    </div>
  );
}
