import { useState, useCallback } from 'react';
import { deviceService } from '@/services/deviceService';

export function useBackupReviewData() {
  const [contacts, setContacts] = useState<any[]>([]);
  const [apps, setApps] = useState<any[]>([]);
  const [selectedContactIds, setSelectedContactIds] = useState<Set<string>>(new Set());
  const [loadingStructured, setLoadingStructured] = useState(false);

  const fetchLivePreviews = useCallback(async (deviceId: string, selectedData: string[]) => {
    setLoadingStructured(true);
    try {
      const promises: Promise<any>[] = [];

      if (selectedData.includes('contacts')) {
        promises.push(
          deviceService.getLiveData(deviceId, 'contacts')
            .then(data => {
              const list = Array.isArray(data) ? data : [];
              setContacts(list);
              setSelectedContactIds(new Set(list.map((c: any, i: number) => c.id || `${c.display_name}-${i}`)));
            })
            .catch(() => {
              setContacts([]);
              setSelectedContactIds(new Set());
            })
        );
      } else {
        setContacts([]);
        setSelectedContactIds(new Set());
      }

      if (selectedData.includes('apps')) {
        promises.push(
          deviceService.getLiveData(deviceId, 'apps')
            .then(data => setApps(Array.isArray(data) ? data : []))
            .catch(() => setApps([]))
        );
      } else {
        setApps([]);
      }

      await Promise.all(promises);
    } finally {
      setLoadingStructured(false);
    }
  }, []);

  const toggleContactId = useCallback((id: string) => {
    setSelectedContactIds(prev => {
      const next = new Set(prev);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });
  }, []);

  const selectAllContacts = useCallback(() => {
    setSelectedContactIds(new Set(contacts.map((c, i) => c.id || `${c.display_name}-${i}`)));
  }, [contacts]);

  const deselectAllContacts = useCallback(() => {
    setSelectedContactIds(new Set());
  }, []);

  return {
    contacts,
    apps,
    selectedContactIds,
    loadingStructured,
    fetchLivePreviews,
    toggleContactId,
    selectAllContacts,
    deselectAllContacts
  };
}
