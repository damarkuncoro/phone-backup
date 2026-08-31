import { safeInvoke } from "@/shared/lib/ipc";

export interface Device {
  id: string | [string];
  manufacturer: string;
  model: string;
  serial: string;
  os_version: string;
  connection_type: 'Usb' | 'Wifi' | 'Unknown';
  storage_used_bytes: number;
  storage_total_bytes: number;
  storage_free_bytes: number;
}

export function getDeviceId(device: Device): string {
  return typeof device.id === 'string' ? device.id : device.id[0];
}

export interface FileEntry {
  name: string;
  path: string;
  size_bytes: number;
  modified_at: string;
  is_dir: boolean;
  mime_type: string;
}

export const deviceService = {
  async getAll(): Promise<Device[]> {
    return await safeInvoke("get_devices");
  },

  async getAllKnown(): Promise<Device[]> {
    return await safeInvoke("get_all_known_devices");
  },

  async scan(deviceId: string): Promise<FileEntry[]> {
    return await safeInvoke("scan_device", { device_id: deviceId });
  },

  async browse(deviceId: string, path: string): Promise<FileEntry[]> {
    return await safeInvoke("browse_directory", { device_id: deviceId, path });
  },

  async getBattery(deviceId: string): Promise<[number, number]> {
    return await safeInvoke("get_device_battery", { device_id: deviceId });
  },

  async getStatus() {
    return await safeInvoke("get_doctor_report");
  }
};
