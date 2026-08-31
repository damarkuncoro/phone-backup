import { cn } from "@/shared/lib/utils";

interface StatCardProps {
  title: string;
  value: string;
  icon: any;
  color: string;
  subtitle?: string;
  onClick?: () => void;
}

export function StatCard({
  title,
  value,
  icon: Icon,
  color,
  subtitle,
  onClick
}: StatCardProps) {
  return (
    <div
      onClick={onClick}
      className={cn(
        "bg-white p-6 rounded-[32px] border border-slate-100 shadow-sm flex items-center gap-5 transition-all select-none",
        onClick ? "cursor-pointer hover:shadow-lg hover:border-indigo-100 hover:scale-[1.01] active:scale-95 group" : "hover:shadow-md"
      )}
    >
      <div className={`${color} w-12 h-12 rounded-2xl flex items-center justify-center text-white shadow-lg shrink-0 transition-transform group-hover:scale-105`}>
        <Icon className="w-6 h-6" />
      </div>
      <div className="min-w-0">
        <p className="text-[10px] font-black text-slate-400 uppercase tracking-widest truncate">{title}</p>
        <p className="text-2xl font-black text-slate-900 tracking-tighter truncate mt-0.5">{value}</p>
        {subtitle && (
          <p className="text-[10px] font-bold text-indigo-500 mt-0.5 opacity-0 group-hover:opacity-100 transition-opacity truncate">
            {subtitle} &rarr;
          </p>
        )}
      </div>
    </div>
  );
}
