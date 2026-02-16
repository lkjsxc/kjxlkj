/**
 * Frontend entry point per /docs/spec/ui/web-app.md
 */
export { App, boot } from './app.js';
export type { AppState, AppView, EditorState } from './state.js';
export type { ClientMessage, ServerMessage, MessageListener } from './ws.js';
export { WsClient } from './ws.js';
export { createEditor, onContentChange, saveEditor, disposeEditor } from './editor.js';
export { computeLayout, applyLayoutClasses, onNoteSelected, toggleMenu } from './layout.js';
export { extractWikiLinks, markdownToHtml, mapKeyboardShortcut, applyFormatting } from './markdown.js';
export { cyclePreviewMode, updatePreview, renderPreviewPanel, renderPreviewToggle, findBacklinks } from './preview.js';
export type { PreviewMode, PreviewState } from './preview.js';
export { createNoteListState, filterNotes, renderNoteList, createAndSelectNote, propagateTitle } from './note-list.js';
export type { NoteListState } from './note-list.js';
export { noConflict, enterConflict, resolveConflict, renderConflictBanner, saveCursor, restoreCursor } from './conflict.js';
export type { ConflictState, ConflictResolution, CursorPosition } from './conflict.js';
export * from './types.js';
export * from './api.js';
