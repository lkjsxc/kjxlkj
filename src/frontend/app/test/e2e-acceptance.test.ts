/**
 * E2E acceptance tests for UI behavior per /docs/spec/technical/testing.md
 *
 * Covers:
 * - E2E-12: compact top-right menu behavior
 * - E2E-19: 320px layout remains usable, no horizontal scroll
 * - E2E-24: Obsidian-like markdown editing shortcuts
 * - E2E-25: compact mode at <=1280px, closes menu on select
 */

import {
  computeLayout,
  applyLayoutClasses,
  onNoteSelected,
  toggleMenu,
  LAYOUT_CSS,
} from '../src/layout.js';
import { createEditor, onContentChange, AUTOSAVE_DELAY_MS } from '../src/editor.js';
import { mapKeyboardShortcut, applyFormatting } from '../src/markdown.js';
import type { NoteProjection } from '../src/types.js';

function assert(condition: boolean, msg: string): void {
  if (!condition) throw new Error(`FAIL: ${msg}`);
}

// --- E2E-12: compact menu toggle ---
(() => {
  // Toggle opens and closes
  const open = toggleMenu(false);
  assert(open === true, 'E2E-12: toggleMenu false→true');
  const closed = toggleMenu(true);
  assert(closed === false, 'E2E-12: toggleMenu true→false');
  console.log('PASS: E2E-12 menu toggle');
})();

// --- E2E-19: 320px layout usable, no horizontal scroll ---
(() => {
  const layout = computeLayout(320, false);
  assert(layout.compact === true, 'E2E-19: 320px is compact');
  assert(layout.editorFullWidth === true, 'E2E-19: editor full width at 320px');
  assert(layout.noteListVisible === false, 'E2E-19: note list hidden at 320px');
  // CSS must include overflow-x: hidden
  assert(LAYOUT_CSS.includes('overflow-x: hidden'), 'E2E-19: CSS prevents h-scroll');
  assert(LAYOUT_CSS.includes('max-width: 100vw'), 'E2E-19: CSS max-width 100vw');
  console.log('PASS: E2E-19 320px layout');
})();

// --- E2E-24: Obsidian-like markdown shortcuts ---
(() => {
  // Bold Ctrl+B
  const boldAction = mapKeyboardShortcut({
    ctrlKey: true, metaKey: false, shiftKey: false, key: 'b',
    preventDefault: () => {},
  } as unknown as KeyboardEvent);
  assert(boldAction === 'bold', 'E2E-24: Ctrl+B→bold');

  // Italic Ctrl+I  
  const italicAction = mapKeyboardShortcut({
    ctrlKey: true, metaKey: false, shiftKey: false, key: 'i',
    preventDefault: () => {},
  } as unknown as KeyboardEvent);
  assert(italicAction === 'italic', 'E2E-24: Ctrl+I→italic');

  // Apply bold formatting
  const result = applyFormatting('hello world', 0, 5, 'bold');
  assert(result.text.includes('**hello**'), 'E2E-24: bold wraps selection');

  // Apply heading
  const h1 = applyFormatting('title', 0, 5, 'heading1');
  assert(h1.text.startsWith('# '), 'E2E-24: heading1 prefixes #');

  // Apply wiki link
  const link = applyFormatting('page', 0, 4, 'wiki_link');
  assert(link.text.includes('[[page]]'), 'E2E-24: wiki_link wraps in [[]]');

  console.log('PASS: E2E-24 markdown shortcuts');
})();

// --- E2E-25: compact mode at <=1280px, closes on select ---
(() => {
  // At 1280px → compact
  const at1280 = computeLayout(1280, false);
  assert(at1280.compact === true, 'E2E-25: 1280px is compact');

  // At 1281px → not compact
  const at1281 = computeLayout(1281, false);
  assert(at1281.compact === false, 'E2E-25: 1281px not compact');

  // Compact + menu open → select closes menu
  const menuAfterSelect = onNoteSelected(true, true);
  assert(menuAfterSelect === false, 'E2E-25: compact select closes menu');

  // Non-compact → menu stays as-is
  const menuStays = onNoteSelected(false, true);
  assert(menuStays === true, 'E2E-25: non-compact select keeps menu');

  // Compact with menu open → note list visible
  const withMenu = computeLayout(1000, true);
  assert(withMenu.noteListVisible === true, 'E2E-25: compact+menu→list visible');

  console.log('PASS: E2E-25 compact mode');
})();

// --- E2E-06: autosave editor lifecycle assertion ---
(() => {
  const proj: NoteProjection = {
    note_id: '00000000-0000-0000-0000-000000000001',
    title: 'Test',
    version: 1,
    markdown: 'initial',
    metadata_json: {},
    updated_at: new Date().toISOString(),
  };
  const editor = createEditor(proj);
  assert(editor.isDirty === false, 'E2E-06: fresh editor not dirty');
  assert(editor.baseVersion === 1, 'E2E-06: initial version');

  onContentChange(editor, 'draft text');
  assert(editor.isDirty === true, 'E2E-06: after change, editor dirty');
  assert(editor.currentMarkdown === 'draft text', 'E2E-06: markdown updated');
  assert(AUTOSAVE_DELAY_MS === 2000, 'E2E-06: autosave delay 2s');

  // Clean up timer
  if (editor.autosaveTimer !== null) {
    clearTimeout(editor.autosaveTimer);
  }

  console.log('PASS: E2E-06 autosave lifecycle');
})();

console.log('All E2E acceptance tests passed');
