import { useState, useEffect } from 'react';
import { MainLayout } from './app/layouts/MainLayout';
import { Dashboard } from './features/devices/pages/Dashboard';
import { DevicesPage } from './features/devices/pages/DevicesPage';
import { BackupWizard } from './features/backup/components/BackupWizard';
import { FileBrowser } from './features/files/components/FileBrowser';
import { HistoryPage } from './features/history/pages/HistoryPage';
import { SnapshotExplorer } from './features/explorer/pages/SnapshotExplorer';
import { DiffViewer } from './features/history/components/DiffViewer';
import { DeviceDetailsPage } from './features/devices/pages/DeviceDetailsPage';
import { SettingsPage } from './features/system/pages/SettingsPage';
import { SearchPage } from './features/search/pages/SearchPage';
import { WhatsAppArchivePage } from './features/whatsapp/pages/WhatsAppArchivePage';
import { AppAuditPage } from './features/audit/pages/AppAuditPage';
import { MediaLabPage } from './features/media/pages/MediaLabPage';
import { WirelessPairingPage } from './features/wireless/pages/WirelessPairingPage';
import { useSearch } from './features/search/hooks/useSearch';
import { type Device } from './services/deviceService';
import { useDevices } from './features/devices/hooks/useDevices';
import { safeListen } from './shared/lib/ipc';
import { AddDeviceModal } from './features/devices/components/AddDeviceModal';

interface ProgressPayload {
  type: 'start' | 'inc' | 'finish' | 'error' | 'log';
  total?: number;
  amount?: number;
  message: string;
}

function App() {
  const [activeView, setActiveView] = useState('dashboard');
  const [selectedSnapshotId, setSelectedSnapshotId] = useState<string | null>(null);
  const [comparisonIds, setComparisonIds] = useState<{ oldId: string; newId: string } | null>(null);
  const [selectedDevice, setSelectedDevice] = useState<Device | null>(null);
  const [isAddDeviceOpen, setIsAddDeviceOpen] = useState(false);

  const [activeTaskMsg, setActiveTaskMsg] = useState<string | null>(null);
  const [activeTaskProgress, setActiveTaskProgress] = useState<number | null>(null);
  const [taskTotal, setTaskTotal] = useState(0);
  const [, setTaskCurrent] = useState(0);

  const { devices, loading: isRefreshingDevices, refreshDevices } = useDevices();
  const { query, setQuery, results, isSearching } = useSearch();

  useEffect(() => {
    return safeListen<ProgressPayload>('progress', (event) => {
      const p = event.payload;
      if (p.type === 'start') {
        setActiveTaskMsg(p.message || 'Memulai...');
        setTaskTotal(p.total || 0);
        setTaskCurrent(0);
        setActiveTaskProgress(0);
      } else if (p.type === 'inc') {
        setTaskCurrent((prev) => {
          const next = prev + (p.amount || 0);
          if (taskTotal > 0) setActiveTaskProgress(Math.round((next / taskTotal) * 100));
          return next;
        });
        if (p.message) setActiveTaskMsg(p.message);
      } else if (p.type === 'finish') {
        setActiveTaskProgress(100);
        setActiveTaskMsg('Selesai');
        setTimeout(() => { setActiveTaskMsg(null); setActiveTaskProgress(null); }, 4000);
      } else if (p.type === 'error') {
        setActiveTaskMsg(`Error: ${p.message}`);
        setTimeout(() => { setActiveTaskMsg(null); setActiveTaskProgress(null); }, 5000);
      } else if (p.type === 'log') {
        setActiveTaskMsg(p.message);
      }
    });
  }, [taskTotal]);

  useEffect(() => {
    if (devices.length > 0 && !selectedDevice) setSelectedDevice(devices[0]);
  }, [devices, selectedDevice]);

  return (
    <>
      <MainLayout
        activeView={activeView}
        onViewChange={setActiveView}
        searchQuery={query}
        onSearchChange={setQuery}
        devices={devices}
        selectedDevice={selectedDevice}
        onSelectDevice={setSelectedDevice}
        onRefreshDevices={refreshDevices}
        isRefreshingDevices={isRefreshingDevices}
        onOpenAddDevice={() => setIsAddDeviceOpen(true)}
        activeTaskMsg={activeTaskMsg}
        activeTaskProgress={activeTaskProgress}
      >
        {activeView === 'dashboard' && (
          <Dashboard
            onBackupClick={(d) => { if (d) setSelectedDevice(d); setActiveView('backup'); }}
            onDeviceDetails={(d) => { setSelectedDevice(d); setActiveView('device-details'); }}
            onNavigate={(v) => setActiveView(v as any)}
          />
        )}
        {activeView === 'devices' && (
          <DevicesPage onDeviceDetails={(d) => { setSelectedDevice(d); setActiveView('device-details'); }} />
        )}
        {activeView === 'backup' && (
          <BackupWizard initialDevice={selectedDevice} onFinish={() => setActiveView('dashboard')} />
        )}
        {activeView === 'files' && <FileBrowser />}
        {activeView === 'history' && (
          <HistoryPage
            onBrowse={(id) => { setSelectedSnapshotId(id); setActiveView('explorer'); }}
            onCompare={(oldId, newId) => { setComparisonIds({ oldId, newId }); setActiveView('diff'); }}
          />
        )}
        {activeView === 'explorer' && selectedSnapshotId && (
          <SnapshotExplorer snapshotId={selectedSnapshotId} onBack={() => setActiveView('history')} />
        )}
        {activeView === 'diff' && comparisonIds && (
          <DiffViewer oldId={comparisonIds.oldId} newId={comparisonIds.newId} onBack={() => setActiveView('history')} />
        )}
        {activeView === 'device-details' && selectedDevice && (
          <DeviceDetailsPage
            device={selectedDevice}
            onBack={() => setActiveView('devices')}
            onStartBackup={() => setActiveView('backup')}
            onBrowseHistory={(id) => { setSelectedSnapshotId(id); setActiveView('explorer'); }}
            onNavigate={(v) => setActiveView(v as any)}
          />
        )}
        {activeView === 'whatsapp' && <WhatsAppArchivePage />}
        {activeView === 'audit' && <AppAuditPage />}
        {activeView === 'media' && <MediaLabPage />}
        {activeView === 'wireless' && <WirelessPairingPage />}
        {activeView === 'settings' && <SettingsPage />}
        {activeView === 'search' && (
          <SearchPage query={query} onQueryChange={setQuery} results={results} isSearching={isSearching} onOpenFile={(f) => console.log('Opening file:', f)} />
        )}
      </MainLayout>

      <AddDeviceModal
        isOpen={isAddDeviceOpen}
        onClose={() => setIsAddDeviceOpen(false)}
        onDeviceConnected={() => { refreshDevices(); setIsAddDeviceOpen(false); }}
      />
    </>
  );
}

export default App;
