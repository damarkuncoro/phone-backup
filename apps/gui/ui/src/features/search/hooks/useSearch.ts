import { useState, useEffect, useCallback } from 'react';
import { systemService } from '@/services/systemService';
import { type FileEntry } from '@/services/deviceService';

export function useSearch() {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<FileEntry[]>([]);
  const [isSearching, setIsSearching] = useState(false);

  const performSearch = useCallback(async (q: string) => {
    if (!q || q.length < 2) {
      setResults([]);
      return;
    }

    setIsSearching(true);
    try {
      const data = await systemService.searchFiles(q);
      setResults(data);
    } catch (err) {
      console.error("Search failed", err);
      setResults([]);
    } finally {
      setIsSearching(false);
    }
  }, []);

  // Debounce search to 300ms
  useEffect(() => {
    const timer = setTimeout(() => {
      performSearch(query);
    }, 300);

    return () => clearTimeout(timer);
  }, [query, performSearch]);

  return {
    query, setQuery,
    results, isSearching
  };
}
