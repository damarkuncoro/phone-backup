import type { FileEntry } from '@/services/deviceService';

export const SAMPLE_ANALYSIS_FOLDERS = [
  '/storage/emulated/0/DCIM/Camera',
  '/storage/emulated/0/Pictures/Screenshots',
  '/storage/emulated/0/Download',
  '/storage/emulated/0/Documents',
  '/storage/emulated/0/Movies',
  '/storage/emulated/0/WhatsApp/Media'
];

export function resolveDataFilterPaths(selectedData: string[]): string[] {
  if (selectedData.includes('full_storage')) return [];
  const paths: string[] = [];
  if (selectedData.includes('photos')) {
    paths.push('/storage/emulated/0/DCIM', '/storage/emulated/0/Pictures', '/storage/emulated/0/Movies', '/DCIM', '/Pictures', '/Movies');
  }
  if (selectedData.includes('chat_media')) {
    paths.push('/storage/emulated/0/Android/media/com.whatsapp', '/storage/emulated/0/WhatsApp', '/storage/emulated/0/Telegram', '/WhatsApp', '/Telegram');
  }
  if (selectedData.includes('files')) {
    paths.push('/storage/emulated/0/Download', '/storage/emulated/0/Documents', '/Download', '/Documents');
  }
  if (selectedData.includes('audio')) {
    paths.push('/storage/emulated/0/Music', '/storage/emulated/0/Recordings', '/storage/emulated/0/VoiceRecorder', '/storage/emulated/0/Podcasts', '/Music', '/Recordings');
  }
  return paths;
}

export function filterScannedFiles(files: FileEntry[], selectedData: string[], paths: string[]): FileEntry[] {
  return (files || []).filter(f => {
    if (!f || !f.path) return false;
    if (selectedData.includes('full_storage') || paths.length === 0) return true;
    return paths.some(p => f.path.startsWith(p));
  });
}
