import { describe, it, expect } from 'vitest';
import { formatStorageSize, formatBytes, formatUnixTimestamp, formatETA } from './formatters';

describe('formatters', () => {
  describe('formatStorageSize', () => {
    it('should format bytes to GB correctly', () => {
      const bytes = 100 * 1024 * 1024 * 1024;
      expect(formatStorageSize(bytes)).toBe('100.0 GB');
    });
  });

  describe('formatBytes', () => {
    it('should format B correctly', () => {
      expect(formatBytes(512)).toBe('512 B');
    });

    it('should format KB correctly', () => {
      expect(formatBytes(1024)).toBe('1 KB');
    });

    it('should format MB correctly', () => {
      expect(formatBytes(1024 * 1024 * 5.5)).toBe('5.5 MB');
    });

    it('should format GB correctly', () => {
      expect(formatBytes(1024 * 1024 * 1024 * 2.25)).toBe('2.25 GB');
    });

    it('should handle zero', () => {
      expect(formatBytes(0)).toBe('0 B');
    });
  });

  describe('formatUnixTimestamp', () => {
    it('should format a unix timestamp (seconds) correctly', () => {
      // 2024-01-01 12:00:00 UTC
      const timestamp = 1704110400;
      const result = formatUnixTimestamp(timestamp);
      expect(result).toContain('Jan 1, 2024');
    });

    it('should handle zero or null', () => {
        expect(formatUnixTimestamp(0)).toBe('N/A');
    });
  });

  describe('formatETA', () => {
    it('should format seconds correctly', () => {
      expect(formatETA(5000)).toBe('5 detik lagi');
    });

    it('should format minutes correctly', () => {
      expect(formatETA(65000)).toBe('1 menit lagi');
    });

    it('should handle finished state', () => {
      expect(formatETA(0)).toBe('Selesai');
      expect(formatETA(-100)).toBe('Selesai');
    });
  });
});
