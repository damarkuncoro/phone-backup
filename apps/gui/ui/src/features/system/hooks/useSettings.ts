import { useState, useEffect, useCallback } from 'react';
import { systemService, type DoctorReport, type AppSettings } from '@/services/systemService';

export function useSettings() {
  const [report, setReport] = useState<DoctorReport | null>(null);
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [keys, setKeys] = useState<[string, string] | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [msg, setMsg] = useState<{ type: 'success' | 'error', text: string } | null>(null);

  const loadData = useCallback(async () => {
    try {
      const [rep, sett, k] = await Promise.all([
        systemService.getDoctorReport(),
        systemService.getSettings(),
        systemService.generateKeys().catch(() => ["Error", "Key derivation failed"] as [string, string])
      ]);
      setReport(rep);
      setSettings(sett);
      setKeys(k as [string, string]);
    } catch (err) {
      console.error("Failed to load settings", err);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadData();
  }, [loadData]);

  const handleSave = async () => {
    if (!settings) return;
    setSaving(true);
    try {
      await systemService.saveSettings(settings);
      setMsg({ type: 'success', text: 'Pengaturan berhasil disimpan' });
      setTimeout(() => setMsg(null), 3000);
    } catch (err) {
      setMsg({ type: 'error', text: 'Gagal menyimpan pengaturan' });
    } finally {
      setSaving(false);
    }
  };

  const runMaintenance = async (task: 'gc' | 'prune') => {
    try {
        const result = task === 'gc' ? await systemService.runGC() : await systemService.pruneFailed();
        setMsg({ type: 'success', text: `Tugas selesai: ${result} objek diproses` });
        setTimeout(() => setMsg(null), 5000);
    } catch (err) {
        setMsg({ type: 'error', text: 'Tugas pemeliharaan gagal' });
    }
  };

  return {
    report,
    settings, setSettings,
    keys,
    loading,
    saving,
    msg,
    handleSave,
    runMaintenance
  };
}
