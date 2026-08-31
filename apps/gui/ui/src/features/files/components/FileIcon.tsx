import { Folder, File, Image as ImageIcon, Video, Music, FileText, Smartphone } from 'lucide-react';
import { isImage, isVideo, isAudio, isDocument, isApk } from '../lib/fileUtils';
import { cn } from "@/shared/lib/utils";

interface FileIconProps {
  fileName: string;
  isDir: boolean;
  className?: string;
}

export function FileIcon({ fileName, isDir, className }: FileIconProps) {
  if (isDir) {
    return (
      <div className={cn("w-10 h-10 rounded-2xl bg-indigo-50 text-indigo-600 flex items-center justify-center shadow-sm shrink-0", className)}>
        <Folder className="w-5 h-5 fill-current opacity-80" />
      </div>
    );
  }
  if (isImage(fileName)) {
    return (
      <div className={cn("w-10 h-10 rounded-2xl bg-amber-50 text-amber-600 flex items-center justify-center shadow-sm shrink-0", className)}>
        <ImageIcon className="w-5 h-5" />
      </div>
    );
  }
  if (isVideo(fileName)) {
    return (
      <div className={cn("w-10 h-10 rounded-2xl bg-rose-50 text-rose-600 flex items-center justify-center shadow-sm shrink-0", className)}>
        <Video className="w-5 h-5" />
      </div>
    );
  }
  if (isAudio(fileName)) {
    return (
      <div className={cn("w-10 h-10 rounded-2xl bg-purple-50 text-purple-600 flex items-center justify-center shadow-sm shrink-0", className)}>
        <Music className="w-5 h-5" />
      </div>
    );
  }
  if (isDocument(fileName)) {
    return (
      <div className={cn("w-10 h-10 rounded-2xl bg-blue-50 text-blue-600 flex items-center justify-center shadow-sm shrink-0", className)}>
        <FileText className="w-5 h-5" />
      </div>
    );
  }
  if (isApk(fileName)) {
    return (
      <div className={cn("w-10 h-10 rounded-2xl bg-emerald-50 text-emerald-600 flex items-center justify-center shadow-sm shrink-0", className)}>
        <Smartphone className="w-5 h-5" />
      </div>
    );
  }
  return (
    <div className={cn("w-10 h-10 rounded-2xl bg-slate-50 text-slate-400 flex items-center justify-center shadow-sm shrink-0", className)}>
      <File className="w-5 h-5" />
    </div>
  );
}
