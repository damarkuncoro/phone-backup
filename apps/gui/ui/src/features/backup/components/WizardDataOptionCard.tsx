import { Check } from 'lucide-react';
import { cn } from "@/shared/lib/utils";
import type { DataOption } from '../lib/wizardDataOptions';

interface WizardDataOptionCardProps {
  option: DataOption;
  isSelected: boolean;
  isDisabled: boolean;
  onToggle: (id: string) => void;
}

export function WizardDataOptionCard({
  option,
  isSelected,
  isDisabled,
  onToggle
}: WizardDataOptionCardProps) {
  const Icon = option.icon;

  return (
    <div
      onClick={() => !isDisabled && onToggle(option.id)}
      className={cn(
        "p-5 rounded-[28px] border-2 transition-all flex flex-col justify-between space-y-3 relative overflow-hidden select-none",
        isDisabled
          ? "opacity-40 bg-slate-50 border-slate-200/50 cursor-not-allowed"
          : isSelected
          ? "border-indigo-500 bg-indigo-50/40 shadow-md ring-2 ring-indigo-500/10 cursor-pointer"
          : "border-slate-100 hover:border-indigo-200 bg-white cursor-pointer"
      )}
    >
      <div className="flex items-center justify-between">
        <div className={cn(
          "w-11 h-11 rounded-2xl flex items-center justify-center shadow-inner",
          isSelected ? "bg-indigo-600 text-white" : "bg-slate-50 text-slate-400"
        )}>
          <Icon className="w-5 h-5" />
        </div>
        <span className={cn(
          "text-[9px] font-black px-2 py-0.5 rounded uppercase tracking-wider",
          isSelected ? "bg-indigo-100 text-indigo-700" : "bg-slate-100 text-slate-400"
        )}>
          {option.detail}
        </span>
      </div>

      <div>
        <div className="flex items-center justify-between">
          <h4 className="font-black text-slate-900 text-sm">{option.label}</h4>
          {isSelected && <Check className="w-4 h-4 text-indigo-600 stroke-[3]" />}
        </div>
        <p className="text-[11px] text-slate-500 font-medium mt-1 leading-relaxed">{option.description}</p>
      </div>
    </div>
  );
}
