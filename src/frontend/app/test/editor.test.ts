/**
 * Editor contract tests per /docs/spec/technical/testing.md
 *
 * Tests: E2E-06, E2E-17, E2E-24
 */

import { createEditor, onContentChange, disposeEditor } from '../src/editor.js';
import type { NoteProjection } from '../src/types.js';

function makeProjection(): NoteProjection {
  return {
    note_id: 'test-id',
    title: 'Test',
    version: 5,
    markdown: '# Hello',
    metadata_json: {},
    updated_at: '2026-01-01T00:00:00Z',
  };
}

// E2E-24: create editor from projection
function testCreateEditor(): void {
  const editor = createEditor(makeProjection());
  if (editor.noteId !== 'test-id') throw new Error('noteId mismatch');
  if (editor.baseVersion !== 5) throw new Error('baseVersion mismatch');
  if (editor.currentMarkdown !== '# Hello') throw new Error('markdown mismatch');
  if (editor.isDirty !== false) throw new Error('should not be dirty');
  if (editor.lastSavedAt !== null) throw new Error('lastSavedAt should be null');
  disposeEditor(editor);
}

// E2E-06: content change marks dirty and schedules save
function testContentChange(): void {
  const editor = createEditor(makeProjection());
  onContentChange(editor, '# Updated');
  if (editor.currentMarkdown !== '# Updated') throw new Error('markdown not updated');
  if (!editor.isDirty) throw new Error('should be dirty after change');
  if (editor.autosaveTimer === null) throw new Error('autosave timer should be set');
  disposeEditor(editor);
}

// E2E-17: dispose clears timer
function testDispose(): void {
  const editor = createEditor(makeProjection());
  onContentChange(editor, '# Changed');
  disposeEditor(editor);
  if (editor.autosaveTimer !== null) throw new Error('timer should be cleared');
}

testCreateEditor();
testContentChange();
testDispose();
console.log('All editor tests passed');
