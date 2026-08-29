import { api } from '../core/api.js';

/**
 * Service for Application Settings.
 * Modular approach to decouple settings logic from UI.
 */
export const SettingsService = {
    async switchStorageToMock() {
        return await api.invoke('switch_to_mock_storage');
    },

    async generateNewKeys() {
        return await api.invoke('generate_keys');
    },

    async runMaintenance() {
        return await api.invoke('run_gc');
    }
};
