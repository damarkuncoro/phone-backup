import { describe, it, expect } from 'vitest';
import { buildFileTree, shouldShowNode } from './explorerUtils';
import { type FileEntry } from '@/services/deviceService';

describe('explorerUtils', () => {
  describe('buildFileTree', () => {
    const mockData: Partial<FileEntry>[] = [
      { path: '/storage/emulated/0/DCIM/photo.jpg', name: 'photo.jpg', size_bytes: 1000 },
      { path: '/storage/emulated/0/Download/doc.pdf', name: 'doc.pdf', size_bytes: 2000 },
    ];

    it('should build a nested tree structure', () => {
      const tree = buildFileTree(mockData as FileEntry[]);

      expect(tree.children['storage']).toBeDefined();
      expect(tree.children['storage'].type).toBe('folder');

      const emulated = tree.children['storage'].children['emulated'];
      expect(emulated).toBeDefined();

      const dcim = emulated.children['0'].children['DCIM'];
      expect(dcim.children['photo.jpg']).toBeDefined();
      expect(dcim.children['photo.jpg'].type).toBe('file');
      expect(dcim.children['photo.jpg'].size).toBe(1000);
    });

    it('should handle root-level files', () => {
        const data: Partial<FileEntry>[] = [{ path: '/file.txt', name: 'file.txt', size_bytes: 50 }];
        const tree = buildFileTree(data as FileEntry[]);
        expect(tree.children['file.txt']).toBeDefined();
        expect(tree.children['file.txt'].size).toBe(50);
    });
  });

  describe('shouldShowNode', () => {
    const tree = buildFileTree([
      { path: '/DCIM/Camera/pic1.jpg', name: 'pic1.jpg' } as any,
      { path: '/Documents/work.pdf', name: 'work.pdf' } as any,
    ]);

    it('should return true if filename matches', () => {
        const cameraNode = tree.children['DCIM'].children['Camera'];
        const picNode = cameraNode.children['pic1.jpg'];
        expect(shouldShowNode(picNode, 'pic1')).toBe(true);
    });

    it('should return true for parent if child matches', () => {
        const dcimNode = tree.children['DCIM'];
        expect(shouldShowNode(dcimNode, 'pic1')).toBe(true);
    });

    it('should return false if neither node nor children match', () => {
        const dcimNode = tree.children['DCIM'];
        expect(shouldShowNode(dcimNode, 'secret')).toBe(false);
    });
  });
});
