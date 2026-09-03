import { useState } from 'react';
import { ShieldAlert, FileCode, CheckCircle, AlertTriangle } from 'lucide-react';
import { safeInvoke } from '../../../shared/lib/ipc';

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
    <div className="p-8 max-w-6xl mx-auto space-y-6 text-slate-100">
      <div className="flex items-center justify-between pb-6 border-b border-white/10">
        <div>
          <h1 className="text-2xl font-black tracking-tight flex items-center gap-3">
            <ShieldAlert className="w-7 h-7 text-amber-400" />
            App & APK Security Risk Auditor
          </h1>
          <p className="text-sm text-slate-400 mt-1">
            Audit keamanan manifest APK, analisis izin berbahaya (Dangerous Permissions), dan deteksi celah debug.
          </p>
        </div>
      </div>

      <div className="bg-slate-900/60 border border-white/10 rounded-2xl p-6 space-y-4">
        <label className="text-xs font-bold text-slate-300 uppercase tracking-wider block">
          Path ke file APK Standalone di komputer
        </label>
        <div className="flex gap-3">
          <input
            type="text"
            value={apkPath}
            onChange={(e) => setApkPath(e.target.value)}
            placeholder="/Users/username/Downloads/sample.apk"
            className="flex-1 bg-slate-950 border border-white/10 text-white placeholder-slate-500 px-4 py-2.5 rounded-xl text-sm font-mono outline-none focus:ring-2 focus:ring-amber-500/50"
          />
          <button
            onClick={handleRunAudit}
            disabled={isAuditing || !apkPath.trim()}
            className="flex items-center gap-2 bg-amber-600 hover:bg-amber-500 disabled:opacity-50 text-white font-bold px-6 py-2.5 rounded-xl text-sm shadow-lg shadow-amber-600/20 transition-all"
          >
            <ShieldAlert className="w-4 h-4" />
            {isAuditing ? 'Menganalisis...' : 'Audit APK'}
          </button>
        </div>
      </div>

      {errorMsg && (
        <div className="flex items-center gap-3 bg-rose-950/40 border border-rose-500/30 text-rose-300 p-4 rounded-xl text-xs">
          <AlertTriangle className="w-5 h-5 flex-shrink-0" />
          <span>{errorMsg}</span>
        </div>
      )}

      {auditResult && (
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div className="bg-slate-900/60 border border-white/10 rounded-2xl p-6 space-y-4">
            <h3 className="text-xs font-bold text-slate-400 uppercase tracking-widest">Identitas Aplikasi</h3>
            <div className="space-y-2">
              <div>
                <span className="text-[10px] text-slate-500 uppercase font-bold block">Package Name</span>
                <span className="text-sm font-mono text-amber-300 font-bold">{auditResult.package_name}</span>
              </div>
              <div className="flex gap-4">
                <div>
                  <span className="text-[10px] text-slate-500 uppercase font-bold block">Min SDK</span>
                  <span className="text-sm font-bold text-slate-200">{auditResult.min_sdk}</span>
                </div>
                <div>
                  <span className="text-[10px] text-slate-500 uppercase font-bold block">Target SDK</span>
                  <span className="text-sm font-bold text-slate-200">{auditResult.target_sdk}</span>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-slate-900/60 border border-white/10 rounded-2xl p-6 space-y-3">
            <h3 className="text-xs font-bold text-slate-400 uppercase tracking-widest">Skor Risiko Keamanan</h3>
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

          <div className="bg-slate-900/60 border border-white/10 rounded-2xl p-6 space-y-3">
            <h3 className="text-xs font-bold text-slate-400 uppercase tracking-widest">Status Debuggable</h3>
            <div className="flex items-center gap-2">
              {auditResult.is_debuggable ? (
                <span className="text-rose-400 font-bold text-sm">⚠️ Debuggable (Potensi Eksploitasi)</span>
              ) : (
                <span className="text-emerald-400 font-bold text-sm flex items-center gap-1.5">
                  <CheckCircle className="w-4 h-4" /> Release Production (Aman)
                </span>
              )}
            </div>
          </div>

          <div className="md:col-span-3 bg-slate-900/60 border border-white/10 rounded-2xl p-6 space-y-4">
            <h3 className="text-xs font-bold text-slate-400 uppercase tracking-widest flex items-center gap-2">
              <FileCode className="w-4 h-4 text-amber-400" />
              Dangerous Permissions Detected ({auditResult.dangerous_permissions?.length || 0})
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
              {auditResult.dangerous_permissions && auditResult.dangerous_permissions.length > 0 ? (
                auditResult.dangerous_permissions.map((perm, idx) => (
                  <div key={idx} className="bg-slate-950 p-2.5 rounded-xl border border-white/5 font-mono text-xs text-amber-300">
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
