import { store } from './core/store.js';
import { utils } from './core/utils.js';
import { DeviceService } from './services/DeviceService.js';
import { BackupService } from './services/BackupService.js';
import { SettingsService } from './services/SettingsService.js';

import { NavigationManager } from './core/NavigationManager.js';
import { SearchManager } from './core/SearchManager.js';
import { EventManager } from './core/EventManager.js';

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
            } catch (e) { this.notifier?.show("Keygen failed: " + e, "error"); }
        };

        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', () => this.init());
        } else {
            this.init();
        }
    }

    async init() {
        // UI References
        this.hud = document.getElementById('global-progress');
        this.logHud = document.getElementById('log-hud');
        this.browser = document.getElementById('file-browser');
        this.settings = document.getElementById('settings-modal');
        this.notifier = document.getElementById('notifier');

        // Managers
        this.nav = new NavigationManager();
        this.search = new SearchManager(this);
        this.events = new EventManager(this);

        this.setupButtons();
        await this.refreshAll();
    }

    setupButtons() {
        const testBtn = document.getElementById('test-js-btn');
        if (testBtn) testBtn.onclick = () => {
            this.notifier?.show("Modular UI Core: Active", "success");
            this.logHud?.add("User initiated system self-test");
        };

        const settingsBtn = document.getElementById('settings-btn');
        if (settingsBtn) settingsBtn.onclick = () => this.settings?.show();

        const refreshBtn = document.getElementById('refresh-btn');
        if (refreshBtn) refreshBtn.onclick = () => this.refreshAll();

        // Sidebar Navigation
        document.getElementById('nav-dashboard-btn')?.addEventListener('click', () => this.nav.updateSidebar('dashboard'));
        document.getElementById('nav-devices-btn')?.addEventListener('click', () => this.nav.updateSidebar('devices'));
        document.getElementById('nav-contacts-btn')?.addEventListener('click', () => this.nav.updateSidebar('contacts'));
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
            if (elEff) {
                const efficiency = value.total_logical_bytes > 0
                    ? (value.total_deduped_bytes / value.total_logical_bytes * 100).toFixed(1)
                    : 0;
                elEff.setAttribute('value', `${efficiency}%`);
                elEff.setAttribute('subtext', `${utils.formatBytes(value.total_logical_bytes || 0)} protected`);
            }
            if (elSnap) elSnap.setAttribute('value', value.total_snapshots || 0);
        }

        if (key === 'engineStatus') {
            const elAdb = document.getElementById('stat-adb');
            if (elAdb) {
                elAdb.setAttribute('value', value.adb_found ? "Active" : "Error");
                elAdb.setAttribute('subtext', value.adb_version || "ADB missing");
            }
        }
    }

    async runBackup(id, paths = null) {
        try {
            this.notifier?.show(`Starting backup...`, "info");
            await BackupService.run(id, paths);
            this.notifier?.show("Success!", "success");
            await this.refreshAll();
        } catch (e) {
            this.notifier?.show("Error: " + e, "error");
        }
    }

    async runScan(id) {
        try {
            this.hud?.update({ type: 'start', message: 'Scanning device...' });
            const files = await DeviceService.scan(id);
            this.hud?.update({ type: 'finish', message: 'Ready' });
            this.nav.toggleView('browser');
            this.browser?.show("Live Preview", files, id, true);
        } catch (e) {
            this.notifier?.show("Scan failed", "error");
        }
    }

    async loadHistory(deviceId) {
        const historySection = document.getElementById('history-section');
        const historyList = document.getElementById('history-list');
        const historyTitle = document.getElementById('history-title');

        historySection?.classList.remove('hidden');
        if (historyTitle) historyTitle.textContent = `History: ${deviceId}`;

        try {
            const snaps = await BackupService.getSnapshots(deviceId);
            if (historyList) historyList.snapshots = snaps;
        } catch (e) {
            this.notifier?.show("History error", "error");
        }
    }

    async browse(snapshotId) {
        try {
            const files = await BackupService.getFiles(snapshotId);
            this.nav.toggleView('browser');
            this.browser?.show(snapshotId, files);
        } catch (e) {
            this.notifier?.show("Index error", "error");
        }
    }

    async runRestore(id, filter = null) {
        const msg = filter ? `Restoring file: ${filter.split('/').pop()}...` : "Preparing full restore...";
        const target = prompt("Target path (optional):");
        try {
            this.notifier?.show(msg, "info");
            await BackupService.restore(id, target || "", filter);
            this.notifier?.show("Restore Success", "success");
        } catch (e) {
            this.notifier?.show("Restore Failed: " + e, "error");
        }
    }

    async addSchedule(deviceId) {
        try {
            await SettingsService.addSchedule(deviceId);
            this.notifier?.show("Daily schedule set", "success");
        } catch (e) {
            this.notifier?.show("Schedule error", "error");
        }
    }
}

new App();
