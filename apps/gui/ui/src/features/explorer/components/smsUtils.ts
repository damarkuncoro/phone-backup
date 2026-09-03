export interface SmsMessage {
  id?: string;
  address: string;
  body: string;
  date: string | number;
  type_code?: number; // 1: inbox/received, 2: sent, 3: draft, etc.
}

export interface ConversationThread {
  address: string;
  messages: SmsMessage[];
  lastMessage: SmsMessage;
  totalCount: number;
}

export const AVATAR_COLORS = [
  'bg-indigo-500 text-white',
  'bg-emerald-500 text-white',
  'bg-sky-500 text-white',
  'bg-rose-500 text-white',
  'bg-amber-500 text-white',
  'bg-purple-500 text-white',
  'bg-teal-500 text-white',
  'bg-pink-500 text-white',
];

export function getAvatarColor(address: string): string {
  let hash = 0;
  for (let i = 0; i < address.length; i++) {
    hash = address.charCodeAt(i) + ((hash << 5) - hash);
  }
  return AVATAR_COLORS[Math.abs(hash) % AVATAR_COLORS.length];
}

export function getInitials(address: string): string {
  if (!address) return '?';
  const clean = address.replace(/[^a-zA-Z0-9]/g, '');
  if (clean.length <= 2) return clean.toUpperCase();
  return clean.substring(0, 2).toUpperCase();
}

export function formatMessageTime(dateVal: string | number): string {
  if (!dateVal) return '';
  try {
    const d = new Date(typeof dateVal === 'number' ? dateVal : dateVal);
    if (isNaN(d.getTime())) return String(dateVal);
    return (
      d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }) +
      ' • ' +
      d.toLocaleDateString([], { day: 'numeric', month: 'short', year: 'numeric' })
    );
  } catch {
    return String(dateVal);
  }
}
