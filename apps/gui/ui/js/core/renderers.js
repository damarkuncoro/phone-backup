export const renderers = {
    contacts(contacts, colors) {
        try {
            if (!contacts || !Array.isArray(contacts) || contacts.length === 0) {
                return '<div class="text-[10px] text-slate-400 p-8 text-center bg-slate-50/50 rounded-2xl border-2 border-dashed border-slate-100 italic">No contacts found</div>';
            }

            const getInitials = (name) => {
                if (!name || typeof name !== 'string') return "?";
                return name.trim().split(' ').filter(n => n).map(n => n[0]).join('').substring(0, 2).toUpperCase() || "?";
            };

            return contacts.map((c, i) => {
                if (!c) return "";
                const name = c.name || "Unknown";
                const phones = Array.isArray(c.phones) ? c.phones : [];
                const emails = Array.isArray(c.emails) ? c.emails : [];
                const addresses = Array.isArray(c.addresses) ? c.addresses : [];
                const organizations = Array.isArray(c.organizations) ? c.organizations : [];
                const notes = Array.isArray(c.notes) ? c.notes : [];

                return `
                    <div class="p-4 bg-white border border-slate-100 rounded-2xl flex items-start gap-4 shadow-sm hover:shadow-md transition-all group">
                        <div class="w-12 h-12 ${colors[i % colors.length] || 'bg-slate-400'} text-white rounded-2xl flex items-center justify-center font-black text-sm shadow-inner flex-shrink-0">
                            ${getInitials(name)}
                        </div>
                        <div class="flex-1 min-w-0">
                            <div class="font-bold text-slate-800 text-base truncate mb-1">${name}</div>

                            <div class="space-y-1">
                                ${phones.map(p => `
                                    <div class="text-[11px] text-indigo-600 font-mono flex items-center gap-1.5 bg-indigo-50/50 px-2 py-0.5 rounded-md w-fit">
                                        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"/></svg>
                                        ${p}
                                    </div>
                                `).join('')}

                                ${emails.map(em => `
                                    <div class="text-[11px] text-slate-500 truncate flex items-center gap-1.5 px-2">
                                        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/></svg>
                                        ${em}
                                    </div>
                                `).join('')}
                            </div>

                            ${addresses.length > 0 || organizations.length > 0 ? `
                                <div class="mt-3 pt-3 border-t border-slate-50 space-y-1.5">
                                    ${organizations.map(o => `
                                        <div class="text-[10px] text-slate-700 font-bold flex items-center gap-1.5">
                                            <svg class="w-3 h-3 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5"/></svg>
                                            ${o}
                                        </div>
                                    `).join('')}
                                    ${addresses.map(a => `
                                        <div class="text-[10px] text-slate-500 italic flex items-start gap-1.5 leading-tight">
                                            <svg class="w-3 h-3 text-slate-300 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"/><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"/></svg>
                                            ${a}
                                        </div>
                                    `).join('')}
                                </div>
                            ` : ''}

                            ${notes.length > 0 ? `
                                <div class="mt-2 bg-amber-50/40 p-2 rounded-xl text-[10px] text-amber-800 border border-amber-100/50">
                                    <div class="flex items-center gap-1 mb-1 font-bold opacity-60 uppercase text-[8px] tracking-widest">Note</div>
                                    ${notes.join(', ')}
                                </div>
                            ` : ''}
                        </div>
                    </div>
                `;
            }).join('');
        } catch (err) {
            console.error("Renderer Contacts Error:", err);
            return '<div class="text-red-500 text-xs p-8 text-center bg-red-50 rounded-2xl">Visual rendering failed.</div>';
        }
    },

    messages(sms) {
        try {
            if (!sms || !Array.isArray(sms) || sms.length === 0) {
                return '<div class="text-[10px] text-slate-400 p-8 text-center bg-slate-50/50 rounded-2xl border-2 border-dashed border-slate-100 italic">No messages found</div>';
            }

            return sms.map(m => {
                if (!m) return "";
                const isSent = m.type_code === 2;
                const address = m.address || "Unknown";
                const body = m.body || "";
                const dateStr = m.date ? new Date(m.date).toLocaleString([], {month: 'short', day: 'numeric', hour: '2-digit', minute:'2-digit'}) : "--:--";

                return `
                    <div class="flex flex-col ${isSent ? 'items-end' : 'items-start'} mb-1">
                        <div class="max-w-[90%] ${isSent ? 'bg-indigo-600 text-white rounded-2xl rounded-tr-none shadow-sm' : 'bg-white border border-slate-100 text-slate-700 rounded-2xl rounded-tl-none shadow-sm'} p-3 relative group">
                            <div class="flex justify-between items-center gap-6 mb-1 border-b ${isSent ? 'border-indigo-500/50' : 'border-slate-50'} pb-1">
                                <span class="text-[10px] font-black uppercase tracking-tighter ${isSent ? 'text-indigo-100' : 'text-indigo-600'}">${address}</span>
                                <span class="text-[8px] ${isSent ? 'text-indigo-300' : 'text-slate-400'} font-bold whitespace-nowrap">${dateStr}</span>
                            </div>
                            <p class="text-sm leading-relaxed whitespace-pre-wrap">${body}</p>
                        </div>
                    </div>
                `;
            }).join('');
        } catch (err) {
            console.error("Renderer Messages Error:", err);
            return '<div class="text-red-500 text-xs p-8 text-center bg-red-50 rounded-2xl">Visual rendering failed.</div>';
        }
    },

    calls(logs) {
        try {
            if (!logs || !Array.isArray(logs) || logs.length === 0) {
                return '<div class="text-[10px] text-slate-400 p-8 text-center bg-slate-50/50 rounded-2xl border-2 border-dashed border-slate-100 italic">No calls found</div>';
            }

            return logs.map(l => {
                if (!l) return "";
                const isMissed = l.type_code === 3;
                const isIncoming = l.type_code === 1;
                const isOutgoing = l.type_code === 2;

                // Cek apakah name/number adalah string literal "null" yang mungkin lolos
                const cleanName = (l.name && l.name.toLowerCase() !== "null") ? l.name : null;
                const cleanNumber = (l.number && l.number.toLowerCase() !== "null") ? l.number : null;
                const nameOrNumber = cleanName || cleanNumber || "Unknown Caller";

                const dateStr = l.date ? new Date(l.date).toLocaleString([], {month: 'short', day: 'numeric', hour: '2-digit', minute:'2-digit'}) : "Unknown Date";
                const duration = l.duration_seconds || 0;

                let icon = '';
                if (isMissed) icon = '<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M6 18L18 6M6 6l12 12" />';
                else if (isIncoming) icon = '<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M19 14l-7 7m0 0l-7-7m7 7V3" />';
                else icon = '<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M5 10l7-7m0 0l7 7m-7-7v18" />';

                return `
                    <div class="p-3 bg-white border border-slate-100 rounded-2xl flex items-center gap-4 shadow-sm hover:shadow-md transition-all group">
                        <div class="w-10 h-10 rounded-xl flex items-center justify-center flex-shrink-0 ${isMissed ? 'bg-red-50 text-red-500' : (isOutgoing ? 'bg-slate-50 text-slate-400' : 'bg-emerald-50 text-emerald-500')} shadow-inner">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                ${icon}
                            </svg>
                        </div>
                        <div class="flex-1 min-w-0">
                            <div class="font-bold text-slate-800 text-sm truncate group-hover:text-indigo-600 transition-colors">${nameOrNumber}</div>
                            <div class="text-[10px] text-slate-400 font-mono flex items-center gap-1">
                                <svg class="w-2.5 h-2.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-24 0 9 9 0 0124 0z"/></svg>
                                ${dateStr}
                            </div>
                        </div>
                        <div class="text-right">
                            <div class="text-[10px] font-black ${isMissed ? 'text-red-600' : 'text-slate-500'} uppercase tracking-widest">${duration}s</div>
                            ${l.location ? `<div class="text-[8px] text-slate-300 truncate max-w-[60px] italic">${l.location}</div>` : ''}
                        </div>
                    </div>
                `;
            }).join('');
        } catch (err) {
            console.error("Renderer Calls Error:", err);
            return '<div class="text-red-500 text-xs p-8 text-center bg-red-50 rounded-2xl">Visual rendering failed.</div>';
        }
    }
};
