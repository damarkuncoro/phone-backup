import { BackupService } from '../services/BackupService.js';
import { DeviceService } from '../services/DeviceService.js';
import { renderers } from '../core/renderers.js';

export class FileBrowser extends HTMLElement {
    constructor() {
        super();
        this.innerHTML = `
            <div id="container" class="bg-white rounded-3xl shadow-xl flex flex-col overflow-hidden ring-1 ring-slate-200 min-h-[700px]">
                <div class="p-8 border-b bg-slate-50/80 flex justify-between items-start backdrop-blur-md">
                    <div class="flex-1">
                        <div class="flex items-center gap-4 mb-2">
                            <button id="back-btn" class="p-2 -ml-2 hover:bg-white rounded-xl text-slate-400 hover:text-indigo-600 transition-all flex items-center gap-2 font-bold text-xs">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M15 19l-7-7 7-7"/></svg>
                                BACK TO DASHBOARD
                            </button>
                        </div>
                        <h3 id="title" class="text-3xl font-black text-slate-800 tracking-tight">Data Browser</h3>
                        <p id="subtitle" class="text-[10px] text-slate-400 font-mono mt-1 uppercase tracking-[0.2em] bg-slate-200/50 w-fit px-2 py-0.5 rounded"></p>

                        <div class="flex gap-6 mt-8">
                            <button id="tab-files" class="text-[11px] font-black tracking-[0.3em] text-indigo-600 border-b-4 border-indigo-600 pb-2 transition-all">FILES</button>
                            <button id="tab-data" class="text-[11px] font-black tracking-[0.3em] text-slate-400 hover:text-slate-600 pb-2 transition-all border-b-4 border-transparent">ANDROID DATA</button>
                        </div>
                    </div>
                </div>

                <div id="data-tabs-bar" class="hidden bg-indigo-50/50 border-b px-8 py-3 flex gap-8 backdrop-blur-sm">
                    <button data-subtab="contacts" class="text-[10px] font-black tracking-[0.2em] text-indigo-600 border-b-2 border-indigo-600 pb-1">CONTACTS</button>
                    <button data-subtab="messages" class="text-[10px] font-black tracking-[0.2em] text-slate-400 hover:text-slate-600 pb-1 border-b-2 border-transparent">MESSAGES</button>
                    <button data-subtab="calls" class="text-[10px] font-black tracking-[0.2em] text-slate-400 hover:text-slate-600 pb-1 border-b-2 border-transparent">CALL LOGS</button>
                </div>

                <div id="list" class="flex-1 overflow-y-auto bg-white min-h-[500px]"></div>

                <div id="footer" class="p-6 bg-slate-50 border-t flex justify-between items-center hidden">
                    <div class="flex flex-col">
                        <p id="counter" class="text-[10px] text-slate-500 font-black uppercase tracking-widest"></p>
                        <p class="text-[9px] text-slate-400 mt-1 italic">Click items to select them for backup</p>
                    </div>
                    <button id="action-btn" class="bg-indigo-600 text-white px-12 py-4 rounded-2xl font-black text-xs hover:bg-indigo-700 transition-all shadow-xl shadow-indigo-200 uppercase tracking-[0.2em]">Backup Selected</button>
                </div>
            </div>
        `;

        this.setupListeners();
        this._selectedPaths = new Set();
        this._files = [];
        this._currentTab = 'files';
        this._currentDataTab = 'contacts';
        this._dataCache = { contacts: [], sms: [], calls: [] };
    }

    setupListeners() {
        this.querySelector('#back-btn').onclick = () => {
            window.dispatchEvent(new CustomEvent('close-browser'));
        };

        this.querySelector('#tab-files').onclick = () => this.switchTab('files');
        this.querySelector('#tab-data').onclick = () => this.switchTab('data');

        this.querySelectorAll('[data-subtab]').forEach(btn => {
            btn.onclick = () => this.switchDataTab(btn.dataset.subtab);
        });

        this.querySelector('#action-btn').onclick = () => {
            if (this._selectedPaths.size === 0) return;
            window.dispatchEvent(new CustomEvent('run-selective-backup', {
                detail: { deviceId: this._deviceId, paths: Array.from(this._selectedPaths) }
            }));
            window.dispatchEvent(new CustomEvent('close-browser'));
        };
    }

    async switchTab(tab) {
        this._currentTab = tab;
        const tFiles = this.querySelector('#tab-files');
        const tData = this.querySelector('#tab-data');
        const subTabs = this.querySelector('#data-tabs-bar');

        if (tab === 'files') {
            tFiles.className = "text-[11px] font-black tracking-[0.3em] text-indigo-600 border-b-4 border-indigo-600 pb-2";
            tData.className = "text-[11px] font-black tracking-[0.3em] text-slate-400 hover:text-slate-600 pb-2 border-b-4 border-transparent";
            subTabs.classList.add('hidden');
            this.renderFileList(this._files);
        } else {
            tData.className = "text-[11px] font-black tracking-[0.3em] text-indigo-600 border-b-4 border-indigo-600 pb-2";
            tFiles.className = "text-[11px] font-black tracking-[0.3em] text-slate-400 hover:text-slate-600 pb-2 border-b-4 border-transparent";
            subTabs.classList.remove('hidden');
            await this.refreshAndroidData();
        }
    }

    async switchDataTab(subtab) {
        this._currentDataTab = subtab;
        this.querySelectorAll('[data-subtab]').forEach(btn => {
            if (btn.dataset.subtab === subtab) {
                btn.className = "text-[10px] font-black tracking-[0.2em] text-indigo-600 border-b-2 border-indigo-600 pb-1";
            } else {
                btn.className = "text-[10px] font-black tracking-[0.2em] text-slate-400 hover:text-slate-600 pb-1 border-b-2 border-transparent";
            }
        });
        this.renderActiveDataTab();
    }

    async refreshAndroidData() {
        const container = this.querySelector('#list');
        container.innerHTML = `
            <div class="flex flex-col items-center justify-center p-32">
                <div class="w-12 h-12 border-4 border-indigo-600 border-t-transparent rounded-full animate-spin mb-4"></div>
                <div class="text-indigo-600 font-black text-xs uppercase tracking-[0.3em] animate-pulse">Syncing Secure Data...</div>
            </div>
        `;

        try {
            if (this._isScanMode) {
                const [c, s, l] = await Promise.all([
                    DeviceService.getLiveData(this._deviceId, 'contacts').catch(() => []),
                    DeviceService.getLiveData(this._deviceId, 'sms').catch(() => []),
                    DeviceService.getLiveData(this._deviceId, 'call_logs').catch(() => [])
                ]);
                this._dataCache = { contacts: c, sms: s, calls: l };
            } else {
                const [c, s, l] = await Promise.all([
                    BackupService.getStructuredData(this._snapshotId, 'contacts').catch(() => []),
                    BackupService.getStructuredData(this._snapshotId, 'sms').catch(() => []),
                    BackupService.getStructuredData(this._snapshotId, 'call_logs').catch(() => [])
                ]);
                this._dataCache = { contacts: c, sms: s, calls: l };
            }
            this.renderActiveDataTab();
        } catch (e) {
            container.innerHTML = `<div class="p-20 text-center text-red-500 font-black text-xs uppercase tracking-widest">Connection Failed</div>`;
        }
    }

    renderActiveDataTab(filteredData = null) {
        const container = this.querySelector('#list');
        const colors = ['bg-blue-500', 'bg-purple-500', 'bg-indigo-500', 'bg-pink-500', 'bg-teal-500'];
        const data = filteredData || this._dataCache;

        let contentHtml = "";
        let count = 0;

        if (this._currentDataTab === 'contacts') {
            contentHtml = renderers.contacts(data.contacts, colors);
            count = (data.contacts || []).length;
        } else if (this._currentDataTab === 'messages') {
            contentHtml = renderers.messages(data.sms);
            count = (data.sms || []).length;
        } else {
            contentHtml = renderers.calls(data.calls);
            count = (data.calls || []).length;
        }

        container.innerHTML = `
            <div class="p-6 border-b bg-white/50 sticky top-0 z-10 backdrop-blur-md">
                <input type="text" id="data-search" placeholder="Search ${this._currentDataTab}..." class="w-full px-8 py-4 bg-slate-50 border-2 border-transparent rounded-2xl text-sm outline-none focus:border-indigo-100 focus:bg-white transition-all font-medium">
            </div>
            <div class="p-8 bg-slate-50/30 min-h-full">
                <div class="max-w-6xl mx-auto">
                    <div class="flex items-center justify-between mb-8">
                        <h4 class="text-[11px] font-black text-slate-400 uppercase tracking-[0.3em] flex items-center gap-3">
                            <span class="w-2 h-2 bg-indigo-600 rounded-full"></span>
                            Showing ${count} ${this._currentDataTab}
                        </h4>
                    </div>
                    <div class="${this._currentDataTab === 'messages' ? 'space-y-4' : 'grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6'}">
                        ${contentHtml}
                    </div>
                </div>
            </div>
        `;

        const search = this.querySelector('#data-search');
        search.oninput = (e) => {
            const q = e.target.value.toLowerCase();
            const filtered = {
                contacts: this._dataCache.contacts.filter(c => (c.name || "").toLowerCase().includes(q) || (c.phones || []).some(p => p.includes(q))),
                sms: this._dataCache.sms.filter(m => (m.address || "").toLowerCase().includes(q) || (m.body || "").toLowerCase().includes(q)),
                calls: this._dataCache.calls.filter(l => (l.name || "").toLowerCase().includes(q) || (l.number || "").includes(q))
            };
            this.renderActiveDataTab(filtered);
            const input = this.querySelector('#data-search');
            input.focus();
            input.value = e.target.value;
        };
    }

    show(snapshotId, files, deviceId = null, isScanMode = false) {
        this._snapshotId = snapshotId;
        this._files = files || [];
        this._deviceId = deviceId;
        this._isScanMode = isScanMode;
        this._selectedPaths.clear();

        this.querySelector('#footer').style.display = isScanMode ? 'flex' : 'none';
        this.querySelector('#title').textContent = isScanMode ? "Select Media to Backup" : "Snapshot Explorer";
        this.querySelector('#subtitle').textContent = isScanMode ? `Device: ${deviceId}` : `Snapshot: ${snapshotId}`;

        this.switchTab('files');
    }

    renderFileList(files) {
        const container = this.querySelector('#list');
        if (!files || files.length === 0) {
            container.innerHTML = `
                <div class="p-32 text-center text-slate-300 flex flex-col items-center">
                    <svg class="w-16 h-16 mb-6 opacity-20" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-width="1.5" d="M5 19a2 2 0 01-2-2V7a2 2 0 012-2h4l2 2h4a2 2 0 012 2v1M5 19h14a2 2 0 002-2v-5a2 2 0 00-2-2H9l-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/></svg>
                    <div class="font-black text-[10px] uppercase tracking-[0.4em]">No Media Files found</div>
                </div>
            `;
            return;
        }

        container.innerHTML = `
            <div class="p-6 border-b bg-white/80 sticky top-0 z-10 backdrop-blur-md">
                <input type="text" id="file-search" placeholder="Search files in this set..." class="w-full px-8 py-4 bg-slate-100 border-none rounded-2xl text-sm outline-none focus:ring-2 focus:ring-indigo-500/20 transition-all font-medium">
            </div>
            <div class="divide-y divide-slate-50 p-4">
                ${files.map(f => `
                    <div class="flex items-center justify-between p-5 hover:bg-slate-50 rounded-3xl transition-all group cursor-pointer" data-path="${f.path}">
                        <div class="flex items-center gap-6 min-w-0">
                            ${this._isScanMode ? `<input type="checkbox" class="file-check w-6 h-6 rounded-xl border-slate-300 text-indigo-600 focus:ring-indigo-500 transition-all" ${this._selectedPaths.has(f.path) ? 'checked' : ''}>` : ''}
                            <div class="truncate">
                                <div class="font-bold text-slate-700 text-sm truncate group-hover:text-indigo-600 transition-colors uppercase tracking-tight">${f.name}</div>
                                <div class="text-[10px] text-slate-400 font-mono truncate mt-1 tracking-tighter">${f.path}</div>
                            </div>
                        </div>
                        <div class="text-right ml-6 flex-shrink-0">
                            <div class="text-[10px] font-black text-slate-400 uppercase tracking-tighter">${(f.size_bytes/1024/1024).toFixed(2)} MB</div>
                        </div>
                    </div>
                `).join('')}
            </div>
        `;

        const search = container.querySelector('#file-search');
        if (search) {
            search.oninput = (e) => {
                const q = e.target.value.toLowerCase();
                const items = container.querySelectorAll('[data-path]');
                items.forEach(item => {
                    const path = item.getAttribute('data-path').toLowerCase();
                    const name = item.querySelector('.font-bold').textContent.toLowerCase();
                    if (path.includes(q) || name.includes(q)) item.style.display = 'flex';
                    else item.style.display = 'none';
                });
            };
        }

        container.querySelectorAll('[data-path]').forEach(row => {
            row.onclick = (e) => {
                if (e.target.type === 'checkbox') return;
                const checkbox = row.querySelector('.file-check');
                if (checkbox) {
                    checkbox.checked = !checkbox.checked;
                    this._togglePath(row.getAttribute('data-path'), checkbox.checked);
                }
            };
            const checkbox = row.querySelector('.file-check');
            if (checkbox) {
                checkbox.onchange = (e) => this._togglePath(row.getAttribute('data-path'), e.target.checked);
            }
        });
    }

    _togglePath(path, selected) {
        if (selected) this._selectedPaths.add(path);
        else this._selectedPaths.delete(path);
        this._updateCounter();
    }

    _updateCounter() {
        const counter = this.querySelector('#counter');
        if (counter) counter.textContent = `${this._selectedPaths.size} items selected for protection`;
    }
}
customElements.define('pb-file-browser', FileBrowser);
