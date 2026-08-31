export class FileListView {
    constructor(container, options = {}) {
        this.container = container;
        this.onToggle = options.onToggle || (() => {});
        this.onRestore = options.onRestore || (() => {});
        this.onBrowse = options.onBrowse || (() => {});
        this.onDetails = options.onDetails || (() => {});
        this.onDelete = options.onDelete || (() => {});
        this.onRename = options.onRename || (() => {});
        this.onCopy = options.onCopy || (() => {});
        this.onUpload = options.onUpload || (() => {});
        this.isScanMode = options.isScanMode || false;
        this.selectedPaths = options.selectedPaths || new Set();
        this.snapshotId = options.snapshotId || null;
    }

    render(files) {
        if (!files || files.length === 0) {
            this.container.innerHTML = `
                <div class="p-32 text-center text-slate-300 flex flex-col items-center">
                    <svg class="w-16 h-16 mb-6 opacity-20" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-width="1.5" d="M5 19a2 2 0 01-2-2V7a2 2 0 012-2h4l2 2h4a2 2 0 012 2v1M5 19h14a2 2 0 002-2v-5a2 2 0 00-2-2H9l-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/></svg>
                    <div class="font-black text-[10px] uppercase tracking-[0.4em]">Empty or No Files found</div>
                </div>
            `;
            return;
        }

        // Sort: Directories first, then files
        const sorted = [...files].sort((a, b) => {
            const aDir = a.permissions === 'd';
            const bDir = b.permissions === 'd';
            if (aDir && !bDir) return -1;
            if (!aDir && bDir) return 1;
            return a.name.localeCompare(b.name);
        });

        this.container.innerHTML = `
            <div class="p-6 border-b bg-white/80 sticky top-0 z-10 backdrop-blur-md flex gap-4">
                <input type="text" id="file-search" placeholder="Search in this folder..." class="flex-1 px-8 py-4 bg-slate-100 border-none rounded-2xl text-sm outline-none focus:ring-2 focus:ring-indigo-500/20 transition-all font-medium">
                ${this.isScanMode ? `
                    <button id="upload-btn" class="px-6 bg-indigo-50 text-indigo-600 rounded-2xl font-black text-[10px] uppercase tracking-widest hover:bg-indigo-600 hover:text-white transition-all">Upload</button>
                ` : ''}
            </div>
            <div class="divide-y divide-slate-50 p-4">
                ${sorted.map(f => {
                    const isDir = f.permissions === 'd';
                    return `
                    <div class="flex items-center justify-between p-5 hover:bg-slate-50 rounded-3xl transition-all group cursor-pointer" data-path="${f.path}" data-is-dir="${isDir}">
                        <div class="flex items-center gap-6 min-w-0">
                            ${this.isScanMode ? `<input type="checkbox" class="file-check w-6 h-6 rounded-xl border-slate-300 text-indigo-600 focus:ring-indigo-500 transition-all" ${this.selectedPaths.has(f.path) ? 'checked' : ''}>` : ''}
                            <div class="w-10 h-10 flex-shrink-0 flex items-center justify-center rounded-xl ${isDir ? 'bg-amber-50 text-amber-500' : 'bg-slate-50 text-slate-400'}">
                                ${isDir ? `
                                    <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20"><path d="M2 6a2 2 0 012-2h5l2 2h5a2 2 0 012 2v6a2 2 0 01-2 2H4a2 2 0 01-2-2V6z"/></svg>
                                ` : `
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-width="2" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"/></svg>
                                `}
                            </div>
                            <div class="truncate">
                                <div class="flex items-center gap-2">
                                    <div class="font-bold text-slate-700 text-sm truncate group-hover:text-indigo-600 transition-colors uppercase tracking-tight">${f.name}</div>
                                    ${f.media_info?.width ? `<span class="text-[8px] bg-slate-100 text-slate-500 px-1 rounded font-black">${f.media_info.width}x${f.media_info.height}</span>` : ''}
                                    ${f.media_info?.latitude ? `<span class="text-[8px] bg-indigo-50 text-indigo-500 px-1 rounded font-black">📍 GPS</span>` : ''}
                                </div>
                                <div class="text-[10px] text-slate-400 font-mono truncate mt-1 tracking-tighter">${f.path}</div>
                            </div>
                        </div>
                        <div class="flex items-center gap-4 ml-6 flex-shrink-0">
                            ${!isDir ? `<div class="text-[10px] font-black text-slate-400 uppercase tracking-tighter">${(f.size_bytes/1024/1024).toFixed(2)} MB</div>` : ''}

                            <div class="flex gap-1 opacity-0 group-hover:opacity-100 transition-all">
                                ${this.isScanMode ? `
                                    <button class="details-btn p-2 hover:bg-slate-200 rounded-xl" title="Details"><svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/></svg></button>
                                    <button class="copy-btn p-2 hover:bg-slate-200 rounded-xl" title="Copy"><svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-width="2" d="M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2M8 7H6a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2v-2"/></svg></button>
                                    <button class="rename-btn p-2 hover:bg-slate-200 rounded-xl" title="Rename"><svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/></svg></button>
                                    <button class="delete-btn p-2 hover:bg-red-50 hover:text-red-600 rounded-xl" title="Delete"><svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/></svg></button>
                                ` : `
                                    <button class="restore-single-btn p-2 bg-indigo-50 text-indigo-600 rounded-xl hover:bg-indigo-600 hover:text-white" title="Restore this file">
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-width="2.5" d="M4 16v1a2 2 0 002 2h10a2 2 0 002-2v-1m-4-4l-4 4m0 0l-4-4m4 4V4"/></svg>
                                    </button>
                                `}
                            </div>
                        </div>
                    </div>
                `;}).join('')}
            </div>
        `;

        this.setupListeners();
    }

    setupListeners() {
        const search = this.container.querySelector('#file-search');
        if (search) {
            search.oninput = (e) => {
                const q = e.target.value.toLowerCase();
                const items = this.container.querySelectorAll('[data-path]');
                items.forEach(item => {
                    const path = item.getAttribute('data-path').toLowerCase();
                    const name = item.querySelector('.font-bold').textContent.toLowerCase();
                    if (path.includes(q) || name.includes(q)) item.style.display = 'flex';
                    else item.style.display = 'none';
                });
            };
        }

        const uploadBtn = this.container.querySelector('#upload-btn');
        if (uploadBtn) uploadBtn.onclick = () => this.onUpload();

        this.container.querySelectorAll('[data-path]').forEach(row => {
            const path = row.getAttribute('data-path');
            const isDir = row.getAttribute('data-is-dir') === 'true';

            row.onclick = (e) => {
                if (e.target.type === 'checkbox' || e.target.closest('button')) return;
                if (isDir) {
                    this.onBrowse(path);
                } else {
                    const checkbox = row.querySelector('.file-check');
                    if (checkbox) {
                        checkbox.checked = !checkbox.checked;
                        this.onToggle(path, checkbox.checked);
                    }
                }
            };

            const checkbox = row.querySelector('.file-check');
            if (checkbox) {
                checkbox.onchange = (e) => this.onToggle(path, e.target.checked);
            }

            const restoreBtn = row.querySelector('.restore-single-btn');
            if (restoreBtn) {
                restoreBtn.onclick = (e) => {
                    e.stopPropagation();
                    this.onRestore(path);
                };
            }

            const detailsBtn = row.querySelector('.details-btn');
            if (detailsBtn) {
                detailsBtn.onclick = (e) => {
                    e.stopPropagation();
                    this.onDetails(path);
                };
            }

            const copyBtn = row.querySelector('.copy-btn');
            if (copyBtn) {
                copyBtn.onclick = (e) => {
                    e.stopPropagation();
                    const target = prompt("Copy to (full path):", path + "_copy");
                    if (target) this.onCopy(path, target);
                };
            }

            const renameBtn = row.querySelector('.rename-btn');
            if (renameBtn) {
                renameBtn.onclick = (e) => {
                    e.stopPropagation();
                    const newName = prompt("New name:", path.split('/').pop());
                    if (newName) this.onRename(path, newName);
                };
            }

            const deleteBtn = row.querySelector('.delete-btn');
            if (deleteBtn) {
                deleteBtn.onclick = (e) => {
                    e.stopPropagation();
                    if (confirm(`Delete ${isDir ? 'folder' : 'file'}: ${path}?`)) {
                        this.onDelete(path);
                    }
                };
            }
        });
    }
}
