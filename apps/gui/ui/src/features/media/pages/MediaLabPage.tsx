import { useState } from 'react';
import { Music, Image as ImageIcon, Activity, AlertTriangle } from 'lucide-react';
import { safeInvoke } from '../../../shared/lib/ipc';
import { UI_TOKENS } from '../../../shared/theme/tokens';

interface AudioAnalysis {
  format: string;
  category: string;
  title: string | null;
  artist: string | null;
  waveform_points: number[];
}

export function MediaLabPage() {
  const [filePath, setFilePath] = useState('');
  const [isProcessing, setIsProcessing] = useState(false);
  const [audioResult, setAudioResult] = useState<AudioAnalysis | null>(null);
  const [sharpnessScore, setSharpnessScore] = useState<number | null>(null);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  const handleInspectAudio = async () => {
    if (!filePath.trim()) return;
    setIsProcessing(true);
    setErrorMsg(null);
    setAudioResult(null);
    setSharpnessScore(null);

    try {
      const res = await safeInvoke<AudioAnalysis>('analyze_audio_file', { path: filePath });
      setAudioResult(res);
    } catch (e: any) {
      setErrorMsg(typeof e === 'string' ? e : e?.message || 'Gagal memproses file audio');
    } finally {
      setIsProcessing(false);
    }
  };

  const handleCheckSharpness = async () => {
    if (!filePath.trim()) return;
    setIsProcessing(true);
    setErrorMsg(null);
    setAudioResult(null);
    setSharpnessScore(null);

    try {
      const score = await safeInvoke<number>('check_image_sharpness', { path: filePath });
      setSharpnessScore(score);
    } catch (e: any) {
      setErrorMsg(typeof e === 'string' ? e : e?.message || 'Gagal memeriksa ketajaman gambar');
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div className={UI_TOKENS.layout.pageContainer}>
      {/* Hero Header Banner */}
      <div className={UI_TOKENS.card.heroBannerDark}>
        <div className="relative z-10 min-w-0">
          <span className="text-[10px] font-black uppercase tracking-widest text-sky-400 bg-sky-950/80 px-3 py-1 rounded-full border border-sky-800/50">
            Signal & Image Processing
          </span>
          <h1 className="text-2xl md:text-3xl font-black tracking-tight mt-2 truncate">
            Media Lab (Audio & Vision)
          </h1>
          <p className="text-xs text-slate-300 font-medium mt-1 truncate">
            Analisis bentuk gelombang audio (*waveform envelope*), metadata ID3/Vorbis, dan deteksi keburaman foto.
          </p>
        </div>

        <div className="relative z-10 flex items-center gap-2 px-4 py-2 bg-white/10 backdrop-blur-md rounded-2xl border border-white/10 text-xs font-bold text-slate-200">
          <Activity className="w-4 h-4 text-sky-400" />
          <span>DSP Engine Online</span>
        </div>

        <div className="absolute -right-10 -bottom-10 w-64 h-64 bg-sky-600/20 rounded-full blur-3xl pointer-events-none" />
      </div>

      {/* Input Form Card */}
      <div className="bg-slate-900/60 border border-white/10 rounded-[28px] p-6 space-y-4 backdrop-blur-xl">
        <label className="text-[10px] font-black uppercase tracking-widest text-slate-400 block">
          Path berkas Media (Audio .mp3/.opus/.m4a atau Foto .jpg/.png)
        </label>
        <div className="flex flex-col sm:flex-row gap-3">
          <input
            type="text"
            value={filePath}
            onChange={(e) => setFilePath(e.target.value)}
            placeholder="/Users/username/Music/sample.mp3"
            className="flex-1 bg-slate-950/80 border border-white/10 text-white placeholder-slate-500 px-4 py-2.5 rounded-2xl text-xs font-mono outline-none focus:ring-4 focus:ring-sky-500/20 focus:border-sky-400/50 transition-all"
          />
          <div className="flex gap-2">
            <button
              onClick={handleInspectAudio}
              disabled={isProcessing || !filePath.trim()}
              className="flex items-center gap-2 bg-sky-600 hover:bg-sky-500 disabled:opacity-50 text-white font-black text-xs uppercase tracking-wider px-5 py-3 rounded-2xl shadow-lg shadow-sky-600/20 transition-all active:scale-95"
            >
              <Music className="w-4 h-4" />
              Audio Waveform
            </button>
            <button
              onClick={handleCheckSharpness}
              disabled={isProcessing || !filePath.trim()}
              className="flex items-center gap-2 bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 text-white font-black text-xs uppercase tracking-wider px-5 py-3 rounded-2xl shadow-lg shadow-indigo-600/20 transition-all active:scale-95"
            >
              <ImageIcon className="w-4 h-4" />
              Check Blur
            </button>
          </div>
        </div>
      </div>

      {errorMsg && (
        <div className="flex items-center gap-3 bg-rose-950/40 border border-rose-500/30 text-rose-300 p-5 rounded-[24px] text-xs">
          <AlertTriangle className="w-5 h-5 flex-shrink-0" />
          <span>{errorMsg}</span>
        </div>
      )}

      {audioResult && (
        <div className="bg-slate-900/60 border border-white/10 rounded-[32px] p-6 space-y-6 backdrop-blur-xl">
          <div className="flex items-center justify-between">
            <div>
              <span className="text-[10px] font-black text-sky-400 uppercase tracking-widest block">Format & Kategori</span>
              <h3 className="text-lg font-black text-white">{audioResult.format} — {audioResult.category}</h3>
            </div>
            {audioResult.title && (
              <div className="text-right">
                <span className="text-xs text-slate-400">{audioResult.artist || 'Unknown Artist'}</span>
                <p className="text-sm font-bold text-slate-200">{audioResult.title}</p>
              </div>
            )}
          </div>

          <div>
            <span className="text-[10px] font-black text-slate-400 uppercase tracking-widest block mb-3">
              Normalized Waveform Peaks ({audioResult.waveform_points.length} points)
            </span>
            <div className="bg-slate-950/90 p-5 rounded-2xl border border-white/5 flex items-end gap-1 h-36">
              {audioResult.waveform_points.map((pt, idx) => (
                <div
                  key={idx}
                  style={{ height: `${Math.max(pt, 6)}%` }}
                  className="flex-1 bg-gradient-to-t from-sky-600 to-indigo-400 rounded-t-sm transition-all hover:bg-sky-300"
                  title={`Sample ${idx}: ${pt}%`}
                />
              ))}
            </div>
          </div>
        </div>
      )}

      {sharpnessScore !== null && (
        <div className="bg-slate-900/60 border border-white/10 rounded-[28px] p-6 space-y-3 backdrop-blur-xl">
          <h3 className="text-[10px] font-black text-slate-400 uppercase tracking-widest">Skor Ketajaman Citra (Laplacian Variance)</h3>
          <div className="flex items-baseline gap-3">
            <span className="text-4xl font-black text-indigo-400">{sharpnessScore.toFixed(1)}</span>
            <span className="text-xs text-slate-400">
              {sharpnessScore > 100 ? '✅ Foto sangat tajam dan fokus.' : '⚠️ Foto memiliki tingkat keburaman tinggi (*blurry*).'}
            </span>
          </div>
        </div>
      )}
    </div>
  );
}
