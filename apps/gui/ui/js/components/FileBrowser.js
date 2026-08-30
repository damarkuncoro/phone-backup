import { FileListView } from './browser/FileListView.js';
import { AndroidDataView } from './browser/AndroidDataView.js';
import { DeviceService } from '../services/DeviceService.js';
import { BackupService } from '../services/BackupService.js';
import './MediaGallery.js';

export class FileBrowser extends HTMLElement {
    constructor() {
        super();
        this._selectedPaths = new Set();
        this._files = [];
        this._currentTab = 'summary';
        this._currentPath = "/storage/emulated/0";

        this.renderBase();

        this.listView = new FileListView(this.querySelector('#list'), {
            onToggle: (path, selected) => this._togglePath(path, selected),
            onRestore: (path) => this._onRestoreFile(path),
            onBrowse: (path) => this._browseTo(path),
            onDetails: (path) => this._onShowDetails(path),
            onDelete: (path) => this._onDeleteFile(path),
            onRename: (path, newName) => this._onRenameFile(path, newName),
            onCopy: (path, target) => this._onCopyFile(path, target),
            onUpload: () => this._onUploadFile()
        });

        this.dataView = new AndroidDataView(this.querySelector('#list'));
    }

    renderBase() {
        this.innerHTML = `
            <div id="container" class="bg-white rounded-3xl shadow-xl flex flex-col overflow-hidden ring-1 ring-slate-200 min-h-[700px]">
                <div class="p-8 border-b bg-slate-50/80 flex justify-between items-start backdrop-blur-md">
                    <div class="flex-1">
                        <div class="flex items-center gap-4 mb-2">
                            <button id="back-btn" class="p-2 -ml-2 hover:bg-white rounded-xl text-slate-400 hover:text-indigo-600 transition-all flex items-center gap-2 font-bold text-xs uppercase tracking-widest">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M15 19l-7-7 7-7"/></svg>
                                Back
                            </button>
                            <div id="breadcrumb" class="flex items-center gap-2 text-[10px] font-black text-slate-400 font-mono uppercase truncate max-w-xl"></div>
                        </div>
                        <h3 id="title" class="text-3xl font-black text-slate-800 tracking-tight">File Manager</h3>
                        <p id="subtitle" class="text-[10px] text-slate-400 font-mono mt-1 uppercase tracking-[0.2em] bg-slate-200/50 w-fit px-2 py-0.5 rounded"></p>

                        <div class="flex gap-6 mt-8">
                            <button id="tab-summary" class="text-[11px] font-black tracking-[0.3em] text-indigo-600 border-b-4 border-indigo-600 pb-2 transition-all">SUMMARY</button>
                            <button id="tab-files" class="text-[11px] font-black tracking-[0.3em] text-slate-400 hover:text-slate-600 pb-2 transition-all border-b-4 border-transparent">FILES</button>
                            <button id="tab-gallery" class="text-[11px] font-black tracking-[0.3em] text-slate-400 hover:text-slate-600 pb-2 transition-all border-b-4 border-transparent">GALLERY</button>
                            <button id="tab-data" class="text-[11px] font-black tracking-[0.3em] text-slate-400 hover:text-slate-600 pb-2 transition-all border-b-4 border-transparent">ANDROID DATA</button>
                        </div>
                    </div>
                </div>

                <div id="data-tabs-bar" class="hidden bg-indigo-50/50 border-b px-8 py-3 flex gap-8 backdrop-blur-sm">
                    <button data-subtab="contacts" class="text-[10px] font-black tracking-[0.2em] text-indigo-600 border-b-2 border-indigo-600 pb-1">CONTACTS</button>
                    <button data-subtab="messages" class="text-[10px] font-black tracking-[0.2em] text-slate-400 hover:text-slate-600 pb-1 border-b-2 border-transparent">MESSAGES</button>
                    <button data-subtab="calls" class="text-[10px] font-black tracking-[0.2em] text-slate-400 hover:text-slate-600 pb-1 border-b-2 border-transparent">CALL LOGS</button>
                    <button data-subtab="apps" class="text-[10px] font-black tracking-[0.2em] text-slate-400 hover:text-slate-600 pb-1 border-b-2 border-transparent">APPS</button>
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
        this.querySelector('#back-btn').onclick = () => {
            if (this._isScanMode && this._currentPath !== "/" && this._currentPath !== "") {
                const parts = this._currentPath.split('/');
                parts.pop();
                const parent = parts.join('/') || "/";
                this._browseTo(parent);
            } else {
                window.dispatchEvent(new CustomEvent('close-browser'));
            }
        };
        this.querySelector('#tab-summary').onclick = () => this.switchTab('summary');
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

        this.querySelectorAll('#tab-summary, #tab-files, #tab-gallery, #tab-data').forEach(t => {
            t.className = "text-[11px] font-black tracking-[0.3em] text-slate-400 hover:text-slate-600 pb-2 transition-all border-b-4 border-transparent";
        });
        this.querySelector(`#tab-${tab}`).className = "text-[11px] font-black tracking-[0.3em] text-indigo-600 border-b-4 border-indigo-600 pb-2 transition-all";

        if (tab === 'summary') {
            subTabs.classList.add('hidden');
            this.renderSummary();
        } else if (tab === 'files') {
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

    async renderSummary() {
        const container = this.querySelector('#list');

        const counts = {
            photos: this._files.filter(f => f.mime_type.startsWith('image/')).length,
            videos: this._files.filter(f => f.mime_type.startsWith('video/')).length,
            docs: this._files.filter(f => ['application/pdf', 'text/plain', 'application/msword'].includes(f.mime_type)).length,
            downloads: this._files.filter(f => f.path.includes('/Download/')).length,
        };

        let diffHtml = "";
        try {
            // Try to find a previous snapshot for diffing
            const allSnaps = await BackupService.getSnapshots(this._deviceId);
            const currentIdx = allSnaps.findIndex(s => s.id === this._snapshotId);
            if (currentIdx !== -1 && currentIdx < allSnaps.length - 1) {
                const prevId = allSnaps[currentIdx + 1].id;
                const diff = await BackupService.getDiff(prevId, this._snapshotId);

                diffHtml = `
                    <div class="mt-12 p-8 bg-slate-900 rounded-[3rem] text-white shadow-2xl relative overflow-hidden">
                        <div class="absolute top-0 right-0 p-8 opacity-10 text-6-xl font-black">DIFF</div>
                        <h4 class="text-xs font-black text-indigo-400 uppercase tracking-[0.3em] mb-8 flex items-center gap-3">
                            <span class="w-2 h-2 bg-indigo-500 rounded-full animate-pulse"></span>
                            Changes since last backup
                        </h4>
                        <div class="grid grid-cols-4 gap-8">
                            <div class="text-center">
                                <div class="text-3xl font-black text-emerald-400">+${diff.added.length}</div>
                                <div class="text-[9px] font-black text-slate-500 uppercase tracking-widest mt-2">New Files</div>
                            </div>
                            <div class="text-center">
                                <div class="text-3xl font-black text-amber-400">+${diff.modified.length}</div>
                                <div class="text-[9px] font-black text-slate-500 uppercase tracking-widest mt-2">Modified</div>
                            </div>
                            <div class="text-center">
                                <div class="text-3xl font-black text-red-400">-${diff.removed.length}</div>
                                <div class="text-[9px] font-black text-slate-500 uppercase tracking-widest mt-2">Deleted</div>
                            </div>
                            <div class="text-center">
                                <div class="text-3xl font-black text-slate-300">${this._files.length - diff.added.length - diff.modified.length}</div>
                                <div class="text-[9px] font-black text-slate-500 uppercase tracking-widest mt-2">Unchanged</div>
                            </div>
                        </div>
                    </div>
                `;
            }
        } catch (e) { console.error("Diff failed", e); }

        container.innerHTML = `
            <div class="p-12 space-y-8 max-w-5xl mx-auto">
                <div class="grid grid-cols-2 gap-6">
                    ${this.renderCategoryCard("Contacts", "👥", "ANDROID DATA", "data", "contacts")}
                    ${this.renderCategoryCard("Photos", "📸", `${counts.photos} items`, "gallery")}
                    ${this.renderCategoryCard("Videos", "🎥", `${counts.videos} items`, "gallery")}
                    ${this.renderCategoryCard("Documents", "📄", `${counts.docs} items`, "files")}
                    ${this.renderCategoryCard("Downloads", "📥", `${counts.downloads} items`, "files")}
                    ${this.renderCategoryCard("App Metadata", "📦", "System & User Apps", "data", "apps")}
                </div>
                ${diffHtml}
            </div>
        `;

        container.querySelectorAll('[data-target-tab]').forEach(card => {
            card.onclick = () => {
                const tab = card.dataset.targetTab;
                const sub = card.dataset.targetSubtab;
                this.switchTab(tab).then(() => {
                    if (sub) this.switchDataTab(sub);
                });
            };
        });
    }

    renderCategoryCard(name, icon, subtext, targetTab, targetSubtab = "") {
        return `
            <div class="p-8 bg-white border border-slate-100 rounded-[2.5rem] shadow-sm hover:shadow-xl hover:-translate-y-1 transition-all cursor-pointer group"
                 data-target-tab="${targetTab}" data-target-subtab="${targetSubtab}">
                <div class="flex items-center gap-6">
                    <div class="w-16 h-14 bg-slate-50 rounded-2xl flex items-center justify-center text-3xl group-hover:bg-indigo-50 transition-colors">
                        ${icon}
                    </div>
                    <div>
                        <div class="font-black text-slate-800 text-lg tracking-tight uppercase">${name}</div>
                        <div class="text-[10px] text-slate-400 font-bold uppercase tracking-widest mt-1">${subtext}</div>
                    </div>
                </div>
            </div>
        `;
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

    async show(snapshotId, files, deviceId = null, isScanMode = false) {
        this._snapshotId = snapshotId;
        this._files = files || [];
        this._deviceId = deviceId;
        this._isScanMode = isScanMode;
        this._selectedPaths.clear();

        this.querySelector('#footer').style.display = isScanMode ? 'flex' : 'none';
        this.querySelector('#subtitle').textContent = isScanMode ? `Device: ${deviceId}` : `Snapshot: ${snapshotId}`;
        this._updateCounter();

        if (isScanMode && deviceId) {
            await this._browseTo("/storage/emulated/0");
        } else {
            this.switchTab('summary');
        }
    }

    async _browseTo(path) {
        this._currentPath = path;
        this._renderBreadcrumb();
        try {
            const entries = await DeviceService.browse(this._deviceId, path);
            this._files = entries;
            this.switchTab('files');
        } catch (e) {
            console.error("Browse failed", e);
        }
    }

    _renderBreadcrumb() {
        const el = this.querySelector('#breadcrumb');
        const parts = this._currentPath.split('/').filter(x => x);
        el.innerHTML = `<span class="cursor-pointer hover:text-indigo-600" onclick="window.dispatchEvent(new CustomEvent('browse-path', {detail: '/'}))">ROOT</span>`;
        let current = "";
        parts.forEach(p => {
            current += "/" + p;
            const path = current;
            el.innerHTML += ` <span class="opacity-30">/</span> <span class="cursor-pointer hover:text-indigo-600" onclick="window.dispatchEvent(new CustomEvent('browse-path', {detail: '${path}'}))">${p}</span>`;
        });

        // Add event listener for breadcrumb clicks if not already added
        window.addEventListener('browse-path', (e) => this._browseTo(e.detail), { once: true });
    }

    async _onDeleteFile(path) {
        try {
            await DeviceService.deleteFile(this._deviceId, path);
            await this._browseTo(this._currentPath);
        } catch (e) { alert("Delete failed: " + e); }
    }

    async _onRenameFile(path, newName) {
        try {
            const parts = path.split('/');
            parts.pop();
            const newPath = parts.join('/') + "/" + newName;
            await DeviceService.renameFile(this._deviceId, path, newPath);
            await this._browseTo(this._currentPath);
        } catch (e) { alert("Rename failed: " + e); }
    }

    async _onShowDetails(path) {
        const file = this._files.find(f => f.path === path);
        if (!file) return;

        let details = `Path: ${file.path}\nName: ${file.name}\nSize: ${(file.size_bytes/1024/1024).toFixed(2)} MB\nModified: ${file.modified_at}\nMIME: ${file.mime_type}\n`;

        if (file.media_info) {
            details += `\n[Media Info]\nResolution: ${file.media_info.width}x${file.media_info.height}\n`;
            if (file.media_info.latitude) details += `Location: ${file.media_info.latitude}, ${file.media_info.longitude}\n`;
        }

        try {
            const hash = await DeviceService.calculateHash(this._deviceId, path);
            details += `\n[Integrity]\nSHA-256: ${hash}`;
        } catch (e) {
            details += `\n[Integrity]\nSHA-256: (Calculation failed)`;
        }

        alert(details);
    }

    async _onCopyFile(path, target) {
        try {
            await DeviceService.copyFile(this._deviceId, path, target);
            await this._browseTo(this._currentPath);
        } catch (e) { alert("Copy failed: " + e); }
    }

    async _onUploadFile() {
        const localPath = prompt("Enter local file path to upload:");
        if (!localPath) return;
        const remoteName = localPath.split(/[/\\]/).pop();
        const remotePath = (this._currentPath === "/" ? "" : this._currentPath) + "/" + remoteName;
        try {
            await DeviceService.uploadFile(this._deviceId, localPath, remotePath);
            await this._browseTo(this._currentPath);
        } catch (e) { alert("Upload failed: " + e); }
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
