import { type FileEntry } from "@/services/deviceService";

export interface TreeNode {
  name: string;
  path: string;
  type: 'file' | 'folder';
  size: number;
  children: Record<string, TreeNode>;
}

/**
 * Builds a hierarchical tree structure from a flat list of FileEntries.
 */
export function buildFileTree(files: FileEntry[]): TreeNode {
  const root: TreeNode = { name: 'Root', path: '', type: 'folder', size: 0, children: {} };

  (files || []).forEach(file => {
    if (!file || typeof file.path !== 'string') return;

    const parts = file.path.split('/').filter(Boolean);
    let current = root;

    parts.forEach((part, index) => {
      const isLast = index === parts.length - 1;
      const currentPath = '/' + parts.slice(0, index + 1).join('/');

      if (!current.children[part]) {
        current.children[part] = {
          name: part,
          path: currentPath,
          type: isLast ? 'file' : 'folder',
          size: isLast ? (file.size_bytes || 0) : 0,
          children: {}
        };
      } else if (isLast) {
          current.children[part].size = file.size_bytes || 0;
      }

      if (!isLast) {
        current = current.children[part];
      }
    });
  });

  return root;
}

/**
 * Determines if a node should be visible based on a search query.
 * A node is visible if its name matches the query OR if any of its children should be visible.
 */
export function shouldShowNode(node: TreeNode, query: string): boolean {
  if (!query) return true;
  const nameMatch = node.name.toLowerCase().includes(query.toLowerCase());
  if (nameMatch) return true;

  return Object.values(node.children).some(child => shouldShowNode(child, query));
}
