export class Sidebar extends HTMLElement {
    constructor() {
        super();
        this.render();
    }

    render() {
        this.innerHTML = `
            <div class="p-8 border-b border-white/10">
                <div class="flex items-center gap-3">
                    <div class="w-8 h-8 bg-indigo-500 rounded-lg flex items-center justify-center shadow-lg shadow-indigo-500/20">
                        <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z"/></svg>
                    </div>
                    <span class="text-xl font-black tracking-tighter italic">PB PRO</span>
                </div>
            </div>

            <div class="flex-1 overflow-y-auto p-4 space-y-8 mt-4">
                <!-- Quick Search -->
                <div class="px-2">
                    <label class="text-[10px] font-black text-indigo-400 uppercase tracking-widest ml-2 mb-2 block">Quick Search</label>
                    <div class="relative">
                        <input type="text" id="global-search" placeholder="Search objects..." class="w-full bg-indigo-900/50 border border-white/10 text-white placeholder-indigo-300/50 px-4 py-3 rounded-xl text-sm outline-none focus:ring-2 focus:ring-indigo-500/50 transition-all">
                        <kbd class="absolute right-3 top-3 text-[10px] bg-white/10 px-1.5 py-0.5 rounded text-indigo-300 font-mono">⏎</kbd>
                    </div>
                </div>

                <nav class="space-y-1">
                    <button id="nav-dashboard-btn" class="nav-btn active">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/></svg>
                        Dashboard
                    </button>
                    <button id="nav-devices-btn" class="nav-btn">
                        <svg class="w-5 h-5 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z"/></svg>
                        Devices
                    </button>
                    <button id="nav-contacts-btn" class="nav-btn">
                        <svg class="w-5 h-5 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/></svg>
                        Global Contacts
                    </button>
                    <button id="settings-btn" class="nav-btn">
                        <svg class="w-5 h-5 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/></svg>
                        System Settings
                    </button>
                </nav>
            </div>

            <div class="p-6 border-t border-white/5 bg-indigo-950">
                <div class="flex items-center gap-3 text-xs font-black text-white/40 uppercase tracking-widest">
                    <div class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></div>
                    Engine Online
                </div>
            </div>

            <style>
                .nav-btn {
                    width: 100%; display: flex; align-items: center; gap: 0.75rem;
                    padding: 0.75rem 1rem; font-size: 0.875rem; font-weight: 700;
                    border-radius: 0.75rem; transition: all 0.3s; color: #c7d2fe;
                }
                .nav-btn:hover { background: rgba(255,255,255,0.05); }
                .nav-btn.active {
                    background: #6366f1; color: white;
                    box-shadow: 0 10px 15px -3px rgba(99, 102, 241, 0.2);
                }
            </style>
        `;
    }
}
customElements.define('pb-sidebar', Sidebar);
