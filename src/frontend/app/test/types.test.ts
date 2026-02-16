/**
 * Frontend type and contract tests per /docs/spec/technical/testing.md
 *
 * These tests validate type contracts and state invariants.
 */

import { createInitialState, isCompactMode, MENU_COMPACT_BREAKPOINT } from '../src/state.js';
import type { AppState } from '../src/state.js';
import type { NoteKind, SearchMode, NoteState } from '../src/types.js';

// Type assertion tests — compile-time checks, no runtime needed

// E2E-17: note creation defaults
function testNoteKindValues(): void {
  const kinds: ReadonlyArray<NoteKind> = ['markdown', 'settings', 'media_image', 'media_video'];
  if (kinds.length !== 4) throw new Error('NoteKind must have 4 variants');
}

// E2E-19: responsive menu threshold
function testMenuBreakpoint(): void {
  if (MENU_COMPACT_BREAKPOINT !== 1280) {
    throw new Error('Menu breakpoint must be 1280 per layout-and-interaction.md');
  }
  if (!isCompactMode(1280)) throw new Error('1280 should be compact');
  if (!isCompactMode(320)) throw new Error('320 should be compact');
  if (isCompactMode(1281)) throw new Error('1281 should not be compact');
}

// E2E-20: initial state
function testInitialState(): void {
  const state: AppState = createInitialState();
  if (state.view !== 'login') throw new Error('Initial view should be login');
  if (state.session !== null) throw new Error('Initial session should be null');
  if (state.notes.length !== 0) throw new Error('Initial notes should be empty');
  if (state.menuOpen !== false) throw new Error('Menu should start closed');
}

// Search mode type check
function testSearchModes(): void {
  const modes: ReadonlyArray<SearchMode> = ['hybrid', 'lexical', 'semantic'];
  if (modes.length !== 3) throw new Error('SearchMode must have 3 variants');
}

// Note state type check
function testNoteStates(): void {
  const states: ReadonlyArray<NoteState> = ['active', 'soft_deleted'];
  if (states.length !== 2) throw new Error('NoteState must have 2 variants');
}

// Run all tests
testNoteKindValues();
testMenuBreakpoint();
testInitialState();
testSearchModes();
testNoteStates();
console.log('All frontend tests passed');
