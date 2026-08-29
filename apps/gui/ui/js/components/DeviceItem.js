import { utils } from '../core/utils.js';

export class DeviceItem extends HTMLElement {
    set device(data) {
        console.log("Rendering DeviceItem for:", data);

        const id = utils.getSafeId(data);

        this.className = "flex items-center justify-between p-6 hover:bg-slate-50 transition-colors border-b border-slate-50";
        this.innerHTML = `
            <div>
                <div class="font-bold text-slate-800 text-lg">${data.model}</div>
                <div class="text-xs text-slate-400 font-mono">${id}</div>
            </div>
            <div class="flex gap-2">
                <button id="scan-btn" class="px-3 py-2 text-xs font-bold text-slate-600 border border-slate-200 rounded-lg hover:bg-slate-50">SCAN</button>
                <button id="schedule-btn" class="px-3 py-2 text-xs font-bold text-slate-600 border border-slate-200 rounded-lg hover:bg-slate-50">SCHEDULE</button>
                <button id="history-btn" class="px-4 py-2 text-xs font-bold text-indigo-600 border border-indigo-100 rounded-lg hover:bg-indigo-50">HISTORY</button>
                <button id="backup-btn" class="bg-indigo-600 text-white px-5 py-2 rounded-lg text-xs font-bold shadow-sm hover:bg-indigo-700">BACKUP ALL</button>
            </div>
        `;

        this.querySelector('#scan-btn').onclick = () => {
            window.dispatchEvent(new CustomEvent('scan-device', { detail: id }));
        };

        this.querySelector('#schedule-btn').onclick = () => {
            window.dispatchEvent(new CustomEvent('add-schedule', { detail: id }));
        };

        this.querySelector('#history-btn').onclick = () => {
            console.log("History clicked for", id);
            window.dispatchEvent(new CustomEvent('view-history', { detail: id }));
        };
        this.querySelector('#backup-btn').onclick = () => {
            console.log("Backup clicked for", id);
            window.dispatchEvent(new CustomEvent('run-backup', { detail: id }));
        };
    }
}
customElements.define('pb-device-item', DeviceItem);
