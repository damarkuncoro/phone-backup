export class FileListView {
    constructor(container, options = {}) {
        this.container = container;
        this.onToggle = options.onToggle || (() => {});
        this.onRestore = options.onRestore || (() => {});
        this.isScanMode = options.isScanMode || false;
        this.selectedPaths = options.selectedPaths || new Set();
        this.snapshotId = options.snapshotId || null;
    }

    render(files) {
        if (!files || files.length === 0) {
            this.container.innerHTML = `
                <div class="p-32 text-center text-slate-300 flex flex-col items-center">
                    <svg class="w-16 h-16 mb-6 opacity-20" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-width="1.5" d="M5 19a2 2 0 01-2-2V7a2 2 0 012-2h4l2 2h4a2 2 0 012 2v1M5 19h14a2 2 0 002-2v-5a2 2 0 00-2-2H9l-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/></svg>
                    <div class="font-black text-[10px] uppercase tracking-[0.4em]">No Media Files found</div>
                </div>
            `;
            return;
        }

        this.container.innerHTML = `
            <div class="p-6 border-b bg-white/80 sticky top-0 z-10 backdrop-blur-md">
                <input type="text" id="file-search" placeholder="Search files in this set..." class="w-full px-8 py-4 bg-slate-100 border-none rounded-2xl text-sm outline-none focus:ring-2 focus:ring-indigo-500/20 transition-all font-medium">
            </div>
            <div class="divide-y divide-slate-50 p-4">
                ${files.map(f => `
                    <div class="flex items-center justify-between p-5 hover:bg-slate-50 rounded-3xl transition-all group cursor-pointer" data-path="${f.path}">
                        <div class="flex items-center gap-6 min-w-0">
                            ${this.isScanMode ? `<input type="checkbox" class="file-check w-6 h-6 rounded-xl border-slate-300 text-indigo-600 focus:ring-indigo-500 transition-all" ${this.selectedPaths.has(f.path) ? 'checked' : ''}>` : ''}
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
                            <div class="text-[10px] font-black text-slate-400 uppercase tracking-tighter">${(f.size_bytes/1024/1024).toFixed(2)} MB</div>
                            ${!this.isScanMode ? `
                                <button class="restore-single-btn p-2 bg-indigo-50 text-indigo-600 rounded-xl opacity-0 group-hover:opacity-100 transition-all hover:bg-indigo-600 hover:text-white" title="Restore this file" data-path="${f.path}">
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-width="2.5" d="M4 16v1a2 2 0 002 2h10a2 2 0 002-2v-1m-4-4l-4 4m0 0l-4-4m4 4V4"/></svg>
                                </button>
                            ` : ''}
                        </div>
                    </div>
                `).join('')}
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

        this.container.querySelectorAll('[data-path]').forEach(row => {
            row.onclick = (e) => {
                if (e.target.type === 'checkbox' || e.target.closest('button')) return;
                const checkbox = row.querySelector('.file-check');
                if (checkbox) {
                    checkbox.checked = !checkbox.checked;
                    this.onToggle(row.getAttribute('data-path'), checkbox.checked);
                }
            };
            const checkbox = row.querySelector('.file-check');
            if (checkbox) {
                checkbox.onchange = (e) => this.onToggle(row.getAttribute('data-path'), e.target.checked);
            }

            const restoreBtn = row.querySelector('.restore-single-btn');
            if (restoreBtn) {
                restoreBtn.onclick = (e) => {
                    e.stopPropagation();
                    this.onRestore(row.getAttribute('data-path'));
                };
            }
        });
    }
}
