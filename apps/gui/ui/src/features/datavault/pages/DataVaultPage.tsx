import { useState, useEffect } from 'react';
import {
  Wifi, Bookmark, FileText, Calendar, QrCode, Lock,
  Globe, CheckSquare, Search, Copy, Check, AlertTriangle, Clock, MapPin, Download, RefreshCw
} from 'lucide-react';
import {
  dataVaultService,
  type WifiNetworkItem,
  type WifiStats,
  type BookmarkItem,
  type BookmarkStats,
  type NoteItem,
  type NoteStats,
  type CalendarEvent,
  type CalendarStats
} from '../../../services/dataVaultService';
import { UI_TOKENS } from '../../../shared/theme/tokens';

type VaultTab = 'wifi' | 'bookmarks' | 'notes' | 'calendar';

export function DataVaultPage() {
  const [activeTab, setActiveTab] = useState<VaultTab>('wifi');
  const [loading, setLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState('');
  const [showPasswords, setShowPasswords] = useState(false);
  const [selectedWifiForQr, setSelectedWifiForQr] = useState<WifiNetworkItem | null>(null);
  const [qrPayload, setQrPayload] = useState<string | null>(null);
  const [copiedText, setCopiedText] = useState<string | null>(null);

  // Data states
  const [wifiList, setWifiList] = useState<WifiNetworkItem[]>([]);
  const [wifiStats, setWifiStats] = useState<WifiStats | null>(null);

  const [bookmarks, setBookmarks] = useState<BookmarkItem[]>([]);
  const [bookmarkStats, setBookmarkStats] = useState<BookmarkStats | null>(null);

  const [notes, setNotes] = useState<NoteItem[]>([]);
  const [noteStats, setNoteStats] = useState<NoteStats | null>(null);

  const [events, setEvents] = useState<CalendarEvent[]>([]);
  const [calendarStats, setCalendarStats] = useState<CalendarStats | null>(null);
  const [conflicts, setConflicts] = useState<string[]>([]);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    setLoading(true);
    try {
      const [w, b, n, c] = await Promise.all([
        dataVaultService.getWifiVault(),
        dataVaultService.getBookmarksVault(),
        dataVaultService.getNotesVault(),
        dataVaultService.getCalendarVault(),
      ]);
      setWifiList(w.networks || []);
      setWifiStats(w.stats || null);
      setBookmarks(b.bookmarks || []);
      setBookmarkStats(b.stats || null);
      setNotes(n.notes || []);
      setNoteStats(n.stats || null);
      setEvents(c.events || []);
      setCalendarStats(c.stats || null);
      setConflicts(c.conflicts || []);
    } catch (err) {
      console.error('Failed to load data vault:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleOpenQr = async (net: WifiNetworkItem) => {
    setSelectedWifiForQr(net);
    try {
      const payload = await dataVaultService.getWifiQr(
        net.ssid,
        net.pre_shared_key,
        net.security_type,
        net.is_hidden
      );
      setQrPayload(payload);
    } catch {
      setQrPayload(`WIFI:S:${net.ssid};T:${net.security_type};P:${net.pre_shared_key || ''};;`);
    }
  };

  const handleCopy = (text: string) => {
    navigator.clipboard.writeText(text);
    setCopiedText(text);
    setTimeout(() => setCopiedText(null), 2000);
  };

  const filteredWifi = wifiList.filter(n =>
    n.ssid.toLowerCase().includes(searchQuery.toLowerCase())
  );

  const filteredBookmarks = bookmarks.filter(b =>
    b.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    b.url.toLowerCase().includes(searchQuery.toLowerCase()) ||
    b.folder.toLowerCase().includes(searchQuery.toLowerCase())
  );

  const filteredNotes = notes.filter(n =>
    n.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    n.content.toLowerCase().includes(searchQuery.toLowerCase()) ||
    n.tags.some(t => t.toLowerCase().includes(searchQuery.toLowerCase()))
  );

  const filteredEvents = events.filter(e =>
    e.summary.toLowerCase().includes(searchQuery.toLowerCase()) ||
    (e.description && e.description.toLowerCase().includes(searchQuery.toLowerCase())) ||
    (e.location && e.location.toLowerCase().includes(searchQuery.toLowerCase()))
  );

  return (
    <div className={UI_TOKENS.layout.pageContainer}>
      {/* Hero Header Banner */}
      <div className={UI_TOKENS.card.heroBannerDark}>
        <div className="relative z-10 min-w-0">
          <span className="text-[10px] font-black uppercase tracking-widest text-indigo-400 bg-indigo-950/80 px-3 py-1 rounded-full border border-indigo-800/50">
            Specialist Engines Vault
          </span>
          <h1 className="text-2xl md:text-3xl font-black tracking-tight mt-2 truncate">
            Data Vault & Specialists
          </h1>
          <p className="text-xs text-slate-300 font-medium mt-1 truncate">
            Kelola kata sandi Wi-Fi, penanda tautan peramban, daftar periksa, dan agenda kalender secara terenkripsi.
          </p>
        </div>

        {/* Action Controls in Hero Banner */}
        <div className="relative z-10 flex flex-wrap items-center gap-3 shrink-0">
          <button
            onClick={loadData}
            disabled={loading}
            className="flex items-center gap-2 px-4 py-2.5 bg-white/10 hover:bg-white/20 active:scale-95 text-white rounded-2xl text-xs font-black uppercase tracking-wider border border-white/10 transition-all backdrop-blur-md"
          >
            <RefreshCw className={`w-3.5 h-3.5 ${loading ? 'animate-spin text-indigo-400' : ''}`} />
            <span>Segarkan</span>
          </button>
        </div>

        {/* Decorative Background Glow */}
        <div className="absolute -right-10 -bottom-10 w-64 h-64 bg-indigo-600/20 rounded-full blur-3xl pointer-events-none" />
      </div>

      {/* Tabs & Search Bar */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
        {/* Tab Controls */}
        <div className="flex flex-wrap bg-slate-900/80 p-1.5 rounded-2xl border border-white/10 gap-1 backdrop-blur-md">
          <button
            onClick={() => { setActiveTab('wifi'); setSearchQuery(''); }}
            className={`flex items-center gap-2 px-4 py-2.5 rounded-xl text-xs font-black uppercase tracking-wider transition-all ${
              activeTab === 'wifi'
                ? 'bg-indigo-600 text-white shadow-lg shadow-indigo-600/30'
                : 'text-slate-400 hover:text-white'
            }`}
          >
            <Wifi className="w-4 h-4" />
            Wi-Fi ({wifiStats?.total_networks ?? wifiList.length})
          </button>

          <button
            onClick={() => { setActiveTab('bookmarks'); setSearchQuery(''); }}
            className={`flex items-center gap-2 px-4 py-2.5 rounded-xl text-xs font-black uppercase tracking-wider transition-all ${
              activeTab === 'bookmarks'
                ? 'bg-indigo-600 text-white shadow-lg shadow-indigo-600/30'
                : 'text-slate-400 hover:text-white'
            }`}
          >
            <Bookmark className="w-4 h-4" />
            Bookmarks ({bookmarkStats?.total_bookmarks ?? bookmarks.length})
          </button>

          <button
            onClick={() => { setActiveTab('notes'); setSearchQuery(''); }}
            className={`flex items-center gap-2 px-4 py-2.5 rounded-xl text-xs font-black uppercase tracking-wider transition-all ${
              activeTab === 'notes'
                ? 'bg-indigo-600 text-white shadow-lg shadow-indigo-600/30'
                : 'text-slate-400 hover:text-white'
            }`}
          >
            <FileText className="w-4 h-4" />
            Notes ({noteStats?.total_notes ?? notes.length})
          </button>

          <button
            onClick={() => { setActiveTab('calendar'); setSearchQuery(''); }}
            className={`flex items-center gap-2 px-4 py-2.5 rounded-xl text-xs font-black uppercase tracking-wider transition-all ${
              activeTab === 'calendar'
                ? 'bg-indigo-600 text-white shadow-lg shadow-indigo-600/30'
                : 'text-slate-400 hover:text-white'
            }`}
          >
            <Calendar className="w-4 h-4" />
            Calendar ({calendarStats?.total_events ?? events.length})
            {conflicts.length > 0 && (
              <span className="bg-amber-500/20 text-amber-300 text-[10px] px-1.5 py-0.5 rounded-md border border-amber-500/30">
                {conflicts.length}
              </span>
            )}
          </button>
        </div>

        {/* Search input with unified tokens */}
        <div className="relative min-w-[260px]">
          <Search className="absolute left-3.5 top-3 w-4 h-4 text-slate-400" />
          <input
            type="text"
            placeholder={`Cari data ${activeTab}...`}
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full bg-slate-900/60 border border-white/10 text-white placeholder-slate-500 pl-10 pr-4 py-2.5 rounded-2xl text-xs font-medium focus:ring-4 focus:ring-indigo-500/20 focus:border-indigo-400/50 outline-none transition-all"
          />
        </div>
      </div>

      {/* Main Tab Content */}
      {loading ? (
        <div className="py-20 flex flex-col items-center justify-center text-slate-400 bg-slate-900/40 rounded-[32px] border border-white/5">
          <div className="w-8 h-8 border-4 border-indigo-500 border-t-transparent rounded-full animate-spin mb-4" />
          <p className="text-xs font-bold uppercase tracking-wider text-slate-400">Memuat Data Vault...</p>
        </div>
      ) : (
        <>
          {/* TAB: WI-FI */}
          {activeTab === 'wifi' && (
            <div className="space-y-6">
              {/* Stats Bar */}
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <div className="bg-slate-900/60 border border-white/10 p-5 rounded-[24px]">
                  <div className="text-[10px] text-slate-400 uppercase font-black tracking-widest">Total Jaringan</div>
                  <div className="text-2xl font-black text-white mt-1">{wifiStats?.total_networks ?? wifiList.length}</div>
                </div>
                <div className="bg-slate-900/60 border border-white/10 p-5 rounded-[24px]">
                  <div className="text-[10px] text-slate-400 uppercase font-black tracking-widest">Terenkripsi (WPA2/3)</div>
                  <div className="text-2xl font-black text-emerald-400 mt-1">{wifiStats?.secured_networks ?? 0}</div>
                </div>
                <div className="bg-slate-900/60 border border-white/10 p-5 rounded-[24px]">
                  <div className="text-[10px] text-slate-400 uppercase font-black tracking-widest">Terbuka (Open)</div>
                  <div className="text-2xl font-black text-amber-400 mt-1">{wifiStats?.open_networks ?? 0}</div>
                </div>
                <div className="bg-slate-900/60 border border-white/10 p-5 rounded-[24px]">
                  <div className="text-[10px] text-slate-400 uppercase font-black tracking-widest">SSID Tersembunyi</div>
                  <div className="text-2xl font-black text-indigo-400 mt-1">{wifiStats?.hidden_networks ?? 0}</div>
                </div>
              </div>

              {/* Action Bar */}
              <div className="flex items-center justify-between">
                <button
                  onClick={() => setShowPasswords(!showPasswords)}
                  className="flex items-center gap-2 px-4 py-2.5 rounded-2xl bg-slate-900/80 border border-white/10 text-xs font-black uppercase tracking-wider text-slate-300 hover:text-white transition-all shadow-sm active:scale-95"
                >
                  <Lock className="w-3.5 h-3.5 text-indigo-400" />
                  {showPasswords ? 'Sembunyikan Kata Sandi' : 'Tampilkan Kata Sandi'}
                </button>
              </div>

              {/* Table */}
              <div className="bg-slate-900/60 border border-white/10 rounded-[32px] overflow-hidden backdrop-blur-xl">
                <table className="w-full text-left text-sm">
                  <thead className="bg-slate-950/80 text-[10px] font-black uppercase tracking-widest text-slate-400 border-b border-white/10">
                    <tr>
                      <th className="py-4 px-6">Nama SSID</th>
                      <th className="py-4 px-6">Protokol Keamanan</th>
                      <th className="py-4 px-6">Kata Sandi</th>
                      <th className="py-4 px-6">Status Jaringan</th>
                      <th className="py-4 px-6 text-right">Aksi</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-white/5">
                    {filteredWifi.map((net, idx) => (
                      <tr key={idx} className="hover:bg-white/[0.02] transition-colors">
                        <td className="py-4 px-6 font-bold text-white flex items-center gap-2.5">
                          <Wifi className="w-4 h-4 text-indigo-400 shrink-0" />
                          <span className="truncate">{net.ssid}</span>
                        </td>
                        <td className="py-4 px-6">
                          <span className={`px-3 py-1 rounded-full text-[10px] font-black uppercase tracking-wider border ${
                            net.security_type.includes('WPA')
                              ? 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20'
                              : 'bg-amber-500/10 text-amber-400 border-amber-500/20'
                          }`}>
                            {net.security_type}
                          </span>
                        </td>
                        <td className="py-4 px-6 font-mono text-xs text-slate-300">
                          {showPasswords ? (
                            net.pre_shared_key || <span className="text-slate-500 italic">None</span>
                          ) : (
                            net.pre_shared_key ? '••••••••••••' : <span className="text-slate-500 italic">None</span>
                          )}
                        </td>
                        <td className="py-4 px-6">
                          {net.is_hidden && (
                            <span className="bg-slate-800 text-slate-300 text-[10px] font-black uppercase tracking-wider px-2 py-0.5 rounded-full mr-2">Hidden</span>
                          )}
                          {net.is_metered && (
                            <span className="bg-purple-500/20 text-purple-300 text-[10px] font-black uppercase tracking-wider px-2 py-0.5 rounded-full">Metered</span>
                          )}
                        </td>
                        <td className="py-4 px-6 text-right">
                          <div className="flex items-center justify-end gap-2">
                            {net.pre_shared_key && (
                              <button
                                onClick={() => handleCopy(net.pre_shared_key!)}
                                className="p-2 hover:bg-white/10 rounded-xl text-slate-400 hover:text-white transition-all active:scale-95"
                                title="Salin Kata Sandi"
                              >
                                {copiedText === net.pre_shared_key ? (
                                  <Check className="w-4 h-4 text-emerald-400" />
                                ) : (
                                  <Copy className="w-4 h-4" />
                                )}
                              </button>
                            )}
                            <button
                              onClick={() => handleOpenQr(net)}
                              className="flex items-center gap-1.5 px-3.5 py-1.5 bg-indigo-500/20 hover:bg-indigo-500/30 text-indigo-300 rounded-xl text-xs font-bold border border-indigo-500/30 transition-all active:scale-95"
                            >
                              <QrCode className="w-3.5 h-3.5" />
                              QR Code
                            </button>
                          </div>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          )}

          {/* TAB: BOOKMARKS */}
          {activeTab === 'bookmarks' && (
            <div className="space-y-6">
              {/* Stats */}
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div className="bg-slate-900/60 border border-white/10 p-5 rounded-[24px]">
                  <div className="text-[10px] text-slate-400 uppercase font-black tracking-widest">Total Bookmarks</div>
                  <div className="text-2xl font-black text-white mt-1">{bookmarkStats?.total_bookmarks ?? bookmarks.length}</div>
                </div>
                <div className="bg-slate-900/60 border border-white/10 p-5 rounded-[24px]">
                  <div className="text-[10px] text-slate-400 uppercase font-black tracking-widest">Kategori Folder</div>
                  <div className="text-2xl font-black text-indigo-400 mt-1">{bookmarkStats?.total_folders ?? 0}</div>
                </div>
                <div className="bg-slate-900/60 border border-white/10 p-5 rounded-[24px]">
                  <div className="text-[10px] text-slate-400 uppercase font-black tracking-widest">Domain Utama</div>
                  <div className="text-xs font-bold text-slate-300 mt-2 truncate">
                    {bookmarkStats?.top_domains?.slice(0, 3).map(([d]) => d).join(', ') || 'Various'}
                  </div>
                </div>
              </div>

              {/* Bookmarks List */}
              <div className="bg-slate-900/60 border border-white/10 rounded-[32px] overflow-hidden backdrop-blur-xl">
                <table className="w-full text-left text-sm">
                  <thead className="bg-slate-950/80 text-[10px] font-black uppercase tracking-widest text-slate-400 border-b border-white/10">
                    <tr>
                      <th className="py-4 px-6">Judul Halaman</th>
                      <th className="py-4 px-6">Folder</th>
                      <th className="py-4 px-6">URL Target</th>
                      <th className="py-4 px-6 text-right">Aksi</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-white/5">
                    {filteredBookmarks.map((bm, idx) => (
                      <tr key={idx} className="hover:bg-white/[0.02] transition-colors">
                        <td className="py-4 px-6 font-bold text-white flex items-center gap-2.5">
                          <Globe className="w-4 h-4 text-indigo-400 shrink-0" />
                          <span className="truncate max-w-xs">{bm.title}</span>
                        </td>
                        <td className="py-4 px-6 text-slate-400 text-xs">
                          <span className="bg-slate-800/80 px-2.5 py-1 rounded-lg border border-white/5 text-[11px] font-medium">{bm.folder}</span>
                        </td>
                        <td className="py-4 px-6 font-mono text-xs text-slate-400 truncate max-w-sm">
                          {bm.url}
                        </td>
                        <td className="py-4 px-6 text-right">
                          <button
                            onClick={() => handleCopy(bm.url)}
                            className="p-2 hover:bg-white/10 rounded-xl text-slate-400 hover:text-white transition-all active:scale-95"
                            title="Salin URL"
                          >
                            {copiedText === bm.url ? (
                              <Check className="w-4 h-4 text-emerald-400" />
                            ) : (
                              <Copy className="w-4 h-4" />
                            )}
                          </button>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          )}

          {/* TAB: NOTES */}
          {activeTab === 'notes' && (
            <div className="space-y-6">
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
                {filteredNotes.map((note) => (
                  <div
                    key={note.id}
                    className="bg-slate-900/60 border border-white/10 rounded-[28px] p-6 backdrop-blur-xl flex flex-col justify-between hover:border-indigo-500/40 hover:shadow-xl transition-all"
                  >
                    <div>
                      <div className="flex items-start justify-between gap-2">
                        <h3 className="font-black text-white text-base leading-tight">{note.title}</h3>
                        {note.note_type === 'Checklist' && (
                          <span className="bg-indigo-500/20 text-indigo-300 text-[10px] font-black uppercase tracking-wider px-2.5 py-0.5 rounded-full border border-indigo-500/30">
                            Checklist
                          </span>
                        )}
                      </div>

                      {note.content && (
                        <p className="text-slate-300 text-xs mt-3.5 whitespace-pre-wrap line-clamp-4 leading-relaxed">
                          {note.content}
                        </p>
                      )}

                      {note.checklist && note.checklist.length > 0 && (
                        <div className="mt-4 space-y-2">
                          {note.checklist.map((item, i) => (
                            <div key={i} className="flex items-center gap-2.5 text-xs text-slate-300">
                              <CheckSquare className={`w-4 h-4 shrink-0 ${item.is_checked ? 'text-emerald-400' : 'text-slate-500'}`} />
                              <span className={item.is_checked ? 'line-through text-slate-500' : ''}>{item.text}</span>
                            </div>
                          ))}
                        </div>
                      )}
                    </div>

                    <div className="mt-5 pt-4 border-t border-white/5 flex items-center justify-between">
                      <div className="flex flex-wrap gap-1.5">
                        {note.tags.map((tag, i) => (
                          <span key={i} className="bg-slate-800/80 text-slate-300 text-[10px] font-black uppercase px-2.5 py-0.5 rounded-md border border-white/5">
                            #{tag}
                          </span>
                        ))}
                      </div>
                      <button
                        onClick={() => handleCopy(note.title + '\n' + note.content)}
                        className="p-1.5 hover:bg-white/10 rounded-lg text-slate-400 hover:text-white transition-all active:scale-95"
                        title="Salin Isi Catatan"
                      >
                        <Copy className="w-3.5 h-3.5" />
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* TAB: CALENDAR */}
          {activeTab === 'calendar' && (
            <div className="space-y-6">
              {/* Conflict Alert Banner */}
              {conflicts.length > 0 && (
                <div className="bg-amber-500/10 border border-amber-500/30 rounded-[24px] p-5 flex items-start gap-3.5 text-amber-300 backdrop-blur-md">
                  <AlertTriangle className="w-5 h-5 shrink-0 mt-0.5 text-amber-400" />
                  <div>
                    <h4 className="font-black text-sm uppercase tracking-wide">Konflik Jadwal Terdeteksi</h4>
                    <ul className="text-xs text-amber-200/90 mt-1.5 space-y-1">
                      {conflicts.map((c, i) => (
                        <li key={i}>• {c}</li>
                      ))}
                    </ul>
                  </div>
                </div>
              )}

              {/* Event Cards */}
              <div className="space-y-3.5">
                {filteredEvents.map((evt) => (
                  <div
                    key={evt.uid}
                    className="bg-slate-900/60 border border-white/10 rounded-[28px] p-6 backdrop-blur-xl flex flex-col md:flex-row md:items-center justify-between gap-4 hover:border-indigo-500/40 hover:shadow-lg transition-all"
                  >
                    <div className="space-y-1.5">
                      <div className="flex items-center gap-3">
                        <h3 className="font-black text-white text-base tracking-tight">{evt.summary}</h3>
                        <span className="bg-emerald-500/20 text-emerald-400 text-[10px] font-black uppercase tracking-wider px-2.5 py-0.5 rounded-full border border-emerald-500/30">
                          {evt.status}
                        </span>
                      </div>
                      {evt.description && (
                        <p className="text-xs text-slate-400 line-clamp-1">{evt.description}</p>
                      )}
                      <div className="flex items-center gap-4 text-xs text-slate-400 pt-1">
                        <div className="flex items-center gap-1.5">
                          <Clock className="w-3.5 h-3.5 text-indigo-400" />
                          <span>{evt.start_time} {evt.end_time ? `— ${evt.end_time}` : ''}</span>
                        </div>
                        {evt.location && (
                          <div className="flex items-center gap-1.5">
                            <MapPin className="w-3.5 h-3.5 text-rose-400" />
                            <span>{evt.location}</span>
                          </div>
                        )}
                      </div>
                    </div>

                    <button
                      onClick={() => handleCopy(`BEGIN:VCALENDAR\nSUMMARY:${evt.summary}\nEND:VCALENDAR`)}
                      className="px-4 py-2 bg-slate-800 hover:bg-slate-700 active:scale-95 text-slate-200 rounded-2xl text-xs font-black uppercase tracking-wider border border-white/5 transition-all flex items-center gap-2 self-start md:self-auto shadow-sm"
                    >
                      <Download className="w-3.5 h-3.5 text-indigo-400" />
                      Salin ICS
                    </button>
                  </div>
                ))}
              </div>
            </div>
          )}
        </>
      )}

      {/* QR Code Modal for Wi-Fi direct scan */}
      {selectedWifiForQr && qrPayload && (
        <div className={UI_TOKENS.layout.modalOverlay}>
          <div className="bg-slate-900 border border-white/10 rounded-[32px] p-7 max-w-sm w-full space-y-5 shadow-2xl relative overflow-hidden">
            <div className="flex items-center justify-between">
              <h3 className="font-black text-white text-lg flex items-center gap-2">
                <QrCode className="w-5 h-5 text-indigo-400" />
                Pindai Wi-Fi QR
              </h3>
              <button
                onClick={() => setSelectedWifiForQr(null)}
                className="w-8 h-8 rounded-full bg-slate-800 text-slate-400 hover:text-white flex items-center justify-center text-xs font-bold transition-all"
              >
                ✕
              </button>
            </div>

            <div className="bg-white p-6 rounded-[24px] flex flex-col items-center justify-center shadow-inner">
              <img
                src={`https://api.qrserver.com/v1/create-qr-code/?size=220x220&data=${encodeURIComponent(qrPayload)}`}
                alt="Wi-Fi QR Code"
                className="w-48 h-48 rounded-lg"
              />
              <span className="text-[10px] font-black uppercase tracking-widest text-slate-500 mt-3 font-mono">
                Pindai dengan kamera ponsel
              </span>
            </div>

            <div className="bg-slate-950/80 p-4 rounded-2xl border border-white/5 space-y-1">
              <div className="text-xs font-black text-white">{selectedWifiForQr.ssid}</div>
              <div className="text-xs text-slate-400 font-mono">
                Password: {selectedWifiForQr.pre_shared_key || 'None (Open Network)'}
              </div>
            </div>

            <button
              onClick={() => setSelectedWifiForQr(null)}
              className={UI_TOKENS.button.primary + " w-full"}
            >
              Tutup
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
