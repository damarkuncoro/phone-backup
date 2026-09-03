import { useState } from 'react';
import { Copy, Check, ExternalLink, Download, ArrowUpRight, ArrowDownLeft, Sparkles } from 'lucide-react';
import { cn } from '@/shared/lib/utils';
import type { ConversationThread } from './smsUtils';
import { getAvatarColor, getInitials, formatMessageTime } from './smsUtils';

interface SmsChatPaneProps {
  activeThread: ConversationThread | null;
  onExportThreadTxt: (thread: ConversationThread) => void;
}

export function SmsChatPane({ activeThread, onExportThreadTxt }: SmsChatPaneProps) {
  const [copiedField, setCopiedField] = useState<string | null>(null);

  const handleCopy = (text: string, id: string) => {
    navigator.clipboard.writeText(text);
    setCopiedField(id);
    setTimeout(() => setCopiedField(null), 2000);
  };

  if (!activeThread) {
    return (
      <div className="flex-1 bg-slate-50/40 flex flex-col items-center justify-center h-full text-slate-300 space-y-4 py-20">
        <div className="w-16 h-16 bg-white rounded-3xl flex items-center justify-center shadow-sm">
          <Sparkles className="w-8 h-8 opacity-20" />
        </div>
        <p className="text-xs font-bold">Pilih percakapan di sebelah kiri untuk melihat pesan.</p>
      </div>
    );
  }

  return (
    <div className="flex-1 bg-slate-50/40 flex flex-col overflow-hidden">
      <div className="px-6 py-4 bg-white border-b border-slate-200/80 flex items-center justify-between gap-4 shadow-sm shrink-0">
        <div className="flex items-center gap-3.5 min-w-0">
          <div className={cn("w-11 h-11 rounded-2xl flex items-center justify-center font-black text-sm shadow-md shrink-0", getAvatarColor(activeThread.address))}>
            {getInitials(activeThread.address)}
          </div>
          <div className="min-w-0">
            <h3 className="text-base font-black text-slate-900 truncate tracking-tight flex items-center gap-2">
              {activeThread.address}
            </h3>
            <p className="text-[10px] font-bold text-slate-400 uppercase tracking-widest">
              {activeThread.totalCount} Pesan dalam utas ini
            </p>
          </div>
        </div>

        <div className="flex items-center gap-2 shrink-0">
          <button
            onClick={() => handleCopy(activeThread.address, 'thread-addr')}
            title="Salin Nomor"
            className="p-2.5 hover:bg-slate-100 rounded-xl text-slate-500 hover:text-indigo-600 transition-all border border-slate-200/70 shadow-sm"
          >
            {copiedField === 'thread-addr' ? <Check className="w-4 h-4 text-emerald-600" /> : <Copy className="w-4 h-4" />}
          </button>

          {/[0-9]{5,}/.test(activeThread.address) && (
            <a
              href={`https://wa.me/${activeThread.address.replace(/[^0-9]/g, '')}`}
              target="_blank"
              rel="noopener noreferrer"
              title="Buka Chat WhatsApp"
              className="p-2.5 hover:bg-emerald-50 rounded-xl text-emerald-600 border border-emerald-100 transition-all shadow-sm"
            >
              <ExternalLink className="w-4 h-4" />
            </a>
          )}

          <button
            onClick={() => onExportThreadTxt(activeThread)}
            title="Unduh Riwayat Percakapan (.txt)"
            className="flex items-center gap-2 px-3.5 py-2 bg-slate-900 text-white rounded-xl text-xs font-black uppercase tracking-wider hover:bg-slate-800 transition-all shadow-sm"
          >
            <Download className="w-3.5 h-3.5" /> Ekspor .TXT
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto custom-scrollbar p-6 space-y-4">
        {activeThread.messages.map((m, mIdx) => {
          const isSent = m.type_code === 2;
          const bubbleId = `msg-${mIdx}`;

          return (
            <div
              key={mIdx}
              className={cn("flex flex-col group", isSent ? "items-end" : "items-start")}
            >
              <div
                className={cn(
                  "max-w-xl rounded-3xl p-4 shadow-sm relative transition-all group-hover:shadow-md",
                  isSent
                    ? "bg-indigo-600 text-white rounded-br-none"
                    : "bg-white text-slate-800 border border-slate-200/80 rounded-bl-none"
                )}
              >
                <p className="text-xs leading-relaxed font-medium whitespace-pre-wrap select-text">
                  {m.body}
                </p>

                <div
                  className={cn(
                    "flex items-center justify-between gap-3 mt-2 text-[9px] font-bold",
                    isSent ? "text-indigo-200" : "text-slate-400"
                  )}
                >
                  <span className="flex items-center gap-1">
                    {isSent ? <ArrowUpRight className="w-3 h-3" /> : <ArrowDownLeft className="w-3 h-3" />}
                    {isSent ? 'Terkirim' : 'Diterima'}
                  </span>
                  <span>{formatMessageTime(m.date)}</span>
                </div>
              </div>

              <button
                onClick={() => handleCopy(m.body, bubbleId)}
                className={cn(
                  "opacity-0 group-hover:opacity-100 mt-1 px-2.5 py-1 bg-white border border-slate-200 rounded-lg text-[9px] font-bold text-slate-500 hover:text-indigo-600 shadow-sm transition-all flex items-center gap-1",
                  isSent ? "mr-1" : "ml-1"
                )}
              >
                {copiedField === bubbleId ? <Check className="w-3 h-3 text-emerald-600" /> : <Copy className="w-3 h-3" />}
                {copiedField === bubbleId ? 'Tersalin' : 'Salin Teks'}
              </button>
            </div>
          );
        })}
      </div>
    </div>
  );
}
