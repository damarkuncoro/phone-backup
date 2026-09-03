import { useState } from 'react';
import { Sparkles, Zap, Archive, Ban, CheckCircle2, BookOpen } from 'lucide-react';
import { cn } from "@/shared/lib/utils";

export type CompressionMode = 'adaptive' | 'fast' | 'maximum' | 'none';

interface SettingsCompressionSectionProps {
  initialMode?: CompressionMode;
  onModeChange?: (mode: CompressionMode) => void;
}

export function SettingsCompressionSection({
  initialMode = 'adaptive',
  onModeChange,
}: SettingsCompressionSectionProps) {
  const [mode, setMode] = useState<CompressionMode>(initialMode);

  const handleSelect = (selected: CompressionMode) => {
    setMode(selected);
    onModeChange?.(selected);
  };

  return (
    <div className="border-t border-slate-100 pt-6 space-y-4">
      <div>
        <h4 className="text-sm font-black text-slate-900 tracking-tight flex items-center gap-2">
          <Sparkles className="w-4 h-4 text-indigo-600" />
          Smart Compression &amp; Pre-trained Dictionaries
        </h4>
        <p className="text-xs text-slate-500 font-medium mt-0.5">
          Pilih strategi kompresi adaptif untuk memaksimalkan penghematan ruang tanpa membuang CPU.
        </p>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3">
        <CompressionCard
          selected={mode === 'adaptive'}
          onClick={() => handleSelect('adaptive')}
          icon={Sparkles}
          title="Adaptive (Auto)"
          badge="Disarankan"
          desc="Deteksi Magic Bytes, Shannon Entropy, dan kamus bawaan Android (hemat 96-98%)."
        />

        <CompressionCard
          selected={mode === 'fast'}
          onClick={() => handleSelect('fast')}
          icon={Zap}
          title="Ultra Fast"
          badge="Kecepatan"
          desc="Zstd Level 1 untuk transfer USB cepat dengan pemakaian CPU minimal."
        />

        <CompressionCard
          selected={mode === 'maximum'}
          onClick={() => handleSelect('maximum')}
          icon={Archive}
          title="Max Compression"
          badge="Cold Storage"
          desc="Zstd Level 9+ untuk penghematan ruang penyimpanan arsip jangka panjang."
        />

        <CompressionCard
          selected={mode === 'none'}
          onClick={() => handleSelect('none')}
          icon={Ban}
          title="None (Bypass)"
          badge="Raw"
          desc="Penyimpanan chunk mentah tanpa kompresi (0% beban CPU)."
        />
      </div>

      {/* Active Dictionaries Badge Bar */}
      <div className="p-4 bg-slate-50 border border-slate-200/70 rounded-2xl flex flex-wrap items-center justify-between gap-3 text-xs">
        <div className="flex items-center gap-2 text-slate-700 font-bold">
          <BookOpen className="w-4 h-4 text-indigo-600" />
          <span>Kamus Android Aktif:</span>
        </div>
        <div className="flex flex-wrap gap-1.5 font-mono text-[10px]">
          <span className="px-2 py-0.5 bg-white border border-slate-200 rounded-lg text-indigo-700 font-bold flex items-center gap-1">
            <CheckCircle2 className="w-3 h-3 text-emerald-500" /> android-xml-v1
          </span>
          <span className="px-2 py-0.5 bg-white border border-slate-200 rounded-lg text-indigo-700 font-bold flex items-center gap-1">
            <CheckCircle2 className="w-3 h-3 text-emerald-500" /> android-json-v1 (SMS/Chat)
          </span>
          <span className="px-2 py-0.5 bg-white border border-slate-200 rounded-lg text-indigo-700 font-bold flex items-center gap-1">
            <CheckCircle2 className="w-3 h-3 text-emerald-500" /> android-sqlite-v1
          </span>
        </div>
      </div>
    </div>
  );
}

function CompressionCard({
  selected,
  onClick,
  icon: Icon,
  title,
  badge,
  desc,
}: {
  selected: boolean;
  onClick: () => void;
  icon: any;
  title: string;
  badge: string;
  desc: string;
}) {
  return (
    <div
      onClick={onClick}
      className={cn(
        "p-4 rounded-2xl border transition-all cursor-pointer flex flex-col justify-between space-y-2.5",
        selected
          ? "bg-indigo-50/60 border-indigo-300 ring-2 ring-indigo-500/10 shadow-sm"
          : "bg-slate-50/80 border-slate-200/70 hover:border-slate-300"
      )}
    >
      <div className="flex items-center justify-between">
        <div
          className={cn(
            "w-8 h-8 rounded-xl flex items-center justify-center",
            selected
              ? "bg-indigo-600 text-white shadow-sm"
              : "bg-white text-slate-600 border border-slate-200"
          )}
        >
          <Icon className="w-4 h-4" />
        </div>
        <span
          className={cn(
            "text-[9px] font-black px-2 py-0.5 rounded-full uppercase tracking-wider",
            selected
              ? "bg-indigo-600 text-white"
              : "bg-slate-200/80 text-slate-600"
          )}
        >
          {badge}
        </span>
      </div>

      <div>
        <h5 className="text-xs font-black text-slate-900 uppercase tracking-tight">
          {title}
        </h5>
        <p className="text-[10px] text-slate-500 font-medium mt-1 leading-snug">
          {desc}
        </p>
      </div>
    </div>
  );
}
