import { utils } from '../core/utils.js';
import { DeviceService } from '../services/DeviceService.js';

export class DeviceItem extends HTMLElement {
    set device(data) {
        const id = utils.getSafeId(data);

        this.className = "flex items-center justify-between p-6 hover:bg-slate-50 transition-colors border-b border-slate-50";
        this.innerHTML = `
            <div class="flex items-center gap-4">
                <div class="w-12 h-12 bg-indigo-100 text-indigo-600 rounded-2xl flex items-center justify-center font-black text-xs">
                    ${data.manufacturer ? data.manufacturer[0] : '?'}${data.model ? data.model[0] : '?'}
                </div>
                <div>
                    <div class="flex items-center gap-2">
                        <div class="font-bold text-slate-800 text-lg">${data.model}</div>
                        <div id="battery-indicator" class="text-[10px] bg-slate-100 text-slate-500 px-2 py-0.5 rounded-full font-black opacity-0 transition-all">--%</div>
                    </div>
                    <div class="text-xs text-slate-400 font-mono">${id}</div>
                </div>
            </div>
            <div class="flex gap-2">
                <button id="scan-btn" class="px-3 py-2 text-xs font-bold text-slate-600 border border-slate-200 rounded-lg hover:bg-slate-50">SCAN</button>
                <button id="history-btn" class="px-4 py-2 text-xs font-bold text-indigo-600 border border-indigo-100 rounded-lg hover:bg-indigo-50">HISTORY</button>
                <button id="backup-btn" class="bg-indigo-600 text-white px-5 py-2 rounded-lg text-xs font-bold shadow-sm hover:bg-indigo-700">BACKUP ALL</button>
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
    }

    async updateBattery(id) {
        try {
            const [level, temp] = await DeviceService.getBattery(id);
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
        } catch (e) {
            console.warn("Could not fetch battery for", id);
        }
    }
}
customElements.define('pb-device-item', DeviceItem);
