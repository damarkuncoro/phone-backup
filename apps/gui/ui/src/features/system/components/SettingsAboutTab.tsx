interface TechPillProps {
  label: string;
  value: string;
}

function TechPill({ label, value }: TechPillProps) {
  return (
    <div className="p-3 bg-slate-50 rounded-2xl border border-slate-200/70">
      <p className="text-[9px] font-black text-slate-400 uppercase tracking-widest">{label}</p>
      <p className="text-xs font-black text-slate-800 mt-0.5">{value}</p>
    </div>
  );
}

export function SettingsAboutTab() {
  return (
    <div className="space-y-6 animate-in fade-in duration-200">
      <div className="bg-white p-6 md:p-8 rounded-[32px] border border-slate-100 shadow-sm space-y-6">
        <div className="flex items-center gap-4">
          <div className="w-16 h-16 rounded-3xl bg-gradient-to-br from-indigo-600 to-indigo-800 text-white flex items-center justify-center shadow-xl shadow-indigo-200 font-black text-2xl">
            PB
          </div>
          <div>
            <h3 className="text-xl font-black text-slate-900 tracking-tight">
              Phone Backup Platform
            </h3>
            <p className="text-xs text-slate-400 font-mono">Versi 0.3.2 (Production Release)</p>
          </div>
        </div>

        <div className="p-5 bg-slate-50 border border-slate-200/80 rounded-3xl text-xs text-slate-600 leading-relaxed space-y-2">
          <p>
            Aplikasi pencadangan Android canggih dengan teknologi <b>Content-Defined Chunking (FastCDC)</b>, <b>Deduplikasi Global</b>, <b>Enkripsi Kriptografi Age</b>, dan dukungan protokol ganda <b>ADB + MTP</b>.
          </p>
        </div>

        <div className="grid grid-cols-2 sm:grid-cols-4 gap-3 text-center">
          <TechPill label="Rust Core" value="2021 Edition" />
          <TechPill label="UI Framework" value="Tauri v2 + React 19" />
          <TechPill label="Database" value="SQLite (SQLCipher)" />
          <TechPill label="Cloud Storage" value="Apache OpenDAL" />
        </div>
      </div>
    </div>
  );
}
