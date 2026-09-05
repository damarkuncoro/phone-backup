import { useState } from 'react';
import { Radio, FolderTree } from 'lucide-react';
import { WhatsAppLiveSyncTab } from '../components/WhatsAppLiveSyncTab';
import { WhatsAppScopedTab } from '../components/WhatsAppScopedTab';
import { UI_TOKENS } from '../../../shared/theme/tokens';

export function WhatsAppArchivePage() {
  const [activeTab, setActiveTab] = useState<'live' | 'scoped'>('live');

  return (
    <div className={UI_TOKENS.layout.pageContainer}>
      {/* Hero Header Banner */}
      <div className={UI_TOKENS.card.heroBannerDark}>
        <div className="relative z-10 min-w-0">
          <span className="text-[10px] font-black uppercase tracking-widest text-emerald-400 bg-emerald-950/80 px-3 py-1 rounded-full border border-emerald-800/50">
            WhatsApp Multi-Device Engine
          </span>
          <h1 className="text-2xl md:text-3xl font-black tracking-tight mt-2 truncate">
            WhatsApp Archive & Live Sync
          </h1>
          <p className="text-xs text-slate-300 font-medium mt-1 truncate">
            Sinkronisasi obrolan WhatsApp multi-device secara langsung dan analisis media Scoped Storage.
          </p>
        </div>

        {/* Tab Controls */}
        <div className="relative z-10 flex bg-slate-900/80 p-1.5 rounded-2xl border border-white/10 shrink-0 backdrop-blur-md">
          <button
            onClick={() => setActiveTab('live')}
            className={`flex items-center gap-2 px-4 py-2.5 rounded-xl text-xs font-black uppercase tracking-wider transition-all ${
              activeTab === 'live'
                ? 'bg-emerald-600 text-white shadow-lg shadow-emerald-600/30'
                : 'text-slate-400 hover:text-white'
            }`}
          >
            <Radio className="w-3.5 h-3.5" />
            Live Multi-Device QR
          </button>
          <button
            onClick={() => setActiveTab('scoped')}
            className={`flex items-center gap-2 px-4 py-2.5 rounded-xl text-xs font-black uppercase tracking-wider transition-all ${
              activeTab === 'scoped'
                ? 'bg-emerald-600 text-white shadow-lg shadow-emerald-600/30'
                : 'text-slate-400 hover:text-white'
            }`}
          >
            <FolderTree className="w-3.5 h-3.5" />
            Scoped Storage
          </button>
        </div>

        <div className="absolute -right-10 -bottom-10 w-64 h-64 bg-emerald-600/20 rounded-full blur-3xl pointer-events-none" />
      </div>

      {/* Tab Panels */}
      {activeTab === 'live' ? <WhatsAppLiveSyncTab /> : <WhatsAppScopedTab />}
    </div>
  );
}
