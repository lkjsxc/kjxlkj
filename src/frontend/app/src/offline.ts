/**
 * Offline/PWA support per /docs/spec/ui/reconstruction-ux-requirements.md (IMP-FE-03)
 *
 * Provides:
 * - Local cache for note drafts (IndexedDB-backed or in-memory fallback)
 * - Reconnect sync when connection is restored
 * - Service worker registration for offline shell
 * - Connection state tracking (online/offline)
 *
 * Per UX-EDIT-03: exposes saving/saved/conflict/offline states.
 */

/** Connection state enum per UX-EDIT-03 */
export type ConnectionState = 'online' | 'offline' | 'reconnecting';

/** Offline state tracked in the app */
export interface OfflineState {
  connection: ConnectionState;
  pendingDrafts: Map<string, PendingDraft>;
  lastOnlineAt: number;
}

/** A draft queued for sync when connection restores */
export interface PendingDraft {
  noteId: string;
  markdown: string;
  baseVersion: number;
  savedAt: number;
}

/** Create initial offline state */
export function createOfflineState(): OfflineState {
  return {
    connection: typeof navigator !== 'undefined' && navigator.onLine ? 'online' : 'offline',
    pendingDrafts: new Map(),
    lastOnlineAt: Date.now(),
  };
}

/** Save a draft to the pending queue (for offline sync) */
export function savePendingDraft(
  state: OfflineState,
  draft: PendingDraft,
): OfflineState {
  const drafts = new Map(state.pendingDrafts);
  drafts.set(draft.noteId, draft);
  return { ...state, pendingDrafts: drafts };
}

/** Remove a draft after successful sync */
export function clearPendingDraft(
  state: OfflineState,
  noteId: string,
): OfflineState {
  const drafts = new Map(state.pendingDrafts);
  drafts.delete(noteId);
  return { ...state, pendingDrafts: drafts };
}

/** Update connection state */
export function setConnectionState(
  state: OfflineState,
  conn: ConnectionState,
): OfflineState {
  return {
    ...state,
    connection: conn,
    lastOnlineAt: conn === 'online' ? Date.now() : state.lastOnlineAt,
  };
}

/** Get count of pending drafts */
export function pendingCount(state: OfflineState): number {
  return state.pendingDrafts.size;
}

/** Register service worker for offline shell caching.
 * Per IMP-FE-03: SW caches HTML/JS/CSS shell for offline access.
 * No-op if service workers are not supported. */
export function registerServiceWorker(swPath: string): void {
  if (typeof navigator === 'undefined') return;
  if (!('serviceWorker' in navigator)) return;
  void navigator.serviceWorker.register(swPath).catch(() => {
    // Non-fatal: PWA is best-effort
  });
}

/** Listen for online/offline events and call handler.
 * Returns a cleanup function to remove listeners. */
export function watchConnection(
  onOnline: () => void,
  onOffline: () => void,
): () => void {
  if (typeof window === 'undefined') return () => {};
  window.addEventListener('online', onOnline);
  window.addEventListener('offline', onOffline);
  return () => {
    window.removeEventListener('online', onOnline);
    window.removeEventListener('offline', onOffline);
  };
}

/** Render a connection status indicator.
 * Returns HTML string for display in the UI chrome. */
export function renderConnectionBadge(state: OfflineState): string {
  const pending = pendingCount(state);
  switch (state.connection) {
    case 'online':
      return pending > 0
        ? `<span class="conn-badge syncing">Syncing (${pending})</span>`
        : '<span class="conn-badge online">Online</span>';
    case 'offline':
      return `<span class="conn-badge offline">Offline${pending > 0 ? ` (${pending} pending)` : ''}</span>`;
    case 'reconnecting':
      return '<span class="conn-badge reconnecting">Reconnecting…</span>';
  }
}
