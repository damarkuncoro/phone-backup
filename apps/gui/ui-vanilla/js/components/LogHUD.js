export class LogHUD extends HTMLElement {
    constructor() {
        super();
        this.innerHTML = `
            <div id="log-window" class="fixed bottom-32 right-6 w-96 bg-slate-900/90 text-white rounded-2xl shadow-2xl p-4 hidden flex-col border border-slate-700 backdrop-blur-md">
                <div class="flex justify-between items-center mb-3 border-b border-slate-700 pb-2">
                    <span class="text-[9px] font-black uppercase tracking-[0.2em] text-slate-400">Engine Live Logs</span>
                    <button id="clear-logs" class="text-[9px] font-bold text-slate-500 hover:text-white">CLEAR</button>
                </div>
                <div id="log-content" class="flex-1 overflow-y-auto max-h-64 font-mono text-[10px] space-y-1 custom-scrollbar">
                </div>
            </div>
        `;

        this.querySelector('#clear-logs').onclick = () => {
            this.querySelector('#log-content').innerHTML = "";
        };
    }

    add(message) {
        const content = this.querySelector('#log-content');
        const window = this.querySelector('#log-window');

        window.classList.remove('hidden');
        window.style.display = 'flex';

        const line = document.createElement('div');
        line.className = "flex gap-2 opacity-0 translate-y-1 transition-all duration-300";
        line.innerHTML = `
            <span class="text-indigo-400">[${new Date().toLocaleTimeString()}]</span>
            <span class="text-slate-300">${message}</span>
        `;

        content.appendChild(line);
        setTimeout(() => line.classList.remove('opacity-0', 'translate-y-1'), 10);

        content.scrollTop = content.scrollHeight;

        // Auto-hide after inactivity if needed, but for now just keep it.
    }

    hide() {
        this.querySelector('#log-window').style.display = 'none';
    }
}
customElements.define('pb-log-hud', LogHUD);
