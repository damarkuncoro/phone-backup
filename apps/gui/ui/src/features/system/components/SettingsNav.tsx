import { Activity, Database, Shield, HardDrive, Info } from 'lucide-react';
import { cn } from "@/shared/lib/utils";

export type SettingsTab = 'doctor' | 'storage' | 'security' | 'maintenance' | 'about';

interface SettingsNavProps {
  activeTab: SettingsTab;
  onTabChange: (tab: SettingsTab) => void;
  doctorHealthy: boolean;
  currentBackendType: string;
}

export function SettingsNav({
  activeTab,
  onTabChange,
  doctorHealthy,
  currentBackendType
}: SettingsNavProps) {
  return (
    <nav className="flex flex-wrap gap-2 p-1.5 bg-slate-100/80 rounded-2xl border border-slate-200/60 select-none">
      <TabButton
        active={activeTab === 'doctor'}
        onClick={() => onTabChange('doctor')}
        icon={Activity}
        label="System Doctor"
        badge={doctorHealthy ? "Sehat" : "Peringatan"}
        badgeColor={doctorHealthy ? "bg-emerald-100 text-emerald-700" : "bg-amber-100 text-amber-700"}
      />
      <TabButton
        active={activeTab === 'storage'}
        onClick={() => onTabChange('storage')}
        icon={Database}
        label="Penyimpanan"
        badge={currentBackendType}
        badgeColor="bg-indigo-100 text-indigo-700"
      />
      <TabButton
        active={activeTab === 'security'}
        onClick={() => onTabChange('security')}
        icon={Shield}
        label="Keamanan & Kunci"
      />
      <TabButton
        active={activeTab === 'maintenance'}
        onClick={() => onTabChange('maintenance')}
        icon={HardDrive}
        label="Pemeliharaan"
      />
      <TabButton
        active={activeTab === 'about'}
        onClick={() => onTabChange('about')}
        icon={Info}
        label="Tentang"
      />
    </nav>
  );
}

function TabButton({
  active, onClick, icon: Icon, label, badge, badgeColor
}: {
  active: boolean;
  onClick: () => void;
  icon: any;
  label: string;
  badge?: string;
  badgeColor?: string;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      className={cn(
        "flex items-center gap-2 px-4 py-2.5 rounded-xl font-black text-xs transition-all active:scale-95",
        active
          ? "bg-white text-slate-900 shadow-sm ring-1 border border-slate-200/60"
          : "text-slate-500 hover:text-slate-800 hover:bg-white/50"
      )}
    >
      <Icon className={cn("w-4 h-4", active ? "text-indigo-600" : "text-slate-400")} />
      <span>{label}</span>
      {badge && (
        <span className={cn("text-[9px] font-black px-2 py-0.5 rounded-md", badgeColor || "bg-slate-100 text-slate-600")}>
          {badge}
        </span>
      )}
    </button>
  );
}
