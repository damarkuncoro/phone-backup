import { describe, it, expect } from 'vitest';
import { getParentPath, joinPath } from './pathUtils';

describe('pathUtils', () => {
  describe('getParentPath', () => {
    it('should return / for a root-level subdirectory', () => {
      expect(getParentPath('/sdcard')).toBe('/');
    });

    it('should return /sdcard for a nested directory', () => {
      expect(getParentPath('/sdcard/DCIM')).toBe('/sdcard');
    });

    it('should return / for root path', () => {
      expect(getParentPath('/')).toBe('/');
    });

    it('should handle paths with trailing slashes', () => {
      expect(getParentPath('/sdcard/DCIM/')).toBe('/sdcard');
    });
  });

  describe('joinPath', () => {
    it('should join root and directory correctly', () => {
      expect(joinPath('/', 'sdcard')).toBe('/sdcard');
    });

    it('should join directory and subdirectory correctly', () => {
      expect(joinPath('/sdcard', 'DCIM')).toBe('/sdcard/DCIM');
    });

    it('should avoid double slashes', () => {
      expect(joinPath('/sdcard/', 'DCIM')).toBe('/sdcard/DCIM');
    });
  });
});
