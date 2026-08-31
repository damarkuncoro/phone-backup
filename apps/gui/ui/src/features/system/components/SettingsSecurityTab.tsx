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
      </div>
    </div>
  );
}
