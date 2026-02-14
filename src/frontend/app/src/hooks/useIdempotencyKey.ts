// UX-EDIT-03: idempotency key generation MUST work without crypto.randomUUID.
// Falls back to Math.random-based generation.

import { useCallback, useRef } from 'react';

function generateKey(): string {
  // Prefer crypto.randomUUID when available
  if (
    typeof crypto !== 'undefined' &&
    typeof crypto.randomUUID === 'function'
  ) {
    return crypto.randomUUID();
  }
  // Fallback: timestamp + random hex
  const hex = (n: number) =>
    Math.floor(n).toString(16).padStart(8, '0');
  const t = Date.now();
  const r1 = Math.random() * 0xffffffff;
  const r2 = Math.random() * 0xffffffff;
  return `${hex(t)}-${hex(r1)}-${hex(r2)}`;
}

/** Returns stable key for a given note+version pair. */
export function useIdempotencyKey() {
  const cache = useRef(new Map<string, string>());

  const getKey = useCallback((noteId: string, baseVersion: number) => {
    const cacheKey = `${noteId}:${baseVersion}`;
    let key = cache.current.get(cacheKey);
    if (!key) {
      key = generateKey();
      cache.current.set(cacheKey, key);
    }
    return key;
  }, []);

  const clearKey = useCallback((noteId: string, baseVersion: number) => {
    cache.current.delete(`${noteId}:${baseVersion}`);
  }, []);

  return { getKey, clearKey };
}
