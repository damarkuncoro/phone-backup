import { Search, FileText, Folder, Calendar, HardDrive, ArrowRight, Loader2, Database } from 'lucide-react';
import { type FileEntry } from '@/services/deviceService';
import { formatBytes, formatUnixTimestamp } from '@/shared/lib/formatters';
import { cn } from '@/shared/lib/utils';

interface SearchPageProps {
  query: string;
  onQueryChange: (q: string) => void;
  results: FileEntry[];
  isSearching: boolean;
  onOpenFile: (file: FileEntry) => void;
}

export function SearchPage({ query, onQueryChange, results, isSearching, onOpenFile }: SearchPageProps) {
  return (
    <div className="p-8 space-y-8 animate-in fade-in duration-500 max-w-5xl mx-auto">
      <header className="space-y-4">
        <div className="flex items-center gap-4">
            <div className="w-12 h-12 bg-indigo-600 rounded-2xl flex items-center justify-center text-white shadow-lg shadow-indigo-200">
                <Search className="w-6 h-6" />
            </div>
            <div>
                <h1 className="text-3xl font-black text-slate-900 tracking-tight">Pencarian Global</h1>
                <p className="text-slate-500 font-medium text-sm text-balance">Mencari di seluruh arsip backup dan perangkat aktif.</p>
            </div>
        </div>

        <div className="relative group">
            <Search className="absolute left-6 top-5 w-6 h-6 text-slate-400 group-focus-within:text-indigo-600 transition-colors" />
            <input
                autoFocus
                type="text"
                value={query}
                onChange={(e) => onQueryChange(e.target.value)}
                placeholder="Ketik nama file, ekstensi, atau path..."
                className="w-full bg-white border-2 border-slate-100 p-5 pl-16 rounded-[32px] text-xl font-bold shadow-xl shadow-slate-100/50 outline-none focus:border-indigo-500/30 transition-all"
            />
            {isSearching && (
                <div className="absolute right-6 top-5">
                    <Loader2 className="w-6 h-6 text-indigo-500 animate-spin" />
                </div>
            )}
        </div>
      </header>

      <section className="space-y-4">
          <div className="flex items-center justify-between px-2">
              <h2 className="text-[10px] font-black text-slate-400 uppercase tracking-widest">
                  {results.length > 0 ? `${results.length} Hasil Ditemukan` : 'Hasil Pencarian'}
              </h2>
          </div>

          <div className="grid grid-cols-1 gap-3">
              {results.map((file, i) => (
                  <div
                    key={i}
                    onClick={() => onOpenFile(file)}
                    className="group bg-white p-5 rounded-3xl border border-slate-100 hover:border-indigo-200 hover:shadow-lg transition-all flex items-center gap-6 cursor-pointer"
                  >
                      <div className={cn(
                          "w-12 h-12 rounded-2xl flex items-center justify-center shrink-0 transition-transform group-hover:scale-110",
                          file.is_dir ? "bg-indigo-50 text-indigo-600" : "bg-slate-50 text-slate-400"
                      )}>
                          {file.is_dir ? <Folder className="w-6 h-6" /> : <FileText className="w-6 h-6" />}
                      </div>

                      <div className="flex-1 min-w-0">
                          <p className="font-black text-slate-800 text-lg leading-tight truncate">{file.name}</p>
                          <p className="text-[10px] font-mono text-slate-400 truncate mt-1 uppercase tracking-tighter">{file.path}</p>
                      </div>

                      <div className="hidden md:flex flex-col items-end gap-1 shrink-0">
                          <div className="flex items-center gap-2 text-[10px] font-bold text-slate-500">
                              <Calendar className="w-3 h-3" /> {formatUnixTimestamp(Number(file.modified_at) || 0)}
                          </div>
                          <div className="flex items-center gap-2 text-[10px] font-black text-indigo-500 uppercase tracking-widest">
                              <HardDrive className="w-3 h-3" /> {formatBytes(file.size_bytes)}
                          </div>
                      </div>

                      <div className="opacity-0 group-hover:opacity-100 transition-all">
                          <div className="w-10 h-10 bg-indigo-600 rounded-xl flex items-center justify-center text-white shadow-lg">
                              <ArrowRight className="w-5 h-5" />
                          </div>
                      </div>
                  </div>
              ))}

              {results.length === 0 && !isSearching && query.length > 1 && (
                  <div className="py-20 flex flex-col items-center justify-center text-slate-300">
                      <Database className="w-16 h-16 mb-4 opacity-10" />
                      <p className="font-black uppercase tracking-widest text-[10px]">Tidak ada yang cocok</p>
                      <p className="text-xs mt-1 text-slate-400">Coba gunakan kata kunci yang lebih umum.</p>
                  </div>
              )}

              {query.length <= 1 && !isSearching && (
                   <div className="py-32 flex flex-col items-center justify-center text-slate-300 text-center space-y-4">
                        <Search className="w-16 h-16 opacity-5" />
                        <p className="font-medium text-slate-400 max-w-xs mx-auto">Mulai mengetik untuk mencari file di seluruh snapshot backup Anda.</p>
                   </div>
              )}
          </div>
      </section>
    </div>
  );
}
