import {
  Sparkles, HardDrive, Image as ImageIcon, MessageSquare, FolderCheck, Users, PhoneCall, Smartphone
} from 'lucide-react';

export interface DataOption {
  id: string;
  label: string;
  icon: any;
  description: string;
  detail: string;
  requiresAdb?: boolean;
}

export const DATA_OPTIONS: DataOption[] = [
  {
    id: 'full_storage',
    label: 'Seluruh Memori Internal',
    icon: HardDrive,
    description: 'Semua folder & file di memori ponsel (Termasuk WhatsApp, Musik, Rekaman, & Folder Kustom).',
    detail: 'Rekomendasi Total',
    requiresAdb: false
  },
  {
    id: 'photos',
    label: 'Galeri & Media',
    icon: ImageIcon,
    description: 'Foto kamera (DCIM), Gambar (Pictures), dan Video rekaman.',
    detail: 'Volume Tinggi',
    requiresAdb: false
  },
  {
    id: 'chat_media',
    label: 'Media WhatsApp & Chat',
    icon: MessageSquare,
    description: 'Foto, video, voice note, dan dokumen dari percakapan WhatsApp & Telegram.',
    detail: 'Media Sosial',
    requiresAdb: false
  },
  {
    id: 'files',
    label: 'Dokumen & Unduhan',
    icon: FolderCheck,
    description: 'Folder Download, Dokumen, PDF, Arsip Zip, dan file umum.',
    detail: 'File Explorer',
    requiresAdb: false
  },
  {
    id: 'audio',
    label: 'Musik & Rekaman Suara',
    icon: Sparkles,
    description: 'Folder Music, Recordings, VoiceRecorder, Ringtones, dan Podcast.',
    detail: 'Audio & Suara',
    requiresAdb: false
  },
  {
    id: 'contacts',
    label: 'Kontak & Telepon',
    icon: Users,
    description: 'Nama, nomor telepon, email, dan vCard kontak tersimpan.',
    detail: 'E2E Encrypted',
    requiresAdb: true
  },
  {
    id: 'sms',
    label: 'Pesan SMS',
    icon: MessageSquare,
    description: 'Riwayat percakapan SMS masuk & keluar, dan pesan teks.',
    detail: 'Secure Vault',
    requiresAdb: true
  },
  {
    id: 'call_logs',
    label: 'Riwayat Panggilan',
    icon: PhoneCall,
    description: 'Catatan panggilan masuk, keluar, dan panggilan tak terjawab.',
    detail: 'Log Aktivitas',
    requiresAdb: true
  },
  {
    id: 'apps',
    label: 'Daftar Aplikasi',
    icon: Smartphone,
    description: 'Daftar paket aplikasi Android terinstal dan versi APK.',
    detail: 'Metadata Inventory',
    requiresAdb: true
  },
];
