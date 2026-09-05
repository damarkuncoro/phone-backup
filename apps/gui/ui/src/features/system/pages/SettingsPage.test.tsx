import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { SettingsPage } from './SettingsPage';

const { mockDoctorReport, mockSettings, mockKeys } = vi.hoisted(() => ({
  mockDoctorReport: {
    adb_found: true,
    adb_version: 'Android Debug Bridge version 1.0.41',
    device_count: 2,
    db_healthy: true
  },
  mockSettings: {
    storage_backend: { Local: null },
    encryption_public_key: 'age1ql3z7hjy54pw3hyww5ayyfg7zqgvc7w3j2elw8z5w62'
  },
  mockKeys: [
    'AGE-SECRET-KEY-1XYZ...',
    'age1ql3z7hjy54pw3hyww5ayyfg7zqgvc7w3j2elw8z5w62'
  ]
}));

vi.mock('@/services/systemService', () => ({
  systemService: {
    getDoctorReport: vi.fn().mockResolvedValue(mockDoctorReport),
    getSettings: vi.fn().mockResolvedValue(mockSettings),
    saveSettings: vi.fn().mockResolvedValue(undefined),
    generateKeys: vi.fn().mockResolvedValue(mockKeys),
    runGC: vi.fn().mockResolvedValue(15),
    pruneFailed: vi.fn().mockResolvedValue(3),
    openRestoreFolder: vi.fn().mockResolvedValue(undefined),
    openDownloadsFolder: vi.fn().mockResolvedValue(undefined)
  }
}));

describe('SettingsPage Component', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders system settings header and tabs', async () => {
    render(<SettingsPage />);
    
    await waitFor(() => {
      expect(screen.getByText('Pengaturan Sistem')).toBeInTheDocument();
    });

    expect(screen.getByRole('button', { name: /System Doctor/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Penyimpanan/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Keamanan/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Pemeliharaan/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Tentang/i })).toBeInTheDocument();
  });

  it('displays system doctor status details', async () => {
    render(<SettingsPage />);
    
    await waitFor(() => {
      expect(screen.getByText('Android Debug Bridge version 1.0.41')).toBeInTheDocument();
    });

    expect(screen.getByText('SQLite Database')).toBeInTheDocument();
    expect(screen.getByText('2 Perangkat Terhubung')).toBeInTheDocument();
  });

  it('switches to Storage tab and shows folder buttons', async () => {
    render(<SettingsPage />);
    
    await waitFor(() => {
      expect(screen.getByText('Pengaturan Sistem')).toBeInTheDocument();
    });

    const storageTab = screen.getByRole('button', { name: /Penyimpanan/i });
    fireEvent.click(storageTab);

    expect(screen.getByText('Storage Engine Backend')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Buka Folder Restore/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Buka Folder Downloads|Buka Folder Unduhan/i })).toBeInTheDocument();
  });

  it('switches to Maintenance tab and triggers garbage collection', async () => {
    render(<SettingsPage />);
    
    await waitFor(() => {
      expect(screen.getByText('Pengaturan Sistem')).toBeInTheDocument();
    });

    const maintTab = screen.getByRole('button', { name: /Pemeliharaan/i });
    fireEvent.click(maintTab);

    const gcBtn = screen.getByRole('button', { name: /Jalankan Garbage Collection/i });
    fireEvent.click(gcBtn);

    await waitFor(() => {
      expect(screen.getByText(/15 objek/i)).toBeInTheDocument();
    });
  });

  it('switches to Security tab and displays Public Key', async () => {
    render(<SettingsPage />);
    
    await waitFor(() => {
      expect(screen.getByText('Pengaturan Sistem')).toBeInTheDocument();
    });

    const secTab = screen.getByRole('button', { name: /Keamanan/i });
    fireEvent.click(secTab);

    expect(screen.getByText(/age1ql3z7hjy54pw3hyww5ayyfg7zqgvc7w3j2elw8z5w62/i)).toBeInTheDocument();
  });
});
