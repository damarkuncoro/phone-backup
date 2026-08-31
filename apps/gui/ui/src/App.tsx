import { useState, useEffect } from 'react'
import { Sidebar } from './app/layouts/Sidebar'
import { RightAside } from './app/layouts/RightAside'
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

interface LogEntry {
    time: string;
    msg: string;
}

function App() {
  const [activeView, setActiveView] = useState('dashboard')
  const [selectedSnapshotId, setSelectedSnapshotId] = useState<string | null>(null)
  const [comparisonIds, setComparisonIds] = useState<{oldId: string, newId: string} | null>(null)
  const [selectedDevice, setSelectedDevice] = useState<Device | null>(null)
  const [asideCollapsed, setAsideCollapsed] = useState(false)
  const [logs, setLogs] = useState<LogEntry[]>([
    { time: "Sistem", msg: "Engine siap digunakan" }
  ]);

  const { devices } = useDevices();
  const { query, setQuery, results, isSearching } = useSearch();

  // Listen for logs from backend
  useEffect(() => {
    return safeListen('progress', (event) => {
        const payload = event.payload as any;
        if (payload.type === 'log' || payload.type === 'start' || payload.type === 'finish') {
            const newLog = {
                time: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
                msg: payload.message
            };
            setLogs(prev => [newLog, ...prev].slice(0, 10)); // Simpan 10 log terakhir
        }
    });
  }, []);

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
    <div className="flex h-screen w-screen bg-slate-50 overflow-hidden font-sans antialiased text-slate-900">
      <Sidebar
        activeView={activeView}
        onViewChange={setActiveView}
        searchQuery={query}
        onSearchChange={setQuery}
      />

      <main className="flex-1 overflow-y-auto bg-white/50 backdrop-blur-sm relative">
        {activeView === 'dashboard' && (
          <Dashboard
            onBackupClick={() => setActiveView('backup')}
            onDeviceDetails={handleOpenDetails}
          />
        )}
        {activeView === 'devices' && (
          <DevicesPage onDeviceDetails={handleOpenDetails} />
        )}
        {activeView === 'backup' && <BackupWizard />}
        {activeView === 'files' && <FileBrowser />}
        {activeView === 'history' && <HistoryPage onBrowse={handleOpenExplorer} onCompare={handleCompare} />}
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
            onBrowseHistory={handleOpenExplorer}
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
        {activeView !== 'dashboard' && activeView !== 'backup' && activeView !== 'files' && activeView !== 'history' && activeView !== 'explorer' && activeView !== 'diff' && activeView !== 'devices' && activeView !== 'device-details' && activeView !== 'settings' && activeView !== 'search' && (
          <div className="flex items-center justify-center h-full text-slate-400">
            <p className="font-black uppercase tracking-widest text-sm">View "{activeView}" coming soon</p>
          </div>
        )}
      </main>

      <RightAside
        activeView={activeView}
        selectedDevice={selectedDevice}
        logs={logs}
        isCollapsed={asideCollapsed}
        onToggle={() => setAsideCollapsed(!asideCollapsed)}
        onBackupClick={() => setActiveView('backup')}
      />
    </div>
  )
}

export default App
