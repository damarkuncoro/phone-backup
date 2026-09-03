import { useState, useMemo } from 'react';
import { MessageSquare, Download, Check } from 'lucide-react';
import { cn } from '@/shared/lib/utils';
import { backupService } from '@/services/backupService';
import type { SmsMessage, ConversationThread } from './smsUtils';
import { formatMessageTime } from './smsUtils';
import { SmsThreadsPane } from './SmsThreadsPane';
import { SmsChatPane } from './SmsChatPane';

export * from './smsUtils';

interface SmsExplorerProps {
  messages: SmsMessage[];
  snapshotId: string;
}

export function SmsExplorer({ messages = [], snapshotId }: SmsExplorerProps) {
  const [exporting, setExporting] = useState(false);
  const [exportSuccess, setExportSuccess] = useState(false);
  const [filterType, setFilterType] = useState<'all' | 'inbox' | 'sent'>('all');
  const [searchThread, setSearchThread] = useState('');

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

    return threadList.sort((a, b) => {
      const timeA = new Date(a.lastMessage.date).getTime() || 0;
      const timeB = new Date(b.lastMessage.date).getTime() || 0;
      return timeB - timeA;
    });
  }, [messages]);

  const [selectedAddress, setSelectedAddress] = useState<string | null>(threads[0]?.address || null);

  useMemo(() => {
    if (!selectedAddress && threads.length > 0) {
      setSelectedAddress(threads[0].address);
    }
  }, [threads, selectedAddress]);

  const filteredThreads = useMemo(() => {
    return threads.filter(t => {
      if (filterType === 'inbox' && !t.messages.some(m => m.type_code === 1)) return false;
      if (filterType === 'sent' && !t.messages.some(m => m.type_code === 2)) return false;
      if (!searchThread) return true;
      const q = searchThread.toLowerCase();
      return t.address.toLowerCase().includes(q) || t.messages.some(m => (m.body || '').toLowerCase().includes(q));
    });
  }, [threads, filterType, searchThread]);

  const activeThread = useMemo(() => {
    return threads.find(t => t.address === selectedAddress) || threads[0] || null;
  }, [threads, selectedAddress]);

  const handleExportAllSms = async () => {
    setExporting(true);
    try {
      let data = await backupService.exportSmsJson(snapshotId);
      if (!data) data = JSON.stringify(messages, null, 2);
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
            exportSuccess ? "bg-emerald-600 text-white" : "bg-indigo-600 hover:bg-indigo-700 text-white shadow-indigo-100"
          )}
        >
          {exportSuccess ? <Check className="w-4 h-4" /> : <Download className="w-4 h-4" />}
          {exportSuccess ? "JSON SMS Diunduh!" : exporting ? "Mengekspor..." : "Ekspor Semua (.json)"}
        </button>
      </div>

      <div className="flex-1 flex overflow-hidden">
        <SmsThreadsPane
          threads={filteredThreads}
          activeAddress={activeThread?.address || null}
          onSelectAddress={setSelectedAddress}
          searchThread={searchThread}
          onSearchChange={setSearchThread}
          filterType={filterType}
          onFilterChange={setFilterType}
        />
        <SmsChatPane
          activeThread={activeThread}
          onExportThreadTxt={handleExportSingleThreadTxt}
        />
      </div>
    </div>
  );
}
