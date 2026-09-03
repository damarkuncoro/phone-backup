export interface AppData {
  id?: any;
  app_name?: string;
  name?: string;
  package_name: string;
  version_name?: string;
  version_code?: number;
  installer?: string | null;
  is_system?: boolean;
}

export const APP_AVATAR_COLORS = [
  'from-indigo-500 to-purple-600 text-white',
  'from-emerald-500 to-teal-600 text-white',
  'from-sky-500 to-blue-600 text-white',
  'from-amber-500 to-orange-600 text-white',
  'from-rose-500 to-pink-600 text-white',
  'from-violet-500 to-purple-700 text-white',
  'from-teal-500 to-cyan-700 text-white',
];

export function getAppColor(name: string): string {
  let hash = 0;
  for (let i = 0; i < name.length; i++) {
    hash = name.charCodeAt(i) + ((hash << 5) - hash);
  }
  return APP_AVATAR_COLORS[Math.abs(hash) % APP_AVATAR_COLORS.length];
}

export function getAppDisplayName(app: AppData): string {
  if (app.app_name && app.app_name.trim()) return app.app_name.trim();
  if (app.name && app.name.trim()) return app.name.trim();
  
  const parts = (app.package_name || '').split('.');
  const lastPart = parts[parts.length - 1] || app.package_name;
  if (!lastPart) return 'Aplikasi';
  return lastPart.charAt(0).toUpperCase() + lastPart.slice(1);
}

export function isSystemPackage(pkg: string): boolean {
  if (!pkg) return false;
  const p = pkg.toLowerCase();
  return (
    p.startsWith('android.') ||
    p.startsWith('com.android.') ||
    p.startsWith('com.google.android.ext.') ||
    p.startsWith('com.google.android.overlay') ||
    p.startsWith('com.mediatek.') ||
    p.startsWith('com.qualcomm.') ||
    p.startsWith('com.sec.android.app.launcher') ||
    p.includes('overlay') ||
    p.includes('systemui')
  );
}

export function getInstallerLabel(installer?: string | null): { label: string; isPlayStore: boolean } {
  if (!installer) return { label: 'Sideload / APK', isPlayStore: false };
  const inst = installer.toLowerCase();
  if (inst.includes('vending') || inst.includes('google')) {
    return { label: 'Google Play Store', isPlayStore: true };
  }
  if (inst.includes('samsungapps')) {
    return { label: 'Samsung Galaxy Store', isPlayStore: false };
  }
  if (inst.includes('packageinstaller')) {
    return { label: 'Package Installer', isPlayStore: false };
  }
  return { label: installer, isPlayStore: false };
}
