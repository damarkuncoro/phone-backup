import { useState, useMemo } from 'react';
import { Download, Check, Contact as ContactIcon, Sparkles, RotateCcw } from 'lucide-react';
import { cn } from '@/shared/lib/utils';
import { backupService } from '@/services/backupService';
import type { ContactData } from './contactsUtils';
import { getContactPhones, getContactEmails, getContactOrg } from './contactsUtils';
import { detectDuplicateGroups, autoMergeAllDuplicates } from '../lib/contactsDeduplicator';
import { ContactListPane } from './ContactListPane';
import { ContactDetailPane } from './ContactDetailPane';

export * from './contactsUtils';

interface ContactsExplorerProps {
  contacts: ContactData[];
  snapshotId: string;
}

export function ContactsExplorer({ contacts = [], snapshotId }: ContactsExplorerProps) {
  const [selectedContact, setSelectedContact] = useState<ContactData | null>(contacts?.[0] || null);
  const [selectedAlphabet, setSelectedAlphabet] = useState<string>('ALL');
  const [exportingVCard, setExportingVCard] = useState(false);
  const [exportSuccess, setExportSuccess] = useState(false);
  const [isMergedView, setIsMergedView] = useState(false);

  const duplicateStats = useMemo(() => {
    const groups = detectDuplicateGroups(contacts);
    return { groups, count: groups.reduce((acc, g) => acc + (g.contacts.length - 1), 0) };
  }, [contacts]);

  const activeContacts = useMemo(() => isMergedView ? autoMergeAllDuplicates(contacts).merged : contacts, [contacts, isMergedView]);

  useMemo(() => {
    if (!selectedContact && activeContacts && activeContacts.length > 0) setSelectedContact(activeContacts[0]);
  }, [activeContacts, selectedContact]);

  const alphabetList = useMemo(() => {
    const chars = new Set<string>();
    activeContacts.forEach(c => {
      const first = (c.display_name || '')[0]?.toUpperCase();
      if (first && /[A-Z]/.test(first)) chars.add(first);
    });
    return Array.from(chars).sort();
  }, [activeContacts]);

  const filteredContacts = useMemo(() => {
    return activeContacts.filter(c => selectedAlphabet === 'ALL' || (c.display_name || '')[0]?.toUpperCase() === selectedAlphabet);
  }, [activeContacts, selectedAlphabet]);

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
    const phones = getContactPhones(contact);
    const emails = getContactEmails(contact);
    const org = getContactOrg(contact);
    const vcard = ['BEGIN:VCARD', 'VERSION:3.0', `FN:${contact.display_name}`, ...(org ? [`ORG:${org}`] : []), ...phones.map(p => `TEL;TYPE=CELL:${p.number}`), ...emails.map(e => `EMAIL:${e.email}`), 'END:VCARD'].join('\r\n');
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
      <div className="p-5 bg-white border-b border-slate-200/80 flex items-center justify-between gap-4">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-2xl bg-indigo-50 text-indigo-600 flex items-center justify-center shadow-inner">
            <ContactIcon className="w-5 h-5" />
          </div>
          <div>
            <h2 className="text-sm font-black text-slate-900 tracking-tight flex items-center gap-2">
              Buku Kontak Vault
              <span className="text-[10px] font-black uppercase px-2.5 py-0.5 bg-indigo-50 text-indigo-700 rounded-full border border-indigo-100">
                {activeContacts.length} Kontak
              </span>
              {isMergedView && (
                <span className="text-[9px] font-black uppercase px-2 py-0.5 bg-amber-500 text-white rounded-full">
                  Deduplikasi Aktif
                </span>
              )}
            </h2>
            <p className="text-[10px] text-slate-400 font-bold uppercase tracking-widest">
              Data Terdekripsi & Siap Dipulihkan
            </p>
          </div>
        </div>

        <div className="flex items-center gap-2">
          {duplicateStats.count > 0 && (
            <button
              onClick={() => setIsMergedView(prev => !prev)}
              className={cn(
                "flex items-center gap-1.5 px-3.5 py-2.5 rounded-2xl text-xs font-black uppercase tracking-wider transition-all shadow-sm border",
                isMergedView
                  ? "bg-amber-100 text-amber-900 border-amber-300 hover:bg-amber-200"
                  : "bg-amber-500 hover:bg-amber-600 text-white border-amber-600 shadow-amber-100"
              )}
            >
              {isMergedView ? <RotateCcw className="w-3.5 h-3.5" /> : <Sparkles className="w-3.5 h-3.5" />}
              <span>{isMergedView ? 'Batal Gabung' : `Gabung ${duplicateStats.count} Duplikat`}</span>
            </button>
          )}

          <button
            onClick={handleExportAllVCard}
            disabled={exportingVCard || activeContacts.length === 0}
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
      </div>

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

      <div className="flex-1 flex overflow-hidden">
        <ContactListPane
          contacts={filteredContacts}
          selectedContact={selectedContact}
          onSelectContact={setSelectedContact}
        />
        <ContactDetailPane
          selectedContact={selectedContact}
          onExportSingleVCard={handleExportSingleVCard}
        />
      </div>
    </div>
  );
}
