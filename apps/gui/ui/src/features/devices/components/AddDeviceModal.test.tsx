import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { AddDeviceModal } from './AddDeviceModal';

describe('AddDeviceModal Component', () => {
  it('renders modal when isOpen is true', () => {
    render(<AddDeviceModal isOpen={true} onClose={vi.fn()} onDeviceConnected={vi.fn()} />);
    expect(screen.getByText('Tambah Perangkat Baru')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Kabel USB/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Wireless ADB/i })).toBeInTheDocument();
  });

  it('does not render when isOpen is false', () => {
    render(<AddDeviceModal isOpen={false} onClose={vi.fn()} onDeviceConnected={vi.fn()} />);
    expect(screen.queryByText('Tambah Perangkat Baru')).not.toBeInTheDocument();
  });

  it('switches between USB and Wireless tabs', () => {
    render(<AddDeviceModal isOpen={true} onClose={vi.fn()} onDeviceConnected={vi.fn()} />);
    
    // Default USB tab
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
