import { User } from 'lucide-react';
import { cn } from '@/shared/lib/utils';
import type { ContactData } from './contactsUtils';
import { getAvatarColor, getInitials, getContactPhones } from './contactsUtils';



interface ContactListPaneProps {
  contacts: ContactData[];
  selectedContact: ContactData | null;
  onSelectContact: (contact: ContactData) => void;
}

export function ContactListPane({ contacts, selectedContact, onSelectContact }: ContactListPaneProps) {
  return (
    <div className="w-80 md:w-96 bg-white border-r border-slate-200/80 flex flex-col overflow-hidden shrink-0">
      <div className="flex-1 overflow-y-auto custom-scrollbar divide-y divide-slate-50">
        {contacts.map((c, i) => {
          const phones = getContactPhones(c);
          const primaryPhone = phones[0]?.number;
          const isSelected =
            selectedContact?.display_name === c.display_name &&
            getContactPhones(selectedContact || { display_name: '' })[0]?.number === primaryPhone;
          const colorClass = getAvatarColor(c.display_name || '?');

          return (
            <div
              key={i}
              onClick={() => onSelectContact(c)}
              className={cn(
                "p-4 flex items-center gap-3.5 cursor-pointer transition-all select-none group",
                isSelected
                  ? "bg-indigo-50/80 border-r-4 border-r-indigo-600"
                  : "hover:bg-slate-50/80"
              )}
            >
              <div
                className={cn(
                  "w-11 h-11 rounded-2xl flex items-center justify-center font-black text-xs shadow-sm shrink-0 transition-transform group-hover:scale-105",
                  colorClass
                )}
              >
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

              {phones.length > 1 && (
                <span className="text-[9px] font-black px-2 py-0.5 bg-slate-100 text-slate-500 rounded-full shrink-0">
                  +{phones.length - 1}
                </span>
              )}
            </div>
          );
        })}

        {contacts.length === 0 && (
          <div className="p-12 text-center text-slate-400 space-y-2">
            <User className="w-10 h-10 mx-auto opacity-20" />
            <p className="text-xs font-bold">Tidak ada kontak ditemukan</p>
          </div>
        )}
      </div>
    </div>
  );
}
