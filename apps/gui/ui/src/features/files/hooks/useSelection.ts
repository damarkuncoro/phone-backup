import { useState, useCallback } from 'react';

export function useSelection() {
  const [selectedPaths, setSelectedPaths] = useState<string[]>([]);

  const toggle = useCallback((path: string) => {
    setSelectedPaths(prev =>
      prev.includes(path)
        ? prev.filter(p => p !== path)
        : [...prev, path]
    );
  }, []);

  const selectAll = useCallback((paths: string[]) => {
    setSelectedPaths(paths);
  }, []);

  const clear = useCallback(() => {
    setSelectedPaths([]);
  }, []);

  const isSelected = useCallback((path: string) => {
    return selectedPaths.includes(path);
  }, [selectedPaths]);

  return {
    selectedPaths,
    toggle,
    selectAll,
    clear,
    isSelected,
    count: selectedPaths.length
  };
}
