import { Search, FileText, Folder, Calendar, HardDrive, ArrowRight, Loader2, Database } from 'lucide-react';
import { type FileEntry } from '@/services/deviceService';
import { formatBytes, formatUnixTimestamp } from '@/shared/lib/formatters';
import { cn } from '@/shared/lib/utils';
import { UI_TOKENS } from '@/shared/theme/tokens';

interface SearchPageProps {
  query: string;
  onQueryChange: (q: string) => void;
  results: FileEntry[];
  isSearching: boolean;
  onOpenFile: (file: FileEntry) => void;
}

export function SearchPage({ query, onQueryChange, results, isSearching, onOpenFile }: SearchPageProps) {
  return (
    <div className={UI_TOKENS.layout.pageContainerNarrow}>
      
      {/* Header Banner */}
      <header className={UI_TOKENS.card.headerBanner}>
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 bg-indigo-600 rounded-2xl flex items-center justify-center text-white shadow-lg shadow-indigo-200 shrink-0">
            <Search className="w-6 h-6" />
          </div>
          <div>
            <h1 className={UI_TOKENS.text.titlePage}>Pencarian Global</h1>
            <p className={UI_TOKENS.text.subtitle}>
              Mencari berkas di seluruh arsip snapshot cadangan dan penyimpanan perangkat aktif.
            </p>
          </div>
        </div>
      </header>

      {/* Main Search Input Bar */}
      <div className="relative group">
        <Search className="absolute left-5 top-4 w-5 h-5 text-slate-400 group-focus-within:text-indigo-600 transition-colors" />
        <input
          autoFocus
          type="text"
          value={query}
          onChange={(e) => onQueryChange(e.target.value)}
          placeholder="Ketik nama berkas, ekstensi, atau path folder..."
          className="w-full bg-white border border-slate-200/80 p-4 pl-14 pr-12 rounded-[28px] text-sm md:text-base font-bold shadow-md shadow-slate-100/50 outline-none focus:ring-4 focus:ring-indigo-500/10 focus:border-indigo-300 transition-all"
        />
        {isSearching && (
          <div className="absolute right-5 top-4">
            <Loader2 className="w-5 h-5 text-indigo-500 animate-spin" />
          </div>
        )}
      </div>

      {/* Results Section */}
      <section className="space-y-4">
        <div className="flex items-center justify-between px-1">
          <h2 className={UI_TOKENS.text.labelUpper}>
            {results.length > 0 ? `${results.length} Hasil Ditemukan` : 'Hasil Pencarian'}
          </h2>
        </div>

        <div className="grid grid-cols-1 gap-3">
          {results.map((file, i) => (
            <div
              key={i}
              onClick={() => onOpenFile(file)}
              className="group bg-white p-5 rounded-[28px] border border-slate-100 hover:border-indigo-200 hover:shadow-md transition-all flex items-center gap-5 cursor-pointer select-none"
            >
              <div className={cn(
                "w-11 h-11 rounded-2xl flex items-center justify-center shrink-0 transition-transform group-hover:scale-105",
                file.is_dir ? "bg-indigo-50 text-indigo-600" : "bg-slate-50 text-slate-500"
              )}>
                {file.is_dir ? <Folder className="w-5 h-5" /> : <FileText className="w-5 h-5" />}
              </div>

              <div className="flex-1 min-w-0">
                <p className="font-black text-slate-800 text-sm truncate group-hover:text-indigo-600 transition-colors">
                  {file.name}
                </p>
                <p className="text-[10px] font-mono text-slate-400 truncate mt-0.5 select-all">
                  {file.path}
                </p>
              </div>

              <div className="hidden md:flex flex-col items-end gap-1 shrink-0">
                <div className="flex items-center gap-1.5 text-[10px] font-medium text-slate-400">
                  <Calendar className="w-3 h-3 text-slate-400" />
                  <span>{formatUnixTimestamp(Number(file.modified_at) || 0)}</span>
                </div>
                <div className="flex items-center gap-1.5 text-[10px] font-black text-indigo-600 uppercase tracking-wider font-mono">
                  <HardDrive className="w-3 h-3 text-indigo-400" />
                  <span>{formatBytes(file.size_bytes)}</span>
                </div>
              </div>

              <div className="opacity-0 group-hover:opacity-100 transition-all shrink-0">
                <div className="w-8 h-8 bg-indigo-50 text-indigo-600 rounded-xl flex items-center justify-center group-hover:bg-indigo-600 group-hover:text-white transition-all">
                  <ArrowRight className="w-4 h-4" />
                </div>
              </div>
            </div>
          ))}

          {results.length === 0 && !isSearching && query.length > 1 && (
            <div className={UI_TOKENS.emptyState}>
              <Database className="w-12 h-12 opacity-20 text-slate-400" />
              <p className="font-black uppercase tracking-widest text-xs">Tidak Ada Hasil Yang Cocok</p>
              <p className="text-xs text-slate-400">Coba gunakan kata kunci pencarian yang lebih umum.</p>
            </div>
          )}

          {query.length <= 1 && !isSearching && (
            <div className="py-24 flex flex-col items-center justify-center text-slate-300 text-center space-y-3 bg-white rounded-[32px] border border-slate-100 shadow-sm p-8">
              <Search className="w-12 h-12 opacity-20 text-slate-400" />
              <p className="text-xs text-slate-500 font-medium max-w-sm mx-auto">
                Mulai mengetik untuk mencari berkas di seluruh snapshot cadangan dan memori ponsel Anda.
              </p>
            </div>
          )}
        </div>
      </section>

    </div>
  );
}
