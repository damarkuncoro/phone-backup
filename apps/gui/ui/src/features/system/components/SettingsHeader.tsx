import { Settings as SettingsIcon, AlertCircle, CheckCircle2 } from 'lucide-react';
import { cn } from "@/shared/lib/utils";
import { UI_TOKENS } from '@/shared/theme/tokens';

interface SettingsHeaderProps {
  msg: { type: 'success' | 'error'; text: string } | null;
}

export function SettingsHeader({ msg }: SettingsHeaderProps) {
  return (
    <header className={UI_TOKENS.card.headerBanner}>
      <div className="flex items-center gap-4">
        <div className="w-12 h-12 rounded-2xl bg-indigo-600 text-white flex items-center justify-center shadow-lg shadow-indigo-200 shrink-0">
          <SettingsIcon className="w-6 h-6" />
        </div>
        <div>
          <h1 className={UI_TOKENS.text.titlePage}>
            Pengaturan Sistem
          </h1>
          <p className={UI_TOKENS.text.subtitle}>
            Kelola parameter infrastruktur, keamanan enkripsi, penyimpanan, dan pemeliharaan platform.
          </p>
        </div>
      </div>

      {/* Global Toast Alert */}
      {msg && (
        <div className={cn(
          "px-4 py-2.5 rounded-2xl text-xs font-bold flex items-center gap-2.5 animate-in slide-in-from-top-2 border shrink-0",
          msg.type === 'success' ? "bg-emerald-50 text-emerald-800 border-emerald-200" : "bg-red-50 text-red-800 border-red-200"
        )}>
          {msg.type === 'success' ? <CheckCircle2 className="w-4 h-4 text-emerald-600" /> : <AlertCircle className="w-4 h-4 text-red-600" />}
          <span>{msg.text}</span>
        </div>
      )}
    </header>
  );
}
