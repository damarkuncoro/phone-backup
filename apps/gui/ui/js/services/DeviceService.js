import { BaseService } from '../core/BaseService.js';

/**
 * Service for Device Management.
 * Modular approach using inheritance for common API behaviors.
 */
export class DeviceService extends BaseService {
    static async getAll() {
        return await this.call('get_devices');
    }

    static async scan(deviceId) {
        return await this.call('scan_device', { device_id: deviceId });
    }

    static async browse(deviceId, path) {
        return await this.call('browse_directory', { device_id: deviceId, path });
    }

    static async deleteFile(deviceId, path) {
        return await this.call('delete_device_file', { device_id: deviceId, path });
    }

    static async renameFile(deviceId, oldPath, newPath) {
        return await this.call('rename_device_file', { device_id: deviceId, old_path: oldPath, new_path: newPath });
    }

    static async copyFile(deviceId, source, target) {
        return await this.call('copy_device_file', { device_id: deviceId, source, target });
    }

    static async uploadFile(deviceId, localPath, remotePath) {
        return await this.call('upload_to_device', { device_id: deviceId, local_path: localPath, remote_path: remotePath });
    }

    static async calculateHash(deviceId, path) {
        return await this.call('calculate_device_file_hash', { device_id: deviceId, path });
    }

    static async getLiveData(deviceId, dataType) {
        return await this.call('get_live_data', {
            device_id: deviceId,
            data_type: dataType
        });
    }

    static async getStatus() {
        return await this.call('get_doctor_report');
    }

    static async getBattery(deviceId) {
        return await this.call('get_device_battery', { device_id: deviceId });
    }
}
