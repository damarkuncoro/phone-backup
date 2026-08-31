import { BaseService } from '../core/BaseService.js';

export class BackupService extends BaseService {
    static async run(deviceId, includeFiles = null) {
        return await this.call('start_backup', {
            device_id: deviceId,
            include_files: includeFiles
        });
    }

    static async getSnapshots(deviceId) {
        return await this.call('get_snapshots', { device_id: deviceId });
    }

    static async getFiles(snapshotId) {
        return await this.call('get_snapshot_files', { snapshot_id: snapshotId });
    }

    static async getApps(snapshotId) {
        return await this.call('get_snapshot_apps', { snapshot_id: snapshotId });
    }

    static async getDiff(oldId, newId) {
        return await this.call('get_file_diff', { old_snapshot_id: oldId, new_snapshot_id: newId });
    }

    static async getStructuredData(snapshotId, dataType) {
        return await this.call('get_structured_data', {
            snapshot_id: snapshotId,
            data_type: dataType
        });
    }

    static async restore(snapshotId, targetDir = "", filter = null) {
        if (!snapshotId) throw new Error("Snapshot ID is required for restore");

        return await this.call('restore_snapshot', {
            snapshotId: String(snapshotId),
            targetDir: String(targetDir || ""),
            filter: filter ? String(filter) : null
        });
    }

    static async getStats() {
        return await this.call('get_storage_stats');
    }
}
