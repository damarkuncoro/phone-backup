import { api } from './api.js';
import { store } from './store.js';

/**
 * Orchestrates event listening between Rust backend and JS components
 */
export class EventManager {
    constructor(app) {
        this.app = app;
        this.init();
    }

    init() {
        // Register Global JS Event Bus
        window.addEventListener('run-backup', (e) => this.app.runBackup(e.detail));
        window.addEventListener('run-selective-backup', (e) => this.app.runBackup(e.detail.deviceId, e.detail.paths));
        window.addEventListener('view-history', (e) => this.app.loadHistory(e.detail));
        window.addEventListener('scan-device', (e) => this.app.runScan(e.detail));
        window.addEventListener('restore-snapshot', (e) => this.app.runRestore(e.detail));
        window.addEventListener('restore-file', (e) => this.app.runRestore(e.detail.snapshotId, e.detail.path));
        window.addEventListener('add-schedule', (e) => this.app.addSchedule(e.detail));
        window.addEventListener('browse-snapshot', (e) => this.app.browse(e.detail));
        window.addEventListener('close-browser', () => this.app.nav.updateSidebar('dashboard'));

        // Bridge Rust Events to Components
        api.listen('progress', (event) => {
            const payload = event.payload;
            if (this.app.hud && payload.type !== 'log') this.app.hud.update(payload);

            if (payload.type === 'log' && this.app.logHud) {
                this.app.logHud.add(payload.message);
            }

            if (payload.type === 'error' && this.app.notifier) {
                this.app.notifier.show(payload.message, "error");
            }
        });

        // Reactive Hardware Events
        api.listen('device-connected', (event) => {
            const d = event.payload;
            this.app.notifier?.show(`📱 Device Connected: ${d.model || d.serial}`, "success");
            this.app.refreshAll();
        });

        api.listen('device-disconnected', (event) => {
            const serial = event.payload;
            this.app.notifier?.show(`🔌 Device Disconnected: ${serial}`, "warning");
            this.app.refreshAll();
        });

        api.listen('device-status-update', (event) => {
            window.dispatchEvent(new CustomEvent('device-status-update', { detail: event.payload }));
        });

        // Listen for internal state changes
        store.addEventListener('change', (e) => this.app.handleStateChange(e.detail));
    }
}
