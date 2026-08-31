import { utils } from '../core/utils.js';

/**
 * Detail Panel (Right Sidebar)
 * Displays reactive information about devices, files, or background jobs.
 */
export class DetailPanel extends HTMLElement {
    constructor() {
        super();
        this.renderEmpty();
    }

    renderEmpty() {
        this.innerHTML = `
            <div class="p-12 h-full flex flex-col items-center justify-center text-center space-y-4">
                <div class="w-20 h-20 bg-slate-50 rounded-[2.5rem] flex items-center justify-center text-3xl">ℹ️</div>
                <div>
                    <h4 class="font-black text-slate-800 uppercase tracking-widest text-[10px]">Information</h4>
                    <p class="text-slate-400 text-xs mt-2 leading-relaxed">Select a device or file to view detailed metadata and actions.</p>
                </div>
            </div>
        `;
    }

    set device(data) {
        const id = utils.getSafeId(data);
        const storageUsed = utils.formatBytes(data.storage_used_bytes);
        const storageTotal = utils.formatBytes(data.storage_total_bytes);
        const storagePercent = data.storage_total_bytes > 0
            ? Math.round((data.storage_used_bytes / data.storage_total_bytes) * 100)
            : 0;

        this.innerHTML = `
            <div class="p-8 space-y-8 animate-in fade-in slide-in-from-right duration-500">
                <header class="text-center space-y-4">
                    <div class="w-24 h-24 bg-indigo-600 text-white rounded-[3rem] mx-auto flex items-center justify-center text-4xl shadow-xl shadow-indigo-200 font-black">
                        ${data.model[0]}
                    </div>
                    <div>
                        <h3 class="text-2xl font-black text-slate-800 tracking-tight">${data.model}</h3>
                        <p class="text-[10px] text-slate-400 font-bold uppercase tracking-[0.2em] mt-1">${data.manufacturer} • ANDROID ${data.os_version}</p>
                    </div>
                </header>

                <div class="space-y-6">
                    <!-- Status Section -->
                    <div class="bg-slate-50 p-6 rounded-[2rem] space-y-4">
                        <div class="flex justify-between items-center">
                            <span class="text-[10px] font-black text-slate-400 uppercase tracking-widest">Connection</span>
                            <span class="text-[10px] font-black bg-emerald-100 text-emerald-600 px-2 py-0.5 rounded-full uppercase tracking-tighter">${data.connection_type}</span>
                        </div>
                        <div class="flex justify-between items-center">
                            <span class="text-[10px] font-black text-slate-400 uppercase tracking-widest">Serial Number</span>
                            <span class="text-xs font-mono text-slate-600">${id}</span>
                        </div>
                    </div>

                    <!-- Storage Section -->
                    <div class="space-y-4">
                        <div class="flex justify-between items-end">
                            <h4 class="text-[10px] font-black text-slate-800 uppercase tracking-widest">Storage Capacity</h4>
                            <span class="text-[10px] font-bold text-slate-500">${storageUsed} / ${storageTotal}</span>
                        </div>
                        <div class="h-4 w-full bg-slate-100 rounded-2xl overflow-hidden p-1">
                            <div class="h-full bg-indigo-600 rounded-xl transition-all duration-1000" style="width: ${storagePercent}%"></div>
                        </div>
                        <p class="text-[10px] text-slate-400 italic text-right">${storagePercent}% utilized</p>
                    </div>

                    <!-- Quick Actions -->
                    <div class="pt-4 grid grid-cols-1 gap-3">
                        <button class="w-full bg-indigo-600 text-white py-4 rounded-2xl font-black text-xs shadow-xl shadow-indigo-100 hover:bg-indigo-700 transition-all uppercase tracking-widest" onclick="window.runBackup('${id}')">Start Full Backup</button>
                        <button class="w-full bg-white border border-slate-200 text-slate-600 py-4 rounded-2xl font-black text-xs hover:bg-slate-50 transition-all uppercase tracking-widest" onclick="window.viewSnapshots('${id}')">Browse History</button>
                    </div>
                </div>
            </div>
        `;
    }
}
customElements.define('pb-detail-panel', DetailPanel);
