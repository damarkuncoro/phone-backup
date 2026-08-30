import { DeviceService } from '../../services/DeviceService.js';
import { BackupService } from '../../services/BackupService.js';
import { renderers } from '../../core/renderers.js';

export class AndroidDataView {
    constructor(container) {
        this.container = container;
        this.dataCache = { contacts: [], sms: [], calls: [], apps: [] };
        this.currentSubTab = 'contacts';
    }

    async refresh(deviceId, snapshotId, isScanMode) {
        this.container.innerHTML = `
            <div class="flex flex-col items-center justify-center p-32">
                <div class="w-12 h-12 border-4 border-indigo-600 border-t-transparent rounded-full animate-spin mb-4"></div>
                <div class="text-indigo-600 font-black text-xs uppercase tracking-[0.3em] animate-pulse">Syncing Secure Data...</div>
            </div>
        `;

        try {
            if (isScanMode) {
                const [c, s, l, a] = await Promise.all([
                    DeviceService.getLiveData(deviceId, 'contacts').catch(() => []),
                    DeviceService.getLiveData(deviceId, 'sms').catch(() => []),
                    DeviceService.getLiveData(deviceId, 'call_logs').catch(() => []),
                    DeviceService.getLiveData(deviceId, 'apps').catch(() => [])
                ]);
                this.dataCache = { contacts: c, sms: s, calls: l, apps: a };
            } else {
                const [c, s, l, a] = await Promise.all([
                    BackupService.getStructuredData(snapshotId, 'contacts').catch(() => []),
                    BackupService.getStructuredData(snapshotId, 'sms').catch(() => []),
                    BackupService.getStructuredData(snapshotId, 'call_logs').catch(() => []),
                    BackupService.getApps(snapshotId).catch(() => [])
                ]);
                this.dataCache = { contacts: c, sms: s, calls: l, apps: a };
            }
            this.render();
        } catch (e) {
            this.container.innerHTML = `<div class="p-20 text-center text-red-500 font-black text-xs uppercase tracking-widest">Connection Failed</div>`;
        }
    }

    render(filteredData = null) {
        const colors = ['bg-blue-500', 'bg-purple-500', 'bg-indigo-500', 'bg-pink-500', 'bg-teal-500'];
        const data = filteredData || this.dataCache;

        let contentHtml = "";
        let count = 0;

        if (this.currentSubTab === 'contacts') {
            contentHtml = renderers.contacts(data.contacts, colors);
            count = (data.contacts || []).length;
        } else if (this.currentSubTab === 'messages') {
            contentHtml = renderers.messages(data.sms);
            count = (data.sms || []).length;
        } else if (this.currentSubTab === 'calls') {
            contentHtml = renderers.calls(data.calls);
            count = (data.calls || []).length;
        } else {
            contentHtml = renderers.apps(data.apps);
            count = (data.apps || []).length;
        }

        this.container.innerHTML = `
            <div class="p-6 border-b bg-white/50 sticky top-0 z-10 backdrop-blur-md">
                <input type="text" id="data-search" placeholder="Search ${this.currentSubTab}..." class="w-full px-8 py-4 bg-slate-50 border-2 border-transparent rounded-2xl text-sm outline-none focus:border-indigo-100 focus:bg-white transition-all font-medium">
            </div>
            <div class="p-8 bg-slate-50/30 min-h-full">
                <div class="max-w-6xl mx-auto">
                    <div class="flex items-center justify-between mb-8">
                        <h4 class="text-[11px] font-black text-slate-400 uppercase tracking-[0.3em] flex items-center gap-3">
                            <span class="w-2 h-2 bg-indigo-600 rounded-full"></span>
                            Showing ${count} ${this.currentSubTab}
                        </h4>
                    </div>
                    <div class="${this.currentSubTab === 'messages' ? 'space-y-4' : 'grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6'}">
                        ${contentHtml}
                    </div>
                </div>
            </div>
        `;

        this.setupListeners();
    }

    setupListeners() {
        const search = this.container.querySelector('#data-search');
        if (search) {
            search.oninput = (e) => {
                const q = e.target.value.toLowerCase();
                const filtered = {
                    contacts: this.dataCache.contacts.filter(c => (c.name || "").toLowerCase().includes(q) || (c.phones || []).some(p => p.includes(q))),
                    sms: this.dataCache.sms.filter(m => (m.address || "").toLowerCase().includes(q) || (m.body || "").toLowerCase().includes(q)),
                    calls: this.dataCache.calls.filter(l => (l.name || "").toLowerCase().includes(q) || (l.number || "").includes(q)),
                    apps: this.dataCache.apps.filter(a => (a.app_name || "").toLowerCase().includes(q) || (a.package_name || "").toLowerCase().includes(q))
                };
                this.render(filtered);
                const input = this.container.querySelector('#data-search');
                input.focus();
                input.value = e.target.value;
            };
        }
    }
}
