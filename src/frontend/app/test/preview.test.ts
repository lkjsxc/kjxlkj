/**
 * Preview component tests per /docs/spec/ui/editor-flow.md
 *
 * Tests: preview mode cycling, HTML rendering, backlinks detection.
 */

import {
  createPreviewState,
  cyclePreviewMode,
  updatePreview,
  renderPreviewPanel,
  renderPreviewToggle,
  editorAreaClasses,
  findBacklinks,
} from '../src/preview.js';

// --- cyclePreviewMode ---

function testCycleEditToSplit(): void {
  if (cyclePreviewMode('edit') !== 'split') throw new Error('edit→split');
}

function testCycleSplitToPreview(): void {
  if (cyclePreviewMode('split') !== 'preview') throw new Error('split→preview');
}

function testCyclePreviewToEdit(): void {
  if (cyclePreviewMode('preview') !== 'edit') throw new Error('preview→edit');
}

// --- updatePreview ---

function testUpdatePreviewConvertsMarkdown(): void {
  const state = updatePreview(createPreviewState(), '# Hello');
  if (!state.html.includes('<h1>Hello</h1>')) throw new Error('no heading in preview');
}

// --- renderPreviewPanel ---

function testPanelEmptyInEditMode(): void {
  if (renderPreviewPanel(createPreviewState()) !== '') {
    throw new Error('edit mode should render empty');
  }
}

function testPanelRendersInSplitMode(): void {
  const state = updatePreview({ mode: 'split', html: '' }, '# Hi');
  const html = renderPreviewPanel(state);
  if (!html.includes('preview-panel')) throw new Error('missing preview-panel class');
  if (!html.includes('<h1>Hi</h1>')) throw new Error('missing heading');
}

// --- renderPreviewToggle ---

function testToggleLabelEdit(): void {
  if (!renderPreviewToggle('edit').includes('Preview')) {
    throw new Error('edit mode label should say Preview');
  }
}

function testToggleLabelPreview(): void {
  if (!renderPreviewToggle('preview').includes('Edit')) {
    throw new Error('preview mode label should say Edit');
  }
}

// --- editorAreaClasses ---

function testEditorOnlyClass(): void {
  if (!editorAreaClasses('edit').includes('editor-only')) {
    throw new Error('edit should have editor-only');
  }
}

function testEditorSplitClass(): void {
  if (!editorAreaClasses('split').includes('editor-split')) {
    throw new Error('split should have editor-split');
  }
}

function testPreviewOnlyClass(): void {
  if (!editorAreaClasses('preview').includes('preview-only')) {
    throw new Error('preview should have preview-only');
  }
}

// --- findBacklinks ---

function testFindBacklinksMultiple(): void {
  const notes = [
    { id: '1', title: 'A', body: 'see [[B]]' },
    { id: '2', title: 'B', body: 'hello' },
    { id: '3', title: 'C', body: 'link to [[B]] and [[A]]' },
  ];
  const result = findBacklinks('B', notes);
  if (result.length !== 2) throw new Error(`expected 2 backlinks, got ${result.length}`);
  if (!result.some(r => r.id === '1')) throw new Error('note 1 should backlink');
  if (!result.some(r => r.id === '3')) throw new Error('note 3 should backlink');
}

function testFindBacklinksNone(): void {
  const notes = [{ id: '1', title: 'A', body: 'no links' }];
  if (findBacklinks('B', notes).length !== 0) throw new Error('expected 0');
}

// Run all
testCycleEditToSplit();
testCycleSplitToPreview();
testCyclePreviewToEdit();
testUpdatePreviewConvertsMarkdown();
testPanelEmptyInEditMode();
testPanelRendersInSplitMode();
testToggleLabelEdit();
testToggleLabelPreview();
testEditorOnlyClass();
testEditorSplitClass();
testPreviewOnlyClass();
testFindBacklinksMultiple();
testFindBacklinksNone();
console.log('All preview tests passed (13 assertions)');
