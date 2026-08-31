import { type FileEntry } from "@/services/deviceService";

export type SortField = 'name' | 'size' | 'date';
export type SortDirection = 'asc' | 'desc';
export type FileCategory = 'all' | 'images' | 'videos' | 'documents' | 'audio' | 'folders';

/**
 * Sorts file entries: folders first, then files, with custom field & direction.
 */
export function sortFiles(
  files: FileEntry[],
  sortBy: SortField = 'name',
  direction: SortDirection = 'asc'
): FileEntry[] {
  return [...files].sort((a, b) => {
    // Folders always precede regular files
    if (a.is_dir !== b.is_dir) {
      return a.is_dir ? -1 : 1;
    }

    let comparison = 0;
    if (sortBy === 'name') {
      comparison = a.name.localeCompare(b.name, undefined, { numeric: true, sensitivity: 'base' });
    } else if (sortBy === 'size') {
      comparison = (a.size_bytes || 0) - (b.size_bytes || 0);
    } else if (sortBy === 'date') {
      const dateA = a.modified_at ? new Date(a.modified_at).getTime() : 0;
      const dateB = b.modified_at ? new Date(b.modified_at).getTime() : 0;
      comparison = dateA - dateB;
    }

    return direction === 'asc' ? comparison : -comparison;
  });
}

/**
 * Checks if a filename has an image extension.
 */
export function isImage(filename: string): boolean {
  const ext = filename.split('.').pop()?.toLowerCase();
  return ['jpg', 'jpeg', 'png', 'gif', 'webp', 'svg', 'bmp', 'heic', 'heif'].includes(ext || '');
}

/**
 * Checks if a filename has a video extension.
 */
export function isVideo(filename: string): boolean {
  const ext = filename.split('.').pop()?.toLowerCase();
  return ['mp4', 'mkv', 'mov', 'avi', 'webm', '3gp', 'flv', 'm4v'].includes(ext || '');
}

/**
 * Checks if a filename has an audio/music extension.
 */
export function isAudio(filename: string): boolean {
  const ext = filename.split('.').pop()?.toLowerCase();
  return ['mp3', 'm4a', 'flac', 'wav', 'ogg', 'aac', 'opus'].includes(ext || '');
}

/**
 * Checks if a filename is a document.
 */
export function isDocument(filename: string): boolean {
  const ext = filename.split('.').pop()?.toLowerCase();
  return ['pdf', 'doc', 'docx', 'xls', 'xlsx', 'ppt', 'pptx', 'txt', 'md', 'json', 'csv', 'zip', 'tar', 'gz'].includes(ext || '');
}

/**
 * Checks if a filename is an Android APK package.
 */
export function isApk(filename: string): boolean {
  const ext = filename.split('.').pop()?.toLowerCase();
  return ['apk', 'xapk', 'apkm', 'apks'].includes(ext || '');
}

/**
 * Filters files by active category.
 */
export function filterByCategory(files: FileEntry[], category: FileCategory): FileEntry[] {
  if (category === 'all') return files;
  if (category === 'folders') return files.filter(f => f.is_dir);
  if (category === 'images') return files.filter(f => !f.is_dir && isImage(f.name));
  if (category === 'videos') return files.filter(f => !f.is_dir && isVideo(f.name));
  if (category === 'documents') return files.filter(f => !f.is_dir && isDocument(f.name));
  if (category === 'audio') return files.filter(f => !f.is_dir && isAudio(f.name));
  return files;
}

/**
 * Returns true if the list of entries contains at least one folder.
 */
export function hasSubfolders(files: FileEntry[]): boolean {
  return files.some(file => file.is_dir);
}
