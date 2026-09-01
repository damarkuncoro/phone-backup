import { LayoutDashboard, Tablet, History, RefreshCcw, Folder, Search, Settings } from "lucide-react";
import { cn } from "../../shared/lib/utils";

const navItems = [
  { id: 'dashboard', label: 'Dashboard', icon: LayoutDashboard },
  { id: 'devices', label: 'Devices', icon: Tablet },
  { id: 'backup', label: 'Backup', icon: History },
  { id: 'history', label: 'History', icon: RefreshCcw },
  { id: 'files', label: 'Files', icon: Folder },
  { id: 'settings', label: 'Settings', icon: Settings },
];

interface SidebarProps {
  activeView: string;
  onViewChange: (view: string) => void;
  searchQuery: string;
  onSearchChange: (q: string) => void;
}

export function Sidebar({ activeView, onViewChange, searchQuery, onSearchChange }: SidebarProps) {
  return (
    <div className="w-60 h-full bg-slate-900 text-slate-300 flex flex-col border-r border-slate-800">
      <div className="p-6 border-b border-white/10 flex items-center gap-3">
        <img src="/img/logo.png" alt="Phone Backup Logo" className="w-9 h-9 object-contain" />
        <span className="text-xl font-black tracking-tighter italic text-white">PB PRO</span>
      </div>

      <div className="flex-1 overflow-y-auto p-4 space-y-8 mt-4">
        <div className="px-2">
          <label className="text-[10px] font-black text-indigo-400 uppercase tracking-widest ml-2 mb-2 block">Quick Search</label>
          <div className="relative">
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => {
                  onSearchChange(e.target.value);
                  if (activeView !== 'search') onViewChange('search');
              }}
              placeholder="Search objects..."
              className="w-full bg-indigo-950/50 border border-white/10 text-white placeholder-indigo-300/50 px-4 py-2 rounded-xl text-sm outline-none focus:ring-2 focus:ring-indigo-500/50 transition-all"
            />
            <Search className="absolute right-3 top-2.5 w-4 h-4 text-indigo-300/50" />
          </div>
        </div>

        <nav className="space-y-1">
          {navItems.map((item) => (
            <button
              key={item.id}
              onClick={() => onViewChange(item.id)}
              className={cn(
                "w-full flex items-center gap-3 px-4 py-2.5 text-sm font-bold rounded-xl transition-all",
                activeView === item.id
                  ? "bg-indigo-600 text-white shadow-lg shadow-indigo-500/20"
                  : "hover:bg-white/5 text-slate-400 hover:text-white"
              )}
            >
              <item.icon className={cn("w-5 h-5", activeView === item.id ? "opacity-100" : "opacity-50")} />
              {item.label}
            </button>
          ))}
        </nav>
      </div>

      <div className="p-6 border-t border-white/5 bg-slate-950">
        <div className="flex items-center gap-3 text-xs font-black text-white/40 uppercase tracking-widest">
          <div className="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></div>
          Engine Online
        </div>
      </div>
    </div>
  );
}
