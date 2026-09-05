import {
  File, Folder, Image as ImageIcon, Video, Music, FileText, Smartphone
} from 'lucide-react';
import type { FileEntry } from '@/services/deviceService';
import { isImage, isVideo, isAudio, isDocument, isApk } from '../lib/fileUtils';

export function FileInspectorPreview({ file }: { file: FileEntry }) {
  const renderIcon = () => {
    if (file.is_dir) {
      return (
        <div className="w-16 h-16 rounded-3xl bg-indigo-50 text-indigo-600 flex items-center justify-center shadow-inner">
          <Folder className="w-8 h-8 fill-current opacity-80" />
        </div>
      );
    }
    if (isImage(file.name)) {
      return (
        <div className="w-16 h-16 rounded-3xl bg-amber-50 text-amber-600 flex items-center justify-center shadow-inner">
          <ImageIcon className="w-8 h-8" />
        </div>
      );
    }
    if (isVideo(file.name)) {
      return (
        <div className="w-16 h-16 rounded-3xl bg-rose-50 text-rose-600 flex items-center justify-center shadow-inner">
          <Video className="w-8 h-8" />
        </div>
      );
    }
    if (isAudio(file.name)) {
      return (
        <div className="w-16 h-16 rounded-3xl bg-purple-50 text-purple-600 flex items-center justify-center shadow-inner">
          <Music className="w-8 h-8" />
        </div>
      );
    }
    if (isDocument(file.name)) {
      return (
        <div className="w-16 h-16 rounded-3xl bg-blue-50 text-blue-600 flex items-center justify-center shadow-inner">
          <FileText className="w-8 h-8" />
        </div>
      );
    }
    if (isApk(file.name)) {
      return (
        <div className="w-16 h-16 rounded-3xl bg-emerald-50 text-emerald-600 flex items-center justify-center shadow-inner">
          <Smartphone className="w-8 h-8" />
        </div>
      );
    }
    return (
      <div className="w-16 h-16 rounded-3xl bg-slate-100 text-slate-500 flex items-center justify-center shadow-inner">
        <File className="w-8 h-8" />
      </div>
    );
  };

  return (
    <div className="flex flex-col items-center text-center p-5 bg-slate-50 rounded-3xl border border-slate-100">
      <div className="mb-4">{renderIcon()}</div>
      <h4 className="font-black text-slate-900 text-sm break-all leading-snug" title={file.name}>
        {file.name}
      </h4>
      <span className="mt-2 px-3 py-1 bg-white border border-slate-200/60 rounded-full text-[10px] font-black uppercase tracking-wider text-slate-500 shadow-sm">
        {file.is_dir ? 'Direktori Folder' : file.name.split('.').pop()?.toUpperCase() + ' File'}
      </span>
    </div>
  );
}
