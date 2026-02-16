/**
 * Tests for offline/PWA support per IMP-FE-03.
 */

import {
  createOfflineState,
  savePendingDraft,
  clearPendingDraft,
  setConnectionState,
  pendingCount,
  renderConnectionBadge,
} from '../src/offline.js';

function assert(cond: boolean, msg: string): void {
  if (!cond) throw new Error(`FAIL: ${msg}`);
}

// --- createOfflineState ---
{
  const state = createOfflineState();
  assert(state.connection === 'offline' || state.connection === 'online', 'connection should be online or offline');
  assert(state.pendingDrafts.size === 0, 'starts with no pending drafts');
  assert(state.lastOnlineAt > 0, 'lastOnlineAt is positive timestamp');
}

// --- savePendingDraft ---
{
  let state = createOfflineState();
  state = savePendingDraft(state, {
    noteId: 'n1',
    markdown: '# Hello',
    baseVersion: 1,
    savedAt: Date.now(),
  });
  assert(state.pendingDrafts.size === 1, 'one pending draft');
  assert(state.pendingDrafts.get('n1')?.markdown === '# Hello', 'draft content preserved');
}

// --- clearPendingDraft ---
{
  let state = createOfflineState();
  state = savePendingDraft(state, {
    noteId: 'n1',
    markdown: 'test',
    baseVersion: 1,
    savedAt: Date.now(),
  });
  state = clearPendingDraft(state, 'n1');
  assert(state.pendingDrafts.size === 0, 'cleared draft removed');
}

// --- setConnectionState ---
{
  let state = createOfflineState();
  state = setConnectionState(state, 'offline');
  assert(state.connection === 'offline', 'set to offline');
  const before = state.lastOnlineAt;
  state = setConnectionState(state, 'online');
  assert(state.connection === 'online', 'set to online');
  assert(state.lastOnlineAt >= before, 'lastOnlineAt updated on online');
}

// --- pendingCount ---
{
  let state = createOfflineState();
  assert(pendingCount(state) === 0, 'zero pending initially');
  state = savePendingDraft(state, { noteId: 'a', markdown: '', baseVersion: 1, savedAt: 0 });
  state = savePendingDraft(state, { noteId: 'b', markdown: '', baseVersion: 1, savedAt: 0 });
  assert(pendingCount(state) === 2, 'two pending after saves');
}

// --- renderConnectionBadge ---
{
  let state = createOfflineState();
  state = setConnectionState(state, 'online');
  const badge = renderConnectionBadge(state);
  assert(badge.includes('Online'), 'online badge when connected');
}
{
  let state = createOfflineState();
  state = setConnectionState(state, 'offline');
  state = savePendingDraft(state, { noteId: 'x', markdown: '', baseVersion: 1, savedAt: 0 });
  const badge = renderConnectionBadge(state);
  assert(badge.includes('Offline'), 'offline badge');
  assert(badge.includes('1 pending'), 'shows pending count');
}
{
  let state = createOfflineState();
  state = setConnectionState(state, 'reconnecting');
  const badge = renderConnectionBadge(state);
  assert(badge.includes('Reconnecting'), 'reconnecting badge');
}

console.log('offline.test: all 10 assertions passed');
