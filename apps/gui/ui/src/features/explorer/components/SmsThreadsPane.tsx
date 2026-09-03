import { Search, ArrowDownLeft, ArrowUpRight, MessageSquare } from 'lucide-react';
import { cn } from '@/shared/lib/utils';
import { formatDate } from '@/shared/lib/formatters';
import type { ConversationThread } from './smsUtils';
import { getAvatarColor, getInitials } from './smsUtils';

interface SmsThreadsPaneProps {
  threads: ConversationThread[];
  activeAddress: string | null;
  onSelectAddress: (address: string) => void;
  searchThread: string;
  onSearchChange: (search: string) => void;
  filterType: 'all' | 'inbox' | 'sent';
  onFilterChange: (type: 'all' | 'inbox' | 'sent') => void;
}

export function SmsThreadsPane({
  threads,
  activeAddress,
  onSelectAddress,
  searchThread,
  onSearchChange,
  filterType,
  onFilterChange,
}: SmsThreadsPaneProps) {
  return (
    <div className="w-80 md:w-96 bg-white border-r border-slate-200/80 flex flex-col overflow-hidden shrink-0">
      <div className="p-3 border-b border-slate-100 space-y-2">
        <div className="relative">
          <Search className="absolute left-3 top-2.5 w-3.5 h-3.5 text-slate-400" />
          <input
            type="text"
            placeholder="Cari pengirim atau isi pesan..."
            value={searchThread}
            onChange={(e) => onSearchChange(e.target.value)}
            className="w-full bg-slate-50 border border-slate-200/70 pl-9 pr-3 py-1.5 rounded-xl text-xs outline-none focus:ring-2 focus:ring-indigo-500/10 focus:border-indigo-300 transition-all"
          />
        </div>

        <div className="flex items-center gap-1 bg-slate-100 p-1 rounded-xl">
          <button
            onClick={() => onFilterChange('all')}
            className={cn(
              "flex-1 py-1 rounded-lg text-[10px] font-black uppercase tracking-wider transition-all text-center",
              filterType === 'all' ? "bg-white text-indigo-600 shadow-sm" : "text-slate-400 hover:text-slate-700"
            )}
          >
            Semua
          </button>
          <button
            onClick={() => onFilterChange('inbox')}
            className={cn(
              "flex-1 py-1 rounded-lg text-[10px] font-black uppercase tracking-wider transition-all text-center flex items-center justify-center gap-1",
              filterType === 'inbox' ? "bg-white text-indigo-600 shadow-sm" : "text-slate-400 hover:text-slate-700"
            )}
          >
            <ArrowDownLeft className="w-3 h-3" /> Masuk
          </button>
          <button
            onClick={() => onFilterChange('sent')}
            className={cn(
              "flex-1 py-1 rounded-lg text-[10px] font-black uppercase tracking-wider transition-all text-center flex items-center justify-center gap-1",
              filterType === 'sent' ? "bg-white text-indigo-600 shadow-sm" : "text-slate-400 hover:text-slate-700"
            )}
          >
            <ArrowUpRight className="w-3 h-3" /> Terkirim
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto custom-scrollbar divide-y divide-slate-50">
        {threads.map((t, idx) => {
          const isSelected = activeAddress === t.address;
          const colorClass = getAvatarColor(t.address);
          const isLastSent = t.lastMessage.type_code === 2;

          return (
            <div
              key={idx}
              onClick={() => onSelectAddress(t.address)}
              className={cn(
                "p-4 flex items-start gap-3.5 cursor-pointer transition-all select-none group relative",
                isSelected ? "bg-indigo-50/80 border-r-4 border-r-indigo-600" : "hover:bg-slate-50/80"
              )}
            >
              <div className={cn("w-11 h-11 rounded-2xl flex items-center justify-center font-black text-xs shadow-sm shrink-0 mt-0.5", colorClass)}>
                {getInitials(t.address)}
              </div>

              <div className="flex-1 min-w-0">
                <div className="flex items-center justify-between gap-1 mb-0.5">
                  <p className={cn("text-xs font-bold truncate", isSelected ? "text-indigo-950" : "text-slate-800")}>
                    {t.address}
                  </p>
                  <span className="text-[9px] font-medium text-slate-400 shrink-0">
                    {formatDate(String(t.lastMessage.date))}
                  </span>
                </div>

                <p className="text-[11px] text-slate-500 line-clamp-2 leading-relaxed">
                  {isLastSent && <span className="text-indigo-600 font-bold mr-1">Anda:</span>}
                  {t.lastMessage.body}
                </p>

                <div className="flex items-center gap-1.5 mt-2">
                  <span className="text-[9px] font-black uppercase px-2 py-0.5 bg-slate-100 text-slate-500 rounded-full">
                    {t.totalCount} Pesan
                  </span>
                </div>
              </div>
            </div>
          );
        })}

        {threads.length === 0 && (
          <div className="p-12 text-center text-slate-400 space-y-2">
            <MessageSquare className="w-10 h-10 mx-auto opacity-20" />
            <p className="text-xs font-bold">Tidak ada pesan yang sesuai filter</p>
          </div>
        )}
      </div>
    </div>
  );
}
