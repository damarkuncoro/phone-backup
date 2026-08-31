import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import { MainLayout } from './MainLayout';
import { type Device } from '@/services/deviceService';

const mockDevice: Device = {
  id: 'pixel8',
  model: 'Google Pixel 8',
  manufacturer: 'Google',
  serial: 'P8123',
  os_version: '15',
  connection_type: 'Usb',
  storage_used_bytes: 40000000000,
  storage_total_bytes: 128000000000,
  storage_free_bytes: 88000000000,
};

describe('MainLayout Component', () => {
  it('renders Sidebar, GlobalHeader, Content Canvas, and GlobalFooter', () => {
    render(
      <MainLayout
        activeView="dashboard"
        onViewChange={vi.fn()}
        searchQuery=""
        onSearchChange={vi.fn()}
        devices={[mockDevice]}
        selectedDevice={mockDevice}
        onSelectDevice={vi.fn()}
        onRefreshDevices={vi.fn()}
      >
        <div data-testid="test-content">Dashboard Content Canvas</div>
      </MainLayout>
    );

    // Sidebar & GlobalHeader
    expect(screen.getByText('PB PRO')).toBeInTheDocument();
    expect(screen.getAllByText('Dashboard').length).toBe(2);

    // GlobalHeader
    expect(screen.getByText('Beranda')).toBeInTheDocument();
    expect(screen.getByText('Google Pixel 8 (USB)')).toBeInTheDocument();
    expect(screen.getByText('Age Encrypted')).toBeInTheDocument();

    // Content Canvas
    expect(screen.getByTestId('test-content')).toBeInTheDocument();

    // GlobalFooter
    expect(screen.getByText('SQLite SQLCipher')).toBeInTheDocument();
    expect(screen.getByText('v0.3.2 PRO')).toBeInTheDocument();
  });

  it('renders live background progress in GlobalFooter when active', () => {
    render(
      <MainLayout
        activeView="files"
        onViewChange={vi.fn()}
        searchQuery=""
        onSearchChange={vi.fn()}
        devices={[mockDevice]}
        selectedDevice={mockDevice}
        onSelectDevice={vi.fn()}
        onRefreshDevices={vi.fn()}
        activeTaskMsg="Mencadangkan DCIM/Camera/IMG_01.jpg"
        activeTaskProgress={65}
      >
        <div>Files Content</div>
      </MainLayout>
    );

    expect(screen.getByText('Mencadangkan DCIM/Camera/IMG_01.jpg')).toBeInTheDocument();
    expect(screen.getByText('65%')).toBeInTheDocument();
  });
});
