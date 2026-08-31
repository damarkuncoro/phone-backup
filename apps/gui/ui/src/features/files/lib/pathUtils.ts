/**
 * Returns the parent directory path.
 * Example: /sdcard/DCIM -> /sdcard
 * Example: /sdcard -> /
 */
export function getParentPath(path: string): string {
  if (path === '/') return '/';

  // Remove trailing slash if exists
  const normalized = path.endsWith('/') && path.length > 1
    ? path.slice(0, -1)
    : path;

  const parts = normalized.split('/').filter(Boolean);
  parts.pop();

  return '/' + parts.join('/');
}

/**
 * Joins a directory and a file/folder name correctly.
 */
export function joinPath(base: string, name: string): string {
  const b = base.endsWith('/') ? base.slice(0, -1) : base;
  const n = name.startsWith('/') ? name.slice(1) : name;
  return `${b}/${n}`;
}
