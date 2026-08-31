import { useState } from 'react';
import { RefreshCw, Save } from 'lucide-react';
import { useSettings } from '../hooks/useSettings';
import { systemService } from '@/services/systemService';
import { UI_TOKENS } from '@/shared/theme/tokens';
import { SettingsHeader } from '../components/SettingsHeader';
import { SettingsNav, type SettingsTab } from '../components/SettingsNav';
import { SettingsDoctorTab } from '../components/SettingsDoctorTab';
import { SettingsStorageTab } from '../components/SettingsStorageTab';
import { SettingsSecurityTab } from '../components/SettingsSecurityTab';
import { SettingsMaintenanceTab } from '../components/SettingsMaintenanceTab';
import { SettingsAboutTab } from '../components/SettingsAboutTab';

export function SettingsPage() {
  const {
    report,
    settings, setSettings,
    keys,
    loading,
    saving,
    msg,
    handleSave,
    runMaintenance
  } = useSettings();

  const [activeTab, setActiveTab] = useState<SettingsTab>('doctor');
  const [copiedKey, setCopiedKey] = useState(false);
  const [refreshingDoctor, setRefreshingDoctor] = useState(false);

  // S3 Form State if cloud backend is selected
  const [s3Bucket, setS3Bucket] = useState('');
  const [s3Endpoint, setS3Endpoint] = useState('');
  const [s3Region, setS3Region] = useState('us-east-1');
  const [s3AccessKey, setS3AccessKey] = useState('');
  const [s3SecretKey, setS3SecretKey] = useState('');

  if (loading) {
    return (
      <div className="h-full flex flex-col items-center justify-center gap-4 text-slate-400 py-24">
        <RefreshCw className="w-8 h-8 animate-spin text-indigo-500" />
        <p className="text-xs font-black uppercase tracking-widest">Mendiagnosa Infrastruktur Sistem...</p>
      </div>
    );
  }

  const currentBackendType = typeof settings?.storage_backend === 'string'
    ? settings.storage_backend
    : Object.keys(settings?.storage_backend || { Local: null })[0];

  const handleCopyPublicKey = () => {
    if (keys && keys[1]) {
      navigator.clipboard.writeText(keys[1]);
      setCopiedKey(true);
      setTimeout(() => setCopiedKey(false), 2500);
    }
  };

  const handleOpenFolder = async (folderType: 'restore' | 'downloads') => {
    try {
      if (folderType === 'restore') {
        await systemService.openRestoreFolder();
      } else {
        await systemService.openDownloadsFolder();
      }
    } catch {
      console.warn("Gagal membuka folder di sistem operasi");
    }
  };

  const handleRefreshDoctor = async () => {
    setRefreshingDoctor(true);
    try {
      await systemService.getDoctorReport();
    } catch {
      console.warn("Refresh doctor failed");
    } finally {
      setTimeout(() => setRefreshingDoctor(false), 600);
    }
  };

  return (
    <div className={UI_TOKENS.layout.pageContainer}>
      
      {/* Header Banner */}
      <SettingsHeader msg={msg} />

      {/* Navigation Tabs */}
      <SettingsNav
        activeTab={activeTab}
        onTabChange={setActiveTab}
        doctorHealthy={Boolean(report?.adb_found && report?.db_healthy)}
        currentBackendType={currentBackendType}
      />

      {/* Tab Panels */}
      <div className="space-y-6">

        {/* 1. SYSTEM DOCTOR TAB */}
        {activeTab === 'doctor' && (
          <SettingsDoctorTab
            report={report}
            currentBackendType={currentBackendType}
            refreshingDoctor={refreshingDoctor}
            onRefreshDoctor={handleRefreshDoctor}
          />
        )}

        {/* 2. STORAGE TAB */}
        {activeTab === 'storage' && (
          <SettingsStorageTab
            currentBackendType={currentBackendType}
            onSelectBackend={(backend) => setSettings(s => s ? { ...s, storage_backend: backend } : null)}
            s3Bucket={s3Bucket}
            setS3Bucket={setS3Bucket}
            s3Region={s3Region}
            setS3Region={setS3Region}
            s3Endpoint={s3Endpoint}
            setS3Endpoint={setS3Endpoint}
            s3AccessKey={s3AccessKey}
            setS3AccessKey={setS3AccessKey}
            s3SecretKey={s3SecretKey}
            setS3SecretKey={setS3SecretKey}
            onOpenFolder={handleOpenFolder}
          />
        )}

        {/* 3. SECURITY TAB */}
        {activeTab === 'security' && (
          <SettingsSecurityTab
            keys={keys}
            copiedKey={copiedKey}
            onCopyPublicKey={handleCopyPublicKey}
          />
        )}

        {/* 4. MAINTENANCE TAB */}
        {activeTab === 'maintenance' && (
          <SettingsMaintenanceTab
            onRunMaintenance={runMaintenance}
          />
        )}

        {/* 5. ABOUT TAB */}
        {activeTab === 'about' && (
          <SettingsAboutTab />
        )}

      </div>

      {/* Sticky Save Action Bar */}
      <div className="sticky bottom-4 z-20 flex justify-end bg-white/90 backdrop-blur-md p-4 rounded-3xl border border-slate-200/80 shadow-2xl">
        <button
          type="button"
          disabled={saving}
          onClick={handleSave}
          className="px-8 py-3.5 bg-indigo-600 hover:bg-indigo-700 text-white rounded-2xl font-black text-xs uppercase tracking-wider shadow-lg shadow-indigo-200 hover:shadow-indigo-300 disabled:opacity-50 transition-all flex items-center gap-2 active:scale-95"
        >
          {saving ? <RefreshCw className="w-4 h-4 animate-spin" /> : <Save className="w-4 h-4" />}
          {saving ? "Menyimpan..." : "Simpan Perubahan Pengaturan"}
        </button>
      </div>

    </div>
  );
}
