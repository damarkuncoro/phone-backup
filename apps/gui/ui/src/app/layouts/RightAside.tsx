import { Smartphone, Info, ShieldCheck, Zap, Activity, ChevronRight, ChevronLeft, Database, History } from 'lucide-react'
import { type Device } from '@/services/deviceService'
import { cn } from '@/shared/lib/utils'

interface LogEntry {
    time: string;
    msg: string;
}

interface RightAsideProps {
    activeView: string;
    selectedDevice: Device | null;
    logs: LogEntry[];
    isCollapsed: boolean;
    onToggle: () => void;
    onBackupClick: () => void;
}

export function RightAside({ activeView, selectedDevice, logs, isCollapsed, onToggle, onBackupClick }: RightAsideProps) {
    return (
        <aside className={cn(
            "relative border-l border-slate-100 bg-white transition-all duration-500 ease-in-out flex flex-col",
            isCollapsed ? "w-0 opacity-0 invisible" : "w-80 opacity-100 visible"
        )}>
            {/* Collapse Toggle Handle */}
            <button
                onClick={onToggle}
                className={cn(
                    "absolute -left-4 top-1/2 -translate-y-1/2 w-8 h-8 bg-white border border-slate-100 rounded-full shadow-lg flex items-center justify-center text-slate-400 hover:text-indigo-600 transition-all z-50",
                    isCollapsed ? "translate-x-4" : ""
                )}
            >
                {isCollapsed ? <ChevronLeft className="w-4 h-4" /> : <ChevronRight className="w-4 h-4" />}
            </button>

            <div className="flex-1 flex flex-col overflow-y-auto p-8 gap-8 min-w-[20rem]">
                {/* Context-Aware Header */}
                <div className="shrink-0">
                    <h3 className="text-[10px] font-black text-slate-400 uppercase tracking-[0.2em] mb-6 flex items-center gap-2">
                        <div className="w-1 h-1 bg-indigo-500 rounded-full" />
                        {activeView === 'files' ? 'File Inspector' :
                         activeView === 'history' ? 'Vault Insight' :
                         'Inspect Perangkat'}
                    </h3>

                    {selectedDevice ? (
                        <div className="space-y-8 animate-in fade-in slide-in-from-right-4 duration-500">
                            {/* Visual Identity Card */}
                            <div className="flex items-center gap-4 p-4 bg-indigo-50/50 rounded-3xl border border-indigo-100/50">
                                <div className={cn(
                                    "w-12 h-12 rounded-2xl flex items-center justify-center text-white shadow-lg shadow-indigo-200",
                                    activeView === 'files' ? "bg-amber-500" : "bg-indigo-600"
                                )}>
                                    {activeView === 'files' ? <Database className="w-6 h-6" /> : <Smartphone className="w-6 h-6" />}
                                </div>
                                <div className="min-w-0">
                                    <p className="font-black text-slate-900 truncate">{selectedDevice.model}</p>
                                    <p className="text-[10px] font-bold text-indigo-600 uppercase tracking-widest leading-none mt-1">
                                        {selectedDevice.connection_type} Mode
                                    </p>
                                </div>
                            </div>

                            {/* Dynamic Content based on View */}
                            <div className="space-y-4">
                                {activeView === 'files' ? (
                                    <div className="p-5 bg-amber-50/50 rounded-[32px] border border-amber-100 space-y-4">
                                        <div className="flex items-center gap-2 text-[10px] font-black text-amber-600 uppercase tracking-widest">
                                            <Info className="w-3.5 h-3.5" /> Live Storage
                                        </div>
                                        <p className="text-[11px] text-amber-900/70 leading-relaxed font-medium">
                                            Anda sedang menjelajahi filesystem internal perangkat secara real-time melalui protokol ADB.
                                        </p>
                                    </div>
                                ) : activeView === 'history' ? (
                                    <div className="p-5 bg-indigo-50 rounded-[32px] border border-indigo-100 space-y-4">
                                        <div className="flex items-center gap-2 text-[10px] font-black text-indigo-600 uppercase tracking-widest">
                                            <History className="w-3.5 h-3.5" /> Snapshot Delta
                                        </div>
                                        <p className="text-[11px] text-indigo-900/70 leading-relaxed font-medium">
                                            Vault ini menyimpan data deduplikasi. Setiap backup hanya menyimpan perubahan untuk efisiensi ruang disk.
                                        </p>
                                    </div>
                                ) : (
                                    <div className="p-5 bg-slate-50 rounded-[32px] border border-slate-100 space-y-4">
                                        <div className="flex items-center gap-2 text-[10px] font-black text-slate-400 uppercase tracking-widest">
                                            <ShieldCheck className="w-3.5 h-3.5 text-emerald-500" /> Keamanan Data
                                        </div>
                                        <p className="text-[11px] text-slate-500 leading-relaxed font-medium">
                                            Perangkat ini terverifikasi. Seluruh proses backup akan dienkripsi menggunakan kunci <span className="text-indigo-600 font-bold">AES-256</span>.
                                        </p>
                                    </div>
                                )}
                            </div>

                            <div className="space-y-1">
                                <DetailRow label="Manufaktur" value={selectedDevice.manufacturer} />
                                <DetailRow label="Serial" value={selectedDevice.serial} mono />
                                <DetailRow label="Android" value={selectedDevice.os_version} />
                                <DetailRow label="Status" value="Ready" color="text-emerald-600" />
                            </div>

                            <button
                                onClick={onBackupClick}
                                className="w-full py-4 bg-slate-900 text-white rounded-2xl font-black text-xs uppercase tracking-[0.1em] hover:bg-slate-800 transition-all flex items-center justify-center gap-2 shadow-xl shadow-slate-200"
                            >
                                <Zap className="w-4 h-4 text-amber-400 fill-amber-400" /> Cek Progres Backup
                            </button>
                        </div>
                    ) : (
                        <div className="h-full flex flex-col items-center justify-center text-center py-20 opacity-30">
                            <Smartphone className="w-12 h-12 mb-4" />
                            <p className="text-xs font-black uppercase tracking-widest">Pilih Perangkat</p>
                        </div>
                    )}
                </div>

                {/* Activity Logs - Always visible at bottom */}
                {selectedDevice && (
                    <div className="mt-auto pt-8 border-t border-slate-100">
                        <div className="flex items-center gap-2 text-[10px] font-black text-slate-400 uppercase tracking-widest mb-4">
                            <Activity className="w-3.5 h-3.5" /> Log Aktivitas
                        </div>
                        <div className="space-y-3">
                            {logs.map((log, i) => (
                                <LogItem key={i} time={log.time} msg={log.msg} />
                            ))}
                        </div>
                    </div>
                )}
            </div>
        </aside>
    );
}

function DetailRow({ label, value, mono, color }: { label: string, value: string, mono?: boolean, color?: string }) {
    return (
        <div className="flex justify-between items-center py-3 border-b border-slate-50 last:border-0">
            <span className="text-[11px] font-bold text-slate-400 uppercase tracking-wider">{label}</span>
            <span className={cn("text-xs font-black text-slate-700", mono && "font-mono", color)}>{value}</span>
        </div>
    )
}

function LogItem({ time, msg }: { time: string, msg: string }) {
    return (
        <div className="flex gap-3 items-start">
            <div className="w-1 h-1 rounded-full bg-indigo-500 mt-1.5 shrink-0" />
            <div>
                <p className="text-[10px] font-bold text-slate-700 leading-none">{msg}</p>
                <p className="text-[8px] font-medium text-slate-400 mt-1 uppercase">{time}</p>
            </div>
        </div>
    )
}
