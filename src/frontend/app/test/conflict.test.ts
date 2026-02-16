/**
 * Conflict resolution tests per /docs/spec/ui/editor-flow.md
 *
 * Tests: conflict state management, resolution, banner rendering,
 * cursor preservation.
 */

import {
  noConflict,
  enterConflict,
  resolveConflict,
  renderConflictBanner,
  saveCursor,
  restoreCursor,
} from '../src/conflict.js';

// --- noConflict ---

function testNoConflict(): void {
  const state = noConflict();
  if (state.hasConflict) throw new Error('should not have conflict');
}

// --- enterConflict ---

function testEnterConflict(): void {
  const state = enterConflict('local', 'server', 5);
  if (!state.hasConflict) throw new Error('should have conflict');
  if (state.localBody !== 'local') throw new Error('wrong local body');
  if (state.serverBody !== 'server') throw new Error('wrong server body');
  if (state.serverVersion !== 5) throw new Error('wrong version');
}

// --- resolveConflict ---

function testResolveAcceptServer(): void {
  const r = resolveConflict(enterConflict('mine', 'theirs', 3), 'accept_server');
  if (r.body !== 'theirs') throw new Error('should use server body');
  if (r.version !== 3) throw new Error('should use server version');
}

function testResolveReapplyLocal(): void {
  const r = resolveConflict(enterConflict('mine', 'theirs', 3), 'reapply_local');
  if (r.body !== 'mine') throw new Error('should keep local body');
  if (r.version !== 3) throw new Error('should use server version');
}

// --- renderConflictBanner ---

function testBannerEmptyNoConflict(): void {
  if (renderConflictBanner(noConflict()) !== '') {
    throw new Error('should be empty');
  }
}

function testBannerRendersActions(): void {
  const html = renderConflictBanner(enterConflict('a', 'b', 1));
  if (!html.includes('conflict-banner')) throw new Error('missing banner class');
  if (!html.includes('conflict-accept')) throw new Error('missing accept button');
  if (!html.includes('conflict-reapply')) throw new Error('missing reapply button');
}

// --- saveCursor / restoreCursor ---

function testSaveCursor(): void {
  const pos = saveCursor({ selectionStart: 5, selectionEnd: 10 });
  if (pos.start !== 5 || pos.end !== 10) throw new Error('cursor not saved');
}

function testRestoreCursor(): void {
  const el = { selectionStart: 0, selectionEnd: 0, value: 'hello world' };
  restoreCursor(el, { start: 3, end: 7 });
  if (el.selectionStart !== 3 || el.selectionEnd !== 7) {
    throw new Error('cursor not restored');
  }
}

function testRestoreCursorClamps(): void {
  const el = { selectionStart: 0, selectionEnd: 0, value: 'hi' };
  restoreCursor(el, { start: 100, end: 200 });
  if (el.selectionStart !== 2 || el.selectionEnd !== 2) {
    throw new Error('cursor not clamped');
  }
}

// Run all
testNoConflict();
testEnterConflict();
testResolveAcceptServer();
testResolveReapplyLocal();
testBannerEmptyNoConflict();
testBannerRendersActions();
testSaveCursor();
testRestoreCursor();
testRestoreCursorClamps();
console.log('All conflict tests passed (9 assertions)');
