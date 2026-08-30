import { api } from './core/api.js';
import { store } from './core/store.js';
import { utils } from './core/utils.js';
import { DeviceService } from './services/DeviceService.js';
import { BackupService } from './services/BackupService.js';
import { SettingsService } from './services/SettingsService.js';

import './components/StatCard.js';
import './components/DeviceItem.js';
import './components/ProgressHUD.js';
import './components/SnapshotList.js';
import './components/FileBrowser.js';
import './components/SettingsModal.js';
import './components/Notification.js';
import './components/LogHUD.js';

class App {
    constructor() {
        console.log("Modular App Orchestrator Instantiated");

        // Expose critical functions to window for atomic component interoperability
        window.runBackup = (id, paths) => this.runBackup(id, paths);
        window.loadDevices = () => this.refreshAll();
        window.viewSnapshots = (id) => this.loadHistory(id);
        window.viewSnapshotDetails = (id) => this.browse(id);
        window.runKeygen = async () => {
            try {
                const [secret, public_key] = await SettingsService.generateNewKeys();
                prompt("New Keypair Generated!\n\nPUBLIC KEY (Safe to share):\n" + public_key + "\n\nSECRET KEY (KEEP HIDDEN!):", secret);
            } catch (e) { this.notifier.show("Keygen failed: " + e, "error"); }
        };

        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', () => this.init());
        } else {
            this.init();
        }
    }

    async init() {
        this.hud = document.getElementById('global-progress');
        this.logHud = document.getElementById('log-hud');
        this.browser = document.getElementById('file-browser');
        this.settings = document.getElementById('settings-modal');
        this.notifier = document.getElementById('notifier');

        // Setup UI Listeners
        const testBtn = document.getElementById('test-js-btn');
        if (testBtn) testBtn.onclick = () => {
            this.notifier.show("Modular UI Core: Active", "success");
            if (this.logHud) this.logHud.add("User initiated system self-test");
        };

        const settingsBtn = document.getElementById('settings-btn');
        if (settingsBtn) settingsBtn.onclick = () => this.settings && this.settings.show();

        const refreshBtn = document.getElementById('refresh-btn');
        if (refreshBtn) refreshBtn.onclick = () => this.refreshAll();

        // Sidebar Navigation
        const dashBtn = document.getElementById('nav-dashboard-btn');
        const devBtn = document.getElementById('nav-devices-btn');
        const conBtn = document.getElementById('nav-contacts-btn');

        if (dashBtn) dashBtn.onclick = () => this.updateSidebar('dashboard');
        if (devBtn) devBtn.onclick = () => this.updateSidebar('devices');
        if (conBtn) conBtn.onclick = () => this.updateSidebar('contacts');

        // Register Global Event Bus
        store.addEventListener('change', (e) => this.handleStateChange(e.detail));
        window.addEventListener('run-backup', (e) => this.runBackup(e.detail));
        window.addEventListener('run-selective-backup', (e) => this.runBackup(e.detail.deviceId, e.detail.paths));
        window.addEventListener('view-history', (e) => this.loadHistory(e.detail));
        window.addEventListener('scan-device', (e) => this.runScan(e.detail));
        window.addEventListener('restore-snapshot', (e) => this.runRestore(e.detail));
        window.addEventListener('restore-file', (e) => this.runRestore(e.detail.snapshotId, e.detail.path));
        window.addEventListener('add-schedule', (e) => this.addSchedule(e.detail));
        window.addEventListener('browse-snapshot', (e) => this.browse(e.detail));
        window.addEventListener('close-browser', () => this.updateSidebar('dashboard'));

        // Global Search
        const searchInput = document.getElementById('global-search');
        if (searchInput) {
            searchInput.onkeydown = async (e) => {
                if (e.key === 'Enter') {
                    const query = e.target.value;
                    if (!query) return;
                    this.notifier.show(`Searching for "${query}"...`, "info");
                    try {
                        const files = await api.invoke('search_files', { query });
                        this.toggleView('browser');
                        if (this.browser) this.browser.show("Search Results", files);
                    } catch (err) { this.notifier.show("Search failed: " + err, "error"); }
                }
            };
        }

        // Global Contact Search
        const conSearch = document.getElementById('contact-global-search');
        if (conSearch) {
            conSearch.oninput = async (e) => {
                const query = e.target.value;
                if (query.length < 2) return;
                try {
                    const results = await api.invoke('search_contacts', { query });
                    this.renderGlobalContacts(results);
                } catch (err) { console.error(err); }
            };
        }

        // Bridge Rust Events to HUD & Logger
        api.listen('progress', (event) => {
            const payload = event.payload;
            if (this.hud && payload.type !== 'log') this.hud.update(payload);

            if (payload.type === 'log' && this.logHud) {
                this.logHud.add(payload.message);
            }

            if (payload.type === 'error' && this.notifier) {
                this.notifier.show(payload.message, "error");
            }
        });

        // Reactive Hardware Events
        api.listen('device-connected', (event) => {
            const d = event.payload;
            this.notifier.show(`📱 Device Connected: ${d.model || d.serial}`, "success");
            this.refreshAll();
        });

        api.listen('device-disconnected', (event) => {
            const serial = event.payload;
            this.notifier.show(`🔌 Device Disconnected: ${serial}`, "warning");
            this.refreshAll();
        });

        await this.refreshAll();
    }

    async refreshAll() {
        try {
            const [devices, stats, status] = await Promise.all([
                DeviceService.getAll().catch(() => []),
                BackupService.getStats().catch(() => ({})),
                DeviceService.getStatus().catch(() => ({ adb_found: false }))
            ]);

            store.setState('devices', devices);
            store.setState('stats', stats);
            store.setState('engineStatus', status);
        } catch (err) {
            console.error("Critical Refresh Error:", err);
        }
    }

    handleStateChange({ key, value }) {
        if (key === 'devices') {
            const list = document.getElementById('device-list');
            if (!list) return;
            list.innerHTML = "";
            value.forEach(d => {
                const el = document.createElement('pb-device-item');
                el.device = d;
                list.appendChild(el);
            });
        }

        if (key === 'stats') {
            const elEff = document.getElementById('stat-efficiency');
            const elSnap = document.getElementById('stat-snapshots');
            if (!elEff || !elSnap) return;

            const efficiency = value.total_logical_bytes > 0
                ? (value.total_deduped_bytes / value.total_logical_bytes * 100).toFixed(1)
                : 0;
            elEff.setAttribute('value', `${efficiency}%`);
            elEff.setAttribute('subtext', `${utils.formatBytes(value.total_logical_bytes || 0)} protected`);
            elSnap.setAttribute('value', value.total_snapshots || 0);
        }

        if (key === 'engineStatus') {
            const elAdb = document.getElementById('stat-adb');
            if (elAdb) {
                elAdb.setAttribute('value', value.adb_found ? "Active" : "Error");
                elAdb.setAttribute('subtext', value.adb_version || "ADB missing");
            }
        }
    }

    toggleView(view) {
        const dashboard = document.getElementById('dashboard-view');
        const browser = document.getElementById('browser-view');
        const contacts = document.getElementById('contacts-view');

        dashboard.classList.add('hidden');
        browser.classList.add('hidden');
        contacts.classList.add('hidden');

        if (view === 'browser') {
            browser.classList.remove('hidden');
            window.scrollTo(0,0);
        } else if (view === 'contacts') {
            contacts.classList.remove('hidden');
        } else {
            dashboard.classList.remove('hidden');
        }
    }

    updateSidebar(active) {
        this.toggleView(active === 'contacts' ? 'contacts' : 'dashboard');

        const dashBtn = document.getElementById('nav-dashboard-btn');
        const devBtn = document.getElementById('nav-devices-btn');
        const conBtn = document.getElementById('nav-contacts-btn');

        const activeClass = "w-full flex items-center gap-3 px-4 py-3 text-sm font-bold bg-indigo-500 text-white rounded-xl shadow-lg shadow-indigo-500/20";
        const inactiveClass = "w-full flex items-center gap-3 px-4 py-3 text-sm font-bold text-indigo-200 hover:bg-white/5 rounded-xl transition-all";

        if (dashBtn) dashBtn.className = (active === 'dashboard') ? activeClass : inactiveClass;
        if (devBtn) devBtn.className = (active === 'devices') ? activeClass : inactiveClass;
        if (conBtn) conBtn.className = (active === 'contacts') ? activeClass : inactiveClass;

        if (active === 'dashboard') {
            window.scrollTo({ top: 0, behavior: 'smooth' });
        } else if (active === 'devices') {
            document.getElementById('device-list')?.scrollIntoView({ behavior: 'smooth' });
        }
    }

    async runBackup(id, paths = null) {
        try {
            if (this.notifier) this.notifier.show(`Starting backup...`, "info");
            await BackupService.run(id, paths);
            if (this.notifier) this.notifier.show("Success!", "success");
            await this.refreshAll();
        } catch (e) {
            if (this.notifier) this.notifier.show("Error: " + e, "error");
            else console.error("Backup error:", e);
        }
    }

    async runScan(id) {
        try {
            if (this.hud) this.hud.update({ type: 'start', message: 'Scanning device...' });
            const files = await DeviceService.scan(id);
            if (this.hud) this.hud.update({ type: 'finish', message: 'Ready' });
            this.toggleView('browser');
            if (this.browser) this.browser.show("Live Preview", files, id, true);
        } catch (e) {
            if (this.notifier) this.notifier.show("Scan failed", "error");
            else console.error("Scan failed:", e);
        }
    }

    async loadHistory(deviceId) {
        const historySection = document.getElementById('history-section');
        const historyList = document.getElementById('history-list');
        const historyTitle = document.getElementById('history-title');

        if (historySection) historySection.classList.remove('hidden');
        if (historyTitle) historyTitle.textContent = `History: ${deviceId}`;

        try {
            const snaps = await BackupService.getSnapshots(deviceId);
            if (historyList) historyList.snapshots = snaps;
        } catch (e) {
            if (this.notifier) this.notifier.show("History error", "error");
            else console.error("History error:", e);
        }
    }

    async browse(snapshotId) {
        try {
            const files = await BackupService.getFiles(snapshotId);
            this.toggleView('browser');
            if (this.browser) this.browser.show(snapshotId, files);
        } catch (e) {
            if (this.notifier) this.notifier.show("Index error", "error");
            else console.error("Index error:", e);
        }
    }

    async runRestore(id, filter = null) {
        const msg = filter ? `Restoring file: ${filter.split('/').pop()}...` : "Preparing full restore...";
        const target = prompt("Target path (optional):");
        try {
            this.notifier.show(msg, "info");
            await BackupService.restore(id, target || "", filter);
            this.notifier.show("Restore Success", "success");
        } catch (e) {
            this.notifier.show("Restore Failed: " + e, "error");
        }
    }

    async addSchedule(deviceId) {
        try {
            await SettingsService.addSchedule(deviceId);
            if (this.notifier) this.notifier.show("Daily schedule set", "success");
        } catch (e) {
            if (this.notifier) this.notifier.show("Schedule error", "error");
            else console.error("Schedule error:", e);
        }
    }

    renderGlobalContacts(results) {
        const container = document.getElementById('global-contacts-results');
        if (!container) return;

        if (!results || results.length === 0) {
            container.innerHTML = '<div class="col-span-full p-20 text-center text-slate-300 font-bold uppercase tracking-widest">No matching contacts</div>';
            return;
        }

        const colors = ['bg-blue-500', 'bg-purple-500', 'bg-indigo-500', 'bg-pink-500', 'bg-teal-500'];

        container.innerHTML = results.map((res, i) => {
            const c = res.contact;
            const sId = res.snapshot_id;
            const name = c.display_name || "Unknown";
            const getInitials = (n) => n.split(' ').filter(x => x).map(x => x[0]).join('').substring(0, 2).toUpperCase() || "?";

            const org = (c.organizations || [])[0] || {};
            const jobTitle = org.title || "";

            return `
                <div class="p-6 bg-white border border-slate-100 rounded-[2rem] shadow-sm hover:shadow-xl transition-all cursor-pointer group relative" onclick="window.viewSnapshotDetails('${sId}')">
                    <div class="absolute top-4 right-6 text-[8px] font-black bg-slate-100 text-slate-400 px-2 py-1 rounded-full uppercase tracking-tighter">Snapshot: ${sId.substring(0,8)}</div>
                    <div class="flex items-center gap-4 mb-4 mt-2">
                        <div class="w-12 h-12 ${colors[i % colors.length]} text-white rounded-2xl flex items-center justify-center font-black text-sm shadow-inner">
                            ${getInitials(name)}
                        </div>
                        <div class="min-w-0 flex-1">
                            <div class="font-bold text-slate-800 text-lg truncate">${name}</div>
                            <div class="text-[10px] text-indigo-500 font-black uppercase tracking-widest">${jobTitle}</div>
                        </div>
                    </div>
                    <div class="space-y-1">
                        ${(c.phones || []).slice(0,2).map(p => `<div class="text-xs text-slate-600 font-mono flex items-center gap-2"><span class="opacity-30">📞</span> ${p.raw_value}</div>`).join('')}
                        ${(c.emails || []).slice(0,1).map(em => `<div class="text-xs text-slate-400 truncate flex items-center gap-2"><span class="opacity-30">✉️</span> ${em.value}</div>`).join('')}
                    </div>
                </div>
            `;
        }).join('');
    }
}

new App();
