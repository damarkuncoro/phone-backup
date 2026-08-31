import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import { FileBrowser } from './FileBrowser';

// Mock the hooks and services
vi.mock('@/features/devices/hooks/useDevices', () => ({
  useDevices: () => ({
    devices: [
      { id: 'dev1', model: 'Pixel 6', manufacturer: 'Google', os_version: '13', connection_type: 'Usb', storage_used_bytes: 0, storage_total_bytes: 100 }
    ],
    loading: false,
    error: null
  })
}));

vi.mock('@/services/deviceService', () => ({
  deviceService: {
    browse: vi.fn().mockResolvedValue([]),
    downloadFile: vi.fn().mockResolvedValue(undefined),
    deleteFile: vi.fn().mockResolvedValue(undefined),
    renameFile: vi.fn().mockResolvedValue(undefined),
    uploadFile: vi.fn().mockResolvedValue(undefined),
    calculateHash: vi.fn().mockResolvedValue('hash123')
  },
  getDeviceId: (d: any) => d.id
}));

describe('FileBrowser Component', () => {
  it('renders header correctly with search and quick actions', () => {
    render(<FileBrowser />);
    expect(screen.getByText('File Manager')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Cari file di folder ini...')).toBeInTheDocument();
    expect(screen.getByText('Unggah ke HP')).toBeInTheDocument();
  });

  it('shows device selector with correct devices', () => {
    render(<FileBrowser />);
    const selector = screen.getByRole('combobox');
    expect(selector).toBeInTheDocument();
    expect(screen.getByText('Pixel 6')).toBeInTheDocument();
  });
});
