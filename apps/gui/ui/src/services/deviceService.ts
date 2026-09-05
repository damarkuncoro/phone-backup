import { safeInvoke } from "@/shared/lib/ipc";

export interface Device {
  id: string | [string];
  manufacturer: string;
  model: string;
  serial: string;
  os_version: string;
  connection_type: 'Usb' | 'Wifi' | 'Mtp' | 'Unknown';
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

export interface ScanCategorySummary {
  file_count: number;
  total_bytes: number;
}

export interface ScanMetrics {
  duration_ms: number;
  directories_scanned: number;
  files_scanned: number;
  throughput_files_per_sec: number;
}

export interface ScanWarning {
  source: string;
  path: string;
  message: string;
}

export interface ScanResultPayload {
  files: FileEntry[];
  warnings: ScanWarning[];
  categories: Record<string, ScanCategorySummary>;
  metrics: ScanMetrics | null;
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

  async scanDetailed(deviceId: string, roots?: string[], filter?: any): Promise<ScanResultPayload> {
    try {
      const res: any = await safeInvoke("scan_device_detailed", {
        device_id: deviceId,
        roots: roots || null,
        filter: filter || null,
      });
      if (res && Array.isArray(res.files)) {
        return {
          files: (res.files || []).map((f: any) => ({
            ...f,
            is_dir: f.is_dir === true || f.mime_type === 'inode/directory' || (typeof f.permissions === 'string' && f.permissions.startsWith('d'))
          })),
          warnings: res.warnings || [],
          categories: res.categories || {},
          metrics: res.metrics || null,
        };
      }
    } catch (e) {
      console.warn("scan_device_detailed fallback to standard scan:", e);
    }

    const files = await this.scan(deviceId);
    return {
      files,
      warnings: [],
      categories: {},
      metrics: null,
    };
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

  async downloadFile(deviceId: string, remotePath: string, localPath: string): Promise<void> {
    return await safeInvoke("download_from_device", { device_id: deviceId, remote_path: remotePath, local_path: localPath });
  },

  async uploadFile(deviceId: string, localPath: string, remotePath: string): Promise<void> {
    return await safeInvoke("upload_to_device", { device_id: deviceId, local_path: localPath, remote_path: remotePath });
  },

  async deleteFile(deviceId: string, path: string): Promise<void> {
    return await safeInvoke("delete_device_file", { device_id: deviceId, path });
  },

  async renameFile(deviceId: string, oldPath: string, newPath: string): Promise<void> {
    return await safeInvoke("rename_device_file", { device_id: deviceId, old_path: oldPath, new_path: newPath });
  },

  async calculateHash(deviceId: string, path: string): Promise<string> {
    return await safeInvoke("calculate_device_file_hash", { device_id: deviceId, path });
  },

  async getStatus() {
    return await safeInvoke("get_doctor_report");
  },

  async getLiveData(deviceId: string, dataType: 'contacts' | 'sms' | 'call_logs' | 'apps'): Promise<any[]> {
    return await safeInvoke("get_live_data", { device_id: deviceId, data_type: dataType });
  },

  async connectWireless(host: string, port: number = 5555): Promise<string> {
    return await safeInvoke("connect_wireless_device", { host, port });
  }
};
