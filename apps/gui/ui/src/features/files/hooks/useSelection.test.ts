import { describe, it, expect } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { useSelection } from './useSelection';

describe('useSelection hook', () => {
  it('should start with empty selection', () => {
    const { result } = renderHook(() => useSelection());
    expect(result.current.selectedPaths).toEqual([]);
  });

  it('should toggle selection correctly', () => {
    const { result } = renderHook(() => useSelection());

    act(() => {
      result.current.toggle('/file1');
    });
    expect(result.current.selectedPaths).toEqual(['/file1']);

    act(() => {
      result.current.toggle('/file1');
    });
    expect(result.current.selectedPaths).toEqual([]);
  });

  it('should clear selection', () => {
    const { result } = renderHook(() => useSelection());

    act(() => {
      result.current.toggle('/file1');
      result.current.clear();
    });
    expect(result.current.selectedPaths).toEqual([]);
  });

  it('should handle select all', () => {
    const { result } = renderHook(() => useSelection());
    const paths = ['/a', '/b', '/c'];

    act(() => {
      result.current.selectAll(paths);
    });
    expect(result.current.selectedPaths).toEqual(paths);
  });
});
