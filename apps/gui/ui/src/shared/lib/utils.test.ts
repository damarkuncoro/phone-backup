import { describe, it, expect } from 'vitest';
import { cn } from './utils';

describe('cn utility', () => {
  it('should merge tailwind classes correctly', () => {
    const result = cn('px-2 py-2', 'px-4');
    // twMerge should prefer the later 'px-4' over 'px-2'
    expect(result).toBe('py-2 px-4');
  });

  it('should handle conditional classes', () => {
    const result = cn('base', true && 'active', false && 'hidden');
    expect(result).toBe('base active');
  });

  it('should return empty string for no inputs', () => {
    expect(cn()).toBe('');
  });
});
