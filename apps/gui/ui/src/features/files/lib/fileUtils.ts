import { type FileEntry } from "@/services/deviceService";

/**
 * Sorts file entries: folders first, then files, both alphabetically.
 */
export function sortFiles(files: FileEntry[]): FileEntry[] {
  return [...files].sort((a, b) => {
    // Both are directories or both are files
    if (a.is_dir === b.is_dir) {
      return a.name.localeCompare(b.name);
    }
    // Folders come first
    return a.is_dir ? -1 : 1;
  });
}

/**
 * Checks if a filename has an image extension.
 */
export function isImage(filename: string): boolean {
  const ext = filename.split('.').pop()?.toLowerCase();
  return ['jpg', 'jpeg', 'png', 'gif', 'webp', 'svg', 'bmp'].includes(ext || '');
}

/**
 * Returns true if the list of entries contains at least one folder.
 * This is used to determine if a directory "has subfolders" when viewing its contents.
 */
export function hasSubfolders(files: FileEntry[]): boolean {
  return files.some(file => file.is_dir);
}
