import { useState, useMemo } from 'react';
import { Phone, Users, ShieldCheck, Search, Loader2, CheckSquare, Square, Check, Sparkles, RotateCcw } from 'lucide-react';
import { cn } from '@/shared/lib/utils';
import { getAvatarColor, getInitials, getContactPhones, type ContactData } from '@/features/explorer/components/contactsUtils';
import { autoMergeAllDuplicates, detectDuplicateGroups } from '@/features/explorer/lib/contactsDeduplicator';

interface WizardContactsPreviewProps {
  contacts: ContactData[];
  searchQuery: string;
  onSearchChange: (q: string) => void;
  isLoading: boolean;
  selectedContactIds?: Set<string>;
  onToggleContact?: (id: string) => void;
  onSelectAll?: () => void;
  onDeselectAll?: () => void;
}

export function WizardContactsPreview({
  contacts,
  searchQuery,
  onSearchChange,
  isLoading,
  selectedContactIds = new Set(),
  onToggleContact,
  onSelectAll,
  onDeselectAll
}: WizardContactsPreviewProps) {
  const [isMergedView, setIsMergedView] = useState(false);

  const duplicateStats = useMemo(() => {
    const groups = detectDuplicateGroups(contacts);
    const count = groups.reduce((acc, g) => acc + (g.contacts.length - 1), 0);
    return { groups, count };
  }, [contacts]);

  const activeContacts = useMemo(() => {
    if (!isMergedView) return contacts;
    const { merged } = autoMergeAllDuplicates(contacts);
    return merged;
  }, [contacts, isMergedView]);

  const filtered = useMemo(() => {
    const q = searchQuery.toLowerCase().trim();
    if (!q) return activeContacts;
    return activeContacts.filter(c => {
      const name = (c.display_name || '').toLowerCase();
      const phones = getContactPhones(c);
      return name.includes(q) || phones.some(p => p.number.includes(q));
    });
  }, [activeContacts, searchQuery]);

  if (isLoading) {
    return (
      <div className="flex flex-col items-center justify-center py-20 text-slate-400 gap-3">
        <Loader2 className="w-8 h-8 text-indigo-600 animate-spin" />
        <p className="text-xs font-black uppercase tracking-wider">Membaca Kontak dari HP...</p>
      </div>
    );
  }

  const selectedCount = selectedContactIds.size;
  const isAllSelected = activeContacts.length > 0 && selectedCount >= activeContacts.length;

  return (
    <div className="space-y-4 max-w-3xl mx-auto pb-10">
      {/* Top Banner with Stats & Controls */}
      <div className="p-4 bg-indigo-50/70 border border-indigo-100 rounded-2xl flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-xl bg-indigo-600 text-white flex items-center justify-center shadow-md shadow-indigo-200 shrink-0">
            <Users className="w-5 h-5" />
          </div>
          <div>
            <h4 className="text-xs font-black text-indigo-950 uppercase tracking-wider flex items-center gap-2">
              <span>{selectedCount} dari {activeContacts.length} Kontak Dipilih</span>
              {isMergedView && (
                <span className="text-[9px] bg-amber-500 text-white font-sans px-2 py-0.5 rounded-full font-black uppercase">
                  Deduplikasi Aktif
                </span>
              )}
            </h4>
            <p className="text-[11px] text-indigo-700/80 font-medium">
              {duplicateStats.count > 0 && !isMergedView
                ? `Terdeteksi ${duplicateStats.count} kontak duplikat yang dapat digabungkan.`
                : 'Hanya kontak yang dicentang yang akan diekstraksi ke SQLite Vault & vCard.'}
            </p>
          </div>
        </div>

        {/* Batch & Merge selection buttons */}
        <div className="flex items-center gap-2 shrink-0 flex-wrap">
          {duplicateStats.count > 0 && (
            <button
              type="button"
              onClick={() => setIsMergedView(prev => !prev)}
              className={cn(
                "px-3 py-1.5 rounded-xl text-[11px] font-bold transition-all flex items-center gap-1.5 shadow-sm border",
                isMergedView
                  ? "bg-amber-100 text-amber-900 border-amber-300 hover:bg-amber-200"
                  : "bg-amber-500 hover:bg-amber-600 text-white border-amber-600"
              )}
            >
              {isMergedView ? <RotateCcw className="w-3.5 h-3.5" /> : <Sparkles className="w-3.5 h-3.5" />}
              <span>{isMergedView ? 'Batal Gabung' : `Gabung ${duplicateStats.count} Duplikat`}</span>
            </button>
          )}

          <button
            type="button"
            onClick={isAllSelected ? onDeselectAll : onSelectAll}
            className="px-3 py-1.5 bg-white hover:bg-indigo-100/50 border border-indigo-200/80 rounded-xl text-[11px] font-bold text-indigo-800 transition-all flex items-center gap-1.5 shadow-sm"
          >
            {isAllSelected ? <Square className="w-3.5 h-3.5 text-indigo-600" /> : <CheckSquare className="w-3.5 h-3.5 text-indigo-600" />}
            <span>{isAllSelected ? 'Batal Semua' : 'Pilih Semua'}</span>
          </button>
          <div className="hidden lg:flex items-center gap-1.5 px-3 py-1.5 bg-white/80 border border-indigo-200/60 rounded-xl text-[10px] font-black text-indigo-700 uppercase tracking-wider">
            <ShieldCheck className="w-3.5 h-3.5 text-emerald-600" />
            <span>Encrypted</span>
          </div>
        </div>
      </div>

      {/* Search Bar */}
      <div className="relative">
        <Search className="absolute left-4 top-3.5 w-4 h-4 text-slate-400" />
        <input
          type="text"
          placeholder="Cari nama atau nomor telepon kontak..."
          value={searchQuery}
          onChange={(e) => onSearchChange(e.target.value)}
          className="w-full bg-white border border-slate-200/80 pl-11 pr-4 py-3 rounded-2xl text-xs font-medium outline-none focus:ring-4 focus:ring-indigo-500/10 focus:border-indigo-300 transition-all shadow-sm"
        />
      </div>

      {/* Contact Cards Grid */}
      {filtered.length === 0 ? (
        <div className="text-center py-16 bg-white rounded-3xl border border-slate-100 p-6 space-y-2">
          <Users className="w-10 h-10 text-slate-300 mx-auto" />
          <p className="text-xs font-bold text-slate-600">Tidak ada kontak yang cocok dengan pencarian.</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-3 max-h-[460px] overflow-y-auto custom-scrollbar pr-1">
          {filtered.map((c, idx) => {
            const cid = c.id || `${c.display_name}-${idx}`;
            const isSelected = selectedContactIds.has(cid);
            const phones = getContactPhones(c);
            const primaryPhone = phones[0]?.number || 'Tidak ada nomor';

            return (
              <div
                key={cid}
                onClick={() => onToggleContact?.(cid)}
                className={cn(
                  "p-3.5 rounded-2xl transition-all shadow-sm flex items-center gap-3.5 cursor-pointer select-none border group",
                  isSelected
                    ? "bg-white border-indigo-300 ring-2 ring-indigo-500/10"
                    : "bg-white/60 border-slate-200/60 opacity-65 hover:opacity-100 hover:bg-white"
                )}
              >
                {/* Checkbox Icon */}
                <div
                  className={cn(
                    "w-5 h-5 rounded-lg flex items-center justify-center transition-all shrink-0",
                    isSelected
                      ? "bg-indigo-600 text-white shadow-sm shadow-indigo-300"
                      : "border-2 border-slate-300 bg-white group-hover:border-slate-400"
                  )}
                >
                  {isSelected && <Check className="w-3.5 h-3.5 stroke-[3]" />}
                </div>

                <div className={cn("w-10 h-10 rounded-xl flex items-center justify-center text-xs font-black shadow-inner shrink-0", getAvatarColor(c.display_name))}>
                  {getInitials(c.display_name)}
                </div>

                <div className="min-w-0 flex-1">
                  <p className={cn("font-bold text-xs truncate transition-colors", isSelected ? "text-slate-900 group-hover:text-indigo-950" : "text-slate-600")}>
                    {c.display_name || 'Tanpa Nama'}
                  </p>
                  <p className="text-[11px] font-mono text-slate-400 flex items-center gap-1 mt-0.5 truncate">
                    <Phone className="w-3 h-3 text-slate-400 shrink-0" />
                    <span>{primaryPhone}</span>
                    {phones.length > 1 && (
                      <span className="text-[9px] bg-slate-100 text-slate-500 font-sans px-1.5 py-0.2 rounded-full font-bold">
                        +{phones.length - 1}
                      </span>
                    )}
                  </p>
                </div>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}
