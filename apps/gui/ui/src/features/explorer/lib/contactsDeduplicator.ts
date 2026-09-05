import { type ContactData, getContactPhones, getContactEmails } from '../components/contactsUtils';

export interface DuplicateGroup {
  primaryId: string;
  reason: 'phone' | 'email' | 'name_exact' | 'name_fuzzy';
  matchValue: string;
  contacts: ContactData[];
}

export function normalizePhoneForMatching(raw: string): string {
  if (!raw) return '';
  const digits = raw.replace(/\D/g, '');
  if (!digits) return '';
  // Normalize Indonesian (+62 / 08 / 628) or generic international numbers
  if (digits.startsWith('620')) return '62' + digits.slice(3);
  if (digits.startsWith('0')) return '62' + digits.slice(1);
  return digits;
}

export function levenshteinDistance(s1: string, s2: string): number {
  const v1 = Array.from(s1);
  const v2 = Array.from(s2);
  let prev = Array.from({ length: v2.length + 1 }, (_, i) => i);
  let curr = new Array(v2.length + 1).fill(0);

  for (let i = 0; i < v1.length; i++) {
    curr[0] = i + 1;
    for (let j = 0; j < v2.length; j++) {
      const cost = v1[i] === v2[j] ? 0 : 1;
      curr[j + 1] = Math.min(curr[j] + 1, prev[j + 1] + 1, prev[j] + cost);
    }
    prev = [...curr];
  }
  return prev[v2.length];
}

export function nameSimilarity(a: string, b: string): number {
  const s1 = a.trim().toLowerCase();
  const s2 = b.trim().toLowerCase();
  if (s1 === s2) return 1.0;
  const maxLen = Math.max(s1.length, s2.length);
  if (maxLen === 0) return 1.0;
  const dist = levenshteinDistance(s1, s2);
  return 1.0 - dist / maxLen;
}

export function detectDuplicateGroups(contacts: ContactData[]): DuplicateGroup[] {
  const groups: DuplicateGroup[] = [];
  const assigned = new Set<number>();

  for (let i = 0; i < contacts.length; i++) {
    if (assigned.has(i)) continue;
    const a = contacts[i];
    const groupMembers: ContactData[] = [a];
    let groupReason: DuplicateGroup['reason'] = 'name_exact';
    let groupMatchVal = a.display_name;

    const phonesA = getContactPhones(a).map(p => normalizePhoneForMatching(p.number)).filter(Boolean);
    const emailsA = getContactEmails(a).map(e => e.email.trim().toLowerCase()).filter(Boolean);
    const nameA = (a.display_name || '').trim().toLowerCase();

    for (let j = i + 1; j < contacts.length; j++) {
      if (assigned.has(j)) continue;
      const b = contacts[j];
      const phonesB = getContactPhones(b).map(p => normalizePhoneForMatching(p.number)).filter(Boolean);
      const emailsB = getContactEmails(b).map(e => e.email.trim().toLowerCase()).filter(Boolean);
      const nameB = (b.display_name || '').trim().toLowerCase();

      // 1. Phone match
      const matchingPhone = phonesA.find(pa => phonesB.includes(pa));
      if (matchingPhone) {
        groupMembers.push(b);
        assigned.add(j);
        groupReason = 'phone';
        groupMatchVal = matchingPhone;
        continue;
      }

      // 2. Email match
      const matchingEmail = emailsA.find(ea => emailsB.includes(ea));
      if (matchingEmail) {
        groupMembers.push(b);
        assigned.add(j);
        groupReason = 'email';
        groupMatchVal = matchingEmail;
        continue;
      }

      // 3. Exact Name match
      if (nameA && nameA === nameB) {
        groupMembers.push(b);
        assigned.add(j);
        groupReason = 'name_exact';
        groupMatchVal = a.display_name;
        continue;
      }

      // 4. High fuzzy name similarity
      if (nameA && nameB && nameSimilarity(nameA, nameB) >= 0.88) {
        groupMembers.push(b);
        assigned.add(j);
        groupReason = 'name_fuzzy';
        groupMatchVal = `${a.display_name} ≈ ${b.display_name}`;
        continue;
      }
    }

    if (groupMembers.length > 1) {
      assigned.add(i);
      groups.push({
        primaryId: a.id || `${a.display_name}-${i}`,
        reason: groupReason,
        matchValue: groupMatchVal,
        contacts: groupMembers
      });
    }
  }

  return groups;
}

export function mergeContactList(contacts: ContactData[]): ContactData {
  if (contacts.length === 0) throw new Error('Cannot merge empty contacts');
  if (contacts.length === 1) return contacts[0];

  // Pick longest / most informative name
  let bestName = contacts[0].display_name || '';
  for (const c of contacts) {
    if ((c.display_name || '').length > bestName.length) {
      bestName = c.display_name;
    }
  }

  // Merge unique phone numbers
  const phoneMap = new Map<string, string | any>();
  for (const c of contacts) {
    const phones = getContactPhones(c);
    for (const p of phones) {
      const norm = normalizePhoneForMatching(p.number);
      if (norm && !phoneMap.has(norm)) {
        phoneMap.set(norm, p.number);
      }
    }
  }

  // Merge unique emails
  const emailSet = new Set<string>();
  for (const c of contacts) {
    const emails = getContactEmails(c);
    for (const e of emails) {
      if (e.email.trim()) emailSet.add(e.email.trim());
    }
  }

  // Pick first organization & note
  const org = contacts.find(c => c.organization || (c.organizations && c.organizations.length > 0))?.organization;
  const note = contacts.map(c => c.notes).filter(Boolean).join(' | ');

  return {
    id: contacts[0].id || `${bestName}-merged`,
    display_name: bestName,
    phones: Array.from(phoneMap.values()).map(num => ({ raw_value: num })),
    emails: Array.from(emailSet).map(email => ({ value: email })),
    organization: org,
    notes: note || undefined
  };
}

export function autoMergeAllDuplicates(contacts: ContactData[]): { merged: ContactData[]; duplicateCount: number } {
  const groups = detectDuplicateGroups(contacts);
  if (groups.length === 0) {
    return { merged: [...contacts], duplicateCount: 0 };
  }

  const processedIds = new Set<string>();
  const result: ContactData[] = [];
  let duplicateCount = 0;

  for (const group of groups) {
    const mergedContact = mergeContactList(group.contacts);
    result.push(mergedContact);
    group.contacts.forEach((c, idx) => {
      processedIds.add(c.id || `${c.display_name}-${idx}`);
    });
    duplicateCount += group.contacts.length - 1;
  }

  // Add non-duplicate contacts
  contacts.forEach((c, idx) => {
    const cid = c.id || `${c.display_name}-${idx}`;
    if (!processedIds.has(cid)) {
      result.push(c);
    }
  });

  return { merged: result, duplicateCount };
}
