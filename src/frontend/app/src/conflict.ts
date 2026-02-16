/**
 * Conflict resolution UX per /docs/spec/ui/editor-flow.md
 *
 * Implements:
 * - Conflict detection (409 status from API)
 * - Explicit conflict state: reload or reapply
 * - Cursor/selection preservation across saves
 */

/** Conflict state for the editor */
export interface ConflictState {
  readonly hasConflict: boolean;
  readonly localBody: string;
  readonly serverBody: string;
  readonly serverVersion: number;
}

/** No conflict (default state) */
export function noConflict(): ConflictState {
  return { hasConflict: false, localBody: '', serverBody: '', serverVersion: 0 };
}

/** Enter conflict state with local and server copies */
export function enterConflict(
  localBody: string,
  serverBody: string,
  serverVersion: number,
): ConflictState {
  return { hasConflict: true, localBody, serverBody, serverVersion };
}

/** Resolution strategy */
export type ConflictResolution = 'accept_server' | 'reapply_local';

/**
 * Resolve conflict.
 * - accept_server: discard local changes, use server version
 * - reapply_local: keep local body, bump to server version for next save
 * Returns the body text and version to use going forward.
 */
export function resolveConflict(
  state: ConflictState,
  resolution: ConflictResolution,
): { body: string; version: number } {
  switch (resolution) {
    case 'accept_server':
      return { body: state.serverBody, version: state.serverVersion };
    case 'reapply_local':
      return { body: state.localBody, version: state.serverVersion };
  }
}

/** Render conflict banner HTML */
export function renderConflictBanner(state: ConflictState): string {
  if (!state.hasConflict) return '';
  return [
    '<div class="conflict-banner" role="alert">',
    '<p class="conflict-message">This note was modified elsewhere. Choose an action:</p>',
    '<div class="conflict-actions">',
    '<button class="conflict-accept" data-action="accept_server">',
    'Load server version</button>',
    '<button class="conflict-reapply" data-action="reapply_local">',
    'Keep my changes</button>',
    '</div>',
    '</div>',
  ].join('\n');
}

/** Cursor position for preservation across saves */
export interface CursorPosition {
  readonly start: number;
  readonly end: number;
}

/**
 * Save cursor position before an operation.
 * Per editor-flow.md: cursor/selection preserved across autosave commits.
 */
export function saveCursor(textarea: { selectionStart: number; selectionEnd: number }): CursorPosition {
  return { start: textarea.selectionStart, end: textarea.selectionEnd };
}

/**
 * Restore cursor position after an operation.
 * Clamps to text length to prevent out-of-bounds.
 */
export function restoreCursor(
  textarea: { selectionStart: number; selectionEnd: number; value: string },
  pos: CursorPosition,
): void {
  const len = textarea.value.length;
  textarea.selectionStart = Math.min(pos.start, len);
  textarea.selectionEnd = Math.min(pos.end, len);
}
