/**
 * Markdown utility tests per /docs/spec/ui/editor-flow.md
 *
 * Tests wiki-link extraction, markdown preview, keyboard shortcuts,
 * and formatting application.
 */

import {
  extractWikiLinks,
  markdownToHtml,
  mapKeyboardShortcut,
  applyFormatting,
} from '../src/markdown.js';

// --- extractWikiLinks ---

function testExtractSimpleWikiLink(): void {
  const links = extractWikiLinks('see [[My Note]] for details');
  if (links.length !== 1) throw new Error(`expected 1 link, got ${links.length}`);
  if (links[0]!.target !== 'My Note') throw new Error('wrong target');
  if (links[0]!.display !== 'My Note') throw new Error('wrong display');
}

function testExtractWikiLinkWithDisplay(): void {
  const links = extractWikiLinks('see [[target|shown text]] ok');
  if (links.length !== 1) throw new Error('expected 1 link');
  if (links[0]!.target !== 'target') throw new Error('wrong target');
  if (links[0]!.display !== 'shown text') throw new Error('wrong display');
}

function testExtractMultipleWikiLinks(): void {
  const links = extractWikiLinks('[[A]] and [[B]] end');
  if (links.length !== 2) throw new Error('expected 2 links');
  if (links[0]!.target !== 'A') throw new Error('first target wrong');
  if (links[1]!.target !== 'B') throw new Error('second target wrong');
}

function testExtractNoLinks(): void {
  const links = extractWikiLinks('no links here');
  if (links.length !== 0) throw new Error('expected 0 links');
}

function testExtractPositions(): void {
  const links = extractWikiLinks('ab[[cd]]ef');
  if (links[0]!.start !== 2) throw new Error('wrong start');
  if (links[0]!.end !== 8) throw new Error('wrong end');
}

// --- markdownToHtml ---

function testMarkdownHeading(): void {
  const html = markdownToHtml('# Title');
  if (!html.includes('<h1>Title</h1>')) throw new Error('heading not converted');
}

function testMarkdownH3(): void {
  const html = markdownToHtml('### Sub');
  if (!html.includes('<h3>Sub</h3>')) throw new Error('h3 not converted');
}

function testMarkdownBlockquote(): void {
  const html = markdownToHtml('> quoted');
  if (!html.includes('<blockquote>quoted</blockquote>')) throw new Error('blockquote failed');
}

function testMarkdownList(): void {
  const html = markdownToHtml('- item');
  if (!html.includes('<li>item</li>')) throw new Error('list item failed');
}

function testMarkdownWikiLinkRendered(): void {
  const html = markdownToHtml('see [[Note]]');
  if (!html.includes('class="wiki-link"')) throw new Error('wiki-link class missing');
  if (!html.includes('data-target="Note"')) throw new Error('wiki-link data-target missing');
}

function testMarkdownEscapesHtml(): void {
  const html = markdownToHtml('# <script>');
  if (html.includes('<script>')) throw new Error('raw script tag not escaped');
  if (!html.includes('&lt;script&gt;')) throw new Error('escaped entity missing');
}

// --- mapKeyboardShortcut ---

function fakeEvent(key: string, ctrl: boolean): KeyboardEvent {
  return { key, ctrlKey: ctrl, metaKey: false } as unknown as KeyboardEvent;
}

function testShortcutBold(): void {
  if (mapKeyboardShortcut(fakeEvent('b', true)) !== 'bold') throw new Error('Ctrl+B should be bold');
}

function testShortcutItalic(): void {
  if (mapKeyboardShortcut(fakeEvent('i', true)) !== 'italic') throw new Error('Ctrl+I should be italic');
}

function testShortcutWikiLink(): void {
  if (mapKeyboardShortcut(fakeEvent('k', true)) !== 'wiki_link') throw new Error('Ctrl+K should be wiki_link');
}

function testShortcutSave(): void {
  if (mapKeyboardShortcut(fakeEvent('s', true)) !== 'save') throw new Error('Ctrl+S should be save');
}

function testShortcutHeading1(): void {
  if (mapKeyboardShortcut(fakeEvent('1', true)) !== 'heading1') throw new Error('Ctrl+1 should be heading1');
}

function testShortcutNone(): void {
  if (mapKeyboardShortcut(fakeEvent('b', false)) !== null) throw new Error('unmodified b should be null');
}

// --- applyFormatting ---

function testFormatBold(): void {
  const r = applyFormatting('hello world', 6, 11, 'bold');
  if (r.text !== 'hello **world**') throw new Error(`bold: ${r.text}`);
}

function testFormatItalicPlaceholder(): void {
  const r = applyFormatting('abc', 3, 3, 'italic');
  if (r.text !== 'abc*text*') throw new Error(`italic placeholder: ${r.text}`);
}

function testFormatWikiLink(): void {
  const r = applyFormatting('my note', 3, 7, 'wiki_link');
  if (r.text !== 'my [[note]]') throw new Error(`wiki_link: ${r.text}`);
}

function testFormatHeading2(): void {
  const r = applyFormatting('title', 0, 5, 'heading2');
  if (r.text !== '## title') throw new Error(`heading2: ${r.text}`);
}

function testFormatCodeFence(): void {
  const r = applyFormatting('x', 0, 1, 'code_fence');
  if (!r.text.startsWith('```\n')) throw new Error('code_fence start');
  if (!r.text.endsWith('\n```')) throw new Error('code_fence end');
}

// Run all
testExtractSimpleWikiLink();
testExtractWikiLinkWithDisplay();
testExtractMultipleWikiLinks();
testExtractNoLinks();
testExtractPositions();
testMarkdownHeading();
testMarkdownH3();
testMarkdownBlockquote();
testMarkdownList();
testMarkdownWikiLinkRendered();
testMarkdownEscapesHtml();
testShortcutBold();
testShortcutItalic();
testShortcutWikiLink();
testShortcutSave();
testShortcutHeading1();
testShortcutNone();
testFormatBold();
testFormatItalicPlaceholder();
testFormatWikiLink();
testFormatHeading2();
testFormatCodeFence();
console.log('All markdown tests passed (22 assertions)');
