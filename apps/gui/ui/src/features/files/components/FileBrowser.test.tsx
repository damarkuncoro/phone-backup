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
    browse: vi.fn().mockResolvedValue([])
  },
  getDeviceId: (d: any) => d.id
}));

describe('FileBrowser Component', () => {
  it('renders header correctly', () => {
    render(<FileBrowser />);
    expect(screen.getByText('File Explorer')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Search in current folder...')).toBeInTheDocument();
  });

  it('shows device selector with correct devices', () => {
    render(<FileBrowser />);
    const selector = screen.getByRole('combobox');
    expect(selector).toBeInTheDocument();
    expect(screen.getByText('Pixel 6')).toBeInTheDocument();
  });
});
