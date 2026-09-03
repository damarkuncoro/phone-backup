import { useState } from 'react';
import {
  Phone, Mail, Copy, Check, Download,
  MessageSquare, ExternalLink, Sparkles, PhoneCall, Building2
} from 'lucide-react';
import { cn } from '@/shared/lib/utils';
import type { ContactData } from './contactsUtils';
import {
  getAvatarColor, getInitials,
  getContactPhones, getContactEmails, getContactOrg
} from './contactsUtils';


interface ContactDetailPaneProps {
  selectedContact: ContactData | null;
  onExportSingleVCard: (contact: ContactData) => void;
}

export function ContactDetailPane({ selectedContact, onExportSingleVCard }: ContactDetailPaneProps) {
  const [copiedField, setCopiedField] = useState<string | null>(null);

  const handleCopy = (text: string, fieldId: string) => {
    navigator.clipboard.writeText(text);
    setCopiedField(fieldId);
    setTimeout(() => setCopiedField(null), 2000);
  };

  const selectedPhones = selectedContact ? getContactPhones(selectedContact) : [];
  const selectedEmails = selectedContact ? getContactEmails(selectedContact) : [];
  const selectedOrg = selectedContact ? getContactOrg(selectedContact) : undefined;

  if (!selectedContact) {
    return (
      <div className="flex-1 bg-slate-50/30 overflow-y-auto custom-scrollbar p-6 lg:p-8 flex flex-col items-center justify-center">
        <div className="flex flex-col items-center justify-center h-full text-slate-300 space-y-4 py-20">
          <div className="w-16 h-16 bg-white rounded-3xl flex items-center justify-center shadow-sm">
            <Sparkles className="w-8 h-8 opacity-20" />
          </div>
          <p className="text-xs font-bold">Pilih kontak dari daftar di sebelah kiri untuk melihat rincian.</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex-1 bg-slate-50/30 overflow-y-auto custom-scrollbar p-6 lg:p-8 flex flex-col items-center">
      <div className="w-full max-w-xl space-y-6 animate-in fade-in zoom-in-95 duration-300">
        
        {/* Profile Card Header */}
        <div className="bg-white rounded-3xl p-6 lg:p-8 border border-slate-200/80 shadow-sm relative overflow-hidden flex flex-col items-center text-center">
          <div className="absolute -top-16 -right-16 w-36 h-36 bg-indigo-50 rounded-full blur-2xl pointer-events-none" />
          <div className="absolute -bottom-16 -left-16 w-36 h-36 bg-amber-50 rounded-full blur-2xl pointer-events-none" />

          <div className={cn("w-24 h-24 rounded-3xl flex items-center justify-center font-black text-2xl shadow-xl mb-4 relative z-10", getAvatarColor(selectedContact.display_name))}>
            {getInitials(selectedContact.display_name)}
          </div>

          <h3 className="text-xl font-black text-slate-900 tracking-tight leading-snug">
            {selectedContact.display_name || 'Tanpa Nama'}
          </h3>
          {selectedOrg && (
            <p className="text-xs font-bold text-slate-400 mt-1 uppercase tracking-wider flex items-center gap-1.5 justify-center">
              <Building2 className="w-3.5 h-3.5" /> {selectedOrg}
            </p>
          )}

          <div className="flex items-center gap-2 mt-6">
            <button
              onClick={() => onExportSingleVCard(selectedContact)}
              className="flex items-center gap-2 px-4 py-2 bg-indigo-50 text-indigo-700 hover:bg-indigo-100 rounded-xl text-xs font-black uppercase tracking-wider transition-all"
            >
              <Download className="w-3.5 h-3.5" /> Unduh .VCF
            </button>
            {selectedPhones[0]?.number && (
              <a
                href={`https://wa.me/${selectedPhones[0].number.replace(/[^0-9]/g, '')}`}
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
              {selectedPhones.length} Nomor
            </span>
          </div>

          <div className="space-y-2">
            {selectedPhones.map((phone, idx) => (
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
                      {phone.number}
                    </p>
                    <span className="text-[9px] font-black uppercase tracking-wider text-slate-400">
                      {phone.type || (idx === 0 ? 'Utama / Mobile' : `Alternatif ${idx + 1}`)}
                    </span>
                  </div>
                </div>

                <div className="flex items-center gap-1.5">
                  <button
                    onClick={() => handleCopy(phone.number, `phone-${idx}`)}
                    title="Salin Nomor"
                    className="p-2 hover:bg-white text-slate-400 hover:text-indigo-600 rounded-xl transition-all shadow-sm border border-transparent hover:border-slate-200"
                  >
                    {copiedField === `phone-${idx}` ? <Check className="w-4 h-4 text-emerald-600" /> : <Copy className="w-4 h-4" />}
                  </button>
                </div>
              </div>
            ))}

            {selectedPhones.length === 0 && (
              <p className="text-xs text-slate-400 italic py-2">Tidak ada nomor telepon yang tercatat.</p>
            )}
          </div>
        </div>

        {/* Emails Card */}
        {selectedEmails.length > 0 && (
          <div className="bg-white rounded-3xl p-6 border border-slate-200/80 shadow-sm space-y-4">
            <div className="flex items-center justify-between border-b border-slate-100 pb-3">
              <span className="text-[10px] font-black uppercase tracking-widest text-slate-400 flex items-center gap-2">
                <Mail className="w-3.5 h-3.5 text-indigo-600" /> Email
              </span>
            </div>

            <div className="space-y-2">
              {selectedEmails.map((email, idx) => (
                <div
                  key={idx}
                  className="flex items-center justify-between p-3.5 bg-slate-50 hover:bg-indigo-50/50 rounded-2xl border border-slate-100 transition-all group"
                >
                  <div className="flex items-center gap-3 min-w-0">
                    <div className="w-8 h-8 rounded-xl bg-white text-slate-600 flex items-center justify-center shadow-sm shrink-0">
                      <Mail className="w-4 h-4" />
                    </div>
                    <p className="font-mono text-xs text-slate-800 truncate select-all">
                      {email.email}
                    </p>
                  </div>

                  <button
                    onClick={() => handleCopy(email.email, `email-${idx}`)}
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
    </div>
  );
}
