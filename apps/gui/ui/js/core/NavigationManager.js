/**
 * Manages UI view switching and sidebar states
 */
export class NavigationManager {
    constructor() {
        this.views = {
            dashboard: document.getElementById('dashboard-view'),
            browser: document.getElementById('browser-view'),
            contacts: document.getElementById('contacts-view')
        };

        this.navBtns = {
            dashboard: document.getElementById('nav-dashboard-btn'),
            devices: document.getElementById('nav-devices-btn'),
            contacts: document.getElementById('nav-contacts-btn')
        };
    }

    toggleView(viewName) {
        Object.values(this.views).forEach(v => v?.classList.add('hidden'));

        const target = this.views[viewName];
        if (target) {
            target.classList.remove('hidden');
            if (viewName === 'browser') window.scrollTo(0, 0);
        }
    }

    updateSidebar(activeTab) {
        // Logic for setting active view
        if (activeTab === 'contacts') {
            this.toggleView('contacts');
        } else {
            this.toggleView('dashboard');
        }

        const activeClass = "w-full flex items-center gap-3 px-4 py-3 text-sm font-bold bg-indigo-500 text-white rounded-xl shadow-lg shadow-indigo-500/20";
        const inactiveClass = "w-full flex items-center gap-3 px-4 py-3 text-sm font-bold text-indigo-200 hover:bg-white/5 rounded-xl transition-all";

        Object.entries(this.navBtns).forEach(([name, btn]) => {
            if (btn) btn.className = (activeTab === name) ? activeClass : inactiveClass;
        });

        if (activeTab === 'dashboard') {
            window.scrollTo({ top: 0, behavior: 'smooth' });
        } else if (activeTab === 'devices') {
            document.getElementById('device-list')?.scrollIntoView({ behavior: 'smooth' });
        }
    }
}
