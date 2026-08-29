import { SettingsService } from '../services/SettingsService.js';

export class SettingsModal extends HTMLElement {
    constructor() {
        super();
        this.innerHTML = `
            <div id="modal" class="fixed inset-0 bg-slate-900/60 backdrop-blur-sm z-[110] hidden items-center justify-center p-4">
                <div class="bg-white w-full max-w-2xl rounded-2xl shadow-2xl flex flex-col overflow-hidden">
                    <div class="p-6 border-b bg-slate-50 flex justify-between items-center">
                        <h3 class="text-xl font-bold text-slate-800">Advanced Settings</h3>
                        <button id="close-btn" class="text-slate-400 hover:text-slate-600 text-3xl font-light">&times;</button>
                    </div>

                    <div class="p-6 space-y-6">
                        <!-- Storage Backend Section -->
                        <div>
                            <label class="block text-sm font-bold text-slate-700 mb-2 uppercase tracking-wider">Storage Provider</label>
                            <div class="grid grid-cols-2 gap-4">
                                <div class="p-4 border-2 border-indigo-600 rounded-xl bg-indigo-50 relative">
                                    <div class="font-bold text-indigo-700">Local Disk</div>
                                    <div class="text-xs text-indigo-500">Fast, local workspace</div>
                                    <div class="absolute top-2 right-2 text-indigo-600">✓</div>
                                </div>
                                <button id="switch-mock-btn" class="p-4 border border-slate-200 rounded-xl hover:border-indigo-400 text-left transition-all group">
                                    <div class="font-bold text-slate-700 group-hover:text-indigo-600">Mock / Cloud</div>
                                    <div class="text-xs text-slate-400">Virtual storage for testing</div>
                                </button>
                            </div>
                        </div>

                        <!-- Maintenance Section -->
                        <div class="pt-6 border-t border-slate-100">
                            <label class="block text-sm font-bold text-slate-700 mb-2 uppercase tracking-wider">Maintenance</label>
                            <button id="gc-btn" class="w-full bg-slate-100 text-slate-700 py-3 rounded-xl font-bold hover:bg-slate-200 transition-colors">
                                Run Garbage Collection (GC)
                            </button>
                        </div>
                    </div>

                    <div class="p-4 bg-slate-50 border-t flex justify-end">
                        <button id="done-btn" class="bg-indigo-600 text-white px-8 py-2 rounded-xl font-bold">Save & Close</button>
                    </div>
                </div>
            </div>
        `;

        this.querySelector('#close-btn').onclick = () => this.hide();
        this.querySelector('#done-btn').onclick = () => this.hide();

        this.querySelector('#switch-mock-btn').onclick = async () => {
            if (confirm("Switch to Mock Storage? Real backups will not be saved.")) {
                try {
                    await SettingsService.switchStorageToMock();
                    alert("Switched to Mock Storage Backend.");
                } catch (e) { alert(e); }
            }
        };

        this.querySelector('#gc-btn').onclick = async () => {
            try {
                const deleted = await SettingsService.runMaintenance();
                alert(`GC Finished. Removed ${deleted} objects.`);
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
