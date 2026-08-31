import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { AddDeviceModal } from './AddDeviceModal';

describe('AddDeviceModal Component', () => {
  it('renders modal when isOpen is true with all 3 connection tabs', () => {
    render(<AddDeviceModal isOpen={true} onClose={vi.fn()} onDeviceConnected={vi.fn()} />);
    expect(screen.getByText('Tambah Perangkat Baru')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Kabel Biasa \(MTP\)/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /USB Debugging/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Wireless ADB/i })).toBeInTheDocument();
  });

  it('does not render when isOpen is false', () => {
    render(<AddDeviceModal isOpen={false} onClose={vi.fn()} onDeviceConnected={vi.fn()} />);
    expect(screen.queryByText('Tambah Perangkat Baru')).not.toBeInTheDocument();
  });

  it('defaults to MTP tab and switches between tabs correctly', () => {
    render(<AddDeviceModal isOpen={true} onClose={vi.fn()} onDeviceConnected={vi.fn()} />);
    
    // Default MTP tab
    expect(screen.getByText(/Rekomendasi Utama untuk Pengguna Awam/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Pindai Ulang Sambungan MTP/i })).toBeInTheDocument();
    
    // Switch to USB Debugging tab
    const usbTabBtn = screen.getByRole('button', { name: /USB Debugging/i });
    fireEvent.click(usbTabBtn);
    expect(screen.getByText(/Langkah-Langkah Mengaktifkan USB Debugging/i)).toBeInTheDocument();
    
    // Switch to Wireless tab
    const wirelessTabBtn = screen.getByRole('button', { name: /Wireless ADB/i });
    fireEvent.click(wirelessTabBtn);
    expect(screen.getByPlaceholderText('192.168.1.100')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Sambungkan Nirkabel/i })).toBeInTheDocument();
  });

  it('calls onClose when close button is clicked', () => {
    const handleClose = vi.fn();
    render(<AddDeviceModal isOpen={true} onClose={handleClose} onDeviceConnected={vi.fn()} />);
    
    const closeBtn = screen.getByTestId('close-add-device-modal');
    fireEvent.click(closeBtn);
    expect(handleClose).toHaveBeenCalledTimes(1);
  });
});
