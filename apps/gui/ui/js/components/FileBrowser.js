import { FileListView } from './browser/FileListView.js';
import { AndroidDataView } from './browser/AndroidDataView.js';
import './MediaGallery.js';

export class FileBrowser extends HTMLElement {
    constructor() {
        super();
        this.renderBase();

        this._selectedPaths = new Set();
        this._files = [];
        this._currentTab = 'files';

        this.listView = new FileListView(this.querySelector('#list'), {
            onToggle: (path, selected) => this._togglePath(path, selected),
            onRestore: (path) => this._onRestoreFile(path)
        });

        this.dataView = new AndroidDataView(this.querySelector('#list'));
    }

    renderBase() {
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
                            <button id="tab-gallery" class="text-[11px] font-black tracking-[0.3em] text-slate-400 hover:text-slate-600 pb-2 transition-all border-b-4 border-transparent">GALLERY</button>
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
    }

    setupListeners() {
        this.querySelector('#back-btn').onclick = () => window.dispatchEvent(new CustomEvent('close-browser'));
        this.querySelector('#tab-files').onclick = () => this.switchTab('files');
        this.querySelector('#tab-gallery').onclick = () => this.switchTab('gallery');
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
        const subTabs = this.querySelector('#data-tabs-bar');

        this.querySelectorAll('#tab-files, #tab-gallery, #tab-data').forEach(t => {
            t.className = "text-[11px] font-black tracking-[0.3em] text-slate-400 hover:text-slate-600 pb-2 transition-all border-b-4 border-transparent";
        });
        this.querySelector(`#tab-${tab}`).className = "text-[11px] font-black tracking-[0.3em] text-indigo-600 border-b-4 border-indigo-600 pb-2 transition-all";

        if (tab === 'files') {
            subTabs.classList.add('hidden');
            this.listView.isScanMode = this._isScanMode;
            this.listView.selectedPaths = this._selectedPaths;
            this.listView.render(this._files);
        } else if (tab === 'gallery') {
            subTabs.classList.add('hidden');
            this.renderGallery(this._files);
        } else {
            subTabs.classList.remove('hidden');
            await this.dataView.refresh(this._deviceId, this._snapshotId, this._isScanMode);
        }
    }

    renderGallery(files) {
        const container = this.querySelector('#list');
        container.innerHTML = `<pb-media-grid></pb-media-grid>`;
        container.querySelector('pb-media-grid').media = files;
    }

    async switchDataTab(subtab) {
        this.dataView.currentSubTab = subtab;
        this.querySelectorAll('[data-subtab]').forEach(btn => {
            btn.className = (btn.dataset.subtab === subtab)
                ? "text-[10px] font-black tracking-[0.2em] text-indigo-600 border-b-2 border-indigo-600 pb-1"
                : "text-[10px] font-black tracking-[0.2em] text-slate-400 hover:text-slate-600 pb-1 border-b-2 border-transparent";
        });
        this.dataView.render();
    }

    show(snapshotId, files, deviceId = null, isScanMode = false) {
        this._snapshotId = snapshotId;
        this._files = files || [];
        this._deviceId = deviceId;
        this._isScanMode = isScanMode;
        this._selectedPaths.clear();

        this.querySelector('#footer').style.display = isScanMode ? 'flex' : 'none';
        this.querySelector('#subtitle').textContent = isScanMode ? `Device: ${deviceId}` : `Snapshot: ${snapshotId}`;
        this._updateCounter();

        this.switchTab('files');
    }

    _togglePath(path, selected) {
        if (selected) this._selectedPaths.add(path);
        else this._selectedPaths.delete(path);
        this._updateCounter();
    }

    _onRestoreFile(path) {
        window.dispatchEvent(new CustomEvent('restore-file', {
            detail: { snapshotId: this._snapshotId, path }
        }));
    }

    _updateCounter() {
        const counter = this.querySelector('#counter');
        if (counter) counter.textContent = `${this._selectedPaths.size} items selected for protection`;
    }
}
customElements.define('pb-file-browser', FileBrowser);
