import { useState, useMemo } from 'react';
import {
  MessageSquare, Search, Download, Copy, Check,
  ArrowDownLeft, ArrowUpRight,
  Sparkles, ExternalLink
} from 'lucide-react';
import { cn } from '@/shared/lib/utils';
import { formatDate } from '@/shared/lib/formatters';
import { backupService } from '@/services/backupService';

export interface SmsMessage {
  id?: string;
  address: string;
  body: string;
  date: string | number;
  type_code?: number; // 1: inbox/received, 2: sent, 3: draft, etc.
}

interface SmsExplorerProps {
  messages: SmsMessage[];
  snapshotId: string;
}

interface ConversationThread {
  address: string;
  messages: SmsMessage[];
  lastMessage: SmsMessage;
  totalCount: number;
}

const AVATAR_COLORS = [
  'bg-indigo-500 text-white',
  'bg-emerald-500 text-white',
  'bg-sky-500 text-white',
  'bg-rose-500 text-white',
  'bg-amber-500 text-white',
  'bg-purple-500 text-white',
  'bg-teal-500 text-white',
  'bg-pink-500 text-white',
];

function getAvatarColor(address: string): string {
  let hash = 0;
  for (let i = 0; i < address.length; i++) {
    hash = address.charCodeAt(i) + ((hash << 5) - hash);
  }
  return AVATAR_COLORS[Math.abs(hash) % AVATAR_COLORS.length];
}

function getInitials(address: string): string {
  if (!address) return '?';
  const clean = address.replace(/[^a-zA-Z0-9]/g, '');
  if (clean.length <= 2) return clean.toUpperCase();
  return clean.substring(0, 2).toUpperCase();
}

function formatMessageTime(dateVal: string | number): string {
  if (!dateVal) return '';
  try {
    const d = new Date(typeof dateVal === 'number' ? dateVal : dateVal);
    if (isNaN(d.getTime())) return String(dateVal);
    return d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }) + ' • ' + d.toLocaleDateString([], { day: 'numeric', month: 'short', year: 'numeric' });
  } catch {
    return String(dateVal);
  }
}

export function SmsExplorer({ messages = [], snapshotId }: SmsExplorerProps) {
  const [copiedField, setCopiedField] = useState<string | null>(null);
  const [exporting, setExporting] = useState(false);
  const [exportSuccess, setExportSuccess] = useState(false);
  const [filterType, setFilterType] = useState<'all' | 'inbox' | 'sent'>('all');
  const [searchThread, setSearchThread] = useState('');

  // Group messages into conversation threads by address
  const threads = useMemo(() => {
    const threadMap: Record<string, SmsMessage[]> = {};

    for (const msg of messages) {
      if (!msg) continue;
      const addr = (msg.address || 'Nomor Tak Dikenal').trim();
      if (!threadMap[addr]) {
        threadMap[addr] = [];
      }
      threadMap[addr].push(msg);
    }

    const threadList: ConversationThread[] = Object.entries(threadMap).map(([address, msgList]) => {
      // Sort messages chronologically
      const sorted = [...msgList].sort((a, b) => {
        const timeA = new Date(a.date).getTime() || 0;
        const timeB = new Date(b.date).getTime() || 0;
        return timeA - timeB;
      });

      return {
        address,
        messages: sorted,
        lastMessage: sorted[sorted.length - 1],
        totalCount: sorted.length
      };
    });

    // Sort threads by newest last message first
    return threadList.sort((a, b) => {
      const timeA = new Date(a.lastMessage.date).getTime() || 0;
      const timeB = new Date(b.lastMessage.date).getTime() || 0;
      return timeB - timeA;
    });
  }, [messages]);

  const [selectedAddress, setSelectedAddress] = useState<string | null>(threads[0]?.address || null);

  // Sync selected thread if current becomes invalid
  useMemo(() => {
    if (!selectedAddress && threads.length > 0) {
      setSelectedAddress(threads[0].address);
    }
  }, [threads, selectedAddress]);

  // Filter threads by search and type
  const filteredThreads = useMemo(() => {
    return threads.filter(t => {
      if (filterType === 'inbox' && !t.messages.some(m => m.type_code === 1)) return false;
      if (filterType === 'sent' && !t.messages.some(m => m.type_code === 2)) return false;

      if (!searchThread) return true;
      const q = searchThread.toLowerCase();
      return (
        t.address.toLowerCase().includes(q) ||
        t.messages.some(m => (m.body || '').toLowerCase().includes(q))
      );
    });
  }, [threads, filterType, searchThread]);

  const activeThread = useMemo(() => {
    return threads.find(t => t.address === selectedAddress) || threads[0] || null;
  }, [threads, selectedAddress]);

  const handleCopy = (text: string, id: string) => {
    navigator.clipboard.writeText(text);
    setCopiedField(id);
    setTimeout(() => setCopiedField(null), 2000);
  };

  const handleExportAllSms = async () => {
    setExporting(true);
    try {
      let data = await backupService.exportSmsJson(snapshotId);
      if (!data) {
        data = JSON.stringify(messages, null, 2);
      }
      const blob = new Blob([data], { type: 'application/json;charset=utf-8;' });
      const url = URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.setAttribute('download', `SMS_Snapshot_${snapshotId.substring(0, 8)}.json`);
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      URL.revokeObjectURL(url);

      setExportSuccess(true);
      setTimeout(() => setExportSuccess(false), 3000);
    } catch (err) {
      console.error("Gagal mengekspor SMS", err);
      // Fallback export local messages
      const blob = new Blob([JSON.stringify(messages, null, 2)], { type: 'application/json;charset=utf-8;' });
      const url = URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.setAttribute('download', `SMS_Snapshot_${snapshotId.substring(0, 8)}.json`);
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      URL.revokeObjectURL(url);
      setExportSuccess(true);
      setTimeout(() => setExportSuccess(false), 3000);
    } finally {
      setExporting(false);
    }
  };

  const handleExportSingleThreadTxt = (thread: ConversationThread) => {
    const lines = [
      `=== RIWAYAT PERCAKAPAN SMS: ${thread.address} ===`,
      `Total Pesan: ${thread.totalCount}`,
      `Waktu Ekspor: ${new Date().toLocaleString()}`,
      `-----------------------------------------------------\n`,
      ...thread.messages.map(m => {
        const isSent = m.type_code === 2;
        const sender = isSent ? '[SAYA / TERKIRIM]' : `[DARI: ${m.address}]`;
        const time = formatMessageTime(m.date);
        return `${time} ${sender}\n${m.body}\n`;
      })
    ];

    const blob = new Blob([lines.join('\n')], { type: 'text/plain;charset=utf-8;' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.setAttribute('download', `SMS_${thread.address.replace(/[^a-zA-Z0-9]/g, '_')}.txt`);
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
  };

  return (
    <div className="flex flex-col h-full bg-slate-50/50 rounded-[32px] border border-slate-200/80 overflow-hidden shadow-sm">
      
      {/* Header Bar */}
      <div className="p-5 bg-white border-b border-slate-200/80 flex items-center justify-between gap-4">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-2xl bg-indigo-50 text-indigo-600 flex items-center justify-center shadow-inner">
            <MessageSquare className="w-5 h-5" />
          </div>
          <div>
            <h2 className="text-sm font-black text-slate-900 tracking-tight flex items-center gap-2">
              Pesan SMS Vault
              <span className="text-[10px] font-black uppercase px-2.5 py-0.5 bg-indigo-50 text-indigo-700 rounded-full border border-indigo-100">
                {messages.length} Pesan • {threads.length} Percakapan
              </span>
            </h2>
            <p className="text-[10px] text-slate-400 font-bold uppercase tracking-widest">
              Riwayat Percakapan SMS Terdekripsi
            </p>
          </div>
        </div>

        <button
          onClick={handleExportAllSms}
          disabled={exporting || messages.length === 0}
          className={cn(
            "flex items-center gap-2 px-5 py-2.5 rounded-2xl text-xs font-black uppercase tracking-wider transition-all shadow-md active:scale-95 disabled:opacity-50",
            exportSuccess
              ? "bg-emerald-600 text-white"
              : "bg-indigo-600 hover:bg-indigo-700 text-white shadow-indigo-100"
          )}
        >
          {exportSuccess ? <Check className="w-4 h-4" /> : <Download className="w-4 h-4" />}
          {exportSuccess ? "JSON SMS Diunduh!" : exporting ? "Mengekspor..." : "Ekspor Semua (.json)"}
        </button>
      </div>

      {/* Main Dual Pane Layout */}
      <div className="flex-1 flex overflow-hidden">

        {/* Left Pane: Conversation Threads */}
        <div className="w-80 md:w-96 bg-white border-r border-slate-200/80 flex flex-col overflow-hidden shrink-0">
          
          {/* Threads Filter & Search */}
          <div className="p-3 border-b border-slate-100 space-y-2">
            <div className="relative">
              <Search className="absolute left-3 top-2.5 w-3.5 h-3.5 text-slate-400" />
              <input
                type="text"
                placeholder="Cari pengirim atau isi pesan..."
                value={searchThread}
                onChange={(e) => setSearchThread(e.target.value)}
                className="w-full bg-slate-50 border border-slate-200/70 pl-9 pr-3 py-1.5 rounded-xl text-xs outline-none focus:ring-2 focus:ring-indigo-500/10 focus:border-indigo-300 transition-all"
              />
            </div>

            {/* Filter Tabs */}
            <div className="flex items-center gap-1 bg-slate-100 p-1 rounded-xl">
              <button
                onClick={() => setFilterType('all')}
                className={cn(
                  "flex-1 py-1 rounded-lg text-[10px] font-black uppercase tracking-wider transition-all text-center",
                  filterType === 'all' ? "bg-white text-indigo-600 shadow-sm" : "text-slate-400 hover:text-slate-700"
                )}
              >
                Semua
              </button>
              <button
                onClick={() => setFilterType('inbox')}
                className={cn(
                  "flex-1 py-1 rounded-lg text-[10px] font-black uppercase tracking-wider transition-all text-center flex items-center justify-center gap-1",
                  filterType === 'inbox' ? "bg-white text-indigo-600 shadow-sm" : "text-slate-400 hover:text-slate-700"
                )}
              >
                <ArrowDownLeft className="w-3 h-3" /> Masuk
              </button>
              <button
                onClick={() => setFilterType('sent')}
                className={cn(
                  "flex-1 py-1 rounded-lg text-[10px] font-black uppercase tracking-wider transition-all text-center flex items-center justify-center gap-1",
                  filterType === 'sent' ? "bg-white text-indigo-600 shadow-sm" : "text-slate-400 hover:text-slate-700"
                )}
              >
                <ArrowUpRight className="w-3 h-3" /> Terkirim
              </button>
            </div>
          </div>

          {/* Threads Scrollable List */}
          <div className="flex-1 overflow-y-auto custom-scrollbar divide-y divide-slate-50">
            {filteredThreads.map((t, idx) => {
              const isSelected = activeThread?.address === t.address;
              const colorClass = getAvatarColor(t.address);
              const isLastSent = t.lastMessage.type_code === 2;

              return (
                <div
                  key={idx}
                  onClick={() => setSelectedAddress(t.address)}
                  className={cn(
                    "p-4 flex items-start gap-3.5 cursor-pointer transition-all select-none group relative",
                    isSelected
                      ? "bg-indigo-50/80 border-r-4 border-r-indigo-600"
                      : "hover:bg-slate-50/80"
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

            {filteredThreads.length === 0 && (
              <div className="p-12 text-center text-slate-400 space-y-2">
                <MessageSquare className="w-10 h-10 mx-auto opacity-20" />
                <p className="text-xs font-bold">Tidak ada pesan yang sesuai filter</p>
              </div>
            )}
          </div>
        </div>

        {/* Right Pane: Chat Bubble Stream View */}
        <div className="flex-1 bg-slate-50/40 flex flex-col overflow-hidden">
          {activeThread ? (
            <>
              {/* Conversation Top Header */}
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
                    onClick={() => handleExportSingleThreadTxt(activeThread)}
                    title="Unduh Riwayat Percakapan (.txt)"
                    className="flex items-center gap-2 px-3.5 py-2 bg-slate-900 text-white rounded-xl text-xs font-black uppercase tracking-wider hover:bg-slate-800 transition-all shadow-sm"
                  >
                    <Download className="w-3.5 h-3.5" /> Ekspor .TXT
                  </button>
                </div>
              </div>

              {/* Message Bubbles Scroll Stream */}
              <div className="flex-1 overflow-y-auto custom-scrollbar p-6 space-y-4">
                {activeThread.messages.map((m, mIdx) => {
                  const isSent = m.type_code === 2;
                  const bubbleId = `msg-${mIdx}`;

                  return (
                    <div
                      key={mIdx}
                      className={cn(
                        "flex flex-col group",
                        isSent ? "items-end" : "items-start"
                      )}
                    >
                      {/* Bubble Wrapper */}
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

                        {/* Bubble Footer info */}
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

                      {/* Quick copy bubble action */}
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
            </>
          ) : (
            <div className="flex flex-col items-center justify-center h-full text-slate-300 space-y-4 py-20">
              <div className="w-16 h-16 bg-white rounded-3xl flex items-center justify-center shadow-sm">
                <Sparkles className="w-8 h-8 opacity-20" />
              </div>
              <p className="text-xs font-bold">Pilih percakapan di sebelah kiri untuk melihat pesan.</p>
            </div>
          )}
        </div>

      </div>
    </div>
  );
}
