/**
 * Preview component per /docs/spec/ui/editor-flow.md
 *
 * Implements split/toggle markdown preview.
 * The preview state is managed in AppState and the
 * preview panel renders HTML from markdownToHtml().
 */

import { markdownToHtml, extractWikiLinks } from './markdown.js';

/** Preview display mode */
export type PreviewMode = 'edit' | 'split' | 'preview';

/** Preview panel state */
export interface PreviewState {
  mode: PreviewMode;
  html: string;
}

/** Create initial preview state */
export function createPreviewState(): PreviewState {
  return { mode: 'edit', html: '' };
}

/** Cycle preview mode: edit → split → preview → edit */
export function cyclePreviewMode(current: PreviewMode): PreviewMode {
  switch (current) {
    case 'edit': return 'split';
    case 'split': return 'preview';
    case 'preview': return 'edit';
  }
}

/** Update preview HTML from markdown content */
export function updatePreview(state: PreviewState, markdown: string): PreviewState {
  return { ...state, html: markdownToHtml(markdown) };
}

/**
 * Render preview container.
 * Returns an HTML string for the preview panel.
 * Wiki-links in the preview are clickable anchors with data-target attribute.
 */
export function renderPreviewPanel(state: PreviewState): string {
  if (state.mode === 'edit') return '';
  return [
    '<div class="preview-panel" role="region" aria-label="Markdown preview">',
    state.html,
    '</div>',
  ].join('\n');
}

/**
 * Render the preview toggle button.
 * Per editor-flow.md: preview is toggled via toolbar button.
 */
export function renderPreviewToggle(mode: PreviewMode): string {
  const labels: Record<PreviewMode, string> = {
    edit: 'Preview',
    split: 'Full preview',
    preview: 'Edit',
  };
  return `<button class="preview-toggle" aria-label="Toggle preview">${labels[mode]}</button>`;
}

/**
 * Compute CSS classes for the editor area based on preview mode.
 * Per editor-flow.md: split view shows editor + preview side by side.
 */
export function editorAreaClasses(mode: PreviewMode): string {
  switch (mode) {
    case 'edit': return 'editor-area editor-only';
    case 'split': return 'editor-area editor-split';
    case 'preview': return 'editor-area preview-only';
  }
}

/** Preview panel CSS constants */
export const PREVIEW_CSS = {
  splitEditorWidth: '50%',
  splitPreviewWidth: '50%',
  previewPadding: '1rem',
  wikiLinkColor: 'var(--accent, #0969da)',
} as const;

/**
 * Extract backlinks from a list of notes for a given note title.
 * Per editor-flow.md: backlinks are shown in a sidebar section.
 */
export function findBacklinks(
  targetTitle: string,
  allNotes: ReadonlyArray<{ readonly id: string; readonly title: string; readonly body: string }>,
): ReadonlyArray<{ readonly id: string; readonly title: string }> {
  const results: { id: string; title: string }[] = [];
  for (const note of allNotes) {
    const links = extractWikiLinks(note.body);
    if (links.some(l => l.target === targetTitle)) {
      results.push({ id: note.id, title: note.title });
    }
  }
  return results;
}
