import { utils } from '../core/utils.js';
import { DeviceService } from '../services/DeviceService.js';

export class DeviceItem extends HTMLElement {
    set device(data) {
        const id = utils.getSafeId(data);
        const storageUsed = utils.formatBytes(data.storage_used_bytes);
        const storageTotal = utils.formatBytes(data.storage_total_bytes);
        const storagePercent = data.storage_total_bytes > 0
            ? Math.round((data.storage_used_bytes / data.storage_total_bytes) * 100)
            : 0;

        this.className = "group flex flex-col p-6 hover:bg-slate-50 transition-all border-b border-slate-50";
        this.innerHTML = `
            <div class="flex items-start justify-between mb-4">
                <div class="flex items-center gap-4">
                    <div class="w-14 h-14 bg-indigo-100 text-indigo-600 rounded-2xl flex items-center justify-center font-black text-sm shadow-inner transition-transform group-hover:scale-105">
                        ${data.manufacturer ? data.manufacturer[0].toUpperCase() : '?'}${data.model ? data.model[0].toUpperCase() : '?'}
                    </div>
                    <div>
                        <div class="flex items-center gap-2">
                            <div class="font-black text-slate-800 text-xl tracking-tight">${data.model}</div>
                            <div id="battery-indicator" class="text-[10px] bg-slate-100 text-slate-500 px-2 py-0.5 rounded-full font-black opacity-0 transition-all">--%</div>
                            <div class="text-[10px] bg-emerald-100 text-emerald-600 px-2 py-0.5 rounded-full font-black uppercase tracking-widest">${data.connection_type}</div>
                        </div>
                        <div class="flex items-center gap-3 mt-1">
                            <div class="text-[11px] text-slate-400 font-bold uppercase tracking-wider">${data.manufacturer}</div>
                            <div class="w-1 h-1 bg-slate-200 rounded-full"></div>
                            <div class="text-[11px] text-indigo-500 font-black">ANDROID ${data.os_version}</div>
                        </div>
                    </div>
                </div>
                <div class="flex gap-2">
                    <button id="scan-btn" class="px-3 py-2 text-[10px] font-black text-slate-600 border border-slate-200 rounded-xl hover:bg-white hover:shadow-sm transition-all uppercase tracking-widest">Scan</button>
                    <button id="history-btn" class="px-4 py-2 text-[10px] font-black text-indigo-600 border border-indigo-100 rounded-xl hover:bg-white hover:shadow-sm transition-all uppercase tracking-widest">History</button>
                    <button id="backup-btn" class="bg-indigo-600 text-white px-5 py-2 rounded-xl text-[10px] font-black shadow-lg shadow-indigo-200 hover:bg-indigo-700 hover:-translate-y-0.5 transition-all uppercase tracking-widest">Backup All</button>
                </div>
            </div>

            <div class="grid grid-cols-3 gap-6 mt-2">
                <div class="flex flex-col gap-1">
                    <div class="text-[9px] font-black text-slate-400 uppercase tracking-widest">Device ID</div>
                    <div class="text-xs text-slate-600 font-mono bg-slate-100/50 px-2 py-1 rounded-lg w-fit">${id}</div>
                </div>
                <div class="col-span-2 flex flex-col gap-2">
                    <div class="flex justify-between items-end">
                        <div class="text-[9px] font-black text-slate-400 uppercase tracking-widest">Internal Storage</div>
                        <div class="text-[10px] font-bold text-slate-500">${storageUsed} / ${storageTotal} (${storagePercent}%)</div>
                    </div>
                    <div class="h-1.5 w-full bg-slate-100 rounded-full overflow-hidden">
                        <div class="h-full bg-indigo-500 rounded-full" style="width: ${storagePercent}%"></div>
                    </div>
                </div>
            </div>
        `;

        this.updateBattery(id);

        this.querySelector('#scan-btn').onclick = () => {
            window.dispatchEvent(new CustomEvent('scan-device', { detail: id }));
        };

        this.querySelector('#history-btn').onclick = () => {
            window.dispatchEvent(new CustomEvent('view-history', { detail: id }));
        };

        this.querySelector('#backup-btn').onclick = () => {
            window.dispatchEvent(new CustomEvent('run-backup', { detail: id }));
        };

        window.addEventListener('device-status-update', (e) => {
            if (e.detail.device_id === id) {
                this.renderBattery(e.detail.battery_level, e.detail.temperature);
            }
        });
    }

    async updateBattery(id) {
        try {
            const [level, temp] = await DeviceService.getBattery(id);
            this.renderBattery(level, temp);
        } catch (e) {
            console.warn("Could not fetch battery for", id);
        }
    }

    renderBattery(level, temp) {
        const indicator = this.querySelector('#battery-indicator');
        if (indicator) {
            indicator.textContent = `${level}%`;
            indicator.title = `${temp}°C`;
            indicator.classList.remove('opacity-0');

            if (level < 20) {
                indicator.className = "text-[10px] bg-red-100 text-red-600 px-2 py-0.5 rounded-full font-black transition-all";
            } else if (level > 80) {
                indicator.className = "text-[10px] bg-green-100 text-green-600 px-2 py-0.5 rounded-full font-black transition-all";
            } else {
                indicator.className = "text-[10px] bg-blue-100 text-blue-600 px-2 py-0.5 rounded-full font-black transition-all";
            }
        }
    }
}
customElements.define('pb-device-item', DeviceItem);
