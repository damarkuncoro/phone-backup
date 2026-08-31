/**
 * Core Layout Component (Rails-style "application.html.erb")
 * Defines the 3-area structure: Sidebar | Main Content | Details
 */
export class AppLayout extends HTMLElement {
    constructor() {
        super();
        this.innerHTML = `
            <div class="flex h-screen w-screen overflow-hidden bg-slate-50 font-sans">
                <!-- AREA 1: SIDEBAR (Fixed 280px) -->
                <aside id="app-sidebar" class="w-[280px] bg-indigo-950 text-white flex flex-col flex-shrink-0 shadow-2xl z-50 transition-all duration-500">
                    <!-- Sidebar content will be static or injected -->
                </aside>

                <!-- AREA 2: MAIN CONTENT (Flexible) -->
                <main id="app-content" class="flex-1 flex flex-col h-screen overflow-hidden relative">
                    <!-- Views (Dashboard, Browser, etc.) will be injected here -->
                    <div id="view-container" class="flex-1 overflow-y-auto custom-scrollbar relative"></div>
                </main>

                <!-- AREA 3: DETAIL PANEL (Fixed 340px, Toggleable) -->
                <aside id="app-details" class="w-[340px] bg-white border-l border-slate-100 flex flex-col flex-shrink-0 z-40 transition-all duration-500 overflow-y-auto">
                    <pb-detail-panel id="detail-panel"></pb-detail-panel>
                </aside>
            </div>
        `;
    }

    get container() { return this.querySelector('#view-container'); }
    get sidebar() { return this.querySelector('#app-sidebar'); }
    get details() { return this.querySelector('#app-details'); }

    toggleDetails(show) {
        if (show === undefined) {
            this.details.classList.toggle('hidden');
        } else {
            show ? this.details.classList.remove('hidden') : this.details.classList.add('hidden');
        }
    }
}
customElements.define('pb-app-layout', AppLayout);
