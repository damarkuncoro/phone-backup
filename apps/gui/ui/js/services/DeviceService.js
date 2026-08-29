import { api } from '../core/api.js';

/**
 * Service untuk menangani logika bisnis perangkat (Devices).
 * Sesuai prinsip SRP, kelas ini hanya peduli pada data perangkat.
 */
export const DeviceService = {
    async getAll() {
        return await api.invoke('get_devices');
    },

    async scan(deviceId) {
        return await api.invoke('scan_device', { deviceId });
    },

    async getStatus() {
        return await api.invoke('get_doctor_report');
    }
};
