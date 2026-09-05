import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { BackupWizard } from './BackupWizard';

const { mockDevice, mockFiles } = vi.hoisted(() => ({
  mockDevice: {
    id: 'dev1',
    model: 'Google Pixel 8 Pro',
    manufacturer: 'Google',
    serial: 'PIXEL8PRO123',
    os_version: '15',
    connection_type: 'Usb',
    storage_used_bytes: 45000000000,
    storage_total_bytes: 128000000000,
    storage_free_bytes: 83000000000,
  },
  mockFiles: [
    {
      id: 'f1',
      name: 'IMG_2026.jpg',
      path: '/storage/emulated/0/DCIM/Camera/IMG_2026.jpg',
      size_bytes: 3500000,
      modified_at: '2026-08-30T10:00:00Z',
      is_dir: false,
      mime_type: 'image/jpeg'
    },
    {
      id: 'f2',
      name: 'Doc.pdf',
      path: '/storage/emulated/0/Documents/Doc.pdf',
      size_bytes: 1200000,
      modified_at: '2026-08-30T11:00:00Z',
      is_dir: false,
      mime_type: 'application/pdf'
    }
  ]
}));

vi.mock('@/features/devices/hooks/useDevices', () => ({
  useDevices: () => ({
    devices: [mockDevice],
    loading: false,
    error: null,
    refreshDevices: vi.fn()
  })
}));

vi.mock('@/services/deviceService', () => ({
  deviceService: {
    getAll: vi.fn().mockResolvedValue([mockDevice]),
    scan: vi.fn().mockResolvedValue(mockFiles),
    scanDetailed: vi.fn().mockResolvedValue({
      files: mockFiles,
      warnings: [],
      categories: {
        photos: { file_count: 1, total_bytes: 3500000 },
        documents: { file_count: 1, total_bytes: 1200000 }
      },
      metrics: {
        duration_ms: 15,
        directories_scanned: 2,
        files_scanned: 2,
        throughput_files_per_sec: 133.3
      }
    }),
    getBattery: vi.fn().mockResolvedValue([90, 31]),
    getLiveData: vi.fn().mockResolvedValue([
      { id: '1', display_name: 'Budi Santoso', phones: [{ number: '+628123456789' }] }
    ])
  },
  getDeviceId: (d: any) => (typeof d.id === 'string' ? d.id : d.id?.[0] || 'dev1')
}));

vi.mock('@/services/backupService', () => ({
  backupService: {
    startBackup: vi.fn().mockResolvedValue({ snapshot_id: 'snap-123' })
  }
}));

describe('BackupWizard Component', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders Step 1 (Device Selection) with available device', () => {
    render(<BackupWizard />);
    expect(screen.getByText('Backup Wizard')).toBeInTheDocument();
    expect(screen.getByText('Pilih Perangkat Sumber')).toBeInTheDocument();
    expect(screen.getByText('Google Pixel 8 Pro')).toBeInTheDocument();
  });

  it('allows navigating to Step 2 (Select Data) after selecting device', async () => {
    render(<BackupWizard />);
    const devCard = screen.getByText('Google Pixel 8 Pro').closest('div');
    if (devCard) fireEvent.click(devCard);

    const nextBtn = screen.getByRole('button', { name: /Lanjutkan/i });
    fireEvent.click(nextBtn);

    await waitFor(() => {
      expect(screen.getByText('Apa yang ingin Anda cadangkan?')).toBeInTheDocument();
    });

    expect(screen.getByText('Seluruh Memori Internal')).toBeInTheDocument();
    expect(screen.getByText('Kontak & Telepon')).toBeInTheDocument();
    expect(screen.getByText('Pesan SMS')).toBeInTheDocument();
    expect(screen.getByText('Galeri & Media')).toBeInTheDocument();
  });

  it('allows navigating to Step 3 (Review) and displays files summary', async () => {
    render(<BackupWizard />);
    
    // Step 1 -> Select device
    const devCard = screen.getByText('Google Pixel 8 Pro').closest('div');
    if (devCard) fireEvent.click(devCard);
    fireEvent.click(screen.getByRole('button', { name: /Lanjutkan/i }));

    // Step 2 -> Click Review
    await waitFor(() => {
      expect(screen.getByText('Apa yang ingin Anda cadangkan?')).toBeInTheDocument();
    });
    const reviewBtn = screen.getByRole('button', { name: /Review Rencana/i });
    fireEvent.click(reviewBtn);

    // Step 3 -> Verify Review page
    await waitFor(() => {
      expect(screen.getByText('Eksplorasi Rencana Backup')).toBeInTheDocument();
    });
    expect(screen.getByRole('button', { name: /Konfirmasi & Mulai Backup/i })).toBeInTheDocument();
  });

  it('allows previewing and selecting specific contacts in Review step', async () => {
    render(<BackupWizard />);
    
    // Select device and navigate to step 2
    const devCard = screen.getByText('Google Pixel 8 Pro').closest('div');
    if (devCard) fireEvent.click(devCard);
    fireEvent.click(screen.getByRole('button', { name: /Lanjutkan/i }));

    // Step 2 -> Navigate to Step 3
    await waitFor(() => {
      expect(screen.getByText('Apa yang ingin Anda cadangkan?')).toBeInTheDocument();
    });
    fireEvent.click(screen.getByRole('button', { name: /Review Rencana/i }));

    // Switch to Kontak HP tab
    await waitFor(() => {
      expect(screen.getByText(/Kontak HP/i)).toBeInTheDocument();
    });
    fireEvent.click(screen.getByText(/Kontak HP/i));

    // Verify contact card is rendered
    await waitFor(() => {
      expect(screen.getByText('Budi Santoso')).toBeInTheDocument();
      expect(screen.getByText('+628123456789')).toBeInTheDocument();
    });

    // Toggle contact or batch actions
    expect(screen.getByText(/1 dari 1 Kontak Dipilih/i)).toBeInTheDocument();
    const batchBtn = screen.getByRole('button', { name: /Batal Semua/i });
    fireEvent.click(batchBtn);

    await waitFor(() => {
      expect(screen.getByText(/0 dari 1 Kontak Dipilih/i)).toBeInTheDocument();
    });
  });
});
