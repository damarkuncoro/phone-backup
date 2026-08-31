import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { Dashboard } from './Dashboard';

const { mockRefresh, mockDevice } = vi.hoisted(() => ({
  mockRefresh: vi.fn(),
  mockDevice: {
    id: 'dev1',
    model: 'Samsung Galaxy S22',
    manufacturer: 'Samsung',
    serial: 'R5CW12345',
    os_version: '14',
    connection_type: 'Usb',
    storage_used_bytes: 40000000000,
    storage_total_bytes: 128000000000,
    storage_free_bytes: 88000000000,
  }
}));

vi.mock('../hooks/useDevices', () => ({
  useDevices: () => ({
    devices: [mockDevice],
    loading: false,
    error: null,
    refreshDevices: mockRefresh
  })
}));

vi.mock('@/services/deviceService', () => ({
  deviceService: {
    getAll: vi.fn().mockResolvedValue([mockDevice]),
    scan: vi.fn().mockResolvedValue([]),
    getBattery: vi.fn().mockResolvedValue([85, 32]),
    connectWireless: vi.fn().mockResolvedValue('connected')
  },
  getDeviceId: (d: any) => (typeof d.id === 'string' ? d.id : d.id?.[0] || 'dev1')
}));

vi.mock('@/services/backupService', () => ({
  backupService: {
    getStorageStats: vi.fn().mockResolvedValue({
      total_logical_bytes: 50000000000,
      total_deduped_bytes: 25000000000,
      total_snapshots: 5
    })
  }
}));

describe('Dashboard Component', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders stats, header, and devices correctly', async () => {
    render(<Dashboard />);
    expect(screen.getByText('Dashboard')).toBeInTheDocument();
    expect(screen.getByText('Active Devices')).toBeInTheDocument();
    expect(screen.getByText('Total Backups')).toBeInTheDocument();
    expect(screen.getAllByText('Samsung Galaxy S22').length).toBeGreaterThan(0);
  });

  it('navigates to files when "Jelajahi Berkas" is clicked', () => {
    const handleNavigate = vi.fn();
    render(<Dashboard onNavigate={handleNavigate} />);
    const browseBtn = screen.getByRole('button', { name: /Jelajahi Berkas/i });
    fireEvent.click(browseBtn);
    expect(handleNavigate).toHaveBeenCalledWith('files');
  });

  it('invokes onBackupClick when "Backup Now" is clicked', () => {
    const handleBackup = vi.fn();
    render(<Dashboard onBackupClick={handleBackup} />);
    const backupBtn = screen.getByRole('button', { name: /Backup Now/i });
    fireEvent.click(backupBtn);
    expect(handleBackup).toHaveBeenCalledTimes(1);
  });

  it('invokes onBackupClick with specific device when "Quick Backup" is clicked', () => {
    const handleBackup = vi.fn();
    render(<Dashboard onBackupClick={handleBackup} />);
    const quickBackupBtns = screen.getAllByRole('button', { name: /Quick Backup/i });
    fireEvent.click(quickBackupBtns[0]);
    expect(handleBackup).toHaveBeenCalledWith(mockDevice);
  });

  it('navigates when clicking on StatCards', () => {
    const handleNavigate = vi.fn();
    render(<Dashboard onNavigate={handleNavigate} />);
    
    const activeDevCard = screen.getByText('Active Devices').closest('div');
    if (activeDevCard) fireEvent.click(activeDevCard);
    expect(handleNavigate).toHaveBeenCalledWith('files');

    const totalBackupsCard = screen.getByText('Total Backups').closest('div');
    if (totalBackupsCard) fireEvent.click(totalBackupsCard);
    expect(handleNavigate).toHaveBeenCalledWith('history');
  });
});
