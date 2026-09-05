import { useState } from 'react';
import { ShieldAlert, FileCode, CheckCircle, AlertTriangle, ShieldCheck } from 'lucide-react';
import { safeInvoke } from '../../../shared/lib/ipc';
import { UI_TOKENS } from '../../../shared/theme/tokens';

interface AuditData {
  package_name: string;
  min_sdk: number;
  target_sdk: number;
  risk_score: number;
  dangerous_permissions: string[];
  is_debuggable: boolean;
}

export function AppAuditPage() {
  const [apkPath, setApkPath] = useState('');
  const [isAuditing, setIsAuditing] = useState(false);
  const [auditResult, setAuditResult] = useState<AuditData | null>(null);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  const handleRunAudit = async () => {
    if (!apkPath.trim()) return;
    setIsAuditing(true);
    setErrorMsg(null);
    setAuditResult(null);

    try {
      const rawJson = await safeInvoke<string>('audit_apk_file', { path: apkPath });
      const parsed = JSON.parse(rawJson);
      setAuditResult(parsed);
    } catch (e: any) {
      setErrorMsg(typeof e === 'string' ? e : e?.message || 'Failed to audit APK file');
    } finally {
      setIsAuditing(false);
    }
  };

  return (
    <div className={UI_TOKENS.layout.pageContainer}>
      {/* Hero Header Banner */}
      <div className={UI_TOKENS.card.heroBannerDark}>
        <div className="relative z-10 min-w-0">
          <span className="text-[10px] font-black uppercase tracking-widest text-amber-400 bg-amber-950/80 px-3 py-1 rounded-full border border-amber-800/50">
            Security & Compliance Engine
          </span>
          <h1 className="text-2xl md:text-3xl font-black tracking-tight mt-2 truncate">
            App & APK Security Auditor
          </h1>
          <p className="text-xs text-slate-300 font-medium mt-1 truncate">
            Audit keamanan manifest APK, analisis izin berbahaya (Dangerous Permissions), dan celah debug.
          </p>
        </div>

        <div className="relative z-10 flex items-center gap-2 px-4 py-2 bg-white/10 backdrop-blur-md rounded-2xl border border-white/10 text-xs font-bold text-slate-200">
          <ShieldCheck className="w-4 h-4 text-emerald-400" />
          <span>AXML Engine Ready</span>
        </div>

        <div className="absolute -right-10 -bottom-10 w-64 h-64 bg-amber-600/20 rounded-full blur-3xl pointer-events-none" />
      </div>

      {/* Input Form Card */}
      <div className="bg-slate-900/60 border border-white/10 rounded-[28px] p-6 space-y-4 backdrop-blur-xl">
        <label className="text-[10px] font-black uppercase tracking-widest text-slate-400 block">
          Path berkas APK Standalone
        </label>
        <div className="flex flex-col sm:flex-row gap-3">
          <input
            type="text"
            value={apkPath}
            onChange={(e) => setApkPath(e.target.value)}
            placeholder="/Users/username/Downloads/sample.apk"
            className="flex-1 bg-slate-950/80 border border-white/10 text-white placeholder-slate-500 px-4 py-2.5 rounded-2xl text-xs font-mono outline-none focus:ring-4 focus:ring-amber-500/20 focus:border-amber-400/50 transition-all"
          />
          <button
            onClick={handleRunAudit}
            disabled={isAuditing || !apkPath.trim()}
            className="flex items-center justify-center gap-2 bg-amber-600 hover:bg-amber-500 disabled:opacity-50 text-white font-black text-xs uppercase tracking-wider px-6 py-3 rounded-2xl shadow-lg shadow-amber-600/20 transition-all active:scale-95"
          >
            <ShieldAlert className="w-4 h-4" />
            {isAuditing ? 'Menganalisis...' : 'Audit APK'}
          </button>
        </div>
      </div>

      {errorMsg && (
        <div className="flex items-center gap-3 bg-rose-950/40 border border-rose-500/30 text-rose-300 p-5 rounded-[24px] text-xs">
          <AlertTriangle className="w-5 h-5 flex-shrink-0" />
          <span>{errorMsg}</span>
        </div>
      )}

      {auditResult && (
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div className="bg-slate-900/60 border border-white/10 rounded-[28px] p-6 space-y-4 backdrop-blur-xl">
            <h3 className="text-[10px] font-black uppercase tracking-widest text-slate-400">Identitas Aplikasi</h3>
            <div className="space-y-3">
              <div>
                <span className="text-[10px] text-slate-500 uppercase font-black tracking-wider block">Package Name</span>
                <span className="text-sm font-mono text-amber-300 font-bold break-all">{auditResult.package_name}</span>
              </div>
              <div className="flex gap-4 pt-1">
                <div>
                  <span className="text-[10px] text-slate-500 uppercase font-black tracking-wider block">Min SDK</span>
                  <span className="text-sm font-black text-white">{auditResult.min_sdk}</span>
                </div>
                <div>
                  <span className="text-[10px] text-slate-500 uppercase font-black tracking-wider block">Target SDK</span>
                  <span className="text-sm font-black text-white">{auditResult.target_sdk}</span>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-slate-900/60 border border-white/10 rounded-[28px] p-6 space-y-3 backdrop-blur-xl">
            <h3 className="text-[10px] font-black uppercase tracking-widest text-slate-400">Skor Risiko Keamanan</h3>
            <div className="flex items-baseline gap-2">
              <span className={`text-4xl font-black ${auditResult.risk_score > 30 ? 'text-rose-400' : 'text-emerald-400'}`}>
                {auditResult.risk_score}
              </span>
              <span className="text-xs text-slate-500 font-bold">/ 100</span>
            </div>
            <p className="text-xs text-slate-400">
              {auditResult.risk_score > 30 ? '⚠️ Terdeteksi izin berisiko tinggi privasi data.' : '✅ Risiko rendah dan aman.'}
            </p>
          </div>

          <div className="bg-slate-900/60 border border-white/10 rounded-[28px] p-6 space-y-3 backdrop-blur-xl">
            <h3 className="text-[10px] font-black uppercase tracking-widest text-slate-400">Status Build Debuggable</h3>
            <div className="flex items-center gap-2 pt-2">
              {auditResult.is_debuggable ? (
                <span className="text-rose-400 font-bold text-sm">⚠️ Debuggable (Celah Eksploitasi)</span>
              ) : (
                <span className="text-emerald-400 font-bold text-sm flex items-center gap-1.5">
                  <CheckCircle className="w-4 h-4" /> Release Production (Aman)
                </span>
              )}
            </div>
          </div>

          <div className="md:col-span-3 bg-slate-900/60 border border-white/10 rounded-[32px] p-6 space-y-4 backdrop-blur-xl">
            <h3 className="text-xs font-black uppercase tracking-widest text-slate-400 flex items-center gap-2">
              <FileCode className="w-4 h-4 text-amber-400" />
              Dangerous Permissions Detected ({auditResult.dangerous_permissions?.length || 0})
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-2.5">
              {auditResult.dangerous_permissions && auditResult.dangerous_permissions.length > 0 ? (
                auditResult.dangerous_permissions.map((perm, idx) => (
                  <div key={idx} className="bg-slate-950/80 p-3 rounded-xl border border-white/5 font-mono text-xs text-amber-300">
                    {perm}
                  </div>
                ))
              ) : (
                <p className="text-xs text-slate-500 italic">Tidak ada izin berbahaya yang ditemukan.</p>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
