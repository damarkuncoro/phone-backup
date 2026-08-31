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
  id?: { 0: string } | string;
  name: string;
  path: string;
  size_bytes: number;
  modified_at: string;
  is_dir: boolean;
  mime_type: string;
  permissions?: string;
}

export const deviceService = {
  async getAll(): Promise<Device[]> {
    return await safeInvoke("get_devices");
  },

  async getAllKnown(): Promise<Device[]> {
    return await safeInvoke("get_all_known_devices");
  },

  async scan(deviceId: string): Promise<FileEntry[]> {
    const list: any[] = await safeInvoke("scan_device", { device_id: deviceId });
    return (list || []).map(f => ({
      ...f,
      is_dir: f.is_dir === true || f.mime_type === 'inode/directory' || (typeof f.permissions === 'string' && f.permissions.startsWith('d'))
    }));
  },

  async browse(deviceId: string, path: string): Promise<FileEntry[]> {
    const list: any[] = await safeInvoke("browse_directory", { device_id: deviceId, path });
    return (list || []).map(f => ({
      ...f,
      is_dir: f.is_dir === true || f.mime_type === 'inode/directory' || (typeof f.permissions === 'string' && f.permissions.startsWith('d'))
    }));
  },

  async getBattery(deviceId: string): Promise<[number, number]> {
    return await safeInvoke("get_device_battery", { device_id: deviceId });
  },

  async getStatus() {
    return await safeInvoke("get_doctor_report");
  }
};
