// Notes list — searchable note index within scope.
// UX-EDIT-04: title propagation updates list items in same cycle.

import { useCallback, useEffect, useState } from 'react';
import { api } from '../api';
import type { NoteProjection } from '../types';

interface Props {
  workspaceId: string | null;
  selectedNoteId: string | null;
  onSelectNote: (noteId: string) => void;
  /** Map of noteId→title overrides from NoteDetail title edits. */
  titleOverrides: Map<string, string>;
}

export default function NotesList({
  workspaceId,
  selectedNoteId,
  onSelectNote,
  titleOverrides,
}: Props) {
  const [notes, setNotes] = useState<NoteProjection[]>([]);
  const [search, setSearch] = useState('');
  const [loading, setLoading] = useState(false);

  const load = useCallback(async () => {
    if (!workspaceId) return;
    setLoading(true);
    try {
      const path = search
        ? `/api/workspaces/${workspaceId}/search?q=${encodeURIComponent(search)}`
        : `/api/workspaces/${workspaceId}/notes`;
      const data = await api.get<NoteProjection[]>(path);
      setNotes(data);
    } catch {
      // keep existing list on error
    } finally {
      setLoading(false);
    }
  }, [workspaceId, search]);

  useEffect(() => {
    void load();
  }, [load]);

  function resolvedTitle(note: NoteProjection): string {
    return titleOverrides.get(note.note_id) ?? note.title;
  }

  return (
    <div className="notes-list">
      <div className="notes-list-header">
        <input
          className="notes-search"
          type="search"
          placeholder="Search notes…"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          aria-label="Search notes"
        />
      </div>

      {loading && <p className="notes-loading">Loading…</p>}

      <ul className="notes-items" role="listbox">
        {notes.map((note) => (
          <li key={note.note_id} role="option" aria-selected={note.note_id === selectedNoteId}>
            <button
              type="button"
              className={`note-list-item${note.note_id === selectedNoteId ? ' selected' : ''}`}
              onClick={() => onSelectNote(note.note_id)}
            >
              <span className="note-list-title">
                {resolvedTitle(note) || 'Untitled'}
              </span>
              <span className="note-list-kind">{note.note_kind}</span>
            </button>
          </li>
        ))}
      </ul>

      {!loading && notes.length === 0 && (
        <p className="notes-empty">No notes yet.</p>
      )}
    </div>
  );
}
