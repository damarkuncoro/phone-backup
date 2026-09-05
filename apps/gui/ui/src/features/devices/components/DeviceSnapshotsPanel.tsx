import { Clock, History, ExternalLink, HardDrive, Activity } from 'lucide-react';
import { type Snapshot, getSnapshotId } from '@/services/backupService';
import { formatBytes, formatDate } from '@/shared/lib/formatters';
import { cn } from '@/shared/lib/utils';

interface DeviceSnapshotsPanelProps {
  snapshots: Snapshot[];
  loadingHistory: boolean;
  onBrowseHistory: (snapshotId: string) => void;
  onNavigate?: (view: 'dashboard' | 'devices' | 'backup' | 'files' | 'history' | 'explorer' | 'settings') => void;
  onRefreshHardware: () => void;
}

export function DeviceSnapshotsPanel({
  snapshots,
  loadingHistory,
  onBrowseHistory,
  onNavigate,
  onRefreshHardware
}: DeviceSnapshotsPanelProps) {
  return (
    <div className="lg:col-span-2 space-y-6">
      <section className="bg-white p-6 md:p-8 rounded-[32px] border border-slate-100 shadow-sm min-h-[460px] flex flex-col justify-between space-y-6">
        <div>
          <div className="flex items-center justify-between mb-6">
            <h2 className="text-lg font-black text-slate-900 tracking-tight flex items-center gap-2.5">
              <History className="w-5 h-5 text-indigo-600" /> Riwayat Snapshot Perangkat
            </h2>
            {loadingHistory && <Clock className="w-4 h-4 text-indigo-500 animate-spin" />}
          </div>

          <div className="space-y-3">
            {snapshots.slice(0, 5).map(snapshot => (
              <div
                key={getSnapshotId(snapshot)}
                onClick={() => onBrowseHistory(getSnapshotId(snapshot))}
                className="group p-4 bg-slate-50 hover:bg-indigo-50/60 border border-slate-100 hover:border-indigo-100 rounded-2xl transition-all flex items-center justify-between cursor-pointer select-none"
              >
                <div className="flex items-center gap-3.5">
                  <div className="w-10 h-10 bg-white rounded-xl flex items-center justify-center text-slate-400 group-hover:text-indigo-600 shadow-sm shrink-0">
                    <Clock className="w-4 h-4" />
                  </div>
                  <div>
                    <p className="text-xs font-bold text-slate-800 group-hover:text-indigo-950">Snapshot_{getSnapshotId(snapshot).substring(0, 8)}</p>
                    <p className="text-[10px] font-medium text-slate-400 uppercase tracking-widest">{formatDate(snapshot.started_at)}</p>
                  </div>
                </div>
                <div className="flex items-center gap-4">
                  <div className="text-right">
                    <p className="text-xs font-mono font-bold text-slate-700">{formatBytes(snapshot.total_bytes)}</p>
                    <span className="text-[9px] font-black uppercase px-2 py-0.5 rounded-full bg-emerald-50 text-emerald-700">
                      {snapshot.status}
                    </span>
                  </div>
                  <ExternalLink className="w-4 h-4 text-slate-400 group-hover:text-indigo-600 transition-colors" />
                </div>
              </div>
            ))}

            {!loadingHistory && snapshots.length === 0 && (
              <div className="flex flex-col items-center justify-center text-slate-300 py-16">
                <History className="w-12 h-12 mb-3 opacity-20" />
                <p className="text-xs font-black uppercase tracking-widest">Belum Ada Backup</p>
              </div>
            )}
          </div>
        </div>

        <div className="pt-4 border-t border-slate-100 flex justify-center">
          <button
            type="button"
            onClick={() => onNavigate?.('history')}
            className="text-xs font-black text-indigo-600 uppercase tracking-wider hover:underline active:scale-95"
          >
            Lihat Semua Snapshot di Arsip Vault &rarr;
          </button>
        </div>
      </section>

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <ActionCard
          title="File Explorer Live"
          desc="Jelajahi berkas dan folder aktif di penyimpanan ponsel"
          icon={HardDrive}
          color="text-emerald-600"
          bgColor="bg-emerald-50"
          onClick={() => onNavigate?.('files')}
        />
        <ActionCard
          title="Diagnostik Hardware"
          desc="Uji kesehatan baterai, koneksi, dan temperatur sensor"
          icon={Activity}
          color="text-indigo-600"
          bgColor="bg-indigo-50"
          onClick={onRefreshHardware}
        />
      </div>
    </div>
  );
}

function ActionCard({ title, desc, icon: Icon, color, bgColor, onClick }: {
  title: string;
  desc: string;
  icon: any;
  color: string;
  bgColor: string;
  onClick?: () => void;
}) {
  return (
    <div
      onClick={onClick}
      className="bg-white p-6 rounded-[32px] border border-slate-100 shadow-sm hover:shadow-md hover:border-indigo-100 transition-all flex items-start gap-4 group cursor-pointer active:scale-95 select-none"
    >
      <div className={cn("w-12 h-12 rounded-2xl flex items-center justify-center shrink-0 transition-transform group-hover:scale-105", bgColor)}>
        <Icon className={cn("w-6 h-6", color)} />
      </div>
      <div>
        <p className="font-black text-slate-900 text-sm group-hover:text-indigo-600 transition-colors">{title}</p>
        <p className="text-xs text-slate-500 font-medium mt-0.5 leading-relaxed">{desc}</p>
      </div>
    </div>
  );
}
