import { SettingsService } from '../services/SettingsService.js';

export class SettingsModal extends HTMLElement {
    constructor() {
        super();
        this.render();
    }

    render() {
        this.innerHTML = `
            <div id="modal" class="fixed inset-0 bg-slate-900/60 backdrop-blur-sm z-[110] hidden items-center justify-center p-4 overflow-y-auto">
                <div class="bg-white w-full max-w-2xl rounded-2xl shadow-2xl flex flex-col overflow-hidden my-auto">
                    <div class="p-6 border-b bg-slate-50 flex justify-between items-center">
                        <h3 class="text-xl font-bold text-slate-800">Advanced Settings</h3>
                        <button id="close-btn" class="text-slate-400 hover:text-slate-600 text-3xl font-light">&times;</button>
                    </div>

                    <div class="p-6 space-y-6 flex-1 overflow-y-auto max-h-[70vh]">
                        <!-- Storage Backend Section -->
                        <div>
                            <label class="block text-sm font-bold text-slate-700 mb-4 uppercase tracking-wider text-xs">Storage Provider</label>
                            <div class="grid grid-cols-2 gap-4 mb-6">
                                <button id="use-local-btn" class="p-4 border-2 border-indigo-600 rounded-xl bg-indigo-50 text-left relative">
                                    <div class="font-bold text-indigo-700">Local Disk</div>
                                    <div class="text-[10px] text-indigo-500 uppercase font-bold mt-1">Default</div>
                                    <div class="absolute top-2 right-2 text-indigo-600 font-bold">✓</div>
                                </button>
                                <button id="use-mock-btn" class="p-4 border border-slate-200 rounded-xl hover:border-indigo-400 text-left transition-all group">
                                    <div class="font-bold text-slate-700 group-hover:text-indigo-600">Mock Mode</div>
                                    <div class="text-[10px] text-slate-400 uppercase font-bold mt-1">Virtual / Test</div>
                                </button>
                            </div>

                            <!-- S3 Configuration (Hidden by default or expandable) -->
                            <div class="bg-slate-50 p-6 rounded-2xl border border-slate-200">
                                <h4 class="text-sm font-bold text-slate-800 mb-4 flex items-center gap-2">
                                    <svg class="w-4 h-4 text-indigo-600" fill="currentColor" viewBox="0 0 20 20"><path d="M5.5 13a3.5 3.5 0 01-.369-6.98 4 4 0 117.753-1.977A4.5 4.5 0 1113.5 13H11V9.413l1.293 1.293a1 1 0 001.414-1.414l-3-3a1 1 0 00-1.414 0l-3 3a1 1 0 001.414 1.414L9 9.413V13H5.5z"/><path d="M9 13h2v5a1 1 0 11-2 0v-5z"/></svg>
                                    S3 Compatible Cloud Storage
                                </h4>
                                <div class="grid grid-cols-2 gap-4">
                                    <div class="space-y-1">
                                        <label class="text-[10px] font-bold text-slate-400 ml-1 uppercase">Bucket Name</label>
                                        <input type="text" id="s3-bucket" placeholder="phone-backups" class="w-full px-3 py-2 text-sm bg-white border border-slate-200 rounded-lg outline-none focus:ring-2 focus:ring-indigo-500">
                                    </div>
                                    <div class="space-y-1">
                                        <label class="text-[10px] font-bold text-slate-400 ml-1 uppercase">Region</label>
                                        <input type="text" id="s3-region" placeholder="us-east-1" class="w-full px-3 py-2 text-sm bg-white border border-slate-200 rounded-lg outline-none focus:ring-2 focus:ring-indigo-500">
                                    </div>
                                    <div class="col-span-2 space-y-1">
                                        <label class="text-[10px] font-bold text-slate-400 ml-1 uppercase">Endpoint URL</label>
                                        <input type="text" id="s3-endpoint" placeholder="https://s3.amazonaws.com" class="w-full px-3 py-2 text-sm bg-white border border-slate-200 rounded-lg outline-none focus:ring-2 focus:ring-indigo-500">
                                    </div>
                                    <div class="space-y-1">
                                        <label class="text-[10px] font-bold text-slate-400 ml-1 uppercase">Access Key</label>
                                        <input type="password" id="s3-access" class="w-full px-3 py-2 text-sm bg-white border border-slate-200 rounded-lg outline-none focus:ring-2 focus:ring-indigo-500">
                                    </div>
                                    <div class="space-y-1">
                                        <label class="text-[10px] font-bold text-slate-400 ml-1 uppercase">Secret Key</label>
                                        <input type="password" id="s3-secret" class="w-full px-3 py-2 text-sm bg-white border border-slate-200 rounded-lg outline-none focus:ring-2 focus:ring-indigo-500">
                                    </div>
                                </div>
                                <button id="apply-s3-btn" class="w-full mt-6 bg-indigo-600 text-white py-2.5 rounded-xl font-bold text-sm hover:bg-indigo-700 transition-all shadow-md">
                                    CONNECT CLOUD STORAGE
                                </button>
                            </div>
                        </div>

                        <!-- Maintenance Section -->
                        <div class="pt-6 border-t border-slate-100 space-y-3">
                            <label class="block text-sm font-bold text-slate-700 mb-2 uppercase tracking-wider text-xs">System Maintenance</label>
                            <button id="prune-btn" class="w-full bg-amber-50 text-amber-700 py-3 rounded-xl font-bold hover:bg-amber-100 transition-colors flex items-center justify-center gap-2 border border-amber-100">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/></svg>
                                Prune Failed Snapshots
                            </button>
                            <button id="gc-btn" class="w-full bg-slate-100 text-slate-700 py-3 rounded-xl font-bold hover:bg-slate-200 transition-colors flex items-center justify-center gap-2">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>
                                Run Garbage Collection
                            </button>
                        </div>
                    </div>

                    <div class="p-4 bg-slate-50 border-t flex justify-end">
                        <button id="done-btn" class="bg-indigo-600 text-white px-8 py-2 rounded-xl font-bold">Done</button>
                    </div>
                </div>
            </div>
        `;

        this.setupListeners();
    }

    setupListeners() {
        this.querySelector('#close-btn').onclick = () => this.hide();
        this.querySelector('#done-btn').onclick = () => this.hide();

        this.querySelector('#use-mock-btn').onclick = async () => {
            if (confirm("Switch to Mock Storage? Real backups will not be saved.")) {
                try {
                    await SettingsService.switchStorageToMock();
                    alert("Switched to Mock Storage Backend.");
                } catch (e) { alert(e); }
            }
        };

        this.querySelector('#apply-s3-btn').onclick = async () => {
            const config = {
                bucket: this.querySelector('#s3-bucket').value,
                region: this.querySelector('#s3-region').value,
                endpoint: this.querySelector('#s3-endpoint').value,
                access_key: this.querySelector('#s3-access').value,
                secret_key: this.querySelector('#s3-secret').value
            };

            if (!config.bucket || !config.access_key || !config.secret_key) {
                alert("Please fill in Bucket, Access Key, and Secret Key.");
                return;
            }

            try {
                this.querySelector('#apply-s3-btn').textContent = "CONNECTING...";
                this.querySelector('#apply-s3-btn').disabled = true;

                await SettingsService.switchStorageToS3(config);
                alert("Successfully connected to S3 Storage!");

                this.querySelector('#apply-s3-btn').textContent = "CONNECT CLOUD STORAGE";
                this.querySelector('#apply-s3-btn').disabled = false;
            } catch (e) {
                alert("Cloud Connection Failed: " + e);
                this.querySelector('#apply-s3-btn').textContent = "CONNECT CLOUD STORAGE";
                this.querySelector('#apply-s3-btn').disabled = false;
            }
        };

        this.querySelector('#gc-btn').onclick = async () => {
            try {
                const deleted = await SettingsService.runMaintenance();
                alert(`GC Finished. Removed ${deleted} objects.`);
            } catch (e) { alert(e); }
        };

        this.querySelector('#prune-btn').onclick = async () => {
            try {
                const count = await SettingsService.pruneFailedSnapshots();
                alert(`Pruning complete. Deleted ${count} incomplete snapshots.`);
                window.loadDevices(); // Refresh lists
            } catch (e) { alert(e); }
        };
    }

    show() {
        this.querySelector('#modal').style.display = 'flex';
    }

    hide() {
        this.querySelector('#modal').style.display = 'none';
    }
}
customElements.define('pb-settings-modal', SettingsModal);
