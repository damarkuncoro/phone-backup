import { api } from '../core/api.js';

/**
 * Service untuk menangani logika backup dan pemulihan.
 * Memisahkan pemanggilan API dari komponen UI (Decoupling).
 */
export const BackupService = {
    async runBackup(deviceId, includeFiles = null) {
        return await api.invoke('start_backup', { deviceId, includeFiles });
    },

    async getSnapshots(deviceId) {
        return await api.invoke('get_snapshots', { deviceId });
    },

    async getSnapshotFiles(snapshotId) {
        return await api.invoke('get_snapshot_files', { snapshotId });
    },

    async restore(snapshotId, targetDir = "") {
        return await api.invoke('restore_snapshot', { snapshotId, targetDir });
    },

    async getStorageStats() {
        return await api.invoke('get_storage_stats');
    }
};
