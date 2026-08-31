import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { BreadcrumbNav } from './BreadcrumbNav';

describe('BreadcrumbNav Component', () => {
  const sampleBreadcrumbs = [
    { name: 'Root', path: '/' },
    { name: 'sdcard', path: '/sdcard' },
    { name: 'DCIM', path: '/sdcard/DCIM' },
    { name: 'Camera', path: '/sdcard/DCIM/Camera' }
  ];

  it('renders all breadcrumb parts', () => {
    const onNavigate = vi.fn();
    render(
      <BreadcrumbNav
        currentPath="/sdcard/DCIM/Camera"
        breadcrumbs={sampleBreadcrumbs}
        onNavigate={onNavigate}
      />
    );

    expect(screen.getByText('Root')).toBeInTheDocument();
    expect(screen.getByText('sdcard')).toBeInTheDocument();
    expect(screen.getByText('DCIM')).toBeInTheDocument();
    expect(screen.getByText('Camera')).toBeInTheDocument();
  });

  it('calls onNavigate when a breadcrumb segment is clicked', () => {
    const onNavigate = vi.fn();
    render(
      <BreadcrumbNav
        currentPath="/sdcard/DCIM/Camera"
        breadcrumbs={sampleBreadcrumbs}
        onNavigate={onNavigate}
      />
    );

    fireEvent.click(screen.getByText('DCIM'));
    expect(onNavigate).toHaveBeenCalledWith('/sdcard/DCIM');
  });

  it('navigates to parent path when Back button is clicked', () => {
    const onNavigate = vi.fn();
    render(
      <BreadcrumbNav
        currentPath="/sdcard/DCIM/Camera"
        breadcrumbs={sampleBreadcrumbs}
        onNavigate={onNavigate}
      />
    );

    const backButton = screen.getByTitle('Naik ke folder induk (Parent Directory)');
    fireEvent.click(backButton);
    expect(onNavigate).toHaveBeenCalledWith('/sdcard/DCIM');
  });

  it('allows switching to manual path input mode (terminal icon)', () => {
    const onNavigate = vi.fn();
    render(
      <BreadcrumbNav
        currentPath="/sdcard/DCIM"
        breadcrumbs={sampleBreadcrumbs}
        onNavigate={onNavigate}
      />
    );

    const editButton = screen.getByTitle('Ketik atau tempel path langsung (Manual Path Input)');
    fireEvent.click(editButton);

    const input = screen.getByPlaceholderText('/storage/emulated/0/...');
    expect(input).toBeInTheDocument();

    fireEvent.change(input, { target: { value: '/sdcard/Download' } });
    fireEvent.click(screen.getByText('Buka'));

    expect(onNavigate).toHaveBeenCalledWith('/sdcard/Download');
  });
});
