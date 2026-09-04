import { PhoneIncoming, PhoneOutgoing, PhoneMissed, MapPin, Clock } from 'lucide-react';
import { type CallLogItem, formatCallDate, formatCallDuration, getCallType } from './callsUtils';
import { cn } from '@/shared/lib/utils';

interface CallListPaneProps {
  calls: CallLogItem[];
  filterType: 'all' | 'incoming' | 'outgoing' | 'missed';
  onFilterChange: (type: 'all' | 'incoming' | 'outgoing' | 'missed') => void;
}

export function CallListPane({ calls, filterType, onFilterChange }: CallListPaneProps) {
  const filtered = calls.filter(c => {
    if (filterType === 'all') return true;
    return getCallType(c) === filterType;
  });

  return (
    <div className="flex flex-col h-full bg-white rounded-2xl border border-slate-100 overflow-hidden shadow-sm">
      {/* Filter Tabs */}
      <div className="p-3 border-b border-slate-100 flex items-center justify-between bg-slate-50/50">
        <div className="flex items-center gap-1.5 bg-slate-100 p-1 rounded-xl">
          <FilterTab label="Semua" count={calls.length} active={filterType === 'all'} onClick={() => onFilterChange('all')} />
          <FilterTab label="Masuk" count={calls.filter(c => getCallType(c) === 'incoming').length} active={filterType === 'incoming'} onClick={() => onFilterChange('incoming')} />
          <FilterTab label="Keluar" count={calls.filter(c => getCallType(c) === 'outgoing').length} active={filterType === 'outgoing'} onClick={() => onFilterChange('outgoing')} />
          <FilterTab label="Tak Terjawab" count={calls.filter(c => getCallType(c) === 'missed').length} active={filterType === 'missed'} onClick={() => onFilterChange('missed')} />
        </div>
        <span className="text-[10px] font-black text-slate-400 uppercase tracking-widest">{filtered.length} Panggilan</span>
      </div>

      {/* Call Table / Rows */}
      <div className="flex-1 overflow-y-auto custom-scrollbar divide-y divide-slate-50">
        {filtered.length === 0 ? (
          <div className="py-20 text-center text-slate-300">
            <p className="text-[10px] font-black uppercase tracking-widest">Tidak ada catatan panggilan</p>
          </div>
        ) : (
          filtered.map((call, idx) => {
            const t = getCallType(call);
            return (
              <div
                key={call.id || `${call.number}-${call.date}-${idx}`}
                className="p-3.5 hover:bg-slate-50/80 transition-colors flex items-center justify-between gap-4"
              >
                <div className="flex items-center gap-3.5 min-w-0">
                  <div className={cn(
                    "w-9 h-9 rounded-xl flex items-center justify-center shrink-0",
                    t === 'incoming' && "bg-emerald-50 text-emerald-600",
                    t === 'outgoing' && "bg-blue-50 text-blue-600",
                    t === 'missed' && "bg-rose-50 text-rose-600"
                  )}>
                    {t === 'incoming' && <PhoneIncoming className="w-4 h-4" />}
                    {t === 'outgoing' && <PhoneOutgoing className="w-4 h-4" />}
                    {t === 'missed' && <PhoneMissed className="w-4 h-4" />}
                  </div>
                  <div className="min-w-0">
                    <p className="text-xs font-black text-slate-900 truncate">
                      {call.name || call.number}
                    </p>
                    <div className="flex items-center gap-2 text-[10px] text-slate-400 font-medium mt-0.5">
                      {call.name && <span className="text-slate-500 font-mono">{call.number}</span>}
                      {call.geocoded_location && (
                        <span className="flex items-center gap-0.5 text-slate-400">
                          <MapPin className="w-2.5 h-2.5" /> {call.geocoded_location}
                        </span>
                      )}
                    </div>
                  </div>
                </div>

                <div className="text-right shrink-0">
                  <p className="text-[11px] font-bold text-slate-700">{formatCallDate(call.date)}</p>
                  <p className="text-[10px] text-slate-400 font-mono flex items-center justify-end gap-1 mt-0.5">
                    <Clock className="w-2.5 h-2.5" /> {formatCallDuration(Number(call.duration || 0))}
                  </p>
                </div>
              </div>
            );
          })
        )}
      </div>
    </div>
  );
}

function FilterTab({
  label,
  count,
  active,
  onClick
}: {
  label: string;
  count: number;
  active: boolean;
  onClick: () => void;
}) {
  return (
    <button
      onClick={onClick}
      className={cn(
        "px-2.5 py-1 rounded-lg text-[10px] font-black uppercase tracking-wider transition-all flex items-center gap-1.5",
        active ? "bg-white text-slate-900 shadow-sm" : "text-slate-500 hover:text-slate-700"
      )}
    >
      <span>{label}</span>
      <span className={cn(
        "px-1 py-0.2 rounded text-[9px] font-mono",
        active ? "bg-slate-100 text-slate-700" : "bg-slate-200/60 text-slate-400"
      )}>
        {count}
      </span>
    </button>
  );
}
