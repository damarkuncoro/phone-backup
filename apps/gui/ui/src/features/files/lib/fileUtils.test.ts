import { describe, it, expect } from 'vitest';
import { sortFiles, isImage, hasSubfolders } from './fileUtils';
import { type FileEntry } from '@/services/deviceService';

describe('fileUtils', () => {
  describe('sortFiles', () => {
    it('should sort folders before files', () => {
      const files: Partial<FileEntry>[] = [
        { name: 'b.txt', is_dir: false },
        { name: 'a_folder', is_dir: true },
        { name: 'a.txt', is_dir: false },
        { name: 'b_folder', is_dir: true },
      ];

      const sorted = sortFiles(files as FileEntry[]);

      expect(sorted[0].name).toBe('a_folder');
      expect(sorted[1].name).toBe('b_folder');
      expect(sorted[2].name).toBe('a.txt');
      expect(sorted[3].name).toBe('b.txt');
    });
  });

  describe('isImage', () => {
    it('should return true for image extensions', () => {
      expect(isImage('photo.jpg')).toBe(true);
      expect(isImage('image.PNG')).toBe(true);
      expect(isImage('graphic.webp')).toBe(true);
    });

    it('should return false for non-image extensions', () => {
      expect(isImage('document.pdf')).toBe(false);
      expect(isImage('archive.zip')).toBe(false);
      expect(isImage('folder')).toBe(false);
    });
  });

  describe('folder detection', () => {
    it('should return true if list contains folders', () => {
      const files: Partial<FileEntry>[] = [
        { name: 'file.txt', is_dir: false },
        { name: 'folder', is_dir: true },
      ];
      expect(hasSubfolders(files as FileEntry[])).toBe(true);
    });

    it('should return false if list has no folders', () => {
      const files: Partial<FileEntry>[] = [
        { name: 'file.txt', is_dir: false },
      ];
      expect(hasSubfolders(files as FileEntry[])).toBe(false);
    });
  });
});
