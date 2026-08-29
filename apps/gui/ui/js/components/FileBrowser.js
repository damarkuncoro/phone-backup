export class FileBrowser extends HTMLElement {
    constructor() {
        super();
        this.innerHTML = `
            <div id="modal" class="fixed inset-0 bg-slate-900/60 backdrop-blur-sm z-[100] hidden items-center justify-center p-4">
                <div class="bg-white w-full max-w-4xl max-h-[85vh] rounded-2xl shadow-2xl flex flex-col overflow-hidden">
                    <div class="p-6 border-b bg-slate-50 flex justify-between items-center">
                        <div>
                            <h3 id="title" class="text-xl font-bold text-slate-800">Snapshot Content</h3>
                            <p id="subtitle" class="text-[10px] text-slate-400 font-mono mt-1"></p>
                            <div class="flex gap-4 mt-3">
                                <button id="tab-files" class="text-xs font-black text-indigo-600 border-b-2 border-indigo-600 pb-1">FILES</button>
                                <button id="tab-data" class="text-xs font-bold text-slate-400 hover:text-slate-600 pb-1">ANDROID DATA</button>
                            </div>
                        </div>
                        <button id="close-btn" class="text-slate-400 hover:text-slate-600 text-3xl font-light">&times;</button>
                    </div>
                    <div id="search-container" class="p-4 border-b bg-white">
                        <input type="text" id="search" placeholder="Search..." class="w-full px-4 py-2 bg-slate-100 border-none rounded-xl focus:ring-2 focus:ring-indigo-500 outline-none text-sm">
                    </div>
                    <div id="list" class="flex-1 overflow-y-auto p-2 min-h-[300px]"></div>
                    <div class="p-4 bg-slate-50 border-t flex justify-between items-center">
                        <p id="counter" class="text-xs text-slate-500 font-bold uppercase"></p>
                        <div class="flex gap-2">
                            <button id="cancel-btn" class="px-6 py-2 rounded-xl font-bold text-slate-500 hover:bg-slate-200 transition-colors">Cancel</button>
                            <button id="action-btn" class="bg-indigo-600 text-white px-8 py-2 rounded-xl font-bold hover:bg-indigo-700 transition-all shadow-lg">Backup Selected</button>
                        </div>
                    </div>
                </div>
            </div>
        `;

        this.querySelector('#close-btn').onclick = () => this.hide();
        this.querySelector('#cancel-btn').onclick = () => this.hide();

        this.querySelector('#search').oninput = (e) => {
            this.filter(e.target.value);
        };

        this.querySelector('#tab-files').onclick = () => this.switchTab('files');
        this.querySelector('#tab-data').onclick = () => this.switchTab('data');

        this._selectedPaths = new Set();
        this._files = [];
        this._deviceId = null;
        this._isScanMode = false;
        this._currentTab = 'files';
        this._snapshotId = null;
    }

    async switchTab(tab) {
        this._currentTab = tab;
        const tFiles = this.querySelector('#tab-files');
        const tData = this.querySelector('#tab-data');

        if (tab === 'files') {
            tFiles.className = "text-xs font-black text-indigo-600 border-b-2 border-indigo-600 pb-1";
            tData.className = "text-xs font-bold text-slate-400 hover:text-slate-600 pb-1";
            this.render(this._files);
        } else {
            tData.className = "text-xs font-black text-indigo-600 border-b-2 border-indigo-600 pb-1";
            tFiles.className = "text-xs font-bold text-slate-400 hover:text-slate-600 pb-1";
            await this.renderAndroidData();
        }
    }

    async renderAndroidData() {
        const container = this.querySelector('#list');
        container.innerHTML = '<div class="p-12 text-center text-indigo-500 animate-pulse font-bold">Accessing Secure Android Storage...</div>';

        try {
            // Ambil data secara terpisah agar kita bisa tahu mana yang gagal
            let contacts = [];
            let sms = [];

            try {
                contacts = await window.__TAURI__.core.invoke('get_structured_data', { snapshotId: this._snapshotId, dataType: 'contacts' });
            } catch (err) {
                console.warn("Contacts load failed:", err);
            }

            try {
                sms = await window.__TAURI__.core.invoke('get_structured_data', { snapshotId: this._snapshotId, dataType: 'sms' });
            } catch (err) {
                console.warn("SMS load failed:", err);
            }

            if (contacts.length === 0 && sms.length === 0) {
                container.innerHTML = `
                    <div class="p-12 text-center">
                        <div class="text-slate-400 font-medium mb-4">No Android Data found in this snapshot.</div>
                        <div class="bg-amber-50 border border-amber-200 p-4 rounded-xl text-left inline-block max-w-md">
                            <p class="text-xs text-amber-700 font-bold mb-2 uppercase">💡 Tips:</p>
                            <ul class="text-xs text-amber-600 space-y-1 list-disc ml-4">
                                <li>Pastikan "USB Debugging (Security Settings)" aktif di HP Xiaomi Anda.</li>
                                <li>Izinkan akses Kontak/SMS saat muncul pop-up di layar HP.</li>
                                <li>Gunakan snapshot yang dibuat tanpa enkripsi asimetris untuk melihat data langsung di sini.</li>
                            </ul>
                        </div>
                    </div>
                `;
                return;
            }

            container.innerHTML = `
                <div class="p-4 space-y-8">
                    <section>
                        <h4 class="text-xs font-black text-slate-400 mb-4 uppercase tracking-widest">Contacts (${contacts.length})</h4>
                        ${contacts.map(c => `
                            <div class="p-3 bg-slate-50 rounded-xl mb-2 flex justify-between items-center">
                                <span class="font-bold text-slate-700">${c.name || "Unknown"}</span>
                                <span class="text-xs font-mono text-slate-500">${(c.phones && c.phones.length > 0) ? c.phones[0] : ""}</span>
                            </div>
                        `).join('')}
                    </section>
                    <section>
                        <h4 class="text-xs font-black text-slate-400 mb-4 uppercase tracking-widest">Messages (${sms.length})</h4>
                        ${sms.map(m => `
                            <div class="p-3 border-l-4 border-indigo-200 bg-white shadow-sm rounded-r-xl mb-3">
                                <div class="flex justify-between mb-1">
                                    <span class="text-[10px] font-black text-indigo-600">${m.address}</span>
                                    <span class="text-[10px] text-slate-400">${new Date(m.date).toLocaleString()}</span>
                                </div>
                                <p class="text-xs text-slate-600">${m.body}</p>
                            </div>
                        `).join('')}
                    </section>
                </div>
            `;
        } catch (e) {
            container.innerHTML = `<div class="p-12 text-center text-red-500">Error loading data: ${e}</div>`;
        }
    }

    show(snapshotId, files, deviceId = null, isScanMode = false) {
        this._snapshotId = snapshotId;
        console.log("FileBrowser.show() called. Files:", files?.length, "Mode:", isScanMode);
        this._files = files;
        this._deviceId = deviceId;
        this._isScanMode = isScanMode;
        this._selectedPaths.clear();

        const modal = this.querySelector('#modal');
        modal.classList.remove('hidden');
        modal.style.display = 'flex';

        this.querySelector('#title').textContent = isScanMode ? "Select Files to Backup" : "Snapshot Content";
        this.querySelector('#subtitle').textContent = isScanMode ? `Device: ${deviceId}` : `Snapshot: ${snapshotId}`;
        this.querySelector('#action-btn').style.display = isScanMode ? 'block' : 'none';

        this.querySelector('#action-btn').onclick = () => {
            if (this._selectedPaths.size > 0) {
                window.dispatchEvent(new CustomEvent('run-selective-backup', {
                    detail: { deviceId: this._deviceId, paths: Array.from(this._selectedPaths) }
                }));
                this.hide();
            } else {
                alert("Please select at least one file.");
            }
        };

        this.render(files);
        this.updateCounter();
    }

    hide() {
        this.querySelector('#modal').style.display = 'none';
    }

    updateCounter() {
        const counter = this.querySelector('#counter');
        counter.textContent = this._isScanMode ? `${this._selectedPaths.size} files selected` : "";
    }

    filter(query) {
        const q = query.toLowerCase();
        const filtered = this._files.filter(f => f.name.toLowerCase().includes(q) || f.path.toLowerCase().includes(q));
        this.render(filtered);
    }

    render(files) {
        const container = this.querySelector('#list');
        if (!files || files.length === 0) {
            container.innerHTML = '<div class="p-12 text-center text-slate-400 font-medium">No files found.</div>';
            return;
        }

        container.innerHTML = files.map(f => `
            <div class="flex items-center justify-between p-3 border-b border-slate-50 hover:bg-slate-50 rounded-xl transition-colors group cursor-pointer" data-path="${f.path}">
                <div class="flex items-center gap-4 flex-1 min-w-0">
                    ${this._isScanMode ? `<input type="checkbox" class="file-check w-5 h-5 rounded border-slate-300 text-indigo-600 focus:ring-indigo-500" ${this._selectedPaths.has(f.path) ? 'checked' : ''}>` : ''}
                    <div class="min-w-0">
                        <div class="font-medium text-slate-700 text-sm truncate">${f.name}</div>
                        <div class="text-[10px] text-slate-400 font-mono truncate">${f.path}</div>
                    </div>
                </div>
                <div class="text-right ml-4">
                    <div class="text-[10px] font-black text-slate-500">${(f.size_bytes/1024).toFixed(1)} KB</div>
                </div>
            </div>
        `).join('');

        // Add row click listener for selection
        container.querySelectorAll('.group').forEach(row => {
            row.onclick = (e) => {
                if (!this._isScanMode) return;
                const path = row.dataset.path;
                const checkbox = row.querySelector('.file-check');

                if (this._selectedPaths.has(path)) {
                    this._selectedPaths.delete(path);
                    if (checkbox) checkbox.checked = false;
                } else {
                    this._selectedPaths.add(path);
                    if (checkbox) checkbox.checked = true;
                }
                this.updateCounter();
            };
        });
    }
}
customElements.define('pb-file-browser', FileBrowser);
