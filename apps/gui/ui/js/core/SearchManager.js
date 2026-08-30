import { api } from './api.js';

/**
 * Handles global search functionality for files and contacts
 */
export class SearchManager {
    constructor(app) {
        this.app = app;
        this.fileInput = document.getElementById('global-search');
        this.contactInput = document.getElementById('contact-global-search');
        this.contactContainer = document.getElementById('global-contacts-results');

        this.init();
    }

    init() {
        if (this.fileInput) {
            this.fileInput.onkeydown = async (e) => {
                if (e.key === 'Enter') {
                    const query = e.target.value;
                    if (!query) return;
                    this.app.notifier?.show(`Searching for "${query}"...`, "info");
                    try {
                        const files = await api.invoke('search_files', { query });
                        this.app.nav.toggleView('browser');
                        if (this.app.browser) this.app.browser.show("Search Results", files);
                    } catch (err) {
                        this.app.notifier?.show("Search failed: " + err, "error");
                    }
                }
            };
        }

        if (this.contactInput) {
            this.contactInput.oninput = async (e) => {
                const query = e.target.value;
                if (query.length < 2) return;
                try {
                    const results = await api.invoke('search_contacts', { query });
                    this.renderGlobalContacts(results);
                } catch (err) { console.error(err); }
            };
        }
    }

    renderGlobalContacts(results) {
        if (!this.contactContainer) return;

        if (!results || results.length === 0) {
            this.contactContainer.innerHTML = '<div class="col-span-full p-20 text-center text-slate-300 font-bold uppercase tracking-widest">No matching contacts</div>';
            return;
        }

        const colors = ['bg-blue-500', 'bg-purple-500', 'bg-indigo-500', 'bg-pink-500', 'bg-teal-500'];

        this.contactContainer.innerHTML = results.map((res, i) => {
            const c = res.contact;
            const sId = res.snapshot_id;
            const name = c.display_name || "Unknown";
            const getInitials = (n) => n.split(' ').filter(x => x).map(x => x[0]).join('').substring(0, 2).toUpperCase() || "?";

            const org = (c.organizations || [])[0] || {};
            const jobTitle = org.title || "";

            return `
                <div class="p-6 bg-white border border-slate-100 rounded-[2rem] shadow-sm hover:shadow-xl transition-all cursor-pointer group relative" onclick="window.viewSnapshotDetails('${sId}')">
                    <div class="absolute top-4 right-6 text-[8px] font-black bg-slate-100 text-slate-400 px-2 py-1 rounded-full uppercase tracking-tighter">Snapshot: ${sId.substring(0,8)}</div>
                    <div class="flex items-center gap-4 mb-4 mt-2">
                        <div class="w-12 h-12 ${colors[i % colors.length]} text-white rounded-2xl flex items-center justify-center font-black text-sm shadow-inner">
                            ${getInitials(name)}
                        </div>
                        <div class="min-w-0 flex-1">
                            <div class="font-bold text-slate-800 text-lg truncate">${name}</div>
                            <div class="text-[10px] text-indigo-500 font-black uppercase tracking-widest">${jobTitle}</div>
                        </div>
                    </div>
                    <div class="space-y-1">
                        ${(c.phones || []).slice(0,2).map(p => `<div class="text-xs text-slate-600 font-mono flex items-center gap-2"><span class="opacity-30">📞</span> ${p.raw_value}</div>`).join('')}
                        ${(c.emails || []).slice(0,1).map(em => `<div class="text-xs text-slate-400 truncate flex items-center gap-2"><span class="opacity-30">✉️</span> ${em.value}</div>`).join('')}
                    </div>
                </div>
            `;
        }).join('');
    }
}
