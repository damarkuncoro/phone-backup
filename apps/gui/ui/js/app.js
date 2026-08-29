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

class App {
    constructor() {
        console.log("Modular App Orchestrator Instantiated");

        // Expose critical functions to window for atomic component interoperability
        window.runBackup = (id, paths) => this.runBackup(id, paths);
        window.loadDevices = () => this.refreshAll();
        window.viewSnapshots = (id) => this.loadHistory(id);
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
        this.browser = document.getElementById('file-browser');
        this.settings = document.getElementById('settings-modal');
        this.notifier = document.getElementById('notifier');

        // Setup UI Listeners
        const testBtn = document.getElementById('test-js-btn');
        if (testBtn) testBtn.onclick = () => this.notifier.show("Modular UI Core: Active", "success");

        const settingsBtn = document.getElementById('settings-btn');
        if (settingsBtn) settingsBtn.onclick = () => this.settings && this.settings.show();

        const refreshBtn = document.getElementById('refresh-btn');
        if (refreshBtn) refreshBtn.onclick = () => this.refreshAll();

        // Register Global Event Bus
        store.addEventListener('change', (e) => this.handleStateChange(e.detail));
        window.addEventListener('run-backup', (e) => this.runBackup(e.detail));
        window.addEventListener('run-selective-backup', (e) => this.runBackup(e.detail.deviceId, e.detail.paths));
        window.addEventListener('view-history', (e) => this.loadHistory(e.detail));
        window.addEventListener('scan-device', (e) => this.runScan(e.detail));
        window.addEventListener('restore-snapshot', (e) => this.runRestore(e.detail));
        window.addEventListener('add-schedule', (e) => this.addSchedule(e.detail));
        window.addEventListener('browse-snapshot', (e) => this.browse(e.detail));
        window.addEventListener('close-browser', () => this.toggleView('dashboard'));

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
                        if (this.browser) this.browser.show("Search Results", files);
                    } catch (err) { this.notifier.show("Search failed: " + err, "error"); }
                }
            };
        }

        // Bridge Rust Events to HUD
        api.listen('progress', (event) => {
            if (this.hud) this.hud.update(event.payload);
            if (event.payload.type === 'error' && this.notifier) {
                this.notifier.show(event.payload.message, "error");
            }
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
        if (view === 'browser') {
            dashboard.classList.add('hidden');
            browser.classList.remove('hidden');
            window.scrollTo(0,0);
        } else {
            browser.classList.add('hidden');
            dashboard.classList.remove('hidden');
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

    async runRestore(id) {
        const target = prompt("Target path (optional):");
        try {
            await BackupService.restore(id, target || "");
            if (this.notifier) this.notifier.show("Restore Success", "success");
        } catch (e) {
            if (this.notifier) this.notifier.show("Restore Failed", "error");
            else console.error("Restore failed:", e);
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
}

new App();
