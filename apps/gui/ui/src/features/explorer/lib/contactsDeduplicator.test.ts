import { describe, it, expect } from 'vitest';
import {
  normalizePhoneForMatching,
  nameSimilarity,
  detectDuplicateGroups,
  mergeContactList,
  autoMergeAllDuplicates
} from './contactsDeduplicator';
import type { ContactData } from '../components/contactsUtils';

describe('Contacts Deduplicator & Merger', () => {
  it('normalizes international and local phone numbers', () => {
    expect(normalizePhoneForMatching('+62 812-3456-7890')).toBe('6281234567890');
    expect(normalizePhoneForMatching('081234567890')).toBe('6281234567890');
    expect(normalizePhoneForMatching('6281234567890')).toBe('6281234567890');
  });

  it('calculates name similarity with levenshtein metric', () => {
    expect(nameSimilarity('Budi Santoso', 'Budi Santoso')).toBe(1.0);
    expect(nameSimilarity('Budi Santoso', 'Budi Santoso ')).toBe(1.0);
    expect(nameSimilarity('Budi Santoso', 'Budi Santoso Kantor')).toBeGreaterThan(0.5);
  });

  it('detects duplicate contacts by phone or email', () => {
    const contacts: ContactData[] = [
      { id: '1', display_name: 'Budi Santoso', phones: [{ raw_value: '+628123456789' }] },
      { id: '2', display_name: 'Budi (Kantor)', phones: [{ raw_value: '08123456789' }] },
      { id: '3', display_name: 'Siti Rahma', phones: [{ raw_value: '08570000000' }] }
    ];

    const duplicates = detectDuplicateGroups(contacts);
    expect(duplicates.length).toBe(1);
    expect(duplicates[0].reason).toBe('phone');
    expect(duplicates[0].contacts.length).toBe(2);
  });

  it('merges multiple duplicate entries into one consolidated contact', () => {
    const group: ContactData[] = [
      { id: '1', display_name: 'Budi', phones: [{ raw_value: '08123456789' }], emails: [{ value: 'budi@gmail.com' }] },
      { id: '2', display_name: 'Budi Santoso', phones: [{ raw_value: '08129999999' }], organization: 'PT Maju Terus' }
    ];

    const merged = mergeContactList(group);
    expect(merged.display_name).toBe('Budi Santoso');
    expect(merged.phones?.length).toBe(2);
    expect(merged.emails?.length).toBe(1);
    expect(merged.organization).toBe('PT Maju Terus');
  });

  it('auto-merges entire contact book and returns merged list', () => {
    const contacts: ContactData[] = [
      { id: '1', display_name: 'Andi', phones: [{ raw_value: '+6281111111' }] },
      { id: '2', display_name: 'Andi Wijaya', phones: [{ raw_value: '081111111' }] },
      { id: '3', display_name: 'Budi', phones: [{ raw_value: '082222222' }] }
    ];

    const { merged, duplicateCount } = autoMergeAllDuplicates(contacts);
    expect(duplicateCount).toBe(1);
    expect(merged.length).toBe(2);
    expect(merged.some(c => c.display_name === 'Andi Wijaya')).toBe(true);
  });
});
