import { useState, useRef, useEffect } from 'react';
import {
  Home, ChevronRight, ArrowLeft, Copy, Check,
  Terminal, CornerDownLeft, RefreshCw, Folder
} from 'lucide-react';
import { cn } from '@/shared/lib/utils';
import { getParentPath } from '../lib/pathUtils';

interface BreadcrumbNavProps {
  currentPath: string;
  breadcrumbs: { name: string; path: string }[];
  onNavigate: (path: string) => void;
  onRefresh?: () => void;
  isLoading?: boolean;
}

export function BreadcrumbNav({
  currentPath,
  breadcrumbs,
  onNavigate,
  onRefresh,
  isLoading
}: BreadcrumbNavProps) {
  const [isEditing, setIsEditing] = useState(false);
  const [inputPath, setInputPath] = useState(currentPath);
  const [copied, setCopied] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    setInputPath(currentPath);
  }, [currentPath]);

  useEffect(() => {
    if (isEditing && inputRef.current) {
      inputRef.current.focus();
      inputRef.current.select();
    }
  }, [isEditing]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    let target = inputPath.trim();
    if (!target.startsWith('/')) {
      target = '/' + target;
    }
    onNavigate(target);
    setIsEditing(false);
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(currentPath);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Escape') {
      setInputPath(currentPath);
      setIsEditing(false);
    }
  };

  return (
    <div className="bg-slate-50/80 border border-slate-200/70 rounded-2xl p-1.5 flex items-center gap-1.5 shadow-sm transition-all">
      {/* Up to Parent Button */}
      <button
        onClick={() => onNavigate(getParentPath(currentPath))}
        disabled={currentPath === '/'}
        title="Naik ke folder induk (Parent Directory)"
        className={cn(
          "p-2 rounded-xl text-slate-500 hover:text-indigo-600 hover:bg-white transition-all shadow-sm border border-transparent hover:border-slate-200 shrink-0",
          currentPath === '/' ? "opacity-30 cursor-not-allowed hover:bg-transparent hover:text-slate-500" : "active:scale-95"
        )}
      >
        <ArrowLeft className="w-4 h-4" />
      </button>

      <div className="h-4 w-px bg-slate-200 shrink-0" />

      {/* Interactive Breadcrumbs OR Direct Path Input Mode */}
      <div className="flex-1 min-w-0 flex items-center overflow-x-auto no-scrollbar">
        {isEditing ? (
          <form onSubmit={handleSubmit} className="flex-1 flex items-center gap-2">
            <div className="relative flex-1">
              <input
                ref={inputRef}
                type="text"
                value={inputPath}
                onChange={(e) => setInputPath(e.target.value)}
                onKeyDown={handleKeyDown}
                placeholder="/storage/emulated/0/..."
                className="w-full bg-white border border-indigo-300 text-slate-800 px-3 py-1.5 rounded-xl text-xs font-mono font-bold outline-none ring-4 ring-indigo-500/10 transition-all"
              />
            </div>
            <button
              type="submit"
              className="px-3 py-1.5 bg-indigo-600 text-white rounded-xl text-xs font-black uppercase tracking-wider flex items-center gap-1 hover:bg-indigo-700 transition-all shadow-sm shrink-0"
            >
              <CornerDownLeft className="w-3.5 h-3.5" /> Buka
            </button>
            <button
              type="button"
              onClick={() => { setInputPath(currentPath); setIsEditing(false); }}
              className="px-3 py-1.5 bg-white border border-slate-200 text-slate-500 rounded-xl text-xs font-bold hover:bg-slate-50 transition-all shrink-0"
            >
              Batal
            </button>
          </form>
        ) : (
          <div className="flex items-center gap-1 py-0.5">
            {breadcrumbs.map((bc, i) => {
              const isLast = i === breadcrumbs.length - 1;
              return (
                <div key={bc.path} className="flex items-center shrink-0">
                  {i > 0 && <ChevronRight className="w-3.5 h-3.5 text-slate-300 mx-0.5 shrink-0" />}
                  <button
                    onClick={() => onNavigate(bc.path)}
                    title={`Buka ${bc.name}`}
                    className={cn(
                      "flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-bold transition-all shrink-0",
                      isLast
                        ? "bg-indigo-600 text-white shadow-sm shadow-indigo-200 font-black"
                        : "text-slate-500 hover:text-slate-800 hover:bg-white/80 hover:shadow-sm"
                    )}
                  >
                    {i === 0 ? (
                      <>
                        <Home className="w-3.5 h-3.5 shrink-0" />
                        <span className="text-[11px] font-black uppercase tracking-wider">Root</span>
                      </>
                    ) : (
                      <>
                        <Folder className={cn("w-3.5 h-3.5 shrink-0", isLast ? "text-indigo-200" : "text-slate-400")} />
                        <span className="truncate max-w-[140px] sm:max-w-[200px]">{bc.name}</span>
                      </>
                    )}
                  </button>
                </div>
              );
            })}
          </div>
        )}
      </div>

      <div className="h-4 w-px bg-slate-200 shrink-0" />

      {/* Action Buttons: Direct Path Edit, Copy Path & Refresh */}
      <div className="flex items-center gap-1 shrink-0">
        <button
          onClick={() => setIsEditing(!isEditing)}
          title="Ketik atau tempel path langsung (Manual Path Input)"
          className={cn(
            "p-2 rounded-xl text-slate-400 hover:text-indigo-600 hover:bg-white transition-all shadow-sm border border-transparent hover:border-slate-200",
            isEditing && "bg-indigo-50 text-indigo-600 border-indigo-200"
          )}
        >
          <Terminal className="w-3.5 h-3.5" />
        </button>

        <button
          onClick={handleCopy}
          title="Salin path saat ini ke clipboard"
          className="p-2 rounded-xl text-slate-400 hover:text-indigo-600 hover:bg-white transition-all shadow-sm border border-transparent hover:border-slate-200"
        >
          {copied ? <Check className="w-3.5 h-3.5 text-emerald-600" /> : <Copy className="w-3.5 h-3.5" />}
        </button>

        {onRefresh && (
          <button
            onClick={onRefresh}
            title="Segarkan isi folder (Refresh Directory)"
            className="p-2 rounded-xl text-slate-400 hover:text-indigo-600 hover:bg-white transition-all shadow-sm border border-transparent hover:border-slate-200"
          >
            <RefreshCw className={cn("w-3.5 h-3.5", isLoading && "animate-spin text-indigo-600")} />
          </button>
        )}
      </div>
    </div>
  );
}
