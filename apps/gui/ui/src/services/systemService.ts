import { safeInvoke } from "@/shared/lib/ipc";

export interface DoctorReport {
  adb_found: boolean;
  adb_version: string;
  device_count: number;
  db_healthy: boolean;
}

export type StorageBackend =
  | { Local: null }
  | { Mock: null }
  | { S3: { bucket: string, region: string, endpoint: string, access_key: string, secret_key: string } };

export interface AppSettings {
  storage_backend: StorageBackend | string; // Handle both tagged enum and simple string if necessary
  encryption_public_key: string | null;
}

export const systemService = {
  async getDoctorReport(): Promise<DoctorReport> {
    return await safeInvoke("get_doctor_report");
  },

  async getSettings(): Promise<AppSettings> {
    return await safeInvoke("get_settings");
  },

  async saveSettings(settings: AppSettings): Promise<void> {
    return await safeInvoke("save_settings", { settings });
  },

  async generateKeys(): Promise<[string, string]> {
    return await safeInvoke("generate_keys");
  },

  async runGC(): Promise<number> {
    return await safeInvoke("run_gc");
  },

  async pruneFailed(): Promise<number> {
    return await safeInvoke("prune_failed_snapshots");
  },

  async openRestoreFolder(): Promise<void> {
    return await safeInvoke("open_restore_folder");
  },

  async searchFiles(query: string): Promise<any[]> {
    return await safeInvoke("search_files", { query });
  }
};
