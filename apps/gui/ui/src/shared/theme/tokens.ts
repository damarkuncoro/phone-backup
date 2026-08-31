/**
 * Phone Backup UI Design Tokens System
 * Unified standard for containers, cards, spacing, typography, and interactive components.
 */

export const UI_TOKENS = {
  // Page Containers & Layouts
  layout: {
    pageContainer: "p-6 md:p-8 space-y-8 max-w-7xl mx-auto animate-in fade-in duration-300",
    pageContainerNarrow: "p-6 md:p-8 space-y-8 max-w-5xl mx-auto animate-in fade-in duration-300",
    fullHeightContainer: "h-full flex flex-col bg-white overflow-hidden",
    modalOverlay: "fixed inset-0 z-50 bg-slate-900/60 backdrop-blur-sm flex items-center justify-center p-4 animate-in fade-in duration-200",
  },

  // Surfaces & Cards
  card: {
    primary: "bg-white p-6 md:p-8 rounded-[32px] border border-slate-100 shadow-sm",
    secondary: "bg-slate-50 p-5 rounded-[28px] border border-slate-200/80",
    headerBanner: "flex flex-col sm:flex-row sm:items-center justify-between gap-4 bg-white p-6 md:p-8 rounded-[32px] border border-slate-100 shadow-sm",
    heroBannerDark: "flex flex-col sm:flex-row sm:items-center justify-between gap-4 bg-gradient-to-r from-slate-900 via-slate-900 to-indigo-950 text-white p-6 md:p-8 rounded-[32px] shadow-xl relative overflow-hidden",
    interactive: "bg-white p-6 rounded-[32px] border border-slate-100 shadow-sm hover:shadow-md hover:border-indigo-100 transition-all flex items-start gap-4 cursor-pointer select-none active:scale-95",
  },

  // Typography Tokens
  text: {
    titlePage: "text-2xl md:text-3xl font-black text-slate-900 tracking-tight",
    titleSection: "text-lg md:text-xl font-black text-slate-900 tracking-tight",
    titleCard: "text-sm md:text-base font-black text-slate-900 tracking-tight",
    subtitle: "text-xs text-slate-500 font-medium",
    badgePill: "text-[10px] font-black uppercase tracking-wider px-2.5 py-0.5 rounded-full",
    monoCode: "font-mono text-xs font-bold",
    labelUpper: "text-[10px] font-black uppercase tracking-widest text-slate-400",
  },

  // Buttons & CTAs
  button: {
    primary: "px-6 py-3 bg-indigo-600 hover:bg-indigo-700 text-white rounded-2xl text-xs font-black uppercase tracking-wider shadow-lg shadow-indigo-200 hover:shadow-indigo-300 transition-all flex items-center justify-center gap-2 active:scale-95 disabled:opacity-50",
    secondary: "px-5 py-3 bg-slate-50 hover:bg-slate-100 border border-slate-200 text-slate-700 hover:text-indigo-600 rounded-2xl text-xs font-black uppercase tracking-wider transition-all flex items-center justify-center gap-2 active:scale-95 shadow-sm disabled:opacity-50",
    danger: "px-5 py-3 bg-rose-600 hover:bg-rose-700 text-white rounded-2xl text-xs font-black uppercase tracking-wider shadow-md shadow-rose-200 transition-all flex items-center justify-center gap-2 active:scale-95 disabled:opacity-50",
    icon: "p-3 bg-white border border-slate-200 rounded-2xl text-slate-500 hover:text-indigo-600 hover:border-indigo-200 transition-all shadow-sm active:scale-95",
  },

  // Form Controls & Inputs
  input: {
    search: "w-full bg-slate-50 border border-slate-200/80 pl-10 pr-4 py-2.5 rounded-2xl text-xs font-medium focus:ring-4 focus:ring-indigo-500/10 focus:border-indigo-300 outline-none transition-all",
    text: "w-full bg-white border border-slate-200 px-4 py-2.5 rounded-2xl text-xs font-medium focus:ring-4 focus:ring-indigo-500/10 focus:border-indigo-300 outline-none transition-all",
  },

  // Empty State Container
  emptyState: "col-span-full py-20 flex flex-col items-center justify-center bg-white rounded-[32px] border-2 border-dashed border-slate-200 text-slate-400 p-8 space-y-3",
} as const;
