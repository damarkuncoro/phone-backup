import { useState } from 'react';
import { X, HelpCircle, Smartphone, AlertTriangle, Apple } from 'lucide-react';
import { safeInvoke } from '../../../shared/lib/ipc';

interface ConnectionGuideModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export function ConnectionGuideModal({ isOpen, onClose }: ConnectionGuideModalProps) {
  const [selectedBrand, setSelectedBrand] = useState<'xiaomi' | 'vivo' | 'samsung' | 'oppo' | 'macos'>('vivo');
  const [isResolvingMtp, setIsResolvingMtp] = useState(false);
  const [mtpMessage, setMtpMessage] = useState<string | null>(null);

  if (!isOpen) return null;

  const handleResolveConflicts = async () => {
    setIsResolvingMtp(true);
    try {
      const count = await safeInvoke<number>('resolve_mtp_conflicts');
      setMtpMessage(`Berhasil membebaskan ${count} proses USB pengunci! Silakan cabut-colok kembali kabel USB.`);
    } catch (e: any) {
      setMtpMessage(`Info: ${e}`);
    } finally {
      setIsResolvingMtp(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-950/70 backdrop-blur-sm animate-in fade-in duration-200">
      <div className="bg-white rounded-[32px] max-w-2xl w-full p-6 md:p-8 shadow-2xl border border-slate-100 flex flex-col max-h-[90vh]">
        
        {/* Header */}
        <div className="flex items-center justify-between pb-4 border-b border-slate-100">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 bg-indigo-50 rounded-2xl flex items-center justify-center text-indigo-600">
              <HelpCircle className="w-5 h-5" />
            </div>
            <div>
              <h2 className="text-lg font-black text-slate-900 tracking-tight">Panduan Koneksi & Debugging</h2>
              <p className="text-xs text-slate-400">Petunjuk menghubungkan ponsel Android ke komputer</p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="w-8 h-8 rounded-full bg-slate-100 hover:bg-slate-200 flex items-center justify-center text-slate-500 transition-colors"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* Brand Selector Tabs */}
        <div className="flex gap-2 overflow-x-auto py-4 border-b border-slate-100">
          {[
            { id: 'vivo', label: 'Vivo / Funtouch' },
            { id: 'xiaomi', label: 'Xiaomi / HyperOS' },
            { id: 'samsung', label: 'Samsung OneUI' },
            { id: 'oppo', label: 'Oppo / Realme' },
            { id: 'macos', label: '🍎 macOS USB Fix' },
          ].map((tab) => (
            <button
              key={tab.id}
              onClick={() => setSelectedBrand(tab.id as any)}
              className={`px-3 py-1.5 rounded-xl text-xs font-black shrink-0 transition-all ${
                selectedBrand === tab.id
                  ? 'bg-indigo-600 text-white shadow-md shadow-indigo-600/20'
                  : 'bg-slate-50 text-slate-600 hover:bg-slate-100'
              }`}
            >
              {tab.label}
            </button>
          ))}
        </div>

        {/* Tab Content */}
        <div className="py-5 overflow-y-auto space-y-4 text-xs text-slate-600 flex-1">
          {selectedBrand === 'vivo' && (
            <div className="space-y-3">
              <p className="font-bold text-slate-800 flex items-center gap-2">
                <Smartphone className="w-4 h-4 text-indigo-500" /> Langkah Khusus Vivo / Funtouch OS:
              </p>
              <ol className="list-decimal pl-5 space-y-2 leading-relaxed">
                <li>Buka <b>Pengaturan</b> ➔ <b>Tentang Ponsel</b> ➔ <b>Informasi Perangkat Lunak</b>.</li>
                <li>Ketuk <b>Nomor Bentukan (Build Number)</b> sebanyak <b>7 kali</b> hingga muncul notifikasi Developer.</li>
                <li>Kembali ke <b>Pengaturan</b> ➔ <b>Sistem</b> ➔ <b>Opsi Pengembang</b>.</li>
                <li>Nyalakan <b>USB Debugging</b>.</li>
                <li>Saat kabel dicolokkan ke laptop, pilih mode <b>Transfer File / MTP</b> (jangan Hanya Mengisi Daya).</li>
                <li>Centang <i>"Selalu izinkan dari komputer ini"</i> pada kotak dialog RSA di layar HP.</li>
              </ol>
            </div>
          )}

          {selectedBrand === 'xiaomi' && (
            <div className="space-y-3">
              <p className="font-bold text-slate-800 flex items-center gap-2">
                <Smartphone className="w-4 h-4 text-amber-500" /> Langkah Khusus Xiaomi / HyperOS / MIUI:
              </p>
              <ol className="list-decimal pl-5 space-y-2 leading-relaxed">
                <li>Buka <b>Pengaturan</b> ➔ <b>Tentang Telepon</b> ➔ Ketuk <b>Versi OS / Versi MIUI 7 kali</b>.</li>
                <li>Buka <b>Pengaturan Tambahan</b> ➔ <b>Opsi Pengembang</b>.</li>
                <li>Aktifkan <b>Debugging USB</b>.</li>
                <li>Aktifkan juga <b>Install via USB</b> dan <b>Debugging USB (Setelan Keamanan)</b> jika diminta.</li>
                <li>Saat popup otorisasi muncul, pilih <b>Izinkan</b>.</li>
              </ol>
            </div>
          )}

          {selectedBrand === 'samsung' && (
            <div className="space-y-3">
              <p className="font-bold text-slate-800 flex items-center gap-2">
                <Smartphone className="w-4 h-4 text-blue-500" /> Langkah Khusus Samsung One UI:
              </p>
              <ol className="list-decimal pl-5 space-y-2 leading-relaxed">
                <li>Buka <b>Pengaturan</b> ➔ <b>Tentang Ponsel</b> ➔ <b>Informasi Perangkat Lunak</b>.</li>
                <li>Ketuk <b>Nomor Versi (Build Number) 7 kali</b>. Masukkan PIN jika diminta.</li>
                <li>Kembali ke menu Pengaturan paling bawah ➔ <b>Pilihan Pengembang</b>.</li>
                <li>Nyalakan <b>Debugging USB</b> dan konfirmasi <b>OK</b>.</li>
              </ol>
            </div>
          )}

          {selectedBrand === 'oppo' && (
            <div className="space-y-3">
              <p className="font-bold text-slate-800 flex items-center gap-2">
                <Smartphone className="w-4 h-4 text-emerald-500" /> Langkah Khusus Oppo / Realme / ColorOS:
              </p>
              <ol className="list-decimal pl-5 space-y-2 leading-relaxed">
                <li>Buka <b>Pengaturan</b> ➔ <b>Tentang Perangkat</b> ➔ <b>Versi</b>.</li>
                <li>Ketuk <b>Nomor Kompilasi 7 kali</b>.</li>
                <li>Buka <b>Pengaturan Tambahan</b> ➔ <b>Opsi Pengembang</b>.</li>
                <li>Aktifkan <b>Debugging USB</b>.</li>
              </ol>
            </div>
          )}

          {selectedBrand === 'macos' && (
            <div className="space-y-4">
              <div className="p-4 bg-amber-50 rounded-2xl border border-amber-200/80 flex items-start gap-3">
                <AlertTriangle className="w-5 h-5 text-amber-600 shrink-0 mt-0.5" />
                <div className="text-amber-800">
                  <p className="font-bold">Konflik USB Driver di macOS</p>
                  <p className="mt-1">
                    macOS secara default menjalankan daemon kamera sistem (<code className="bg-amber-100 px-1 py-0.5 rounded text-[10px]">ptpcamerad</code>) yang mengunci port USB eksklusif, mencegah koneksi MTP.
                  </p>
                </div>
              </div>
              
              <div className="flex flex-col gap-2">
                <button
                  type="button"
                  onClick={handleResolveConflicts}
                  disabled={isResolvingMtp}
                  className="w-full py-3 bg-indigo-600 hover:bg-indigo-500 text-white font-black rounded-2xl shadow-lg shadow-indigo-600/20 flex items-center justify-center gap-2 transition-all disabled:opacity-50"
                >
                  <Apple className="w-4 h-4" />
                  {isResolvingMtp ? 'Membebaskan Port USB...' : 'Auto-Fix Konflik macOS USB (Bebaskan Port)'}
                </button>
                {mtpMessage && (
                  <p className="p-3 bg-emerald-50 border border-emerald-200 text-emerald-700 font-bold rounded-xl text-center">
                    {mtpMessage}
                  </p>
                )}
              </div>
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="pt-4 border-t border-slate-100 flex justify-end">
          <button
            onClick={onClose}
            className="px-6 py-2.5 bg-slate-900 hover:bg-slate-800 text-white font-black rounded-xl text-xs transition-colors"
          >
            Mengerti, Tutup Panduan
          </button>
        </div>
      </div>
    </div>
  );
}
