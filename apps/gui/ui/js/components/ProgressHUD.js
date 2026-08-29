export class ProgressHUD extends HTMLElement {
    constructor() {
        super();
        this.innerHTML = `
            <div id="overlay" class="fixed bottom-6 right-6 w-80 bg-white rounded-2xl shadow-2xl border p-5 transform translate-y-32 opacity-0 transition-all duration-500 z-50">
                <div class="flex justify-between items-center mb-3">
                    <p id="title" class="text-xs font-black text-indigo-600 uppercase tracking-widest italic">Engine Active</p>
                    <span class="animate-ping h-2 w-2 rounded-full bg-indigo-400"></span>
                </div>
                <p id="msg" class="text-xs text-slate-500 truncate mb-4"></p>
                <div class="bg-slate-100 rounded-full h-2 overflow-hidden">
                    <div id="bar" class="bg-indigo-600 h-full transition-all duration-300" style="width: 0%"></div>
                </div>
            </div>
        `;
    }

    update(data) {
        const overlay = this.querySelector('#overlay');
        const bar = this.querySelector('#bar');
        const title = this.querySelector('#title');
        const msg = this.querySelector('#msg');

        overlay.classList.remove('translate-y-32', 'opacity-0');

        if (data.type === 'start') {
            title.textContent = "BACKUP STARTED";
            bar.style.width = '10%';
        } else if (data.type === 'inc') {
            title.textContent = "SYNCING DATA...";
            bar.style.width = '60%';
        } else if (data.type === 'finish') {
            title.textContent = "SUCCESS";
            bar.classList.remove('bg-red-500');
            bar.classList.add('bg-indigo-600');
            bar.style.width = '100%';
            setTimeout(() => {
                overlay.classList.add('translate-y-32', 'opacity-0');
            }, 3000);
        } else if (data.type === 'error') {
            title.textContent = "ENGINE ERROR";
            title.className = "text-xs font-black text-red-600 uppercase tracking-widest italic";
            bar.classList.remove('bg-indigo-600');
            bar.classList.add('bg-red-500');
            bar.style.width = '100%';
            setTimeout(() => {
                overlay.classList.add('translate-y-32', 'opacity-0');
            }, 5000);
        }

        msg.textContent = data.message || "";
    }
}
customElements.define('pb-progress-hud', ProgressHUD);
