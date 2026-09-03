import { useState } from 'react';
import { Music, Image as ImageIcon, Activity, AlertTriangle } from 'lucide-react';
import { safeInvoke } from '../../../shared/lib/ipc';

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
    <div className="p-8 max-w-6xl mx-auto space-y-6 text-slate-100">
      <div className="flex items-center justify-between pb-6 border-b border-white/10">
        <div>
          <h1 className="text-2xl font-black tracking-tight flex items-center gap-3">
            <Activity className="w-7 h-7 text-sky-400" />
            Media Lab (Audio Waveforms & Image Sharpness)
          </h1>
          <p className="text-sm text-slate-400 mt-1">
            Analisis bentuk gelombang audio (*waveform envelope*), metadata ID3/Vorbis, dan deteksi keburaman foto.
          </p>
        </div>
      </div>

      <div className="bg-slate-900/60 border border-white/10 rounded-2xl p-6 space-y-4">
        <label className="text-xs font-bold text-slate-300 uppercase tracking-wider block">
          Path berkas Media (Audio .mp3/.opus/.m4a atau Foto .jpg/.png)
        </label>
        <div className="flex flex-col sm:flex-row gap-3">
          <input
            type="text"
            value={filePath}
            onChange={(e) => setFilePath(e.target.value)}
            placeholder="/Users/username/Music/sample.mp3"
            className="flex-1 bg-slate-950 border border-white/10 text-white placeholder-slate-500 px-4 py-2.5 rounded-xl text-sm font-mono outline-none focus:ring-2 focus:ring-sky-500/50"
          />
          <div className="flex gap-2">
            <button
              onClick={handleInspectAudio}
              disabled={isProcessing || !filePath.trim()}
              className="flex items-center gap-2 bg-sky-600 hover:bg-sky-500 disabled:opacity-50 text-white font-bold px-4 py-2.5 rounded-xl text-sm shadow-lg shadow-sky-600/20 transition-all"
            >
              <Music className="w-4 h-4" />
              Audio Waveform
            </button>
            <button
              onClick={handleCheckSharpness}
              disabled={isProcessing || !filePath.trim()}
              className="flex items-center gap-2 bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 text-white font-bold px-4 py-2.5 rounded-xl text-sm shadow-lg shadow-indigo-600/20 transition-all"
            >
              <ImageIcon className="w-4 h-4" />
              Check Blur
            </button>
          </div>
        </div>
      </div>

      {errorMsg && (
        <div className="flex items-center gap-3 bg-rose-950/40 border border-rose-500/30 text-rose-300 p-4 rounded-xl text-xs">
          <AlertTriangle className="w-5 h-5 flex-shrink-0" />
          <span>{errorMsg}</span>
        </div>
      )}

      {audioResult && (
        <div className="bg-slate-900/60 border border-white/10 rounded-2xl p-6 space-y-6">
          <div className="flex items-center justify-between">
            <div>
              <span className="text-xs font-bold text-sky-400 uppercase tracking-widest block">Format & Kategori</span>
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
            <span className="text-xs font-bold text-slate-400 uppercase tracking-widest block mb-3">
              Normalized Waveform Peaks ({audioResult.waveform_points.length} points)
            </span>
            <div className="bg-slate-950 p-4 rounded-xl border border-white/5 flex items-end gap-1 h-32">
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
        <div className="bg-slate-900/60 border border-white/10 rounded-2xl p-6 space-y-3">
          <h3 className="text-xs font-bold text-slate-400 uppercase tracking-widest">Skor Ketajaman Citra (Laplacian Variance)</h3>
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
