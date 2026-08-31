import { useState, useMemo } from 'react';
import {
  Phone, Mail, Copy, Check, Download,
  User, MessageSquare, ExternalLink,
  Sparkles, Contact as ContactIcon, PhoneCall
} from 'lucide-react';
import { cn } from '@/shared/lib/utils';
import { backupService } from '@/services/backupService';

export interface ContactData {
  id?: string;
  display_name: string;
  phone_numbers?: string[];
  emails?: string[];
  organization?: string;
  notes?: string;
}

interface ContactsExplorerProps {
  contacts: ContactData[];
  snapshotId: string;
}

const AVATAR_COLORS = [
  'bg-indigo-500 text-white',
  'bg-rose-500 text-white',
  'bg-emerald-500 text-white',
  'bg-amber-500 text-white',
  'bg-purple-500 text-white',
  'bg-sky-500 text-white',
  'bg-pink-500 text-white',
  'bg-teal-500 text-white',
];

function getAvatarColor(name?: string): string {
  if (!name || typeof name !== 'string') return AVATAR_COLORS[0];
  let hash = 0;
  for (let i = 0; i < name.length; i++) {
    hash = name.charCodeAt(i) + ((hash << 5) - hash);
  }
  return AVATAR_COLORS[Math.abs(hash) % AVATAR_COLORS.length];
}

function getInitials(name?: string): string {
  if (!name || typeof name !== 'string' || !name.trim()) return '?';
  const parts = name.trim().split(/\s+/);
  if (parts.length >= 2 && parts[0][0] && parts[1][0]) {
    return (parts[0][0] + parts[1][0]).toUpperCase();
  }
  return name.trim().substring(0, 2).toUpperCase();
}

export function ContactsExplorer({ contacts = [], snapshotId }: ContactsExplorerProps) {
  const [selectedContact, setSelectedContact] = useState<ContactData | null>(contacts?.[0] || null);
  const [copiedField, setCopiedField] = useState<string | null>(null);
  const [selectedAlphabet, setSelectedAlphabet] = useState<string>('ALL');
  const [exportingVCard, setExportingVCard] = useState(false);
  const [exportSuccess, setExportSuccess] = useState(false);

  // Sync selected contact when contacts prop changes or is loaded
  useMemo(() => {
    if (!selectedContact && contacts && contacts.length > 0) {
      setSelectedContact(contacts[0]);
    }
  }, [contacts, selectedContact]);

  // Alphabet list that exists in data
  const alphabetList = useMemo(() => {
    const chars = new Set<string>();
    contacts.forEach(c => {
      const first = (c.display_name || '')[0]?.toUpperCase();
      if (first && /[A-Z]/.test(first)) chars.add(first);
    });
    return Array.from(chars).sort();
  }, [contacts]);

  // Filtered contacts based on alphabet
  const filteredContacts = useMemo(() => {
    return contacts.filter(c => {
      if (selectedAlphabet === 'ALL') return true;
      const first = (c.display_name || '')[0]?.toUpperCase();
      return first === selectedAlphabet;
    });
  }, [contacts, selectedAlphabet]);

  const handleCopy = (text: string, fieldId: string) => {
    navigator.clipboard.writeText(text);
    setCopiedField(fieldId);
    setTimeout(() => setCopiedField(null), 2000);
  };

  const handleExportAllVCard = async () => {
    setExportingVCard(true);
    try {
      const vcardData = await backupService.exportContactsVCard(snapshotId);
      const blob = new Blob([vcardData], { type: 'text/vcard;charset=utf-8;' });
      const url = URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.setAttribute('download', `Kontak_Snapshot_${snapshotId.substring(0, 8)}.vcf`);
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      URL.revokeObjectURL(url);

      setExportSuccess(true);
      setTimeout(() => setExportSuccess(false), 3000);
    } catch (err) {
      console.error("Gagal mengekspor vCard", err);
      alert("Gagal mengekspor vCard: " + err);
    } finally {
      setExportingVCard(false);
    }
  };

  const handleExportSingleVCard = (contact: ContactData) => {
    const vcard = [
      'BEGIN:VCARD',
      'VERSION:3.0',
      `FN:${contact.display_name}`,
      ...(contact.phone_numbers || []).map(p => `TEL;TYPE=CELL:${p}`),
      ...(contact.emails || []).map(e => `EMAIL:${e}`),
      'END:VCARD'
    ].join('\r\n');

    const blob = new Blob([vcard], { type: 'text/vcard;charset=utf-8;' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.setAttribute('download', `${contact.display_name.replace(/\s+/g, '_')}.vcf`);
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
  };

  return (
    <div className="flex flex-col h-full bg-slate-50/50 rounded-[32px] border border-slate-200/80 overflow-hidden shadow-sm">
      
      {/* Contacts Header Bar */}
      <div className="p-5 bg-white border-b border-slate-200/80 flex items-center justify-between gap-4">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-2xl bg-indigo-50 text-indigo-600 flex items-center justify-center shadow-inner">
            <ContactIcon className="w-5 h-5" />
          </div>
          <div>
            <h2 className="text-sm font-black text-slate-900 tracking-tight flex items-center gap-2">
              Buku Kontak Vault
              <span className="text-[10px] font-black uppercase px-2.5 py-0.5 bg-indigo-50 text-indigo-700 rounded-full border border-indigo-100">
                {contacts.length} Kontak
              </span>
            </h2>
            <p className="text-[10px] text-slate-400 font-bold uppercase tracking-widest">
              Data Terdekripsi & Siap Dipulihkan
            </p>
          </div>
        </div>

        <button
          onClick={handleExportAllVCard}
          disabled={exportingVCard || contacts.length === 0}
          className={cn(
            "flex items-center gap-2 px-5 py-2.5 rounded-2xl text-xs font-black uppercase tracking-wider transition-all shadow-md active:scale-95 disabled:opacity-50",
            exportSuccess
              ? "bg-emerald-600 text-white"
              : "bg-indigo-600 hover:bg-indigo-700 text-white shadow-indigo-100"
          )}
        >
          {exportSuccess ? <Check className="w-4 h-4" /> : <Download className="w-4 h-4" />}
          {exportSuccess ? "vCard Berhasil Diunduh!" : exportingVCard ? "Mengekspor..." : "Ekspor Semua (.vcf)"}
        </button>
      </div>

      {/* Alphabet Index Filter Bar */}
      {alphabetList.length > 0 && (
        <div className="px-5 py-2 bg-white/70 border-b border-slate-100 flex items-center gap-1 overflow-x-auto no-scrollbar">
          <button
            onClick={() => setSelectedAlphabet('ALL')}
            className={cn(
              "px-2.5 py-1 rounded-xl text-[10px] font-black uppercase tracking-wider transition-all",
              selectedAlphabet === 'ALL'
                ? "bg-slate-900 text-white shadow-sm"
                : "text-slate-400 hover:text-slate-700 hover:bg-slate-100"
            )}
          >
            Semua
          </button>
          {alphabetList.map(char => (
            <button
              key={char}
              onClick={() => setSelectedAlphabet(char)}
              className={cn(
                "w-7 h-7 rounded-xl text-[10px] font-black transition-all flex items-center justify-center shrink-0",
                selectedAlphabet === char
                  ? "bg-indigo-600 text-white shadow-sm shadow-indigo-100"
                  : "text-slate-500 hover:text-indigo-600 hover:bg-indigo-50"
              )}
            >
              {char}
            </button>
          ))}
        </div>
      )}

      {/* Main Dual-Pane Section */}
      <div className="flex-1 flex overflow-hidden">

        {/* Left Pane: Contacts List */}
        <div className="w-80 md:w-96 bg-white border-r border-slate-200/80 flex flex-col overflow-hidden shrink-0">
          <div className="flex-1 overflow-y-auto custom-scrollbar divide-y divide-slate-50">
            {filteredContacts.map((c, i) => {
              const isSelected = (selectedContact?.display_name === c.display_name) &&
                (selectedContact?.phone_numbers?.[0] === c.phone_numbers?.[0]);
              const colorClass = getAvatarColor(c.display_name || '?');
              const primaryPhone = c.phone_numbers?.[0];

              return (
                <div
                  key={i}
                  onClick={() => setSelectedContact(c)}
                  className={cn(
                    "p-4 flex items-center gap-3.5 cursor-pointer transition-all select-none group",
                    isSelected
                      ? "bg-indigo-50/80 border-r-4 border-r-indigo-600"
                      : "hover:bg-slate-50/80"
                  )}
                >
                  <div className={cn("w-11 h-11 rounded-2xl flex items-center justify-center font-black text-xs shadow-sm shrink-0 transition-transform group-hover:scale-105", colorClass)}>
                    {getInitials(c.display_name)}
                  </div>

                  <div className="flex-1 min-w-0">
                    <p className={cn("text-xs font-bold truncate", isSelected ? "text-indigo-950" : "text-slate-800")}>
                      {c.display_name || 'Tanpa Nama'}
                    </p>
                    <p className="text-[11px] font-mono text-slate-400 truncate mt-0.5">
                      {primaryPhone || 'Tidak ada nomor'}
                    </p>
                  </div>

                  {c.phone_numbers && c.phone_numbers.length > 1 && (
                    <span className="text-[9px] font-black px-2 py-0.5 bg-slate-100 text-slate-500 rounded-full shrink-0">
                      +{c.phone_numbers.length - 1}
                    </span>
                  )}
                </div>
              );
            })}

            {filteredContacts.length === 0 && (
              <div className="p-12 text-center text-slate-400 space-y-2">
                <User className="w-10 h-10 mx-auto opacity-20" />
                <p className="text-xs font-bold">Tidak ada kontak ditemukan</p>
              </div>
            )}
          </div>
        </div>

        {/* Right Pane: Rich Contact Profile & Detail Card */}
        <div className="flex-1 bg-slate-50/30 overflow-y-auto custom-scrollbar p-6 lg:p-8 flex flex-col items-center">
          {selectedContact ? (
            <div className="w-full max-w-xl space-y-6 animate-in fade-in zoom-in-95 duration-300">
              
              {/* Profile Card Header */}
              <div className="bg-white rounded-3xl p-6 lg:p-8 border border-slate-200/80 shadow-sm relative overflow-hidden flex flex-col items-center text-center">
                {/* Decorative background aura */}
                <div className="absolute -top-16 -right-16 w-36 h-36 bg-indigo-50 rounded-full blur-2xl pointer-events-none" />
                <div className="absolute -bottom-16 -left-16 w-36 h-36 bg-amber-50 rounded-full blur-2xl pointer-events-none" />

                <div className={cn("w-24 h-24 rounded-3xl flex items-center justify-center font-black text-2xl shadow-xl mb-4 relative z-10", getAvatarColor(selectedContact.display_name))}>
                  {getInitials(selectedContact.display_name)}
                </div>

                <h3 className="text-xl font-black text-slate-900 tracking-tight leading-snug">
                  {selectedContact.display_name}
                </h3>
                {selectedContact.organization && (
                  <p className="text-xs font-bold text-slate-400 mt-1 uppercase tracking-wider">
                    {selectedContact.organization}
                  </p>
                )}

                {/* Quick Action Pills */}
                <div className="flex items-center gap-2 mt-6">
                  <button
                    onClick={() => handleExportSingleVCard(selectedContact)}
                    className="flex items-center gap-2 px-4 py-2 bg-indigo-50 text-indigo-700 hover:bg-indigo-100 rounded-xl text-xs font-black uppercase tracking-wider transition-all"
                  >
                    <Download className="w-3.5 h-3.5" /> Unduh .VCF
                  </button>
                  {selectedContact.phone_numbers?.[0] && (
                    <a
                      href={`https://wa.me/${selectedContact.phone_numbers[0].replace(/[^0-9]/g, '')}`}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="flex items-center gap-2 px-4 py-2 bg-emerald-50 text-emerald-700 hover:bg-emerald-100 rounded-xl text-xs font-black uppercase tracking-wider transition-all"
                    >
                      <MessageSquare className="w-3.5 h-3.5" /> WhatsApp <ExternalLink className="w-3 h-3 opacity-60" />
                    </a>
                  )}
                </div>
              </div>

              {/* Phone Numbers Card */}
              <div className="bg-white rounded-3xl p-6 border border-slate-200/80 shadow-sm space-y-4">
                <div className="flex items-center justify-between border-b border-slate-100 pb-3">
                  <span className="text-[10px] font-black uppercase tracking-widest text-slate-400 flex items-center gap-2">
                    <Phone className="w-3.5 h-3.5 text-indigo-600" /> Nomor Telepon
                  </span>
                  <span className="text-[10px] font-bold text-slate-400">
                    {(selectedContact.phone_numbers || []).length} Nomor
                  </span>
                </div>

                <div className="space-y-2">
                  {(selectedContact.phone_numbers || []).map((phone, idx) => (
                    <div
                      key={idx}
                      className="flex items-center justify-between p-3.5 bg-slate-50 hover:bg-indigo-50/50 rounded-2xl border border-slate-100 transition-all group"
                    >
                      <div className="flex items-center gap-3 min-w-0">
                        <div className="w-8 h-8 rounded-xl bg-white text-slate-600 flex items-center justify-center shadow-sm shrink-0">
                          <PhoneCall className="w-4 h-4" />
                        </div>
                        <div className="min-w-0">
                          <p className="font-mono font-bold text-xs text-slate-800 tracking-wider select-all">
                            {phone}
                          </p>
                          <span className="text-[9px] font-black uppercase tracking-wider text-slate-400">
                            {idx === 0 ? 'Utama / Mobile' : `Alternatif ${idx + 1}`}
                          </span>
                        </div>
                      </div>

                      <div className="flex items-center gap-1.5">
                        <button
                          onClick={() => handleCopy(phone, `phone-${idx}`)}
                          title="Salin Nomor"
                          className="p-2 hover:bg-white text-slate-400 hover:text-indigo-600 rounded-xl transition-all shadow-sm border border-transparent hover:border-slate-200"
                        >
                          {copiedField === `phone-${idx}` ? <Check className="w-4 h-4 text-emerald-600" /> : <Copy className="w-4 h-4" />}
                        </button>
                      </div>
                    </div>
                  ))}

                  {(!selectedContact.phone_numbers || selectedContact.phone_numbers.length === 0) && (
                    <p className="text-xs text-slate-400 italic py-2">Tidak ada nomor telepon yang tercatat.</p>
                  )}
                </div>
              </div>

              {/* Emails Card */}
              {selectedContact.emails && selectedContact.emails.length > 0 && (
                <div className="bg-white rounded-3xl p-6 border border-slate-200/80 shadow-sm space-y-4">
                  <div className="flex items-center justify-between border-b border-slate-100 pb-3">
                    <span className="text-[10px] font-black uppercase tracking-widest text-slate-400 flex items-center gap-2">
                      <Mail className="w-3.5 h-3.5 text-indigo-600" /> Email
                    </span>
                  </div>

                  <div className="space-y-2">
                    {selectedContact.emails.map((email, idx) => (
                      <div
                        key={idx}
                        className="flex items-center justify-between p-3.5 bg-slate-50 hover:bg-indigo-50/50 rounded-2xl border border-slate-100 transition-all group"
                      >
                        <div className="flex items-center gap-3 min-w-0">
                          <div className="w-8 h-8 rounded-xl bg-white text-slate-600 flex items-center justify-center shadow-sm shrink-0">
                            <Mail className="w-4 h-4" />
                          </div>
                          <p className="font-mono text-xs text-slate-800 truncate select-all">
                            {email}
                          </p>
                        </div>

                        <button
                          onClick={() => handleCopy(email, `email-${idx}`)}
                          title="Salin Email"
                          className="p-2 hover:bg-white text-slate-400 hover:text-indigo-600 rounded-xl transition-all shadow-sm border border-transparent hover:border-slate-200"
                        >
                          {copiedField === `email-${idx}` ? <Check className="w-4 h-4 text-emerald-600" /> : <Copy className="w-4 h-4" />}
                        </button>
                      </div>
                    ))}
                  </div>
                </div>
              )}

            </div>
          ) : (
            <div className="flex flex-col items-center justify-center h-full text-slate-300 space-y-4 py-20">
              <div className="w-16 h-16 bg-white rounded-3xl flex items-center justify-center shadow-sm">
                <Sparkles className="w-8 h-8 opacity-20" />
              </div>
              <p className="text-xs font-bold">Pilih kontak dari daftar di sebelah kiri untuk melihat rincian.</p>
            </div>
          )}
        </div>

      </div>
    </div>
  );
}
