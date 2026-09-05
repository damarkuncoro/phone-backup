export interface ContactPhoneItem {
  raw_value?: string;
  normalized_value?: string;
  phone_type?: string;
  type?: string;
  label?: string;
  is_primary?: boolean;
}

export interface ContactEmailItem {
  value?: string;
  email_type?: string;
  type?: string;
  label?: string;
  is_primary?: boolean;
}

export interface ContactOrgItem {
  company_name?: string;
  department?: string;
  title?: string;
  job_description?: string;
  org_type?: string;
  label?: string;
}

export interface ContactData {
  id?: string;
  display_name: string;
  phones?: (string | ContactPhoneItem)[];
  phone_numbers?: (string | ContactPhoneItem)[];
  emails?: (string | ContactEmailItem)[];
  organizations?: (string | ContactOrgItem)[];
  organization?: string;
  notes?: string;
}

export const AVATAR_COLORS = [
  'bg-indigo-500 text-white',
  'bg-rose-500 text-white',
  'bg-emerald-500 text-white',
  'bg-amber-500 text-white',
  'bg-purple-500 text-white',
  'bg-sky-500 text-white',
  'bg-pink-500 text-white',
  'bg-teal-500 text-white',
];

export function getAvatarColor(name?: string): string {
  if (!name || typeof name !== 'string') return AVATAR_COLORS[0];
  let hash = 0;
  for (let i = 0; i < name.length; i++) {
    hash = name.charCodeAt(i) + ((hash << 5) - hash);
  }
  return AVATAR_COLORS[Math.abs(hash) % AVATAR_COLORS.length];
}

export function getInitials(name?: string): string {
  if (!name || typeof name !== 'string' || !name.trim()) return '?';
  const parts = name.trim().split(/\s+/);
  if (parts.length >= 2 && parts[0][0] && parts[1][0]) {
    return (parts[0][0] + parts[1][0]).toUpperCase();
  }
  return name.trim().substring(0, 2).toUpperCase();
}

export function getContactPhones(contact: ContactData): { number: string; type?: string; isPrimary?: boolean }[] {
  if (!contact) return [];
  const list = contact.phones || contact.phone_numbers || [];
  if (!Array.isArray(list)) return [];
  
  const results: { number: string; type?: string; isPrimary?: boolean }[] = [];
  for (const item of list) {
    if (typeof item === 'string' && item.trim()) {
      results.push({ number: item.trim() });
    } else if (item && typeof item === 'object') {
      const num = (item as any).number || item.raw_value || item.normalized_value;
      if (num && String(num).trim()) {
        results.push({
          number: String(num).trim(),
          type: item.phone_type || item.type || item.label,
          isPrimary: item.is_primary === true
        });
      }
    }
  }
  return results;
}

export function getContactEmails(contact: ContactData): { email: string; type?: string }[] {
  if (!contact) return [];
  const list = contact.emails || [];
  if (!Array.isArray(list)) return [];
  
  const results: { email: string; type?: string }[] = [];
  for (const item of list) {
    if (typeof item === 'string' && item.trim()) {
      results.push({ email: item.trim() });
    } else if (item && typeof item === 'object') {
      const val = item.value;
      if (val && String(val).trim()) {
        results.push({
          email: String(val).trim(),
          type: item.email_type || item.type || item.label
        });
      }
    }
  }
  return results;
}

export function getContactOrg(contact: ContactData): string | undefined {
  if (!contact) return undefined;
  if (typeof contact.organization === 'string' && contact.organization.trim()) {
    return contact.organization.trim();
  }
  if (Array.isArray(contact.organizations) && contact.organizations.length > 0) {
    const org = contact.organizations[0];
    if (typeof org === 'string' && org.trim()) return org.trim();
    if (org && typeof org === 'object') {
      const parts = [org.company_name, org.title, org.department].filter(Boolean);
      if (parts.length > 0) return parts.join(' • ');
    }
  }
  return undefined;
}
