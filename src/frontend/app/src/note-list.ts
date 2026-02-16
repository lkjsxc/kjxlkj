/**
 * Note list component per /docs/spec/ui/web-app.md
 *
 * Implements:
 * - Notes list view with search/filter
 * - Create new note (immediate selection per E2E-23)
 * - Title propagation to list on same render cycle
 */

import type { NoteStream } from './types.js';
import { createNote } from './api.js';

/** Note list filter state */
export interface NoteListState {
  searchQuery: string;
  notes: ReadonlyArray<NoteStream>;
  selectedId: string | null;
  loading: boolean;
}

/** Create initial note list state */
export function createNoteListState(): NoteListState {
  return { searchQuery: '', notes: [], selectedId: null, loading: false };
}

/**
 * Filter notes by search query.
 * Matches against title (case-insensitive substring).
 */
export function filterNotes(
  notes: ReadonlyArray<NoteStream>,
  query: string,
): ReadonlyArray<NoteStream> {
  if (!query.trim()) return notes;
  const lower = query.toLowerCase();
  return notes.filter(n => n.title.toLowerCase().includes(lower));
}

/**
 * Render note list HTML.
 * Each item is a clickable row; selected item is highlighted.
 */
export function renderNoteList(state: NoteListState): string {
  const filtered = filterNotes(state.notes, state.searchQuery);
  const items = filtered.map(n => {
    const selected = n.id === state.selectedId ? ' selected' : '';
    return [
      `<li class="note-item${selected}" data-id="${n.id}" role="option"`,
      ` aria-selected="${n.id === state.selectedId}">`,
      `<span class="note-title">${escapeHtml(n.title)}</span>`,
      `<span class="note-kind">${escapeHtml(n.note_kind)}</span>`,
      `</li>`,
    ].join('');
  });

  return [
    '<div class="note-list-container">',
    '<div class="note-list-toolbar">',
    `<input type="search" class="note-search" placeholder="Search notes…"`,
    ` value="${escapeHtml(state.searchQuery)}" aria-label="Search notes">`,
    '<button class="note-create-btn" aria-label="Create note">+ New</button>',
    '</div>',
    `<ul class="note-list" role="listbox" aria-label="Notes">`,
    state.loading ? '<li class="note-loading">Loading…</li>' : '',
    ...items,
    filtered.length === 0 && !state.loading
      ? '<li class="note-empty">No notes</li>'
      : '',
    '</ul>',
    '</div>',
  ].join('\n');
}

/**
 * Create a new note and select it immediately.
 * Per web-app.md E2E-23: "creates and selects note immediately".
 * Returns the updated list state.
 */
export async function createAndSelectNote(
  workspaceId: string,
  title: string,
  state: NoteListState,
): Promise<NoteListState> {
  const result = await createNote({
    workspace_id: workspaceId,
    title,
    markdown: '',
    note_kind: 'markdown',
  });
  if (!result.ok) return state;
  const newStream: NoteStream = result.data;
  return {
    ...state,
    notes: [newStream, ...state.notes],
    selectedId: newStream.id,
  };
}

/**
 * Update a note title in the list (title propagation).
 * Per web-app.md: "Title edits propagate to list/navigation same cycle".
 */
export function propagateTitle(
  state: NoteListState,
  noteId: string,
  newTitle: string,
): NoteListState {
  const notes = state.notes.map(n =>
    n.id === noteId ? { ...n, title: newTitle } : n,
  );
  return { ...state, notes };
}

function escapeHtml(text: string): string {
  return text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}
