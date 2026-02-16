/**
 * Application state management per /docs/spec/ui/web-app.md
 *
 * Manages session, notes list, active note, editor, preview, and conflict state.
 * Per /docs/spec/ui/editor-flow.md: autosave-first, draft recovery.
 */

import type { NoteStream, NoteProjection, SessionInfo, Workspace } from './types.js';
import type { PreviewMode } from './preview.js';
import type { ConflictState } from './conflict.js';
import type { OfflineState } from './offline.js';
import { createOfflineState } from './offline.js';

/** Application view per /docs/spec/ui/web-app.md */
export type AppView = 'setup' | 'login' | 'notes_list' | 'note_detail' | 'agent_runs';

/** Editor state per /docs/spec/ui/editor-flow.md */
export interface EditorState {
  readonly noteId: string;
  readonly baseVersion: number;
  readonly markdown: string;
  readonly isDirty: boolean;
  readonly lastSavedAt: string | null;
}

/** Full app state */
export interface AppState {
  readonly view: AppView;
  readonly session: SessionInfo | null;
  readonly workspaces: ReadonlyArray<Workspace>;
  readonly activeWorkspaceId: string | null;
  readonly notes: ReadonlyArray<NoteStream>;
  readonly activeNote: NoteProjection | null;
  readonly editor: EditorState | null;
  readonly menuOpen: boolean;
  readonly searchQuery: string;
  readonly previewMode: PreviewMode;
  readonly conflict: ConflictState;
  readonly backlinks: ReadonlyArray<{ id: string; title: string }>;
  /** Offline/PWA state per IMP-FE-03 */
  readonly offline: OfflineState;
}

/** Create initial state per /docs/spec/ui/web-app.md */
export function createInitialState(): AppState {
  return {
    view: 'login',
    session: null,
    workspaces: [],
    activeWorkspaceId: null,
    notes: [],
    activeNote: null,
    editor: null,
    menuOpen: false,
    searchQuery: '',
    previewMode: 'edit',
    conflict: { hasConflict: false, localBody: '', serverBody: '', serverVersion: 0 },
    backlinks: [],
    offline: createOfflineState(),
  };
}

/**
 * Menu toggle threshold per /docs/spec/ui/layout-and-interaction.md
 * Compact mode at <= 1280px.
 */
export const MENU_COMPACT_BREAKPOINT = 1280;

/**
 * Check if we should use compact menu mode
 * Per /docs/spec/ui/layout-and-interaction.md: max-width 1280px
 */
export function isCompactMode(windowWidth: number): boolean {
  return windowWidth <= MENU_COMPACT_BREAKPOINT;
}
