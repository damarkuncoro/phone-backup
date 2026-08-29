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

    static async getLiveData(deviceId, dataType) {
        return await this.call('get_live_data', {
            device_id: deviceId,
            data_type: dataType
        });
    }

    static async getStatus() {
        return await this.call('get_doctor_report');
    }
}
