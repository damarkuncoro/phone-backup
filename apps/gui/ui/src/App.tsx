import { useState, useEffect } from 'react'
import { MainLayout } from './app/layouts/MainLayout'
import { Dashboard } from './features/devices/pages/Dashboard'
import { DevicesPage } from './features/devices/pages/DevicesPage'
import { BackupWizard } from './features/backup/components/BackupWizard'
import { FileBrowser } from './features/files/components/FileBrowser'
import { HistoryPage } from './features/history/pages/HistoryPage'
import { SnapshotExplorer } from './features/explorer/pages/SnapshotExplorer'
import { DiffViewer } from './features/history/components/DiffViewer'
import { DeviceDetailsPage } from './features/devices/pages/DeviceDetailsPage'
import { SettingsPage } from './features/system/pages/SettingsPage'
import { SearchPage } from './features/search/pages/SearchPage'
import { useSearch } from './features/search/hooks/useSearch'
import { type Device } from './services/deviceService'
import { useDevices } from './features/devices/hooks/useDevices'
import { safeListen } from './shared/lib/ipc'
import { AddDeviceModal } from './features/devices/components/AddDeviceModal'

interface ProgressPayload {
  type: 'start' | 'inc' | 'finish' | 'error' | 'log';
  total?: number;
  amount?: number;
  message: string;
}

function App() {
  const [activeView, setActiveView] = useState('dashboard')
  const [selectedSnapshotId, setSelectedSnapshotId] = useState<string | null>(null)
  const [comparisonIds, setComparisonIds] = useState<{ oldId: string; newId: string } | null>(null)
  const [selectedDevice, setSelectedDevice] = useState<Device | null>(null)
  const [isAddDeviceOpen, setIsAddDeviceOpen] = useState(false)

  // Global background task telemetry
  const [activeTaskMsg, setActiveTaskMsg] = useState<string | null>(null)
  const [activeTaskProgress, setActiveTaskProgress] = useState<number | null>(null)
  const [taskTotal, setTaskTotal] = useState(0)
  const [, setTaskCurrent] = useState(0)

  const { devices, loading: isRefreshingDevices, refreshDevices } = useDevices();
  const { query, setQuery, results, isSearching } = useSearch();

  // Listen for live background task updates
  useEffect(() => {
    return safeListen<ProgressPayload>('progress', (event) => {
      const payload = event.payload;
      if (payload.type === 'start') {
        setActiveTaskMsg(payload.message || 'Memulai proses...');
        setTaskTotal(payload.total || 0);
        setTaskCurrent(0);
        setActiveTaskProgress(0);
      } else if (payload.type === 'inc') {
        setTaskCurrent(prev => {
          const next = prev + (payload.amount || 0);
          if (taskTotal > 0) {
            setActiveTaskProgress(Math.round((next / taskTotal) * 100));
          }
          return next;
        });
        if (payload.message) setActiveTaskMsg(payload.message);
      } else if (payload.type === 'finish') {
        setActiveTaskProgress(100);
        setActiveTaskMsg('Selesai');
        setTimeout(() => {
          setActiveTaskMsg(null);
          setActiveTaskProgress(null);
        }, 4000);
      } else if (payload.type === 'error') {
        setActiveTaskMsg(`Error: ${payload.message}`);
        setTimeout(() => {
          setActiveTaskMsg(null);
          setActiveTaskProgress(null);
        }, 5000);
      } else if (payload.type === 'log') {
        setActiveTaskMsg(payload.message);
      }
    });
  }, [taskTotal]);

  // Auto-select first device if none selected
  useEffect(() => {
    if (devices.length > 0 && !selectedDevice) {
      setSelectedDevice(devices[0]);
    }
  }, [devices, selectedDevice]);

  const handleOpenExplorer = (id: string) => {
    setSelectedSnapshotId(id);
    setActiveView('explorer');
  };

  const handleCompare = (oldId: string, newId: string) => {
    setComparisonIds({ oldId, newId });
    setActiveView('diff');
  };

  const handleOpenDetails = (device: Device) => {
    setSelectedDevice(device);
    setActiveView('device-details');
  };

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
            onBackupClick={(device) => {
              if (device) setSelectedDevice(device);
              setActiveView('backup');
            }}
            onDeviceDetails={handleOpenDetails}
            onNavigate={(view) => setActiveView(view as any)}
          />
        )}
        {activeView === 'devices' && (
          <DevicesPage onDeviceDetails={handleOpenDetails} />
        )}
        {activeView === 'backup' && (
          <BackupWizard
            initialDevice={selectedDevice}
            onFinish={() => setActiveView('dashboard')}
          />
        )}
        {activeView === 'files' && <FileBrowser />}
        {activeView === 'history' && (
          <HistoryPage
            onBrowse={handleOpenExplorer}
            onCompare={handleCompare}
          />
        )}
        {activeView === 'explorer' && selectedSnapshotId && (
          <SnapshotExplorer
            snapshotId={selectedSnapshotId}
            onBack={() => setActiveView('history')}
          />
        )}
        {activeView === 'diff' && comparisonIds && (
          <DiffViewer
            oldId={comparisonIds.oldId}
            newId={comparisonIds.newId}
            onBack={() => setActiveView('history')}
          />
        )}
        {activeView === 'device-details' && selectedDevice && (
          <DeviceDetailsPage
            device={selectedDevice}
            onBack={() => setActiveView('devices')}
            onStartBackup={() => setActiveView('backup')}
            onBrowseHistory={handleOpenExplorer}
            onNavigate={(v) => setActiveView(v as any)}
          />
        )}
        {activeView === 'settings' && <SettingsPage />}
        {activeView === 'search' && (
          <SearchPage
            query={query}
            onQueryChange={setQuery}
            results={results}
            isSearching={isSearching}
            onOpenFile={(file) => {
              console.log("Opening file:", file);
            }}
          />
        )}
        {activeView !== 'dashboard' &&
          activeView !== 'backup' &&
          activeView !== 'files' &&
          activeView !== 'history' &&
          activeView !== 'explorer' &&
          activeView !== 'diff' &&
          activeView !== 'devices' &&
          activeView !== 'device-details' &&
          activeView !== 'settings' &&
          activeView !== 'search' && (
            <div className="flex items-center justify-center h-full text-slate-400">
              <p className="font-black uppercase tracking-widest text-sm">
                View "{activeView}" coming soon
              </p>
            </div>
          )}
      </MainLayout>

      {/* Global Add Device Modal */}
      <AddDeviceModal
        isOpen={isAddDeviceOpen}
        onClose={() => setIsAddDeviceOpen(false)}
        onDeviceConnected={() => {
          refreshDevices();
          setIsAddDeviceOpen(false);
        }}
      />
    </>
  );
}

export default App;
