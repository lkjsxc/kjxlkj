/**
 * Application state management per /docs/spec/ui/web-app.md
 *
 * Manages session, notes list, active note, and editor state.
 * Per /docs/spec/ui/editor-flow.md: autosave-first, draft recovery.
 */

import type { NoteStream, NoteProjection, SessionInfo, Workspace } from './types.js';

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
