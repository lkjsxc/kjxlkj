/**
 * Note list component tests per /docs/spec/ui/web-app.md
 *
 * Tests: filtering, rendering, title propagation.
 */

import {
  createNoteListState,
  filterNotes,
  renderNoteList,
  propagateTitle,
} from '../src/note-list.js';
import type { NoteStream } from '../src/types.js';

function makeNote(id: string, title: string): NoteStream {
  return {
    id,
    workspace_id: 'ws-1',
    project_id: null,
    title,
    note_kind: 'markdown',
    access_scope: 'workspace',
    state: 'active',
    current_version: 1,
    created_at: '2026-01-01T00:00:00Z',
    updated_at: '2026-01-01T00:00:00Z',
  };
}

// --- filterNotes ---

function testFilterAllForEmpty(): void {
  const notes = [makeNote('1', 'A'), makeNote('2', 'B')];
  if (filterNotes(notes, '').length !== 2) throw new Error('empty query returns all');
}

function testFilterBySubstring(): void {
  const notes = [makeNote('1', 'Alpha'), makeNote('2', 'Beta')];
  const result = filterNotes(notes, 'alp');
  if (result.length !== 1) throw new Error('filter by alp');
  if (result[0]!.title !== 'Alpha') throw new Error('wrong note filtered');
}

function testFilterCaseInsensitive(): void {
  const notes = [makeNote('1', 'Hello')];
  if (filterNotes(notes, 'HELLO').length !== 1) throw new Error('case insensitive');
}

function testFilterNoMatch(): void {
  if (filterNotes([makeNote('1', 'A')], 'zzz').length !== 0) {
    throw new Error('no match should be 0');
  }
}

// --- renderNoteList ---

function testRenderListWithNotes(): void {
  const state = { ...createNoteListState(), notes: [makeNote('1', 'Test')] };
  const html = renderNoteList(state);
  if (!html.includes('note-list')) throw new Error('missing note-list class');
  if (!html.includes('Test')) throw new Error('missing note title');
}

function testRenderEmptyState(): void {
  const html = renderNoteList(createNoteListState());
  if (!html.includes('No notes')) throw new Error('missing empty indicator');
}

function testRenderSearchInput(): void {
  const html = renderNoteList(createNoteListState());
  if (!html.includes('note-search')) throw new Error('missing search input');
}

function testRenderCreateButton(): void {
  const html = renderNoteList(createNoteListState());
  if (!html.includes('note-create-btn')) throw new Error('missing create button');
}

function testRenderSelectedItem(): void {
  const state = {
    ...createNoteListState(),
    notes: [makeNote('1', 'Sel')],
    selectedId: '1',
  };
  const html = renderNoteList(state);
  if (!html.includes('aria-selected="true"')) throw new Error('missing selected aria');
}

// --- propagateTitle ---

function testPropagateTitle(): void {
  const state = { ...createNoteListState(), notes: [makeNote('1', 'Old')] };
  const updated = propagateTitle(state, '1', 'New');
  if (updated.notes[0]!.title !== 'New') throw new Error('title not propagated');
}

function testPropagateDoesNotChangeOthers(): void {
  const state = {
    ...createNoteListState(),
    notes: [makeNote('1', 'A'), makeNote('2', 'B')],
  };
  const updated = propagateTitle(state, '1', 'Changed');
  if (updated.notes[1]!.title !== 'B') throw new Error('other note was changed');
}

// Run all
testFilterAllForEmpty();
testFilterBySubstring();
testFilterCaseInsensitive();
testFilterNoMatch();
testRenderListWithNotes();
testRenderEmptyState();
testRenderSearchInput();
testRenderCreateButton();
testRenderSelectedItem();
testPropagateTitle();
testPropagateDoesNotChangeOthers();
console.log('All note-list tests passed (11 assertions)');
