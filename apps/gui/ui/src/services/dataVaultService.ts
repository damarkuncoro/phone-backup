import { safeInvoke } from "../shared/lib/ipc";

export interface WifiNetworkItem {
  ssid: string;
  bssid?: string;
  security_type: string;
  pre_shared_key?: string;
  wep_keys: string[];
  is_hidden: boolean;
  is_metered: boolean;
  priority: number;
}

export interface WifiStats {
  total_networks: number;
  open_networks: number;
  secured_networks: number;
  hidden_networks: number;
  metered_networks: number;
}

export interface WifiVaultDto {
  networks: WifiNetworkItem[];
  stats: WifiStats;
}

export interface BookmarkItem {
  id: string;
  title: string;
  url: string;
  folder: string;
  browser: string;
  date_added?: string;
}

export interface BookmarkStats {
  total_bookmarks: number;
  total_folders: number;
  top_domains: [string, number][];
}

export interface BookmarkVaultDto {
  bookmarks: BookmarkItem[];
  stats: BookmarkStats;
}

export interface ChecklistItem {
  text: string;
  is_checked: boolean;
}

export interface NoteItem {
  id: string;
  title: string;
  content: string;
  note_type: string;
  checklist: ChecklistItem[];
  tags: string[];
  created_at: string;
  updated_at: string;
}

export interface NoteStats {
  total_notes: number;
  total_checklists: number;
  total_items: number;
  completed_items: number;
  tags_count: Record<string, number>;
}

export interface NotesVaultDto {
  notes: NoteItem[];
  stats: NoteStats;
}

export interface CalendarEvent {
  uid: string;
  summary: string;
  description?: string;
  location?: string;
  start_time: string;
  end_time?: string;
  status: string;
  is_all_day: boolean;
}

export interface CalendarStats {
  total_events: number;
  recurring_events: number;
  events_with_location: number;
  events_with_attendees: number;
}

export interface CalendarVaultDto {
  events: CalendarEvent[];
  stats: CalendarStats;
  conflicts: string[];
}

export const dataVaultService = {
  async getWifiVault(deviceId?: string): Promise<WifiVaultDto> {
    return safeInvoke<WifiVaultDto>('get_wifi_vault', { device_id: deviceId });
  },

  async getWifiQr(ssid: string, password?: string, security = 'WPA_PSK', hidden = false): Promise<string> {
    return safeInvoke<string>('get_wifi_qr', { ssid, password, security, hidden });
  },

  async getBookmarksVault(deviceId?: string): Promise<BookmarkVaultDto> {
    return safeInvoke<BookmarkVaultDto>('get_bookmarks_vault', { device_id: deviceId });
  },

  async getNotesVault(deviceId?: string): Promise<NotesVaultDto> {
    return safeInvoke<NotesVaultDto>('get_notes_vault', { device_id: deviceId });
  },

  async getCalendarVault(deviceId?: string): Promise<CalendarVaultDto> {
    return safeInvoke<CalendarVaultDto>('get_calendar_vault', { device_id: deviceId });
  },
};
