export interface CallLogItem {
  id?: string | number;
  number: string;
  name?: string | null;
  date: string | number;
  duration: number;
  call_type?: 'incoming' | 'outgoing' | 'missed' | 'voicemail' | 'rejected' | 'blocked' | string;
  type?: 'incoming' | 'outgoing' | 'missed' | string;
  geocoded_location?: string | null;
}

export function formatCallDuration(seconds: number): string {
  if (!seconds || seconds <= 0) return '0 dtk';
  const mins = Math.floor(seconds / 60);
  const secs = seconds % 60;
  if (mins === 0) return `${secs} dtk`;
  if (mins < 60) return `${mins} m ${secs} dtk`;
  const hrs = Math.floor(mins / 60);
  const remMins = mins % 60;
  return `${hrs} j ${remMins} m`;
}

export function formatCallDate(val: string | number): string {
  if (!val) return '-';
  const num = typeof val === 'string' ? parseInt(val, 10) : val;
  const date = !isNaN(num) && num > 10000000000 ? new Date(num) : new Date(val);
  if (isNaN(date.getTime())) return String(val);
  return date.toLocaleDateString('id-ID', {
    day: 'numeric',
    month: 'short',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  });
}

export function getCallType(call: CallLogItem): 'incoming' | 'outgoing' | 'missed' {
  const t = (call.call_type || call.type || '').toLowerCase();
  if (t.includes('out') || t === '2') return 'outgoing';
  if (t.includes('miss') || t.includes('reject') || t === '3') return 'missed';
  return 'incoming';
}
