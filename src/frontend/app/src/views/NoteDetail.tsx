// UX-EDIT-01: separate synced snapshot and local draft state.
// UX-EDIT-02: autosave with debounce and status transitions.
// UX-EDIT-04: title rename propagates to list in same cycle.
// UX-EDIT-05: minimal default chrome — no inline save/version/delete.
// UX-EDIT-06: conflict state with explicit recovery actions.

import { useCallback, useEffect, useRef, useState } from 'react';
import { api, ApiError } from '../api';
import { useAutosave } from '../hooks/useAutosave';
import { useIdempotencyKey } from '../hooks/useIdempotencyKey';
import type { NoteProjection } from '../types';
import StatusBar from '../components/StatusBar';

interface Props {
  noteId: string;
  onTitleChange?: (noteId: string, newTitle: string) => void;
}

export default function NoteDetail({ noteId, onTitleChange }: Props) {
  // UX-EDIT-01: synced snapshot — last confirmed server state
  const [synced, setSynced] = useState<NoteProjection | null>(null);
  // UX-EDIT-01: local draft buffer
  const [draftTitle, setDraftTitle] = useState('');
  const [draftBody, setDraftBody] = useState('');
  const [loading, setLoading] = useState(true);
  const [conflictDraft, setConflictDraft] = useState<string | null>(null);
  const prevNoteId = useRef(noteId);
  const { getKey, clearKey } = useIdempotencyKey();

  // Load note data
  useEffect(() => {
    let cancelled = false;
    prevNoteId.current = noteId;
    setLoading(true);
    setConflictDraft(null);
    api
      .get<NoteProjection>(`/api/notes/${noteId}`)
      .then((note) => {
        if (cancelled) return;
        setSynced(note);
        setDraftTitle(note.title);
        setDraftBody(note.markdown);
        setLoading(false);
      })
      .catch(() => {
        if (!cancelled) setLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, [noteId]);

  // Save handler for autosave
  const handleSave = useCallback(
    async (draft: string) => {
      if (!synced) return { ok: false };
      const key = getKey(noteId, synced.version);
      try {
        const updated = await api.patch<NoteProjection>(
          `/api/notes/${noteId}`,
          {
            markdown: draft,
            base_version: synced.version,
            idempotency_key: key,
          },
        );
        clearKey(noteId, synced.version);
        setSynced(updated);
        return { ok: true };
      } catch (err) {
        if (err instanceof ApiError && err.status === 409) {
          return { ok: false, conflict: true };
        }
        return { ok: false };
      }
    },
    [synced, noteId, getKey, clearKey],
  );

  const { status, scheduleSave, setStatus } = useAutosave({
    onSave: handleSave,
    enabled: !!synced,
  });

  // Body change handler
  function handleBodyChange(value: string) {
    setDraftBody(value);
    scheduleSave(value);
  }

  // Title change handler — UX-EDIT-04: propagate in same cycle
  async function handleTitleBlur() {
    if (!synced || draftTitle === synced.title) return;
    try {
      await api.put(`/api/notes/${noteId}/title`, {
        title: draftTitle,
      });
      setSynced((s) => (s ? { ...s, title: draftTitle } : s));
      onTitleChange?.(noteId, draftTitle);
    } catch {
      // Revert on failure
      setDraftTitle(synced.title);
    }
  }

  // UX-EDIT-06: conflict recovery actions
  async function handleReloadLatest() {
    try {
      const note = await api.get<NoteProjection>(`/api/notes/${noteId}`);
      setConflictDraft(draftBody);
      setSynced(note);
      setDraftBody(note.markdown);
      setDraftTitle(note.title);
      setStatus('saved');
    } catch {
      /* keep conflict state */
    }
  }

  function handleCopyDraft() {
    void navigator.clipboard.writeText(draftBody);
  }

  if (loading) {
    return <div className="note-detail-loading">Loading…</div>;
  }

  if (!synced) {
    return <div className="note-detail-error">Note not found.</div>;
  }

  return (
    <div className="note-detail">
      <input
        className="editor-title"
        type="text"
        value={draftTitle}
        onChange={(e) => setDraftTitle(e.target.value)}
        onBlur={handleTitleBlur}
        aria-label="Note title"
      />

      {status === 'conflict' && (
        <div className="conflict-bar" role="alert">
          <span>Conflict detected.</span>
          <button type="button" onClick={handleReloadLatest}>
            Reload latest
          </button>
          <button type="button" onClick={handleCopyDraft}>
            Copy draft
          </button>
        </div>
      )}

      {conflictDraft && (
        <details className="conflict-draft-details">
          <summary>Your previous draft (before reload)</summary>
          <pre>{conflictDraft}</pre>
        </details>
      )}

      <textarea
        className="editor-content"
        value={draftBody}
        onChange={(e) => handleBodyChange(e.target.value)}
        aria-label="Note content"
      />

      <StatusBar status={status} />
    </div>
  );
}
