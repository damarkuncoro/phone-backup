import type { ReactNode } from 'react';
import { Sidebar } from './Sidebar';
import { GlobalHeader } from './GlobalHeader';
import { GlobalFooter } from './GlobalFooter';
import { type Device } from '@/services/deviceService';

interface MainLayoutProps {
  activeView: string;
  onViewChange: (view: string) => void;
  searchQuery: string;
  onSearchChange: (query: string) => void;
  devices: Device[];
  selectedDevice: Device | null;
  onSelectDevice: (device: Device) => void;
  onRefreshDevices: () => void;
  isRefreshingDevices?: boolean;
  onOpenAddDevice?: () => void;
  activeTaskMsg?: string | null;
  activeTaskProgress?: number | null;
  storageBackend?: string;
  children: ReactNode;
}

export function MainLayout({
  activeView,
  onViewChange,
  searchQuery,
  onSearchChange,
  devices,
  selectedDevice,
  onSelectDevice,
  onRefreshDevices,
  isRefreshingDevices,
  onOpenAddDevice,
  activeTaskMsg,
  activeTaskProgress,
  storageBackend,
  children
}: MainLayoutProps) {
  return (
    <div className="flex h-screen w-screen bg-slate-100 overflow-hidden font-sans antialiased text-slate-900 select-none">
      
      {/* 1. Left Sidebar */}
      <Sidebar
        activeView={activeView}
        onViewChange={onViewChange}
        searchQuery={searchQuery}
        onSearchChange={onSearchChange}
      />

      {/* 2. Main Column Container */}
      <div className="flex-1 flex flex-col min-w-0 h-full overflow-hidden bg-slate-50/50">
        
        {/* Global Topbar Header */}
        <GlobalHeader
          activeView={activeView}
          devices={devices}
          selectedDevice={selectedDevice}
          onSelectDevice={onSelectDevice}
          onRefreshDevices={onRefreshDevices}
          isRefreshingDevices={isRefreshingDevices}
          onOpenAddDevice={onOpenAddDevice}
        />

        {/* Scrollable Content Canvas */}
        <main className="flex-1 overflow-y-auto relative custom-scrollbar">
          {children}
        </main>

        {/* Global Bottom Status Bar / Footer */}
        <GlobalFooter
          activeTaskMsg={activeTaskMsg}
          activeTaskProgress={activeTaskProgress}
          storageBackend={storageBackend}
        />

      </div>

    </div>
  );
}
