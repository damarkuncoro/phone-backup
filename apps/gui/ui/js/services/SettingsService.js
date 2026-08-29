import { BaseService } from '../core/BaseService.js';

/**
 * Service for Application Settings.
 */
export class SettingsService extends BaseService {
    static async switchStorageToMock() {
        return await this.call('switch_to_mock_storage');
    }

    static async switchStorageToS3(config) {
        return await this.call('switch_to_s3_storage', {
            bucket: config.bucket,
            region: config.region,
            endpoint: config.endpoint,
            access_key: config.access_key,
            secret_key: config.secret_key
        });
    }

    static async generateNewKeys() {
        return await this.call('generate_keys');
    }

    static async runMaintenance() {
        return await this.call('run_gc');
    }

    static async pruneFailedSnapshots() {
        return await this.call('prune_failed_snapshots');
    }

    static async addSchedule(deviceId) {
        return await this.call('add_schedule', { device_id: deviceId });
    }
}
