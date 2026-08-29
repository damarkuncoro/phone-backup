import { api } from './core/api.js';
import { store } from './core/store.js';
import { DeviceService } from './services/DeviceService.js';
import { BackupService } from './services/BackupService.js';

import './components/StatCard.js';
import './components/DeviceItem.js';
import './components/ProgressHUD.js';
import './components/SnapshotList.js';
import './components/FileBrowser.js';
import './components/SettingsModal.js';
import './components/Notification.js';

class App {
    constructor() {
        console.log("App Class Instantiated");
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', () => this.init());
        } else {
            this.init();
        }
    }

    async init() {
        console.log("Modular App initializing...");

        this.hud = document.getElementById('global-progress');
        this.browser = document.getElementById('file-browser');
        this.settings = document.getElementById('settings-modal');
        this.notifier = document.getElementById('notifier');

        // Setup Module Test Button
        const testBtn = document.getElementById('test-js-btn');
        if (testBtn) {
            testBtn.onclick = () => {
                this.notifier.show("JavaScript Module system is active!", "success");
            };
        }

        // Setup Settings Button
        const settingsBtn = document.getElementById('settings-btn');
        if (settingsBtn) {
            settingsBtn.onclick = () => {
                if (this.settings) this.settings.show();
            };
        }

        // Listen to Store Changes
        store.addEventListener('change', (e) => this.handleStateChange(e.detail));

        // Global Event Listeners
        window.addEventListener('run-backup', (e) => this.runBackup(e.detail));
        window.addEventListener('run-selective-backup', (e) => this.runBackup(e.detail.deviceId, e.detail.paths));
        window.addEventListener('view-history', (e) => this.loadHistory(e.detail));
        window.addEventListener('scan-device', (e) => this.runScan(e.detail));
        window.addEventListener('restore-snapshot', (e) => this.runRestore(e.detail));

        const refreshBtn = document.getElementById('refresh-btn');
        if (refreshBtn) refreshBtn.onclick = () => this.refreshAll();

        // Start Progress Listener
        api.listen('progress', (event) => {
            if (this.hud) this.hud.update(event.payload);
        });

        await this.refreshAll();
    }

    async refreshAll() {
        try {
            const [devices, stats, status] = await Promise.all([
                DeviceService.getAll().catch(e => { console.error(e); return []; }),
                BackupService.getStorageStats().catch(e => { console.error(e); return {}; }),
                DeviceService.getStatus().catch(e => { console.error(e); return { ad_found: false }; })
            ]);

            store.setState('devices', devices);
            store.setState('stats', stats);
            store.setState('engineStatus', status);
        } catch (err) {
            console.error("Refresh failed:", err);
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
            elEff.setAttribute('subtext', `${(value.total_logical_bytes/1024/1024 || 0).toFixed(1)} MB protected`);
            elSnap.setAttribute('value', value.total_snapshots || 0);
        }

        if (key === 'engineStatus') {
            const elAdb = document.getElementById('stat-adb');
            if (elAdb) {
                elAdb.setAttribute('value', value.adb_found ? "Active" : "Error");
                elAdb.setAttribute('subtext', value.adb_version || "Not found");
            }
        }
    }

    async runBackup(id, paths = null) {
        try {
            this.notifier.show(`Starting backup for ${id}...`, "info");
            await BackupService.runBackup(id, paths);
            this.notifier.show("Backup completed successfully!", "success");
            await this.refreshAll();
        } catch (e) {
            this.notifier.show("Backup failed: " + e, "error");
        }
    }

    async runScan(id) {
        try {
            if (this.hud) this.hud.update({ type: 'start', message: 'Scanning device...' });
            const files = await DeviceService.scan(id);
            if (this.hud) this.hud.update({ type: 'finish', message: 'Scan complete' });
            if (this.browser) this.browser.show(null, files, id, true);
        } catch (e) {
            this.notifier.show("Scan failed: " + e, "error");
        }
    }

    async loadHistory(deviceId) {
        const historyList = document.getElementById('history-list');
        const section = document.getElementById('history-section');
        if (section) section.classList.remove('hidden');

        try {
            const snaps = await BackupService.getSnapshots(deviceId);
            if (historyList) historyList.snapshots = snaps;
        } catch (e) { console.error(e); }
    }

    async runRestore(id) {
        const target = prompt("Target folder (optional):");
        try {
            await BackupService.restore(id, target || "");
            this.notifier.show("Restore Success!", "success");
        } catch (e) {
            this.notifier.show("Restore Error: " + e, "error");
        }
    }
}

new App();
