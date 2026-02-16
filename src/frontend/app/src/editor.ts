/**
 * Markdown editor component per /docs/spec/ui/editor-flow.md
 *
 * Implements:
 * - E2E-06: autosave confidence path (debounced PATCH)
 * - E2E-17: draft integrity under conflicts
 * - E2E-24: Obsidian-like markdown editing
 *
 * Autosave interval: 2 seconds after last keystroke.
 * On version conflict (409): retry with current version.
 */

import { patchNote, updateTitle, getNote } from './api.js';
import type { NoteProjection } from './types.js';

/** Autosave debounce interval in ms */
const AUTOSAVE_DELAY_MS = 2000;

/** Editor state tracking */
export interface EditorInstance {
  readonly noteId: string;
  baseVersion: number;
  currentMarkdown: string;
  isDirty: boolean;
  lastSavedAt: string | null;
  autosaveTimer: ReturnType<typeof setTimeout> | null;
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
  };
}

/** Handle content change — mark dirty and schedule autosave */
export function onContentChange(editor: EditorInstance, newMarkdown: string): void {
  editor.currentMarkdown = newMarkdown;
  editor.isDirty = true;
  if (editor.autosaveTimer !== null) {
    clearTimeout(editor.autosaveTimer);
  }
  editor.autosaveTimer = setTimeout(() => {
    void saveEditor(editor);
  }, AUTOSAVE_DELAY_MS);
}

/**
 * Save current editor content via PATCH.
 * Per E2E-06: autosave confidence path.
 * Per E2E-17: on 409. refetch and retry with current version.
 */
export async function saveEditor(editor: EditorInstance): Promise<boolean> {
  if (!editor.isDirty) return true;
  const result = await patchNote(editor.noteId, editor.baseVersion, editor.currentMarkdown);
  if (result.ok) {
    editor.baseVersion = result.data.version;
    editor.isDirty = false;
    editor.lastSavedAt = new Date().toISOString();
    return true;
  }
  // Version conflict — refetch and retry per E2E-17
  if (result.status === 409) {
    const noteResult = await getNote(editor.noteId);
    if (noteResult.ok) {
      editor.baseVersion = noteResult.data.version;
      // Retry once with updated version
      const retry = await patchNote(editor.noteId, editor.baseVersion, editor.currentMarkdown);
      if (retry.ok) {
        editor.baseVersion = retry.data.version;
        editor.isDirty = false;
        editor.lastSavedAt = new Date().toISOString();
        return true;
      }
    }
  }
  return false;
}

/** Rename note title with version check */
export async function renameNote(
  editor: EditorInstance,
  newTitle: string,
): Promise<boolean> {
  const result = await updateTitle(editor.noteId, editor.baseVersion, newTitle);
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
