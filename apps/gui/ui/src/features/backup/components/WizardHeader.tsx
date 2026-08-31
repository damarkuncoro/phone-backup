import { Database, Check } from 'lucide-react';
import { cn } from "@/shared/lib/utils";
import { UI_TOKENS } from '@/shared/theme/tokens';
import type { Step } from '../hooks/useBackupWizard';
import type { Device } from '@/services/deviceService';

interface WizardHeaderProps {
  step: Step;
  setStep: (step: Step) => void;
  selectedDevice: Device | null;
  selectedDataCount: number;
  progressPercent: number;
}

export function WizardHeader({
  step,
  setStep,
  selectedDevice,
  selectedDataCount,
  progressPercent
}: WizardHeaderProps) {
  return (
    <header className={UI_TOKENS.card.headerBanner}>
      <div className="flex items-center gap-4">
        <div className="w-12 h-12 rounded-2xl bg-indigo-600 text-white flex items-center justify-center shadow-lg shadow-indigo-200 shrink-0">
          <Database className="w-6 h-6" />
        </div>
        <div>
          <h1 className={UI_TOKENS.text.titlePage}>
            Backup Wizard
          </h1>
          <p className={UI_TOKENS.text.subtitle}>
            Panduan langkah demi langkah untuk mencadangkan data ponsel Anda secara aman dan terenkripsi.
          </p>
        </div>
      </div>

      {/* Stepper Navigation */}
      <div className="flex items-center gap-2 sm:gap-3 bg-slate-50 p-2 rounded-2xl border border-slate-200/60 overflow-x-auto no-scrollbar shrink-0">
        <StepBadge
          step={1}
          title="Perangkat"
          active={step === 'select-device'}
          completed={!!selectedDevice && step !== 'select-device'}
          onClick={() => setStep('select-device')}
        />
        <div className="w-4 h-0.5 bg-slate-200 shrink-0" />
        <StepBadge
          step={2}
          title="Modul Data"
          active={step === 'select-data'}
          completed={step === 'configure' || step === 'progress'}
          onClick={() => selectedDevice && setStep('select-data')}
        />
        <div className="w-4 h-0.5 bg-slate-200 shrink-0" />
        <StepBadge
          step={3}
          title="Pratinjau"
          active={step === 'configure'}
          completed={step === 'progress'}
          onClick={() => selectedDevice && selectedDataCount > 0 && setStep('configure')}
        />
        <div className="w-4 h-0.5 bg-slate-200 shrink-0" />
        <StepBadge
          step={4}
          title="Proses"
          active={step === 'progress'}
          completed={progressPercent === 100}
        />
      </div>
    </header>
  );
}

function StepBadge({
  step, title, active, completed, onClick
}: {
  step: number;
  title: string;
  active: boolean;
  completed: boolean;
  onClick?: () => void;
}) {
  return (
    <div
      onClick={completed || active ? onClick : undefined}
      className={cn(
        "flex items-center gap-2 transition-all select-none",
        (completed || active) && onClick ? "cursor-pointer" : "cursor-default"
      )}
    >
      <div className={cn(
        "w-7 h-7 rounded-full flex items-center justify-center font-black text-xs transition-all",
        active
          ? "bg-indigo-600 text-white shadow-md shadow-indigo-200 ring-2 ring-indigo-500/20 scale-105"
          : completed
          ? "bg-emerald-500 text-white shadow-sm"
          : "bg-slate-200 text-slate-500"
      )}>
        {completed ? <Check className="w-3.5 h-3.5 stroke-[3]" /> : step}
      </div>
      <span className={cn(
        "text-[11px] font-black uppercase tracking-wider hidden md:inline",
        active ? "text-indigo-600" : completed ? "text-slate-800" : "text-slate-400"
      )}>
        {title}
      </span>
    </div>
  );
}
