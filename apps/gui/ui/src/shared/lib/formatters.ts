/**
 * Formats a size in bytes to a human-readable GB string.
 * @param bytes The size in bytes
 * @returns Formatted string (e.g. "12.5 GB")
 */
export function formatStorageSize(bytes: number): string {
  const gb = bytes / (1024 * 1024 * 1024);
  return `${gb.toFixed(1)} GB`;
}

/**
 * Formats a Date or ISO string to a human-readable format.
 */
export function formatDate(dateStr: string | null): string {
  if (!dateStr) return 'N/A';
  const date = new Date(dateStr);
  return new Intl.DateTimeFormat('en-US', {
    month: 'short',
    day: 'numeric',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  }).format(date);
}

/**
 * Formats a Unix timestamp (seconds) to a human-readable format.
 */
export function formatUnixTimestamp(seconds: number | null): string {
    if (!seconds || seconds === 0) return 'N/A';
    const date = new Date(seconds * 1000);
    return new Intl.DateTimeFormat('en-US', {
        month: 'short',
        day: 'numeric',
        year: 'numeric',
    }).format(date);
}

/**
 * Formats bytes to the most appropriate unit (B, KB, MB, GB, TB).
 */
export function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    const val = parseFloat((bytes / Math.pow(k, i)).toFixed(2));
    return `${val} ${sizes[i]}`;
}

/**
 * Formats milliseconds remaining into a human-readable ETA string.
 */
export function formatETA(ms: number): string {
    if (ms <= 0) return "Selesai";
    const seconds = Math.ceil(ms / 1000);
    if (seconds < 60) return `${seconds} detik lagi`;
    const minutes = Math.floor(seconds / 60);
    return `${minutes} menit lagi`;
}
