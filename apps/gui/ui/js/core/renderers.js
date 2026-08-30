/**
 * Renderers untuk mengubah data mentah (JSON) menjadi HTML cantik.
 * Mengikuti prinsip SRP (Single Responsibility Principle).
 */
export const renderers = {
    contacts: (contacts, colors) => {
        return (contacts || []).map((c, i) => `
            <div class="p-5 bg-white border border-slate-100 rounded-3xl shadow-sm hover:shadow-xl transition-all group relative">
                <div class="flex items-center gap-4">
                    <div class="w-10 h-10 ${colors[i % colors.length]} text-white rounded-2xl flex items-center justify-center font-black text-xs shadow-inner">
                        ${(c.name || "?")[0].toUpperCase()}
                    </div>
                    <div class="min-w-0 flex-1">
                        <div class="font-bold text-slate-800 text-sm truncate">${c.name || "Unknown"}</div>
                        <div class="text-[10px] text-indigo-500 font-black uppercase tracking-widest">${(c.phones && c.phones.length > 0) ? c.phones[0] : "No Number"}</div>
                    </div>
                </div>
            </div>
        `).join('');
    },

    messages: (sms) => {
        return (sms || []).map(m => `
            <div class="p-4 border-l-4 border-indigo-400 bg-white shadow-sm rounded-r-2xl mb-3">
                <div class="flex justify-between mb-1">
                    <span class="text-[10px] font-black text-indigo-600 uppercase">${m.address}</span>
                    <span class="text-[9px] text-slate-400">${new Date(m.date).toLocaleString()}</span>
                </div>
                <p class="text-xs text-slate-600 leading-relaxed">${m.body}</p>
            </div>
        `).join('');
    },

    calls: (calls) => {
        return (calls || []).map(l => `
            <div class="p-4 bg-white border border-slate-100 rounded-2xl flex items-center justify-between">
                <div class="flex items-center gap-3">
                    <div class="text-lg">${l.type_code === 1 ? '📥' : '📤'}</div>
                    <div>
                        <div class="font-bold text-slate-700 text-sm">${l.number}</div>
                        <div class="text-[9px] text-slate-400 uppercase">${new Date(l.date).toLocaleDateString()}</div>
                    </div>
                </div>
                <div class="text-[10px] font-black text-slate-500">${Math.round(l.duration_seconds/60)} MIN</div>
            </div>
        `).join('');
    },

    apps: (apps) => {
        return (apps || []).map(a => `
            <div class="p-4 bg-white border border-slate-100 rounded-3xl flex items-center gap-4 hover:shadow-md transition-all">
                <div class="w-10 h-10 bg-slate-100 rounded-2xl flex items-center justify-center text-lg">📦</div>
                <div class="min-w-0 flex-1">
                    <div class="font-bold text-slate-800 text-xs truncate">${a.app_name}</div>
                    <div class="text-[9px] text-slate-400 truncate font-mono">${a.package_name}</div>
                    <div class="text-[8px] mt-1 bg-indigo-50 text-indigo-600 w-fit px-1.5 rounded font-black uppercase">v${a.version_code}</div>
                </div>
            </div>
        `).join('');
    }
};
