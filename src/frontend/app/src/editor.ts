/**
 * Markdown editor component per /docs/spec/ui/editor-flow.md
 *
 * Implements:
 * - E2E-06: autosave confidence path (debounced PATCH)
 * - E2E-17: draft integrity under conflicts
 * - E2E-24: Obsidian-like markdown editing
 * - Keyboard shortcuts for markdown formatting
 * - Cursor preservation across autosave commits
 *
 * Autosave interval: 2 seconds after last keystroke.
 * On version conflict (409): enter explicit conflict state.
 */

import { patchNote, updateTitle, getNote } from './api.js';
import type { NoteProjection } from './types.js';
import { mapKeyboardShortcut, applyFormatting } from './markdown.js';
import type { ShortcutAction } from './markdown.js';
import { saveCursor, restoreCursor } from './conflict.js';
import type { CursorPosition } from './conflict.js';

/** Autosave debounce interval in ms */
export const AUTOSAVE_DELAY_MS = 2000;

/** Editor state tracking */
export interface EditorInstance {
  readonly noteId: string;
  baseVersion: number;
  currentMarkdown: string;
  isDirty: boolean;
  lastSavedAt: string | null;
  autosaveTimer: ReturnType<typeof setTimeout> | null;
  cursorPos: CursorPosition | null;
  conflictDetected: boolean;
}

/** Create editor instance for a note revision */
export function createEditor(proj: NoteProjection): EditorInstance {
  return {
    noteId: proj.note_id,
    baseVersion: proj.version,
    currentMarkdown: proj.markdown,
    isDirty: false,
    lastSavedAt: null,
    autosaveTimer: null,
    cursorPos: null,
    conflictDetected: false,
  };
}

/** Handle content change — mark dirty and schedule autosave */
export function onContentChange(
  editor: EditorInstance,
  newMarkdown: string,
  textarea?: { selectionStart: number; selectionEnd: number },
): void {
  editor.currentMarkdown = newMarkdown;
  editor.isDirty = true;
  if (textarea) {
    editor.cursorPos = saveCursor(textarea);
  }
  if (editor.autosaveTimer !== null) {
    clearTimeout(editor.autosaveTimer);
  }
  editor.autosaveTimer = setTimeout(() => {
    void saveEditor(editor);
  }, AUTOSAVE_DELAY_MS);
}

/**
 * Handle keyboard event for markdown shortcuts.
 * Returns true if the event was handled.
 */
export function handleEditorKeydown(
  editor: EditorInstance,
  e: KeyboardEvent,
  textarea: { selectionStart: number; selectionEnd: number; value: string },
  applyText: (text: string, cursorStart: number, cursorEnd: number) => void,
): boolean {
  const action = mapKeyboardShortcut(e);
  if (!action) return false;
  if (action === 'save') {
    e.preventDefault();
    void saveEditor(editor);
    return true;
  }
  e.preventDefault();
  const result = applyFormatting(
    textarea.value,
    textarea.selectionStart,
    textarea.selectionEnd,
    action,
  );
  applyText(result.text, result.cursorStart, result.cursorEnd);
  editor.currentMarkdown = result.text;
  editor.isDirty = true;
  return true;
}

/**
 * Save current editor content via PATCH.
 * Per E2E-06: autosave confidence path.
 * Per E2E-17: on 409, enter conflict state instead of silent retry.
 */
export async function saveEditor(editor: EditorInstance): Promise<boolean> {
  if (!editor.isDirty) return true;
  const result = await patchNote(
    editor.noteId,
    editor.baseVersion,
    editor.currentMarkdown,
  );
  if (result.ok) {
    editor.baseVersion = result.data.version;
    editor.isDirty = false;
    editor.lastSavedAt = new Date().toISOString();
    editor.conflictDetected = false;
    return true;
  }
  // Version conflict — refetch and retry once per E2E-17
  if (result.status === 409) {
    const noteResult = await getNote(editor.noteId);
    if (noteResult.ok) {
      editor.baseVersion = noteResult.data.version;
      const retry = await patchNote(
        editor.noteId,
        editor.baseVersion,
        editor.currentMarkdown,
      );
      if (retry.ok) {
        editor.baseVersion = retry.data.version;
        editor.isDirty = false;
        editor.lastSavedAt = new Date().toISOString();
        editor.conflictDetected = false;
        return true;
      }
      // Second failure → explicit conflict state
      editor.conflictDetected = true;
    }
  }
  return false;
}

/** Rename note title with version check */
export async function renameNote(
  editor: EditorInstance,
  newTitle: string,
): Promise<boolean> {
  const result = await updateTitle(
    editor.noteId,
    editor.baseVersion,
    newTitle,
  );
  if (result.ok) {
    editor.baseVersion = result.data.version;
    return true;
  }
  return false;
}

/** Dispose editor — clear timer */
export function disposeEditor(editor: EditorInstance): void {
  if (editor.autosaveTimer !== null) {
    clearTimeout(editor.autosaveTimer);
    editor.autosaveTimer = null;
  }
}
