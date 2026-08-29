export class SnapshotList extends HTMLElement {
    set snapshots(data) {
        if (!data || data.length === 0) {
            this.innerHTML = '<div class="p-12 text-center text-slate-400">No snapshots found for this device.</div>';
            return;
        }

        this.innerHTML = `
            <div class="divide-y divide-slate-50">
                ${data.map(s => {
                    const sId = Array.isArray(s.id) ? s.id[0] : s.id;
                    return `
                    <div class="snapshot-row flex items-center justify-between p-4 hover:bg-slate-50 transition-all cursor-pointer" id="snap-${sId}">
                        <div>
                            <div class="font-semibold text-sm text-slate-700">${new Date(s.started_at).toLocaleString()}</div>
                            <div class="text-[10px] text-slate-400 uppercase font-bold">${(s.total_bytes/1024/1024).toFixed(2)} MB</div>
                        </div>
                        <div class="flex items-center gap-3">
                            <span class="text-[10px] font-black px-2 py-0.5 rounded-full ${s.status==='Completed'?'bg-green-100 text-green-700':'bg-amber-100 text-amber-700'}">${s.status}</span>
                            <button class="restore-btn p-2 text-slate-400 hover:text-indigo-600 transition-colors" data-id="${sId}">
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-width="2.5" d="M4 16v1a2 2 0 002 2h10a2 2 0 002-2v-1m-4-4l-4 4m0 0l-4-4m4 4V4"/></svg>
                            </button>
                        </div>
                    </div>
                `}).join('')}
            </div>
        `;

        // Add event listeners
        this.querySelectorAll('.snapshot-row').forEach((row, index) => {
            const sId = Array.isArray(data[index].id) ? data[index].id[0] : data[index].id;
            row.onclick = (e) => {
                if (e.target.closest('.restore-btn')) return;
                window.dispatchEvent(new CustomEvent('browse-snapshot', { detail: sId }));
            };
        });

        this.querySelectorAll('.restore-btn').forEach(btn => {
            btn.onclick = (e) => {
                e.stopPropagation();
                window.dispatchEvent(new CustomEvent('restore-snapshot', { detail: btn.dataset.id }));
            };
        });
    }
}
customElements.define('pb-snapshot-list', SnapshotList);
