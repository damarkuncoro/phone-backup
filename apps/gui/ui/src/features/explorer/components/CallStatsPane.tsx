import { PhoneIncoming, PhoneOutgoing, PhoneMissed, Clock, Phone } from 'lucide-react';
import { type CallLogItem, formatCallDuration, getCallType } from './callsUtils';

interface CallStatsPaneProps {
  calls: CallLogItem[];
}

export function CallStatsPane({ calls }: CallStatsPaneProps) {
  const total = calls.length;
  let incoming = 0;
  let outgoing = 0;
  let missed = 0;
  let totalSecs = 0;

  calls.forEach(c => {
    const t = getCallType(c);
    if (t === 'incoming') incoming++;
    else if (t === 'outgoing') outgoing++;
    else missed++;
    totalSecs += Number(c.duration || 0);
  });

  const avgSecs = total > 0 ? Math.round(totalSecs / total) : 0;

  return (
    <div className="grid grid-cols-2 lg:grid-cols-5 gap-3 p-4 bg-slate-50/70 rounded-2xl border border-slate-100">
      <StatCard
        icon={Phone}
        label="Total Panggilan"
        value={total.toLocaleString()}
        color="text-slate-900 bg-white"
      />
      <StatCard
        icon={PhoneIncoming}
        label="Masuk"
        value={incoming.toLocaleString()}
        color="text-emerald-600 bg-emerald-50"
      />
      <StatCard
        icon={PhoneOutgoing}
        label="Keluar"
        value={outgoing.toLocaleString()}
        color="text-blue-600 bg-blue-50"
      />
      <StatCard
        icon={PhoneMissed}
        label="Tak Terjawab"
        value={missed.toLocaleString()}
        color="text-rose-600 bg-rose-50"
      />
      <StatCard
        icon={Clock}
        label="Total Durasi"
        value={formatCallDuration(totalSecs)}
        sub={`Rata-rata: ${avgSecs} dtk`}
        color="text-indigo-600 bg-indigo-50"
      />
    </div>
  );
}

function StatCard({
  icon: Icon,
  label,
  value,
  sub,
  color
}: {
  icon: any;
  label: string;
  value: string;
  sub?: string;
  color: string;
}) {
  return (
    <div className="p-3 bg-white rounded-xl border border-slate-100 shadow-sm flex flex-col justify-between">
      <div className="flex items-center justify-between mb-2">
        <span className="text-[10px] font-black uppercase tracking-wider text-slate-400">{label}</span>
        <div className={`p-1.5 rounded-lg ${color}`}>
          <Icon className="w-3.5 h-3.5" />
        </div>
      </div>
      <div>
        <p className="text-base font-black text-slate-900">{value}</p>
        {sub && <p className="text-[10px] text-slate-400 font-medium">{sub}</p>}
      </div>
    </div>
  );
}
