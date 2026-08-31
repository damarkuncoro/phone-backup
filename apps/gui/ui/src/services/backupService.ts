import { safeInvoke } from "@/shared/lib/ipc";
import { type FileEntry } from "./deviceService";

export interface Snapshot {
  id: { 0: string } | string;
  device_id: { 0: string } | string;
  started_at: string;
  finished_at: string | null;
  status: 'Pending' | 'Running' | 'Completed' | 'Failed' | 'Interrupted';
  total_files: number;
  total_bytes: number;
  deduped_bytes: number;
}

export function getSnapshotId(snapshot: Snapshot): string {
    return typeof snapshot.id === 'string' ? snapshot.id : (snapshot.id as any)[0];
}

export const backupService = {
  async getSnapshots(deviceId: string): Promise<Snapshot[]> {
    return await safeInvoke("get_snapshots", { device_id: deviceId });
  },

  async getSnapshotFiles(snapshotId: string): Promise<FileEntry[]> {
    return await safeInvoke("get_snapshot_files", { snapshot_id: snapshotId });
  },

  async startBackup(deviceId: string, includeFiles?: string[]): Promise<Snapshot> {
    return await safeInvoke("start_backup", {
        device_id: deviceId,
        include_files: includeFiles || null
    });
  },

  async deleteSnapshot(snapshotId: string): Promise<void> {
    return await safeInvoke("delete_snapshot", { snapshot_id: snapshotId });
  },

  async getStructuredData(snapshotId: string, dataType: string): Promise<any[]> {
    return await safeInvoke("get_structured_data", {
        snapshot_id: snapshotId,
        data_type: dataType
    }, { silent: true });
  },

  async getFileDiff(oldId: string, newId: string): Promise<FileDiff> {
    return await safeInvoke("get_file_diff", {
        old_snapshot_id: oldId,
        new_snapshot_id: newId
    });
  },

  async getStorageStats(): Promise<{ total_logical_bytes: number, total_deduped_bytes: number, total_snapshots: number }> {
    return await safeInvoke("get_storage_stats");
  },

  async restoreSnapshot(snapshotId: string, targetDir: string, filters?: string[]): Promise<void> {
    return await safeInvoke("restore_snapshot", {
        snapshot_id: snapshotId,
        target_dir: targetDir,
        filter: filters || null
    });
  },

  async exportContactsVCard(snapshotId: string): Promise<string> {
    return await safeInvoke("export_contacts_vcard", { snapshot_id: snapshotId });
  }
};

export interface FileDiff {
  added: string[];
  modified: string[];
  deleted: string[];
  unchanged: string[];
}
