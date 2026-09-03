import { useState } from 'react';
import { MessageSquare, Download, CheckCircle, Smartphone, FolderTree } from 'lucide-react';
import { safeInvoke } from '../../../shared/lib/ipc';

export function WhatsAppArchivePage() {
  const [exportFormat, setExportFormat] = useState<'html' | 'json'>('html');
  const [isExporting, setIsExporting] = useState(false);
  const [exportSuccess, setExportSuccess] = useState<string | null>(null);
  const [previewHtml, setPreviewHtml] = useState<string | null>(null);

  const handleGeneratePreview = async () => {
    setIsExporting(true);
    setExportSuccess(null);
    try {
      const html = await safeInvoke<string>('generate_whatsapp_archive_preview');
      setPreviewHtml(html);
      setExportSuccess('WhatsApp archive preview generated successfully!');
    } catch (e: any) {
      alert(`Export error: ${e}`);
    } finally {
      setIsExporting(false);
    }
  };

  const handleDownloadOffline = () => {
    if (!previewHtml) return;
    const blob = new Blob([previewHtml], { type: 'text/html' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'whatsapp_archive.html';
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="p-8 max-w-6xl mx-auto space-y-6 text-slate-100">
      <div className="flex items-center justify-between pb-6 border-b border-white/10">
        <div>
          <h1 className="text-2xl font-black tracking-tight flex items-center gap-3">
            <MessageSquare className="w-7 h-7 text-emerald-400" />
            WhatsApp Archive & Media Explorer
          </h1>
          <p className="text-sm text-slate-400 mt-1">
            Ekspor arsip chat WhatsApp & media (Scoped Storage Android 11–15 & Legacy) ke format interaktif offline.
          </p>
        </div>
        <div className="flex items-center gap-3">
          <button
            onClick={handleGeneratePreview}
            disabled={isExporting}
            className="flex items-center gap-2 bg-emerald-600 hover:bg-emerald-500 disabled:opacity-50 text-white font-bold px-4 py-2 rounded-xl text-sm shadow-lg shadow-emerald-600/20 transition-all"
          >
            <MessageSquare className="w-4 h-4" />
            {isExporting ? 'Memproses...' : 'Generate Archive Preview'}
          </button>
          {previewHtml && (
            <button
              onClick={handleDownloadOffline}
              className="flex items-center gap-2 bg-indigo-600 hover:bg-indigo-500 text-white font-bold px-4 py-2 rounded-xl text-sm shadow-lg shadow-indigo-600/20 transition-all"
            >
              <Download className="w-4 h-4" />
              Download Offline HTML
            </button>
          )}
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-slate-900/60 border border-white/10 rounded-2xl p-5 space-y-4">
          <div className="flex items-center gap-2 text-emerald-400 font-bold text-sm">
            <FolderTree className="w-4 h-4" />
            <span>Path Discovery Matrix</span>
          </div>
          <p className="text-xs text-slate-400">
            Jalur deteksi otomatis untuk WhatsApp Standar dan WhatsApp Business:
          </p>
          <ul className="text-xs space-y-2 font-mono text-slate-300 bg-slate-950 p-3 rounded-xl border border-white/5">
            <li className="text-emerald-400">✓ /Android/media/com.whatsapp/</li>
            <li className="text-emerald-400">✓ /Android/media/com.whatsapp.w4b/</li>
            <li className="text-slate-400">✓ /WhatsApp/ (Legacy Android &lt;11)</li>
          </ul>
        </div>

        <div className="bg-slate-900/60 border border-white/10 rounded-2xl p-5 space-y-4">
          <div className="flex items-center gap-2 text-indigo-400 font-bold text-sm">
            <Smartphone className="w-4 h-4" />
            <span>Format & Fitur Arsip</span>
          </div>
          <div className="flex gap-2">
            <button
              onClick={() => setExportFormat('html')}
              className={`flex-1 py-1.5 px-3 rounded-lg text-xs font-bold transition-all ${
                exportFormat === 'html' ? 'bg-indigo-600 text-white' : 'bg-slate-800 text-slate-400'
              }`}
            >
              HTML Standalone
            </button>
            <button
              onClick={() => setExportFormat('json')}
              className={`flex-1 py-1.5 px-3 rounded-lg text-xs font-bold transition-all ${
                exportFormat === 'json' ? 'bg-indigo-600 text-white' : 'bg-slate-800 text-slate-400'
              }`}
            >
              JSON Raw
            </button>
          </div>
          <p className="text-xs text-slate-400">
            File HTML yang diekspor berisi viewer chat mandiri (tanpa dependensi internet) dengan tema gelap.
          </p>
        </div>

        <div className="bg-slate-900/60 border border-white/10 rounded-2xl p-5 space-y-3 flex flex-col justify-center">
          {exportSuccess ? (
            <div className="flex items-center gap-3 text-emerald-400 bg-emerald-950/40 p-4 rounded-xl border border-emerald-500/20">
              <CheckCircle className="w-6 h-6 flex-shrink-0" />
              <p className="text-xs font-bold">{exportSuccess}</p>
            </div>
          ) : (
            <p className="text-xs text-slate-500 text-center italic">
              Klik "Generate Archive Preview" untuk memvisualisasikan viewer arsip WhatsApp.
            </p>
          )}
        </div>
      </div>

      {previewHtml && (
        <div className="bg-slate-900 border border-white/10 rounded-2xl overflow-hidden shadow-2xl">
          <div className="bg-slate-950 px-4 py-2.5 border-b border-white/10 flex items-center justify-between text-xs text-slate-400">
            <span className="font-mono">WhatsApp Archive Live Frame</span>
            <span className="text-[10px] bg-emerald-500/20 text-emerald-300 px-2 py-0.5 rounded-full font-bold">READY</span>
          </div>
          <iframe
            srcDoc={previewHtml}
            title="WhatsApp Archive Preview"
            className="w-full h-[520px] bg-slate-950 border-none"
          />
        </div>
      )}
    </div>
  );
}
