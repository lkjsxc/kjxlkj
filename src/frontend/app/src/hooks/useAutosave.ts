// UX-EDIT-02: autosave with bounded debounce and visible status transitions.
// Status: 'idle' | 'saving' | 'saved' | 'conflict' | 'offline' | 'error'.

import { useCallback, useEffect, useRef, useState } from 'react';

export type SaveStatus =
  | 'idle'
  | 'saving'
  | 'saved'
  | 'conflict'
  | 'offline'
  | 'error';

const DEBOUNCE_MS = 800;

export interface UseAutosaveOptions {
  /** Called with current draft when save is triggered. */
  onSave: (draft: string) => Promise<SaveResult>;
  enabled: boolean;
}

export interface SaveResult {
  ok: boolean;
  conflict?: boolean;
}

export function useAutosave({ onSave, enabled }: UseAutosaveOptions) {
  const [status, setStatus] = useState<SaveStatus>('idle');
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const draftRef = useRef<string>('');
  const savingRef = useRef(false);

  const scheduleSave = useCallback(
    (draft: string) => {
      draftRef.current = draft;
      if (!enabled) return;
      if (timerRef.current) clearTimeout(timerRef.current);
      timerRef.current = setTimeout(async () => {
        if (savingRef.current) return;
        savingRef.current = true;
        setStatus('saving');
        try {
          const result = await onSave(draftRef.current);
          if (result.conflict) {
            setStatus('conflict');
          } else if (result.ok) {
            setStatus('saved');
          } else {
            setStatus('error');
          }
        } catch {
          if (!navigator.onLine) {
            setStatus('offline');
          } else {
            setStatus('error');
          }
        } finally {
          savingRef.current = false;
        }
      }, DEBOUNCE_MS);
    },
    [onSave, enabled],
  );

  /** Force immediate save (optional manual trigger). */
  const saveNow = useCallback(async () => {
    if (timerRef.current) clearTimeout(timerRef.current);
    if (savingRef.current) return;
    savingRef.current = true;
    setStatus('saving');
    try {
      const result = await onSave(draftRef.current);
      if (result.conflict) {
        setStatus('conflict');
      } else if (result.ok) {
        setStatus('saved');
      } else {
        setStatus('error');
      }
    } catch {
      setStatus(navigator.onLine ? 'error' : 'offline');
    } finally {
      savingRef.current = false;
    }
  }, [onSave]);

  useEffect(() => {
    return () => {
      if (timerRef.current) clearTimeout(timerRef.current);
    };
  }, []);

  return { status, scheduleSave, saveNow, setStatus };
}
